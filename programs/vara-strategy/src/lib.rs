#![no_std]

extern crate alloc;

use alloc::format;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

pub mod state;

use gstd::{exec, msg, ActorId};
use sails_rs::prelude::*;

use state::*;

pub struct VaraStrategyProgram {
    state: RefCell<StrategyState>,
}

pub struct VaraStrategyService<'a> {
    state: &'a RefCell<StrategyState>,
}

impl<'a> VaraStrategyService<'a> {
    pub fn new(state: &'a RefCell<StrategyState>) -> Self {
        Self { state }
    }
}

#[program]
impl VaraStrategyProgram {
    pub fn new(bridge_pid: ActorId, network_pid: ActorId) -> Self {
        let owner = msg::source();
        Self {
            state: RefCell::new(StrategyState {
                bridge_pid,
                network_pid,
                owner,
                recommendations: vec![],
                total_analyzed: 0,
                total_posted: 0,
                last_analysis_block: 0,
            }),
        }
    }

    pub fn vara_strategy(&self) -> VaraStrategyService<'_> {
        VaraStrategyService::new(&self.state)
    }
}

#[service]
impl VaraStrategyService<'_> {
    #[export]
    pub fn analyze(&mut self, prices: Vec<PriceEntry>) -> Vec<RecommendationView> {
        let block = exec::block_height();
        let mut s = self.state.borrow_mut();
        s.last_analysis_block = block;
        s.total_analyzed += 1;

        let mut new_recs: Vec<Recommendation> = vec![];
        let mut sorted: Vec<&PriceEntry> = prices.iter().collect();

        sorted.sort_by(|a, b| b.change_24h_bps.cmp(&a.change_24h_bps));
        if let Some(top) = sorted.first() {
            if top.change_24h_bps > 50 {
                let pct = top.change_24h_bps / 100;
                new_recs.push(Recommendation {
                    id: s.total_analyzed, rec_type: "momentum".into(),
                    symbol: Some(top.symbol.clone()),
                    title: format!("{} up {}%", top.symbol, pct),
                    body: format!("{} gained {}% in 24h. Vol ${}B.", top.symbol, pct, top.volume_24h_usd / 1_000_000_000),
                    confidence: (top.change_24h_bps.abs() as u32).min(85),
                    created_block: block, posted: false,
                });
            }
        }

        sorted.sort_by(|a, b| a.change_24h_bps.cmp(&b.change_24h_bps));
        if let Some(bottom) = sorted.first() {
            if bottom.change_24h_bps < -50 {
                let pct = bottom.change_24h_bps.abs() / 100;
                new_recs.push(Recommendation {
                    id: s.total_analyzed + 1, rec_type: "value".into(),
                    symbol: Some(bottom.symbol.clone()),
                    title: format!("{} down {}%", bottom.symbol, pct),
                    body: format!("{} dropped {}% in 24h. Vol ${}B.", bottom.symbol, pct, bottom.volume_24h_usd / 1_000_000_000),
                    confidence: (bottom.change_24h_bps.abs() as u32 / 2).min(60),
                    created_block: block, posted: false,
                });
            }
        }

        for rec in &new_recs { s.recommendations.push(rec.clone()); }
        new_recs.iter().map(|r| RecommendationView {
            id: r.id, rec_type: r.rec_type.clone(), title: r.title.clone(),
            body: r.body.clone(), confidence: r.confidence,
            created_block: r.created_block, posted: r.posted,
        }).collect()
    }

    #[export]
    pub fn get_recommendations(&self, limit: u32) -> Vec<RecommendationView> {
        let s = self.state.borrow();
        s.recommendations.iter().rev().take(limit as usize).map(|r| RecommendationView {
            id: r.id, rec_type: r.rec_type.clone(), title: r.title.clone(),
            body: r.body.clone(), confidence: r.confidence,
            created_block: r.created_block, posted: r.posted,
        }).collect()
    }

    #[export]
    pub fn mark_posted(&mut self, rec_id: u64) {
        let caller = msg::source();
        let mut s = self.state.borrow_mut();
        if caller != s.owner { panic!("Owner only"); }
        if let Some(rec) = s.recommendations.iter_mut().find(|r| r.id == rec_id) {
            rec.posted = true;
            s.total_posted += 1;
        }
    }

    #[export]
    pub fn get_stats(&self) -> StrategyStats {
        let s = self.state.borrow();
        StrategyStats {
            total_analyzed: s.total_analyzed,
            total_posted: s.total_posted,
            last_analysis_block: s.last_analysis_block,
            recommendations_count: s.recommendations.len() as u32,
        }
    }
}
