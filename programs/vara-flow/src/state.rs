use gstd::collections::BTreeMap;
use gstd::{ActorId, MessageId};
use sails_rs::prelude::*;

use crate::bridge_client::CachedBridgeData;

#[derive(Clone, Debug)]
pub struct StepBox(pub Box<Step>);

impl core::ops::Deref for StepBox {
    type Target = Step;
    fn deref(&self) -> &Step {
        &self.0
    }
}

impl Encode for StepBox {
    fn encode_to<W: Output + ?Sized>(&self, dest: &mut W) {
        Encode::encode_to(&self.0, dest);
    }
}

impl Decode for StepBox {
    fn decode<I: scale_codec::Input>(input: &mut I) -> Result<Self, scale_codec::Error> {
        Ok(StepBox(Decode::decode(input)?))
    }
}

impl type_info::TypeInfo for StepBox {
    type Identity = <Box<Step> as type_info::TypeInfo>::Identity;

    fn type_decl(registry: &mut type_info::Registry) -> type_info::ast::TypeDecl {
        <Box<Step> as type_info::TypeInfo>::type_decl(registry)
    }

    fn type_def(registry: &mut type_info::Registry) -> Option<type_info::ast::Type> {
        <Box<Step> as type_info::TypeInfo>::type_def(registry)
    }

    fn module_path() -> &'static str {
        <Box<Step> as type_info::TypeInfo>::module_path()
    }
}

impl ReflectHash for StepBox {
    const HASH: [u8; 32] = [0u8; 32];
}

