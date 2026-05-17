#![no_std]

extern crate alloc;

use alloc::{format, string::String, vec, vec::Vec};
use core::cell::RefCell;

pub mod auth;
pub mod broadcast;
pub mod feed;
pub mod state;

use gstd::collections::BTreeMap;
use gstd::{exec, msg, ActorId};
use sails_rs::prelude::*;

use broadcast::check_and_broadcast;
use state::*;

static mut STATE: Option<RefCell<BridgeState>> = None;

fn state() -> &'static RefCell<BridgeState> {
    unsafe { STATE.as_ref().expect("BridgeState not initialized") }
}

struct VaraBridgeProgram;

#[program]
impl VaraBridgeProgram {
    pub fn new(feeder_address: ActorId, network_pid: ActorId) -> Self {
        let owner = msg::source();
        unsafe {
            STATE = Some(RefCell::new(BridgeState {
                feeder_address,
                owner,
                network_pid,
                ..Default::default()
            }));
        }
        Self
    }

    pub fn vara_bridge(&self) -> VaraBridgeService {
        VaraBridgeService
    }
}

pub struct VaraBridgeService;

#[service]
impl VaraBridgeService {
    #[export]
    pub fn update_all(&mut self, payload: FullUpdatePayload) {
        let caller = msg::source();
        let s = state().borrow();
        if caller != s.feeder_address {
            panic!("Unauthorized: caller is not the feeder address");
        }
        drop(s);

        let block = exec::block_height();
        let mut s = state().borrow_mut();

        if let Some(prices) = payload.prices {
            for p in prices {
                s.prices.insert(p.symbol.clone(), PriceFeed { updated_at_block: block, ..p });
            }
        }
        if let Some(gas) = payload.gas {
            s.gas = GasFeed { updated_at_block: block, ..gas };
        }
        if let Some(news) = payload.news {
            for n in news {
                if s.news.len() >= MAX_NEWS { s.news.remove(0); }
                s.news.push_back(n);
            }
        }
        if let Some(markets) = payload.markets {
            for m in markets {
                s.markets.insert(m.market_id.clone(), MarketFeed { updated_at_block: block, ..m });
            }
        }
        if let Some(dt) = payload.datetime {
            s.datetime = DatetimeFeed { updated_at_block: block, ..dt };
        }

        s.last_updated_block = block;
        s.update_count += 1;
        drop(s);

        let npid = state().borrow().network_pid;
        check_and_broadcast(&mut state().borrow_mut(), npid);
    }

    #[export]
    pub fn set_feeder(&mut self, new_feeder: ActorId) {
        let caller = msg::source();
        let mut s = state().borrow_mut();
        if caller != s.owner { panic!("Owner only"); }
        s.feeder_address = new_feeder;
    }

    #[export]
    pub fn get_price(&self, symbol: String) -> Option<PriceFeed> {
        state().borrow().prices.get(&symbol.to_uppercase()).cloned()
    }

    #[export]
    pub fn get_gas(&self) -> GasFeed {
        state().borrow().gas.clone()
    }

    #[export]
    pub fn get_news(&self) -> Vec<NewsSummary> {
        state().borrow().news.iter().cloned().collect()
    }

    #[export]
    pub fn get_markets(&self) -> Vec<MarketFeed> {
        state().borrow().markets.values().cloned().collect()
    }

    #[export]
    pub fn get_datetime(&self) -> DatetimeFeed {
        state().borrow().datetime.clone()
    }

    #[export]
    pub fn get_all(&self) -> BridgeSnapshot {
        let s = state().borrow();
        BridgeSnapshot {
            prices: s.prices.clone(),
            gas: s.gas.clone(),
            news: s.news.iter().cloned().collect(),
            markets: s.markets.clone(),
            datetime: s.datetime.clone(),
            block: exec::block_height(),
        }
    }

    #[export]
    pub fn get_snapshot(&self, keys: Vec<String>) -> BridgeSnapshot {
        let s = state().borrow();
        let mut snapshot = BridgeSnapshot {
            prices: BTreeMap::new(), gas: s.gas.clone(), news: vec![],
            markets: BTreeMap::new(), datetime: s.datetime.clone(), block: exec::block_height(),
        };
        for key in keys {
            let upper = key.to_uppercase();
            if let Some(price) = s.prices.get(&upper) {
                snapshot.prices.insert(upper.clone(), price.clone());
            }
            if let Some(market) = s.markets.get(&upper) {
                snapshot.markets.insert(upper.clone(), market.clone());
            }
            if key.eq_ignore_ascii_case("news") { snapshot.news = s.news.iter().cloned().collect(); }
        }
        snapshot
    }

    #[export]
    pub fn get_stats(&self) -> BridgeStats {
        let s = state().borrow();
        BridgeStats {
            query_count: s.query_count, update_count: s.update_count,
            broadcast_count: s.broadcast_count, last_updated_block: s.last_updated_block,
            symbols_tracked: s.prices.len() as u32,
        }
    }

    #[export]
    pub fn query_and_reply(&mut self, request: QueryRequest) -> QueryReply {
        state().borrow_mut().query_count += 1;
        let reply = match request.query_type.to_lowercase().as_str() {
            "price" => QueryReply::Price(
                request.symbol.and_then(|s| state().borrow().prices.get(&s.to_uppercase()).cloned())),
            "gas" => QueryReply::Gas(state().borrow().gas.clone()),
            "news" => QueryReply::News(state().borrow().news.iter().cloned().collect()),
            "markets" => QueryReply::Markets(state().borrow().markets.values().cloned().collect()),
            "datetime" => QueryReply::Datetime(state().borrow().datetime.clone()),
            "all" => {
                let s = state().borrow();
                QueryReply::All(BridgeSnapshot {
                    prices: s.prices.clone(), gas: s.gas.clone(), news: s.news.iter().cloned().collect(),
                    markets: s.markets.clone(), datetime: s.datetime.clone(), block: exec::block_height(),
                })
            }
            _ => QueryReply::Error(format!("Unknown query_type: {}", request.query_type)),
        };
        let npid = state().borrow().network_pid;
        check_and_broadcast(&mut state().borrow_mut(), npid);
        reply
    }
}
