extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use gstd::ActorId;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct StrategyState {
    pub bridge_pid: ActorId,
    pub network_pid: ActorId,
    pub owner: ActorId,
    pub recommendations: Vec<Recommendation>,
    pub total_analyzed: u64,
    pub total_posted: u64,
    pub last_analysis_block: u32,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum HubCmd {
    PostAnnouncement { body: String },
    ChatPost { body: String, mentions: Vec<String> },
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct BridgePriceFeed {
    pub symbol: String,
    pub price_usd_micro: u64,
    pub change_24h_bps: i32,
    pub market_cap_usd: u64,
    pub volume_24h_usd: u64,
    pub updated_at_block: u32,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct BridgeGasFeed {
    pub current_fee_micro: u64,
    pub suggested_tip: u64,
    pub block_num: u32,
    pub finalized_hash: String,
    pub updated_at_block: u32,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct BridgeNewsSummary {
    pub title: String,
    pub source: String,
    pub published_at: u64,
    pub category: String,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct BridgeMarketFeed {
    pub market_id: String,
    pub question: String,
    pub yes_prob_bps: u32,
    pub volume_usd: u64,
    pub closes_at: u64,
    pub updated_at_block: u32,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct BridgeDatetimeFeed {
    pub unix_ts: u64,
    pub utc_string: String,
    pub day_of_week: String,
    pub updated_at_block: u32,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct BridgeSnapshot {
    pub prices: Vec<(String, BridgePriceFeed)>,
    pub gas: BridgeGasFeed,
    pub news: Vec<BridgeNewsSummary>,
    pub markets: Vec<(String, BridgeMarketFeed)>,
    pub datetime: BridgeDatetimeFeed,
    pub block: u32,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct QueryRequest {
    pub query_type: String,
    pub symbol: Option<String>,
    pub keys: Option<Vec<String>>,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum QueryReply {
    Price(Option<BridgePriceFeed>),
    Gas(BridgeGasFeed),
    News(Vec<BridgeNewsSummary>),
    Markets(Vec<BridgeMarketFeed>),
    Datetime(BridgeDatetimeFeed),
    All(BridgeSnapshot),
    Error(String),
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
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

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct PriceEntry {
    pub symbol: String,
    pub price_usd_micro: u64,
    pub change_24h_bps: i32,
    pub volume_24h_usd: u64,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct RecommendationView {
    pub id: u64,
    pub rec_type: String,
    pub title: String,
    pub body: String,
    pub confidence: u32,
    pub created_block: u32,
    pub posted: bool,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct StrategyStats {
    pub total_analyzed: u64,
    pub total_posted: u64,
    pub last_analysis_block: u32,
    pub recommendations_count: u32,
}
