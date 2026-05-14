use alloc::vec::Vec;
use gstd::{exec, msg, ActorId};

use crate::state::{
    FlowState, Trigger, Workflow, WorkflowInput, WorkflowPatch, WorkflowSummary,
};

pub fn register_workflow(state: &mut FlowState, input: WorkflowInput) -> u64 {
    let id = state.next_workflow_id;
    let block = exec::block_height();
    let next_run = match &input.trigger {
        Trigger::BlockInterval { every_n_blocks } => block + every_n_blocks,
        _ => block + 1,
    };

    state.workflows.insert(
        id,
        Workflow {
            id,
            owner: msg::source(),
            name: input.name,
            description: input.description,
            trigger: input.trigger,
            steps: input.steps,
            active: true,
            created_block: block,
            last_run_block: 0,
            run_count: 0,
            next_run_block: next_run,
        },
    );
    state.next_workflow_id += 1;
    state.workflow_count += 1;
    id
}

pub fn update_workflow(state: &mut FlowState, id: u64, patch: WorkflowPatch) {
    let caller = msg::source();
    let wf = state
        .workflows
        .get_mut(&id)
        .expect("Workflow not found");
    assert_eq!(wf.owner, caller, "Only workflow owner can update");

    if let Some(name) = patch.name {
        wf.name = name;
    }
    if let Some(description) = patch.description {
        wf.description = description;
    }
    if let Some(trigger) = patch.trigger {
        wf.trigger = trigger;
    }
    if let Some(steps) = patch.steps {
        wf.steps = steps;
    }
    if let Some(active) = patch.active {
        wf.active = active;
    }
}

pub fn delete_workflow(state: &mut FlowState, id: u64) {
    let caller = msg::source();
    let wf = state.workflows.get(&id).expect("Workflow not found");
    assert_eq!(wf.owner, caller, "Only workflow owner can delete");
    state.workflows.remove(&id);
}

pub fn pause_workflow(state: &mut FlowState, id: u64) {
    let caller = msg::source();
    let wf = state.workflows.get_mut(&id).expect("Workflow not found");
    assert_eq!(wf.owner, caller, "Only workflow owner can pause");
    wf.active = false;
}

pub fn resume_workflow(state: &mut FlowState, id: u64) {
    let caller = msg::source();
    let wf = state.workflows.get_mut(&id).expect("Workflow not found");
    assert_eq!(wf.owner, caller, "Only workflow owner can resume");
    wf.active = true;
    wf.next_run_block = exec::block_height() + match &wf.trigger {
        Trigger::BlockInterval { every_n_blocks } => *every_n_blocks,
        _ => 1,
    };
}

pub fn get_workflow(state: &FlowState, id: u64) -> Option<Workflow> {
    state.workflows.get(&id).cloned()
}

pub fn list_workflows(
    state: &FlowState,
    owner: Option<ActorId>,
    active_only: bool,
) -> Vec<WorkflowSummary> {
    state
        .workflows
        .values()
        .filter(|wf| {
            let owner_match = owner.map_or(true, |o| wf.owner == o);
            let active_match = !active_only || wf.active;
            owner_match && active_match
        })
        .map(|wf| WorkflowSummary {
            id: wf.id,
            name: wf.name.clone(),
            trigger: wf.trigger.clone(),
            active: wf.active,
            run_count: wf.run_count,
            last_run_block: wf.last_run_block,
            next_run_block: wf.next_run_block,
        })
        .collect()
}
