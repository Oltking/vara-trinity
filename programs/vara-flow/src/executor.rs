use gstd::{exec, msg, MessageId, ToString};

use crate::bridge_client;
use crate::state::{
    Condition, EncodedCall, ExecutionContext, ExecutionStatus, FlowState, HubCmd, PendingStep,
    PulseCmd, QueryReply, Step, StepType, Trigger, TriggerInput,
};

pub fn execute_workflow(state: &mut FlowState, id: u64, input: Option<TriggerInput>) {
    let wf = state.workflows[&id].clone();
    let exec_id = state.next_exec_id;

    let ctx = ExecutionContext {
        exec_id,
        workflow_id: id,
        started_block: exec::block_height(),
        current_step: 0,
        data: input.map(|i| i.data).unwrap_or_default(),
        status: ExecutionStatus::Running,
    };
    state.executions.insert(exec_id, ctx);
    state.next_exec_id += 1;
    state.execution_count += 1;

    execute_step(state, exec_id, &wf.steps[0]);

    let wf_mut = state.workflows.get_mut(&id).unwrap();
    wf_mut.last_run_block = exec::block_height();
    wf_mut.run_count += 1;
    wf_mut.next_run_block = match &wf_mut.trigger {
        Trigger::BlockInterval { every_n_blocks } => exec::block_height() + every_n_blocks,
        _ => exec::block_height() + 1,
    };
}

pub fn execute_step(state: &mut FlowState, exec_id: u64, step: &Step) {
    let network_pid = state.network_pid;
    let bridge_pid = state.bridge_pid;
    let pulse_pid = state.pulse_pid;

    match &step.step_type {
        StepType::QueryBridge { query } => {
            let msg_id = bridge_client::query_bridge(bridge_pid, query.clone(), step.gas_limit);
            state.pending_replies.insert(
                msg_id,
                PendingStep {
                    exec_id,
                    next_step: step.on_success.clone(),
                    timeout_block: exec::block_height() + step.timeout_blocks,
                },
            );
        }
        StepType::CallProgram { pid, method, args } => {
            msg::send(
                *pid,
                EncodedCall {
                    method: method.clone(),
                    args: args.clone(),
                },
                step.gas_limit,
            )
            .expect("CallProgram failed");
            continue_execution(state, exec_id, step.on_success.as_deref());
        }
        StepType::PostBoard { body_template } => {
            let body = render_template(body_template, state, exec_id);
            msg::send(
                network_pid,
                HubCmd::PostAnnouncement { body },
                step.gas_limit,
            )
            .expect("PostBoard failed");
            continue_execution(state, exec_id, step.on_success.as_deref());
        }
        StepType::PostChat {
            body_template,
            mentions,
        } => {
            let body = render_template(body_template, state, exec_id);
            msg::send(
                network_pid,
                HubCmd::ChatPost {
                    body,
                    mentions: mentions.clone(),
                },
                step.gas_limit,
            )
            .expect("PostChat failed");
            continue_execution(state, exec_id, step.on_success.as_deref());
        }
        StepType::ConditionalBranch {
            condition,
            if_true,
            if_false,
        } => {
            if evaluate_condition(condition, state, exec_id) {
                execute_step(state, exec_id, if_true);
            } else if let Some(false_step) = if_false {
                execute_step(state, exec_id, false_step);
            } else {
                complete_execution(state, exec_id);
            }
        }
        StepType::WakePulse => {
            msg::send(pulse_pid, PulseCmd::Run, step.gas_limit).expect("WakePulse failed");
            continue_execution(state, exec_id, step.on_success.as_deref());
        }
        StepType::Done => complete_execution(state, exec_id),
    }
}

pub fn handle_bridge_reply(state: &mut FlowState, msg_id: MessageId, reply: QueryReply) {
    if let Some(pending) = state.pending_replies.remove(&msg_id) {
        bridge_client::update_cache(&mut state.cached_bridge_data, &reply);
        if let Some(next) = pending.next_step {
            execute_step(state, pending.exec_id, &next);
        } else {
            complete_execution(state, pending.exec_id);
        }
    }
}

fn continue_execution(state: &mut FlowState, exec_id: u64, next_step: Option<&Step>) {
    match next_step {
        Some(step) => execute_step(state, exec_id, step),
        None => complete_execution(state, exec_id),
    }
}

fn complete_execution(state: &mut FlowState, exec_id: u64) {
    if let Some(ctx) = state.executions.get_mut(&exec_id) {
        ctx.status = ExecutionStatus::Completed;
    }
}

fn evaluate_condition(condition: &Condition, state: &FlowState, _exec_id: u64) -> bool {
    let cache = &state.cached_bridge_data;

    match condition {
        Condition::PriceAbove {
            symbol,
            threshold_micro,
        } => bridge_client::get_cached_price(cache, symbol)
            .map(|p| p >= *threshold_micro)
            .unwrap_or(false),

        Condition::PriceBelow {
            symbol,
            threshold_micro,
        } => bridge_client::get_cached_price(cache, symbol)
            .map(|p| p <= *threshold_micro)
            .unwrap_or(false),

        Condition::GasBelow { threshold_micro } => {
            bridge_client::get_cached_gas(cache) <= *threshold_micro
        }

        Condition::BlockModulo { n } => exec::block_height() % n == 0,

        Condition::Always => true,
    }
}

fn render_template(template: &str, state: &FlowState, exec_id: u64) -> alloc::string::String {
    let data = &state
        .executions
        .get(&exec_id)
        .map(|ctx| &ctx.data)
        .cloned()
        .unwrap_or_default();

    let mut result = template.to_string();
    for (key, value) in data {
        let placeholder = alloc::format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}
