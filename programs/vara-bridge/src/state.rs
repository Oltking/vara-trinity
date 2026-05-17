extern crate alloc;

use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;
use gstd::collections::BTreeMap;
use gstd::ActorId;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct BridgeState {
    pub prices: BTreeMap<String, PriceFeed>,
    pub gas: GasFeed,
    pub news: VecDeque<NewsSummary>,
    pub markets: BTreeMap<String, MarketFeed>,
    pub datetime: DatetimeFeed,
    pub feeder_address: ActorId,
    pub owner: ActorId,
    pub network_pid: ActorId,
    pub last_updated_block: u32,
    pub last_broadcast_block: u32,
    pub query_count: u64,
    pub update_count: u64,
    pub broadcast_count: u64,
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
pub struct FullUpdatePayload {
    pub prices: Option<Vec<PriceFeed>>,
    pub gas: Option<GasFeed>,
    pub news: Option<Vec<NewsSummary>>,
    pub markets: Option<Vec<MarketFeed>>,
    pub datetime: Option<DatetimeFeed>,
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
    All(BridgeSnapshot),
    Error(String),
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct BridgeSnapshot {
    pub prices: BTreeMap<String, PriceFeed>,
    pub gas: GasFeed,
    pub news: Vec<NewsSummary>,
    pub markets: BTreeMap<String, MarketFeed>,
    pub datetime: DatetimeFeed,
    pub block: u32,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct BridgeStats {
    pub query_count: u64,
    pub update_count: u64,
    pub broadcast_count: u64,
    pub last_updated_block: u32,
    pub symbols_tracked: u32,
}

impl Default for BridgeState {
    fn default() -> Self {
        Self {
            prices: BTreeMap::new(),
            gas: GasFeed::default(),
            news: VecDeque::new(),
            markets: BTreeMap::new(),
            datetime: DatetimeFeed::default(),
            feeder_address: ActorId::zero(),
            owner: ActorId::zero(),
            network_pid: ActorId::zero(),
            last_updated_block: 0,
            last_broadcast_block: 0,
            query_count: 0,
            update_count: 0,
            broadcast_count: 0,
        }
    }
}

impl Default for GasFeed {
    fn default() -> Self {
        Self { current_fee_micro: 0, suggested_tip: 0, block_num: 0, finalized_hash: String::new(), updated_at_block: 0 }
    }
}

impl Default for DatetimeFeed {
    fn default() -> Self {
        Self { unix_ts: 0, utc_string: String::new(), day_of_week: String::new(), updated_at_block: 0 }
    }
}

pub const MAX_NEWS: usize = 10;

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
pub struct InitConfig {
    pub feeder_address: ActorId,
    pub network_pid: ActorId,
}
