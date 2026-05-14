extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use gstd::ActorId;
use sails_rs::prelude::*;

#[derive(Clone, Debug)]
#[sails_type]
pub struct StrategyState {
    pub bridge_pid: ActorId,
    pub network_pid: ActorId,
    pub owner: ActorId,
    pub recommendations: Vec<Recommendation>,
    pub total_analyzed: u64,
    pub total_posted: u64,
    pub last_analysis_block: u32,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct Recommendation {
    pub id: u64,
    pub rec_type: String,
    pub symbol: Option<String>,
    pub title: String,
    pub body: String,
    pub confidence: u32,
    pub created_block: u32,
    pub posted: bool,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct PriceEntry {
    pub symbol: String,
    pub price_usd_micro: u64,
    pub change_24h_bps: i32,
    pub volume_24h_usd: u64,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct RecommendationView {
    pub id: u64,
    pub rec_type: String,
    pub title: String,
    pub body: String,
    pub confidence: u32,
    pub created_block: u32,
    pub posted: bool,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct StrategyStats {
    pub total_analyzed: u64,
    pub total_posted: u64,
    pub last_analysis_block: u32,
    pub recommendations_count: u32,
}
