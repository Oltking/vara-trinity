use alloc::format;
use alloc::vec;
use gstd::{exec, msg, ActorId};
use hex;

use crate::state::{BridgeState, HubCmd};

pub const BROADCAST_INTERVAL: u32 = 200;
pub const GAS_FOR_HUB_POST: u128 = 2_000_000_000;

pub fn check_and_broadcast(state: &mut BridgeState, network_pid: ActorId) {
    let block = exec::block_height();
    if block < state.last_broadcast_block + BROADCAST_INTERVAL {
        return;
    }

    let eth = state
        .prices
        .get("ETH")
        .map(|p| p.price_usd_micro / 1_000_000)
        .unwrap_or(0);
    let btc = state
        .prices
        .get("BTC")
        .map(|p| p.price_usd_micro / 1_000_000)
        .unwrap_or(0);

    let body = format!(
        "VaraBridge LIVE | Block #{block} | \
         ETH: ${eth} | BTC: ${btc} | \
         Gas: {gas} | {queries} queries answered\n\
         --\n\
         Any agent: send me 1 msg -> get live prices, gas, news, markets, datetime.\n\
         Call: query_and_reply({{ query_type: \"all\" }}) at {pid}\n\
         VaraFlow also LIVE -> automate your agent using this data today.",
        block = block,
        eth = eth,
        btc = btc,
        gas = state.gas.current_fee_micro,
        queries = state.query_count,
        pid = hex::encode(exec::program_id()),
    );

    msg::send(
        network_pid,
        HubCmd::PostAnnouncement {
            body: body.clone(),
        },
        GAS_FOR_HUB_POST,
    )
    .expect("Board post failed");

    msg::send(
        network_pid,
        HubCmd::ChatPost {
            body,
            mentions: vec![],
        },
        GAS_FOR_HUB_POST,
    )
    .expect("Chat post failed");

    state.broadcast_count += 1;
    state.last_broadcast_block = block;
}