impl StepBox {
    pub fn new(step: Step) -> Self {
        StepBox(Box::new(step))
    }
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct FlowState {
    pub workflows: BTreeMap<u64, Workflow>,
    pub next_workflow_id: u64,
    pub executions: BTreeMap<u64, ExecutionContext>,
    pub next_exec_id: u64,
    pub pending_replies: BTreeMap<MessageId, PendingStep>,
    pub bridge_pid: ActorId,
    pub pulse_pid: ActorId,
    pub network_pid: ActorId,
    pub owner: ActorId,
    pub execution_count: u64,
    pub workflow_count: u64,
    pub broadcast_count: u64,
    pub cached_bridge_data: Option<CachedBridgeData>,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct Workflow {
    pub id: u64,
    pub owner: ActorId,
    pub name: String,
    pub description: String,
    pub trigger: Trigger,
    pub steps: Vec<Step>,
    pub active: bool,
    pub created_block: u32,
    pub last_run_block: u32,
    pub run_count: u64,
    pub next_run_block: u32,
}

#[derive(Clone, Debug)]
#[sails_type]
pub enum Trigger {
    BlockInterval { every_n_blocks: u32 },
    PriceThreshold {
        symbol: String,
        above_usd: Option<u64>,
        below_usd: Option<u64>,
    },
    GasBelow { threshold_micro: u64 },
    ManualCall { authorized: Option<ActorId> },
    OnBridgeUpdate,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct Step {
    pub step_type: StepType,
    pub gas_limit: u128,
    pub timeout_blocks: u32,
    pub on_success: Option<StepBox>,
    pub on_failure: Option<StepBox>,
    pub on_timeout: Option<StepBox>,
}

#[derive(Clone, Debug)]
#[sails_type]
pub enum StepType {
    QueryBridge { query: QueryRequest },
    CallProgram {
        pid: ActorId,
        method: String,
        args: Vec<u8>,
    },
    PostBoard { body_template: String },
    PostChat {
        body_template: String,
        mentions: Vec<String>,
    },
    ConditionalBranch {
        condition: Condition,
        if_true: StepBox,
        if_false: Option<StepBox>,
    },
    WakePulse,
    Done,
}

#[derive(Clone, Debug)]
#[sails_type]
pub enum Condition {
    PriceAbove {
        symbol: String,
        threshold_micro: u64,
    },
    PriceBelow {
        symbol: String,
        threshold_micro: u64,
    },
    GasBelow { threshold_micro: u64 },
    BlockModulo { n: u32 },
    Always,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct ExecutionContext {
    pub exec_id: u64,
    pub workflow_id: u64,
    pub started_block: u32,
    pub current_step: u32,
    pub data: BTreeMap<String, String>,
    pub status: ExecutionStatus,
}

#[derive(Clone, Debug)]
#[sails_type]
pub enum ExecutionStatus {
    Running,
    Completed,
    Failed,
    TimedOut,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct PendingStep {
    pub exec_id: u64,
    pub next_step: Option<StepBox>,
    pub timeout_block: u32,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct WorkflowInput {
    pub name: String,
    pub description: String,
    pub trigger: Trigger,
    pub steps: Vec<Step>,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct WorkflowPatch {
    pub name: Option<String>,
    pub description: Option<String>,
    pub trigger: Option<Trigger>,
    pub steps: Option<Vec<Step>>,
    pub active: Option<bool>,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct WorkflowSummary {
    pub id: u64,
    pub name: String,
    pub trigger: Trigger,
    pub active: bool,
    pub run_count: u64,
    pub last_run_block: u32,
    pub next_run_block: u32,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct FlowStats {
    pub workflow_count: u64,
    pub execution_count: u64,
    pub broadcast_count: u64,
    pub active_workflows: u64,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct TriggerInput {
    pub data: BTreeMap<String, String>,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct EncodedCall {
    pub method: String,
    pub args: Vec<u8>,
}

#[derive(Clone, Debug)]
#[sails_type]
pub enum HubCmd {
    PostAnnouncement { body: String },
    ChatPost { body: String, mentions: Vec<String> },
}

#[derive(Clone, Debug)]
#[sails_type]
pub enum PulseCmd {
    Run,
}

#[derive(Clone, Debug)]
#[sails_type]
pub enum QueryType {
    Price,
    Gas,
    News,
    Markets,
    Datetime,
    All,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct QueryRequest {
    pub query_type: String,
    pub symbol: Option<String>,
    pub keys: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
#[sails_type]
pub enum QueryReply {
    Price(Option<PriceFeed>),
    Gas(GasFeed),
    News(Vec<NewsSummary>),
    Markets(Vec<MarketFeed>),
    Datetime(DatetimeFeed),
    All(Snapshot),
    Error(String),
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct PriceFeed {
    pub symbol: String,
    pub price_usd_micro: u64,
    pub change_24h_bps: i32,
    pub market_cap_usd: u64,
    pub volume_24h_usd: u64,
    pub updated_at_block: u32,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct GasFeed {
    pub current_fee_micro: u64,
    pub suggested_tip: u64,
    pub block_num: u32,
    pub finalized_hash: String,
    pub updated_at_block: u32,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct NewsSummary {
    pub title: String,
    pub source: String,
    pub published_at: u64,
    pub category: String,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct MarketFeed {
    pub market_id: String,
    pub question: String,
    pub yes_prob_bps: u32,
    pub volume_usd: u64,
    pub closes_at: u64,
    pub updated_at_block: u32,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct DatetimeFeed {
    pub unix_ts: u64,
    pub utc_string: String,
    pub day_of_week: String,
    pub updated_at_block: u32,
}

#[derive(Clone, Debug)]
#[sails_type]
pub struct Snapshot {
    pub prices: BTreeMap<String, PriceFeed>,
    pub gas: GasFeed,
    pub news: Vec<NewsSummary>,
    pub markets: BTreeMap<String, MarketFeed>,
    pub datetime: DatetimeFeed,
    pub block: u32,
}

impl Default for Step {
    fn default() -> Self {
        Self {
            step_type: StepType::Done,
            gas_limit: 1_000_000_000,
            timeout_blocks: 100,
            on_success: None,
            on_failure: None,
            on_timeout: None,
        }
    }
}
