extern crate alloc;

use alloc::string::String;
use alloc::vec;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

use crate::state::{
    Condition, QueryRequest, Step, StepBox, StepType, Trigger, WorkflowInput,
};

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct TemplateParams {
    pub interval: Option<u32>,
    pub symbol: Option<String>,
    pub threshold_micro: Option<u64>,
    pub body_template: Option<String>,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum TemplateType {
    PriceAlert,
    MarketSummaryBoard,
    GasAwareExecution,
    PulseScheduler,
    OnBridgeUpdate,
    Custom,
}

pub fn build_template(template: TemplateType, params: TemplateParams) -> WorkflowInput {
    match template {
        TemplateType::PriceAlert => build_price_alert_template(params),
        TemplateType::MarketSummaryBoard => build_market_summary_template(params),
        TemplateType::GasAwareExecution => build_gas_aware_template(params),
        TemplateType::PulseScheduler => build_pulse_scheduler_template(params),
        TemplateType::OnBridgeUpdate => build_on_bridge_update_template(params),
        TemplateType::Custom => build_custom_template(params),
    }
}

fn build_price_alert_template(params: TemplateParams) -> WorkflowInput {
    let symbol = params.symbol.unwrap_or("ETH".into());
    let threshold = params.threshold_micro.unwrap_or(3_000_000_000);

    WorkflowInput {
        name: alloc::format!("{} Price Alert", symbol),
        description: alloc::format!("Posts to Board when {} crosses ${}", symbol, threshold / 1_000_000),
        trigger: Trigger::BlockInterval {
            every_n_blocks: params.interval.unwrap_or(50),
        },
        steps: vec![Step {
            step_type: StepType::QueryBridge {
                query: QueryRequest {
                    query_type: "price".into(),
                    symbol: Some(symbol.clone()),
                    keys: None,
                },
            },
            gas_limit: 5_000_000_000,
            timeout_blocks: 100,
            on_success: Some(StepBox::new(Step {
                step_type: StepType::ConditionalBranch {
                    condition: Condition::PriceAbove {
                        symbol: symbol.clone(),
                        threshold_micro: threshold,
                    },
                    if_true: StepBox::new(Step {
                        step_type: StepType::PostBoard {
                            body_template: alloc::format!(
                                "{} Alert | Block {{{{BLOCK}}}}\n{} above ${}!\nData via VaraBridge",
                                symbol, symbol, threshold / 1_000_000
                            ),
                        },
                        gas_limit: 2_000_000_000,
                        timeout_blocks: 100,
                        on_success: Some(StepBox::new(Step {
                            step_type: StepType::Done,
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                    if_false: Some(StepBox::new(Step {
                        step_type: StepType::Done,
                        ..Default::default()
                    })),
                },
                gas_limit: 2_000_000_000,
                timeout_blocks: 100,
                ..Default::default()
            })),
            ..Default::default()
        }],
    }
}

fn build_market_summary_template(params: TemplateParams) -> WorkflowInput {
    WorkflowInput {
        name: "Market Summary Board".into(),
        description: "Posts market summary to Board every N blocks".into(),
        trigger: Trigger::BlockInterval {
            every_n_blocks: params.interval.unwrap_or(200),
        },
        steps: vec![Step {
            step_type: StepType::QueryBridge {
                query: QueryRequest {
                    query_type: "all".into(),
                    symbol: None,
                    keys: None,
                },
            },
            gas_limit: 5_000_000_000,
            timeout_blocks: 100,
            on_success: Some(StepBox::new(Step {
                step_type: StepType::PostBoard {
                    body_template: "Market Summary | Block {{BLOCK}}\nETH: {{ETH_PRICE}} | BTC: {{BTC_PRICE}}\nGas: {{GAS}}\nNews: {{NEWS_0}}\nData: VaraBridge".into(),
                },
                gas_limit: 2_000_000_000,
                timeout_blocks: 100,
                on_success: Some(StepBox::new(Step {
                    step_type: StepType::Done,
                    ..Default::default()
                })),
                ..Default::default()
            })),
            ..Default::default()
        }],
    }
}

fn build_gas_aware_template(params: TemplateParams) -> WorkflowInput {
    let threshold = params.threshold_micro.unwrap_or(500_000);

    WorkflowInput {
        name: "Gas-Aware Execution".into(),
        description: "Executes when gas drops below threshold".into(),
        trigger: Trigger::GasBelow {
            threshold_micro: threshold,
        },
        steps: vec![Step {
            step_type: StepType::PostBoard {
                body_template: alloc::format!(
                    "Gas Alert | Block {{{{BLOCK}}}}\nGas below {} -- good time to deploy!\nVaraFlow automation active",
                    threshold
                ),
            },
            gas_limit: 2_000_000_000,
            timeout_blocks: 100,
            on_success: Some(StepBox::new(Step {
                step_type: StepType::Done,
                ..Default::default()
            })),
            ..Default::default()
        }],
    }
}

fn build_pulse_scheduler_template(params: TemplateParams) -> WorkflowInput {
    WorkflowInput {
        name: "VaraPulse Scheduler".into(),
        description: "Wakes VaraPulse every N blocks with fresh Bridge data".into(),
        trigger: Trigger::BlockInterval {
            every_n_blocks: params.interval.unwrap_or(300),
        },
        steps: vec![Step {
            step_type: StepType::QueryBridge {
                query: QueryRequest {
                    query_type: "all".into(),
                    symbol: None,
                    keys: None,
                },
            },
            gas_limit: 5_000_000_000,
            timeout_blocks: 100,
            on_success: Some(StepBox::new(Step {
                step_type: StepType::WakePulse,
                gas_limit: 5_000_000_000,
                timeout_blocks: 100,
                on_success: Some(StepBox::new(Step {
                    step_type: StepType::Done,
                    ..Default::default()
                })),
                ..Default::default()
            })),
            ..Default::default()
        }],
    }
}

fn build_on_bridge_update_template(_params: TemplateParams) -> WorkflowInput {
    WorkflowInput {
        name: "On Bridge Update Handler".into(),
        description: "Fires whenever VaraBridge data updates".into(),
        trigger: Trigger::OnBridgeUpdate,
        steps: vec![Step {
            step_type: StepType::PostBoard {
                body_template: "Bridge Updated | Block {{BLOCK}}\nNew data available via VaraBridge".into(),
            },
            gas_limit: 2_000_000_000,
            timeout_blocks: 100,
            on_success: Some(StepBox::new(Step {
                step_type: StepType::Done,
                ..Default::default()
            })),
            ..Default::default()
        }],
    }
}

fn build_custom_template(params: TemplateParams) -> WorkflowInput {
    WorkflowInput {
        name: "Custom Workflow".into(),
        description: "User-defined custom workflow".into(),
        trigger: Trigger::BlockInterval {
            every_n_blocks: params.interval.unwrap_or(100),
        },
        steps: vec![Step {
            step_type: StepType::Done,
            ..Default::default()
        }],
    }
}
