use alloc::format;
use alloc::string::String;

use crate::state::DataSnapshot;

pub fn format_market_summary(data: &DataSnapshot, pulse_num: u64) -> String {
    format!(
        "Market Summary #{pulse} | Block #{block}\n\
         ETH: ${eth} | BTC: ${btc} | VARA: ${vara}\n\
         Gas: {gas} | {label}",
        pulse = pulse_num,
        block = data.block,
        eth = data.eth_usd / 1_000_000,
        btc = data.btc_usd / 1_000_000,
        vara = data.vara_usd as f64 / 1_000_000.0,
        gas = data.gas_micro,
        label = gas_label(data.gas_micro),
    )
}

pub fn format_gas_tip(data: &DataSnapshot, pulse_num: u64) -> String {
    format!(
        "Gas Tip #{pulse} | Block #{block}\nGas: {gas} ({label})",
        pulse = pulse_num,
        block = data.block,
        gas = data.gas_micro,
        label = gas_label(data.gas_micro),
    )
}

pub fn format_news_brief(data: &DataSnapshot, pulse_num: u64) -> String {
    format!(
        "News Brief #{pulse} | Block #{block}\n{news}",
        pulse = pulse_num,
        block = data.block,
        news = data.top_news,
    )
}

pub fn format_market_spark(data: &DataSnapshot, pulse_num: u64) -> String {
    let market = data.top_market.as_deref().unwrap_or("crypto market");
    format!(
        "Market Spark #{pulse} | Block #{block}\n{market}",
        pulse = pulse_num,
        block = data.block,
        market = market,
    )
}

fn gas_label(gas_micro: u64) -> &'static str {
    match gas_micro {
        g if g < 100_000 => "ULTRA LOW",
        g if g < 500_000 => "LOW",
        g if g < 1_000_000 => "MODERATE",
        g if g < 5_000_000 => "HIGH",
        _ => "VERY HIGH",
    }
}
