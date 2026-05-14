#![no_std]

extern crate alloc;

use core::cell::RefCell;

pub mod bridge_client;
pub mod broadcast;
pub mod executor;
pub mod state;
pub mod templates;
pub mod workflow;

use gstd::collections::BTreeMap;
use gstd::{exec, msg, ActorId};
use sails_rs::prelude::*;
use sails_rs::scale_codec::Encode;

use state::*;
use templates::{build_template, TemplateParams, TemplateType};

pub const TICK_INTERVAL: u32 = 50;

pub struct VaraFlowProgram {
    state: RefCell<FlowState>,
}

pub struct VaraFlowService<'a> {
    state: &'a RefCell<FlowState>,
}

impl<'a> VaraFlowService<'a> {
    pub fn new(state: &'a RefCell<FlowState>) -> Self {
        Self { state }
    }
}

#[program]
impl VaraFlowProgram {
    pub fn new(bridge_pid: ActorId, pulse_pid: ActorId, network_pid: ActorId) -> Self {
        let owner = msg::source();

        let this = Self {
            state: RefCell::new(FlowState {
                workflows: BTreeMap::new(),
                next_workflow_id: 0,
                executions: BTreeMap::new(),
                next_exec_id: 0,
                pending_replies: BTreeMap::new(),
                bridge_pid,
                pulse_pid,
                network_pid,
                owner,
                execution_count: 0,
                workflow_count: 0,
                broadcast_count: 0,
                cached_bridge_data: None,
            }),
        };

        this
    }

    pub fn vara_flow(&self) -> VaraFlowService<'_> {
        VaraFlowService::new(&self.state)
    }
}

#[service]
impl VaraFlowService<'_> {
    #[export]
    pub fn register_workflow(&mut self, input: WorkflowInput) -> u64 {
        let mut s = self.state.borrow_mut();
        workflow::register_workflow(&mut *s, input)
    }

    #[export]
    pub fn update_workflow(&mut self, id: u64, patch: WorkflowPatch) {
        let mut s = self.state.borrow_mut();
        workflow::update_workflow(&mut *s, id, patch);
    }

    #[export]
    pub fn delete_workflow(&mut self, id: u64) {
        let mut s = self.state.borrow_mut();
        workflow::delete_workflow(&mut *s, id);
    }

    #[export]
    pub fn pause_workflow(&mut self, id: u64) {
        let mut s = self.state.borrow_mut();
        workflow::pause_workflow(&mut *s, id);
    }

    #[export]
    pub fn resume_workflow(&mut self, id: u64) {
        let mut s = self.state.borrow_mut();
        workflow::resume_workflow(&mut *s, id);
    }

    #[export]
    pub fn get_workflow(&self, id: u64) -> Option<Workflow> {
        let s = self.state.borrow();
        workflow::get_workflow(&*s, id)
    }

    #[export]
    pub fn list_workflows(
        &self,
        owner: Option<ActorId>,
        active_only: bool,
    ) -> Vec<WorkflowSummary> {
        let s = self.state.borrow();
        workflow::list_workflows(&*s, owner, active_only)
    }

    #[export]
    pub fn trigger_workflow(&mut self, id: u64, input: Option<TriggerInput>) {
        let s = self.state.borrow();
        let wf = s
            .workflows
            .get(&id)
            .cloned()
            .expect("Workflow not found");
        if let Trigger::ManualCall { authorized } = &wf.trigger {
            if let Some(auth) = authorized {
                assert_eq!(msg::source(), *auth, "Not authorized");
            }
        }
        drop(s);

        let mut s = self.state.borrow_mut();
        executor::execute_workflow(&mut *s, id, input);
    }

    #[export]
    pub fn tick(&mut self) {
        let current_block = exec::block_height();
        let mut s = self.state.borrow_mut();

        let triggered_ids: Vec<u64> = s
            .workflows
            .values()
            .filter(|wf| wf.active && current_block >= wf.next_run_block)
            .filter(|wf| is_triggered(&*s, wf, current_block))
            .map(|wf| wf.id)
            .collect();

        for id in triggered_ids {
            executor::execute_workflow(&mut *s, id, None);
        }

        if current_block % 200 == 100 {
            broadcast::do_broadcast(&*s);
        }
        drop(s);

        let route = ["VaraFlow".encode(), "Tick".encode()].concat();
        msg::send_bytes_with_gas_delayed(
            exec::program_id(),
            &route,
            exec::gas_available() * 80 / 100,
            0,
            50,
        )
        .expect("Failed to reschedule tick");

    }

    #[export]
    pub fn get_execution_status(&self, exec_id: u64) -> Option<ExecutionStatus> {
        let s = self.state.borrow();
        s.executions
            .get(&exec_id)
            .map(|ctx| ctx.status.clone())
    }

    #[export]
    pub fn get_stats(&self) -> FlowStats {
        let s = self.state.borrow();
        let active_count = s
            .workflows
            .values()
            .filter(|wf| wf.active)
            .count() as u64;
        FlowStats {
            workflow_count: s.workflow_count,
            execution_count: s.execution_count,
            broadcast_count: s.broadcast_count,
            active_workflows: active_count,
        }
    }

    #[export]
    pub fn use_template(
        &mut self,
        template: TemplateType,
        params: TemplateParams,
    ) -> u64 {
        let input = build_template(template, params);
        let mut s = self.state.borrow_mut();
        workflow::register_workflow(&mut *s, input)
    }

    #[export]
    pub fn start_tick(&mut self) {
        let s = self.state.borrow();
        assert_eq!(msg::source(), s.owner, "Owner only");
        drop(s);
        let route = ["VaraFlow".encode(), "Tick".encode()].concat();
        msg::send_bytes_with_gas_delayed(
            exec::program_id(),
            &route,
            exec::gas_available() * 80 / 100,
            0,
            50,
        )
        .expect("Failed to start tick loop");
    }

    pub fn set_bridge(&mut self, bridge_pid: ActorId) {
        let s = self.state.borrow();
        assert_eq!(msg::source(), s.owner, "Owner only");
        drop(s);
        let mut s = self.state.borrow_mut();
        s.bridge_pid = bridge_pid;
    }

    #[export]
    pub fn set_pulse(&mut self, pulse_pid: ActorId) {
        let s = self.state.borrow();
        assert_eq!(msg::source(), s.owner, "Owner only");
        drop(s);
        let mut s = self.state.borrow_mut();
        s.pulse_pid = pulse_pid;
    }
}

fn is_triggered(state: &FlowState, wf: &Workflow, block: u32) -> bool {
    match &wf.trigger {
        Trigger::BlockInterval { every_n_blocks } => {
            block >= wf.last_run_block + every_n_blocks
        }
        Trigger::PriceThreshold {
            symbol,
            above_usd,
            below_usd,
        } => {
            if let Some(price) = bridge_client::get_cached_price(&state.cached_bridge_data, symbol)
            {
                above_usd.map_or(true, |t| price >= t)
                    && below_usd.map_or(true, |t| price <= t)
            } else {
                false
            }
        }
        Trigger::GasBelow { threshold_micro } => {
            bridge_client::get_cached_gas(&state.cached_bridge_data) <= *threshold_micro
        }
        Trigger::ManualCall { .. } => false,
        Trigger::OnBridgeUpdate => {
            state
                .cached_bridge_data
                .as_ref()
                .map(|c| c.updated_at_block > wf.last_run_block)
                .unwrap_or(false)
        }
    }
}
