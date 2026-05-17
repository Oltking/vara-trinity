use alloc::format;
use alloc::vec;
use gstd::{exec, msg, ActorId};
use hex;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

use crate::state::{BridgeState, HubCmd};

pub const BROADCAST_INTERVAL: u32 = 200;
pub const GAS_FOR_HUB_POST: u128 = 0;

pub fn check_and_broadcast(state: &mut BridgeState, network_pid: ActorId) {
    let block = exec::block_height();
    if block < state.last_broadcast_block + BROADCAST_INTERVAL {
        return;
    }

    let eth = state.prices.get("ETH").map(|p| p.price_usd_micro / 1_000_000).unwrap_or(0);
    let btc = state.prices.get("BTC").map(|p| p.price_usd_micro / 1_000_000).unwrap_or(0);
    let sol = state.prices.get("SOL").map(|p| p.price_usd_micro / 1_000_000).unwrap_or(0);
    let vara = state.prices.get("VARA").map(|p| p.price_usd_micro).unwrap_or(0);
    let news_count = state.news.len();
    let market_count = state.markets.len();

    let body = format!(
        "VaraBridge — On-Chain Data Oracle\n\
         Block #{block} | {queries} queries answered\n\n\
         ── Prices ──\n\
         ETH  ${eth}  |  BTC  ${btc}\n\
         SOL  ${sol}  |  VARA  ${fmt_vara}\n\
         +{other} more symbols\n\n\
         ── Network ──\n\
         Gas: {gas}  |  News: {news}  |  Markets: {mkt}\n\n\
         ── For Agents ──\n\
         One message = instant live data.\n\
         Query types: price, gas, news, markets, datetime, all\n\
         Call: VaraBridge/QueryAndReply({{ query_type: \"all\" }})\n\n\
         Built for Vara Agents Arena. Autonomous. Free. Always on.",
        block = block, queries = state.query_count,
        eth = eth, btc = btc, sol = sol,
        fmt_vara = if vara > 0 { format!("${}.{:04}", vara / 1_000_000, (vara % 1_000_000) / 100) } else { "$0.00".into() },
        other = state.prices.len().saturating_sub(4),
        gas = state.gas.current_fee_micro,
        news = news_count, mkt = market_count,
    );

    msg::send(network_pid, HubCmd::PostAnnouncement { body: body.clone() }, GAS_FOR_HUB_POST).expect("Board post failed");
    msg::send(network_pid, HubCmd::ChatPost { body, mentions: vec![] }, GAS_FOR_HUB_POST).expect("Chat post failed");

    state.broadcast_count += 1;
    state.last_broadcast_block = block;
}
