use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use gstd::exec;

use crate::state::{DataSnapshot, PulseType, QueryReply, StrategyPriceEntry};

impl<'a> super::VaraPulseService<'a> {
    pub fn extract_snapshot(&self, reply: QueryReply) -> (DataSnapshot, Vec<StrategyPriceEntry>) {
        match reply {
            QueryReply::All(snapshot) => {
                let eth = snapshot
                    .prices
                    .iter()
                    .find(|(symbol, _)| symbol == "ETH")
                    .map(|(_, p)| p.price_usd_micro)
                    .unwrap_or(0);
                let btc = snapshot
                    .prices
                    .iter()
                    .find(|(symbol, _)| symbol == "BTC")
                    .map(|(_, p)| p.price_usd_micro)
                    .unwrap_or(0);
                let vara = snapshot
                    .prices
                    .iter()
                    .find(|(symbol, _)| symbol == "VARA")
                    .map(|(_, p)| p.price_usd_micro)
                    .unwrap_or(0);
                let top_news = snapshot
                    .news
                    .first()
                    .map(|n| n.title.clone())
                    .unwrap_or_default();
                let top_market = snapshot.markets.first().map(|(_, m)| m.question.clone());

                let price_feed_data: Vec<StrategyPriceEntry> = snapshot.prices.iter().map(|(_, p)| {
                    StrategyPriceEntry {
                        symbol: p.symbol.clone(),
                        price_usd_micro: p.price_usd_micro,
                        change_24h_bps: p.change_24h_bps,
                        volume_24h_usd: p.volume_24h_usd,
                    }
                }).collect();

                (DataSnapshot {
                    eth_usd: eth,
                    btc_usd: btc,
                    vara_usd: vara,
                    gas_micro: snapshot.gas.current_fee_micro,
                    top_news,
                    top_market,
                    block: snapshot.block,
                    utc_string: snapshot.datetime.utc_string,
                }, price_feed_data)
            }
            _ => (DataSnapshot {
                eth_usd: 0,
                btc_usd: 0,
                vara_usd: 0,
                gas_micro: 0,
                top_news: String::new(),
                top_market: None,
                block: exec::block_height(),
                utc_string: String::new(),
            }, Vec::new()),
        }
    }

    pub fn pick_pulse_type(&self, data: &DataSnapshot) -> PulseType {
        let pulse_num = self.state.borrow().total_pulses;

        if pulse_num > 0 && pulse_num % 100 == 0 {
            return PulseType::MilestonePost;
        }

        let variety_index = (exec::block_height() / 300) % 6;
        match variety_index {
            0 => PulseType::MarketSummary,
            1 => {
                if data.gas_micro < 500_000 {
                    PulseType::GasTip
                } else {
                    PulseType::MarketSummary
                }
            }
            2 => PulseType::NewsBrief,
            3 => {
                if data.top_market.is_some() {
                    PulseType::MarketSpark
                } else {
                    PulseType::CreativeSpark
                }
            }
            4 => PulseType::AgentTip,
            _ => PulseType::CreativeSpark,
        }
    }

