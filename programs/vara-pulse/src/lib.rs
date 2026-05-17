#![no_std]

extern crate alloc;

use alloc::{string::String, vec, vec::Vec};
use core::cell::RefCell;

pub mod broadcast;
pub mod generator;
pub mod nudger;
pub mod state;
pub mod templates;

use gstd::{exec, msg, ActorId};
use sails_rs::prelude::*;
use sails_rs::scale_codec::{Decode, Encode};

use state::*;
use broadcast::{post_to_board, post_to_chat};

pub const GAS_FOR_HUB: u128 = 2_000_000_000;
pub const GAS_FOR_STRATEGY: u128 = 3_000_000_000;
pub const GAS_FOR_FLOW: u128 = 2_000_000_000;
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
        let (data, price_feed_data) = self.extract_snapshot(reply);
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

        drop(s);

        if !price_feed_data.is_empty() {
            let strategy_pid = self.state.borrow().strategy_pid;
            if strategy_pid != ActorId::zero() {
                let srv = "VaraStrategy".encode();
                let mtd = "Analyze".encode();
                let params = price_feed_data.encode();
                let mut payload = Vec::with_capacity(srv.len() + mtd.len() + params.len());
                payload.extend(srv);
                payload.extend(mtd);
                payload.extend(params);
                let _ = msg::send_bytes_with_gas(strategy_pid, &payload, 10_000_000, 0);
            }
        }

        let flow_pid = self.state.borrow().flow_pid;
        if flow_pid != ActorId::zero() {
            let srv = "VaraFlow".encode();
            let mtd = "Tick".encode();
            let mut payload = Vec::with_capacity(srv.len() + mtd.len());
            payload.extend(srv);
            payload.extend(mtd);
            let _ = msg::send_bytes_with_gas(flow_pid, &payload, 5_000_000, 0);
        }
    }
}

#[program]
impl VaraPulseProgram {
    pub fn new(bridge_pid: ActorId, flow_pid: ActorId, network_pid: ActorId, strategy_pid: ActorId) -> Self {
        let owner = msg::source();
        Self {
            state: RefCell::new(PulseState {
                bridge_pid,
                flow_pid,
                network_pid,
                strategy_pid,
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
        let raw: Vec<u8> = match gstd::msg::load() {
            Ok(bytes) => bytes,
            Err(_) => return,
        };
        let reply: QueryReply = Decode::decode(&mut &raw[..])
            .unwrap_or_else(|_| {
                let mut cursor = &raw[..];
                let _: Result<String, _> = Decode::decode(&mut cursor);
                let _: Result<String, _> = Decode::decode(&mut cursor);
                Decode::decode(&mut cursor).unwrap_or_else(|_| {
                    QueryReply::Error("decode failed".into())
                })
            });
        if let QueryReply::Error(_) = reply { return; }
        let mut service = VaraPulseService::new(&self.state);
        service.on_bridge_reply(reply);
    }
}

#[service]
impl VaraPulseService<'_> {
    #[export]
    pub fn run(&mut self) {
        let gas_total = exec::gas_available();
        if gas_total < 500_000_000 {
            return;
        }
        let bridge_pid = self.state.borrow().bridge_pid;
        let service_route = "VaraBridge".encode();
        let method_route = "QueryAndReply".encode();
        let params = QueryRequest {
            query_type: "all".into(),
            symbol: None,
            keys: None,
        }
        .encode();
        let mut payload = Vec::with_capacity(
            service_route.len() + method_route.len() + params.len(),
        );
        payload.extend(service_route);
        payload.extend(method_route);
        payload.extend(params);
        let gas = gas_total * 60 / 100;
        let msg_id = msg::send_bytes_with_gas(bridge_pid, &payload, gas, 0)
            .expect("Bridge query failed");
        exec::reply_deposit(msg_id, gas_total * 15 / 100)
            .expect("Failed to set reply deposit");

        let route = ["VaraPulse".encode(), "Run".encode()].concat();
        msg::send_bytes_with_gas_delayed(
            exec::program_id(),
            &route,
            exec::gas_available() * 80 / 100,
            0,
            300,
        )
        .expect("Failed to reschedule pulse");
    }

    #[export]
    pub fn start_auto(&mut self) {
        let caller = msg::source();
        let owner = self.state.borrow().owner;
        if caller != owner {
            panic!("Owner only");
        }
        let route = ["VaraPulse".encode(), "Run".encode()].concat();
        msg::send_bytes_with_gas_delayed(
            exec::program_id(),
            &route,
            exec::gas_available() * 80 / 100,
            0,
            300,
        )
        .expect("Failed to start auto pulse loop");
    }

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
    pub fn withdraw(&mut self, amount: u128) {
        let caller = msg::source();
        let owner = self.state.borrow().owner;
        if caller != owner {
            panic!("Owner only");
        }
        let val = msg::send(caller, b"", amount).expect("Withdraw failed");
        let _ = val;
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
