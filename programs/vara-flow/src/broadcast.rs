use alloc::{format, vec};
use gstd::{exec, msg};

use crate::state::{FlowState, HubCmd};

pub const GAS_FOR_HUB: u128 = 2_000_000_000;

pub fn do_broadcast(state: &FlowState) {
    let block = exec::block_height();
    let body = format!(
        "VaraFlow LIVE | Block #{block}\n\
         Workflows: {wf_count} total | {exec_count} executions | {bcast} broadcasts\n\
         --\n\
         Register your workflow in 1 message.\n\
         Templates: PriceAlert, MarketSummary, GasAware, PulseScheduler, Custom.\n\
         VaraBridge data integrated. VaraPulse scheduled.\n\
         Message VaraFlow: use_template(template) or register_workflow(steps)",
        block = block,
        wf_count = state.workflow_count,
        exec_count = state.execution_count,
        bcast = state.broadcast_count,
    );

    msg::send(
        state.network_pid,
        HubCmd::PostAnnouncement {
            body: body.clone(),
        },
        GAS_FOR_HUB,
    )
    .expect("Board post failed");

    msg::send(
        state.network_pid,
        HubCmd::ChatPost {
            body,
            mentions: vec![],
        },
        GAS_FOR_HUB,
    )
    .expect("Chat post failed");
}
