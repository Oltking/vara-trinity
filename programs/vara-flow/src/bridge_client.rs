extern crate alloc;

use alloc::string::String;
use gstd::collections::BTreeMap;
use gstd::{exec, msg, ActorId, MessageId};
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

use crate::state::{GasFeed, PriceFeed, QueryReply, QueryRequest};

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct CachedBridgeData {
    pub prices: BTreeMap<String, PriceFeed>,
    pub gas: GasFeed,
    pub updated_at_block: u32,
}

pub const CACHE_TTL_BLOCKS: u32 = 100;

pub fn query_bridge(bridge_pid: ActorId, query: QueryRequest, gas_limit: u128) -> MessageId {
    msg::send(bridge_pid, query, gas_limit).expect("query_bridge failed")
}

pub fn query_bridge_all(bridge_pid: ActorId, gas_limit: u128) -> MessageId {
    msg::send(
        bridge_pid,
        QueryRequest {
            query_type: "all".into(),
            symbol: None,
            keys: None,
        },
        gas_limit,
    )
    .expect("query_bridge_all failed")
}

pub fn get_cached_price(cache: &Option<CachedBridgeData>, symbol: &str) -> Option<u64> {
    cache.as_ref().and_then(|c| {
        if exec::block_height() - c.updated_at_block <= CACHE_TTL_BLOCKS {
            c.prices.get(&symbol.to_uppercase()).map(|p| p.price_usd_micro)
        } else {
            None
        }
    })
}

pub fn get_cached_gas(cache: &Option<CachedBridgeData>) -> u64 {
    cache
        .as_ref()
        .map(|c| c.gas.current_fee_micro)
        .unwrap_or(u64::MAX)
}

pub fn update_cache(cache: &mut Option<CachedBridgeData>, reply: &QueryReply) {
    match reply {
        QueryReply::All(snapshot) => {
            let mut prices = BTreeMap::new();
            for (symbol, feed) in &snapshot.prices {
                prices.insert(symbol.clone(), feed.clone());
            }
            *cache = Some(CachedBridgeData {
                prices,
                gas: snapshot.gas.clone(),
                updated_at_block: exec::block_height(),
            });
        }
        _ => {}
    }
}
