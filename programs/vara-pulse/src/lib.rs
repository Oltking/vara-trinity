#![no_std]

extern crate alloc;

use alloc::{vec, vec::Vec};
use core::cell::RefCell;

pub mod broadcast;
pub mod generator;
pub mod nudger;
pub mod state;
pub mod templates;

use gstd::{exec, msg, ActorId};
use sails_rs::prelude::*;

use state::*;
use broadcast::{post_to_board, post_to_chat};

pub const GAS_FOR_BRIDGE_QUERY: u128 = 5_000_000_000;
pub const GAS_FOR_HUB: u128 = 2_000_000_000;
pub const GAS_RESERVE_TOTAL: u64 = 250_000_000_000_000;
pub const GAS_RESERVE_DURATION: u32 = 1_000_000;

pub struct VaraPulseProgram {
    state: RefCell<PulseState>,
}

pub struct VaraPulseService<'a> {
    state: &'a RefCell<PulseState>,
}

impl<'a> VaraPulseService<'a> {
    pub fn new(state: &'a RefCell<PulseState>) -> Self {
        Self { state }
    }

    pub fn on_bridge_reply(&mut self, reply: QueryReply) {
        let data = self.extract_snapshot(reply);
        let pulse_type = self.pick_pulse_type(&data);
        let body = self.generate_pulse(&data, &pulse_type);

        let network_pid = self.state.borrow().network_pid;
        post_to_board(network_pid, body.clone());
        self.state.borrow_mut().total_board_posts += 1;

        post_to_chat(network_pid, body.clone(), vec![]);
        self.state.borrow_mut().total_board_posts += 1;

        let nudge_targets = self.select_nudge_targets_with_data(&data);
        for target in &nudge_targets {
            let nudge = self.generate_nudge(&data, target);
            post_to_chat(network_pid, nudge, vec![target.handle.clone()]);
            self.state.borrow_mut().total_nudges_sent += 1;
        }

        let pulse_id = self.state.borrow().total_pulses;
        let record = PulseRecord {
            pulse_id,
            block: exec::block_height(),
            pulse_type,
            body,
            data_snapshot: data,
            nudges_sent: nudge_targets.iter().map(|a| a.handle.clone()).collect(),
        };

        let mut s = self.state.borrow_mut();
        if s.pulse_history.len() >= 50 {
            s.pulse_history.remove(0);
        }
        s.pulse_history.push(record);
        s.total_pulses += 1;
        s.last_pulse_block = exec::block_height();
    }
}

#[program]
impl VaraPulseProgram {
    pub fn new(bridge_pid: ActorId, flow_pid: ActorId, network_pid: ActorId) -> Self {
        let owner = msg::source();
        Self {
            state: RefCell::new(PulseState {
                bridge_pid,
                flow_pid,
                network_pid,
                owner,
                ..Default::default()
            }),
        }
    }

    pub fn vara_pulse(&self) -> VaraPulseService<'_> {
        VaraPulseService::new(&self.state)
    }

    #[handle_reply]
    fn handle_bridge_reply(&self) {
        let raw = gstd::msg::load_bytes().expect("Failed to load reply");
        let offset = if raw.len() >= 16 {
            sails_rs::meta::SailsMessageHeader::try_from_bytes(&raw[..16])
                .ok()
                .map(|h| h.hlen().inner() as usize)
                .unwrap_or(0)
        } else {
            0
        };
        let reply: QueryReply = Decode::decode(&mut &raw[offset..]).expect("Bad bridge reply");
        let mut service = VaraPulseService::new(&self.state);
        service.on_bridge_reply(reply);
    }
}

#[service]
impl VaraPulseService<'_> {
    #[export]
    pub fn run(&mut self) {
        let bridge_pid = self.state.borrow().bridge_pid;
        msg::send(
            bridge_pid,
            QueryRequest {
                query_type: "all".into(),
                symbol: None,
                keys: None,
            },
            GAS_FOR_BRIDGE_QUERY,
        )
        .expect("Bridge query failed");
    }

    #[export]
    pub fn force_pulse(&mut self) {
        let caller = msg::source();
        let owner = self.state.borrow().owner;
        if caller != owner {
            panic!("Owner only");
        }
        self.run();
    }

    #[export]
    pub fn update_catalog(&mut self, agents: Vec<AgentRecord>) {
        let caller = msg::source();
        let owner = self.state.borrow().owner;
        if caller != owner {
            panic!("Owner only");
        }
        self.state.borrow_mut().known_agents = agents;
        self.state.borrow_mut().last_catalog_refresh_block = exec::block_height();
    }

    #[export]
    pub fn set_config(&mut self, config: PulseConfig) {
        let caller = msg::source();
        let owner = self.state.borrow().owner;
        if caller != owner {
            panic!("Owner only");
        }
        let mut s = self.state.borrow_mut();
        if let Some(interval) = config.interval {
            s.pulse_interval_blocks = interval;
        }
        if let Some(max_nudges) = config.max_nudges {
            s.max_nudges_per_pulse = max_nudges;
        }
        if let Some(cooldown) = config.cooldown {
            s.nudge_cooldown_blocks = cooldown;
        }
    }

    #[export]
    pub fn get_latest_pulse(&self) -> Option<PulseRecord> {
        self.state.borrow().pulse_history.last().cloned()
    }

    #[export]
    pub fn get_pulse_history(&self, limit: u32) -> Vec<PulseRecord> {
        self.state
            .borrow()
            .pulse_history
            .iter()
            .rev()
            .take(limit as usize)
            .cloned()
            .collect()
    }

    #[export]
    pub fn get_stats(&self) -> PulseStats {
        let s = self.state.borrow();
        PulseStats {
            total_pulses: s.total_pulses,
            total_nudges: s.total_nudges_sent,
            total_board_posts: s.total_board_posts,
            last_pulse_block: s.last_pulse_block,
            known_agents: s.known_agents.len() as u32,
        }
    }
}