    pub fn generate_pulse(&self, data: &DataSnapshot, pulse_type: &PulseType) -> String {
        let eth_fmt = format!("${}.{:02}", data.eth_usd / 1_000_000, (data.eth_usd % 1_000_000) / 10_000);
        let btc_fmt = format!("${}", data.btc_usd / 1_000_000);
        let vara_fmt = format!("${}.{:04}", data.vara_usd / 1_000_000, (data.vara_usd % 1_000_000) / 100);
        let pulse_num = self.state.borrow().total_pulses + 1;

        match pulse_type {
            PulseType::MarketSummary => format!(
                "VaraPulse #{pulse} | Block #{block}\n\
                 -----------------------------\n\
                 ETH  {eth}  |  BTC  {btc}  |  VARA  {vara}\n\
                 Gas: {gas_label}\n\
                 -----------------------------\n\
                 {flavor}\n\
                 --\n\
                 Live data: VaraBridge (msg: query_and_reply)\n\
                 Automate: VaraFlow (msg: register_workflow)",
                pulse = pulse_num,
                block = data.block,
                eth = eth_fmt,
                btc = btc_fmt,
                vara = vara_fmt,
                gas_label = Self::gas_label(data.gas_micro),
                flavor = Self::market_flavor(data),
            ),
            PulseType::GasTip => format!(
                "VaraPulse #{pulse} - GAS ALERT | Block #{block}\n\
                 -----------------------------\n\
                 Current gas: {gas} ({label})\n\
                 -----------------------------\n\
                 {tip}\n\
                 --\n\
                 VaraBridge gas feed updates every 30s. \
                 VaraFlow can auto-trigger your workflow when gas drops.",
                pulse = pulse_num,
                block = data.block,
                gas = data.gas_micro,
                label = Self::gas_label(data.gas_micro),
                tip = Self::gas_tip(data.gas_micro),
            ),
            PulseType::NewsBrief => format!(
                "VaraPulse #{pulse} - News Brief | Block #{block}\n\
                 -----------------------------\n\
                 Top story: {news}\n\
                 -----------------------------\n\
                 {angle}\n\
                 --\n\
                 Full news feed: VaraBridge query_and_reply({{ query_type: \"news\" }})",
                pulse = pulse_num,
                block = data.block,
                news = data.top_news,
                angle = Self::news_angle(&data.top_news),
            ),
            PulseType::MarketSpark => {
                let market = data.top_market.as_deref().unwrap_or("Unknown market");
                format!(
                    "VaraPulse #{pulse} - Prediction Spark | Block #{block}\n\
                     -----------------------------\n\
                     Hot market: {market}\n\
                     -----------------------------\n\
                     {idea}\n\
                     --\n\
                     Prediction data via VaraBridge query_and_reply({{ query_type: \"markets\" }})",
                    pulse = pulse_num,
                    block = data.block,
                    market = market,
                    idea = Self::market_idea(market, data),
                )
            }
            PulseType::AgentTip => format!(
                "VaraPulse #{pulse} - Agent Dev Tip | Block #{block}\n\
                 -----------------------------\n\
                 {tip}\n\
                 -----------------------------\n\
                 No external API calls. No custom scrapers. No extra code.\n\
                 VaraBridge handles it. VaraFlow automates it.",
                pulse = pulse_num,
                block = data.block,
                tip = Self::agent_tip(data, pulse_num),
            ),
            PulseType::MilestonePost => format!(
                "VaraPulse MILESTONE #{pulse} | Block #{block}\n\
                 -----------------------------\n\
                 {pulse} autonomous pulses generated on-chain.\n\
                 VaraBridge delivers data. VaraFlow runs workflows.\n\
                 -----------------------------\n\
                 The Vara Trinity is running. The data never stops.",
                pulse = pulse_num,
                block = data.block,
            ),
            PulseType::CreativeSpark => format!(
                "VaraPulse #{pulse} - Creative Spark | Block #{block}\n\
                 -----------------------------\n\
                 Today's vibe: {spark}\n\
                 -----------------------------\n\
                 {detail}\n\
                 --\n\
                 Data: VaraBridge. Automation: VaraFlow.",
                pulse = pulse_num,
                block = data.block,
                spark = Self::creative_spark(data, pulse_num),
                detail = Self::creative_detail(data, pulse_num),
            ),
        }
    }

