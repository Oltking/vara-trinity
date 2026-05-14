use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use gstd::exec;

use crate::state::{AgentRecord, DataSnapshot};

impl<'a> super::VaraPulseService<'a> {
    pub fn select_nudge_targets(&self) -> Vec<AgentRecord> {
        let current_block = exec::block_height();
        let candidates: Vec<AgentRecord> = {
            let s = self.state.borrow();
            let cooldown = s.nudge_cooldown_blocks;
            let max = s.max_nudges_per_pulse as usize;
            let mut c: Vec<AgentRecord> = s
                .known_agents
                .iter()
                .filter(|a| current_block > a.last_nudged_block + cooldown)
                .cloned()
                .collect();
            c.truncate(max);
            c
        };
        candidates
    }

    pub fn select_nudge_targets_with_data(&self, _data: &DataSnapshot) -> Vec<AgentRecord> {
        self.select_nudge_targets()
    }

    pub fn generate_nudge(&self, data: &DataSnapshot, agent: &AgentRecord) -> String {
        let gas_label = Self::gas_label_static(data.gas_micro);
        format!(
            "Hey @{handle} - VaraBridge just updated - ETH at ${eth}, gas is {gas}.\n\
             Perfect time to run your workflow or try VaraFlow's free automation. \
             One message gets you live data for anything you're building. \
             - VaraPulse",
            handle = agent.handle,
            eth = data.eth_usd / 1_000_000,
            gas = gas_label,
        )
    }

    fn gas_label_static(gas_micro: u64) -> &'static str {
        match gas_micro {
            g if g < 100_000 => "ULTRA LOW",
            g if g < 500_000 => "LOW",
            g if g < 1_000_000 => "MODERATE",
            g if g < 5_000_000 => "HIGH",
            _ => "VERY HIGH",
        }
    }
}
