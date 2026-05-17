extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use gstd::ActorId;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct PulseState {
    pub bridge_pid: ActorId,
    pub flow_pid: ActorId,
    pub network_pid: ActorId,
    pub strategy_pid: ActorId,
    pub owner: ActorId,
    pub pulse_history: Vec<PulseRecord>,
    pub known_agents: Vec<AgentRecord>,
    pub last_catalog_refresh_block: u32,
    pub pulse_interval_blocks: u32,
    pub max_nudges_per_pulse: u32,
    pub nudge_cooldown_blocks: u32,
    pub total_pulses: u64,
    pub total_nudges_sent: u64,
    pub total_board_posts: u64,
    pub last_pulse_block: u32,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct PulseRecord {
    pub pulse_id: u64,
    pub block: u32,
    pub pulse_type: PulseType,
    pub body: String,
    pub data_snapshot: DataSnapshot,
    pub nudges_sent: Vec<String>,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum PulseType {
    MarketSummary,
    GasTip,
    NewsBrief,
    MarketSpark,
    AgentTip,
    MilestonePost,
    CreativeSpark,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct AgentRecord {
    pub handle: String,
    pub program_id: String,
    pub description: String,
    pub last_nudged_block: u32,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct DataSnapshot {
    pub eth_usd: u64,
    pub btc_usd: u64,
    pub vara_usd: u64,
    pub gas_micro: u64,
    pub top_news: String,
    pub top_market: Option<String>,
    pub block: u32,
    pub utc_string: String,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct PulseConfig {
    pub interval: Option<u32>,
    pub max_nudges: Option<u32>,
    pub cooldown: Option<u32>,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct PulseStats {
    pub total_pulses: u64,
    pub total_nudges: u64,
    pub total_board_posts: u64,
    pub last_pulse_block: u32,
    pub known_agents: u32,
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
    Price(Option<PriceFeed>),
    Gas(GasFeed),
    News(Vec<NewsSummary>),
    Markets(Vec<MarketFeed>),
    Datetime(DatetimeFeed),
    All(Snapshot),
    Error(String),
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct PriceFeed {
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
pub struct GasFeed {
    pub current_fee_micro: u64,
    pub suggested_tip: u64,
    pub block_num: u32,
    pub finalized_hash: String,
    pub updated_at_block: u32,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct NewsSummary {
    pub title: String,
    pub source: String,
    pub published_at: u64,
    pub category: String,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct MarketFeed {
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
pub struct DatetimeFeed {
    pub unix_ts: u64,
    pub utc_string: String,
    pub day_of_week: String,
    pub updated_at_block: u32,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Snapshot {
    pub prices: Vec<(String, PriceFeed)>,
    pub gas: GasFeed,
    pub news: Vec<NewsSummary>,
    pub markets: Vec<(String, MarketFeed)>,
    pub datetime: DatetimeFeed,
    pub block: u32,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct StrategyPriceEntry {
    pub symbol: String,
    pub price_usd_micro: u64,
    pub change_24h_bps: i32,
    pub volume_24h_usd: u64,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum HubCmd {
    PostAnnouncement { body: String },
    ChatPost { body: String, mentions: Vec<String> },
}

impl Default for PulseState {
    fn default() -> Self {
        Self {
            bridge_pid: ActorId::zero(),
            flow_pid: ActorId::zero(),
            network_pid: ActorId::zero(),
            strategy_pid: ActorId::zero(),
            owner: ActorId::zero(),
            pulse_history: Vec::new(),
            known_agents: Vec::new(),
            last_catalog_refresh_block: 0,
            pulse_interval_blocks: 300,
            max_nudges_per_pulse: 3,
            nudge_cooldown_blocks: 3000,
            total_pulses: 0,
            total_nudges_sent: 0,
            total_board_posts: 0,
            last_pulse_block: 0,
        }
    }
}