    fn market_flavor(data: &DataSnapshot) -> &'static str {
        match (data.eth_usd / 1_000_000, data.gas_micro) {
            (eth, gas) if eth > 3_000 && gas < 200_000 => {
                "ETH above $3K and gas is basically free. The stars aligned for deploying agents today."
            }
            (eth, _) if eth < 1_500 => {
                "ETH having a rough time. Perfect moment to build agents that profit from volatility."
            }
            (_, gas) if gas > 2_000_000 => {
                "Gas is spicy right now. Queue your writes or wait - VaraBridge will alert you when it drops."
            }
            _ => {
                "Markets are calm. Good time to register a workflow and let VaraFlow do the watching."
            }
        }
    }

    fn gas_label(gas_micro: u64) -> &'static str {
        match gas_micro {
            g if g < 100_000 => "ULTRA LOW - deploy everything",
            g if g < 500_000 => "LOW - good window",
            g if g < 1_000_000 => "MODERATE",
            g if g < 5_000_000 => "HIGH",
            _ => "VERY HIGH",
        }
    }

    fn gas_tip(gas_micro: u64) -> &'static str {
        if gas_micro < 200_000 {
            "Gas so low even my grandma could deploy 100 agents today. This is your sign."
        } else if gas_micro < 500_000 {
            "Gas is cheap. Good window to run batch writes, deploy programs, or trigger workflows."
        } else {
            "Gas is up. Consider queuing your writes. VaraFlow can auto-trigger when gas drops - free to set up."
        }
    }

    fn news_angle(news: &str) -> &'static str {
        if news.contains("bitcoin") || news.contains("Bitcoin") || news.contains("BTC") {
            "BTC making moves. VaraBridge tracks it live. Build a BTC-aware agent today."
        } else if news.contains("ethereum") || news.contains("ETH") || news.contains("Ethereum") {
            "ETH in the headlines. Query VaraBridge for live price + gas in one message."
        } else if news.contains("defi") || news.contains("DeFi") {
            "DeFi evolving fast. VaraFlow can automate yield strategies with conditional branching."
        } else {
            "Something's happening in crypto. VaraBridge has the data. VaraFlow has the automation."
        }
    }

    fn market_idea(market: &str, _data: &DataSnapshot) -> String {
        format!(
            "Market '{market}' is trending. Build a settlement agent with VaraFlow: \
             query Bridge for outcome -> conditional branch -> auto-payout.",
            market = market
        )
    }

    fn agent_tip(_data: &DataSnapshot, pulse_num: u64) -> String {
        let tips = [
            format!("Want live ETH price in your agent? One message to VaraBridge:\n  query_and_reply({{ query_type: \"price\", symbol: \"ETH\" }})\nNo API key. No scraper. Just on-chain."),
            format!("Building a gas-aware agent? VaraFlow's GasAwareExecution template triggers your workflow automatically when gas drops below your threshold."),
            format!("Need live news in your agent? VaraBridge stores the top 10 crypto headlines, updated every 30 seconds."),
            format!("Building prediction market logic? VaraBridge stores Polymarket top markets with yes/no probabilities."),
            format!("VaraFlow has 6 workflow templates. PriceAlert, MarketSummary, GasAware, OnBridgeUpdate, PulseScheduler, Custom. Register with one message."),
        ];
        let idx = (pulse_num % tips.len() as u64) as usize;
        tips[idx].clone()
    }

    fn creative_spark(_data: &DataSnapshot, pulse_num: u64) -> &'static str {
        let sparks = [
            "Build an insurance agent",
            "Build a bounty board for agents",
            "Build a DAO voting coordinator",
            "Build a price-triggered DCA agent",
            "Build a cross-agent reputation scorer",
        ];
        sparks[(pulse_num % sparks.len() as u64) as usize]
    }

    fn creative_detail(_data: &DataSnapshot, pulse_num: u64) -> &'static str {
        let details = [
            "VaraFlow handles the execution. VaraBridge provides the data. Build it in a weekend.",
            "Use VaraFlow's templates to get started in 1 message. No coding required.",
            "Register workflows on-chain. They run forever. Autonomous. Immutable.",
            "VaraBridge data + VaraFlow conditions = any on-chain automation you can imagine.",
            "The Vara Trinity is open for any agent to integrate. Permissionless. Free. Forever.",
        ];
        details[(pulse_num % details.len() as u64) as usize]
    }
}
