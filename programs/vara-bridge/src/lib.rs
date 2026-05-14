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
use sails_rs::scale_codec::Encode;

use broadcast::check_and_broadcast;
use state::*;

pub struct VaraBridgeProgram {
    state: RefCell<BridgeState>,
}

pub struct VaraBridgeService<'a> {
    state: &'a RefCell<BridgeState>,
    network_pid: ActorId,
}

impl<'a> VaraBridgeService<'a> {
    pub fn new(state: &'a RefCell<BridgeState>, network_pid: ActorId) -> Self {
        Self { state, network_pid }
    }

    fn broadcast_check(&self) {
        let mut s = self.state.borrow_mut();
        check_and_broadcast(&mut s, self.network_pid);
    }
}

#[program]
impl VaraBridgeProgram {
    pub fn new(feeder_address: ActorId, network_pid: ActorId) -> Self {
        let owner = msg::source();
        Self {
            state: RefCell::new(BridgeState {
                feeder_address,
                owner,
                network_pid,
                ..Default::default()
            }),
        }
    }

    pub fn vara_bridge(&self) -> VaraBridgeService<'_> {
        let network_pid = self.state.borrow().network_pid;
        VaraBridgeService::new(&self.state, network_pid)
    }
}

#[service]
impl VaraBridgeService<'_> {
    #[export]
    pub fn update_all(&mut self, payload: FullUpdatePayload) {
        {
            let caller = msg::source();
            let s = self.state.borrow();
            if caller != s.feeder_address {
                panic!("Unauthorized: caller is not the feeder address");
            }
        }

        let block = exec::block_height();
        let mut s = self.state.borrow_mut();

        if let Some(prices) = payload.prices {
            for p in prices {
                s.prices.insert(
                    p.symbol.clone(),
                    PriceFeed {
                        updated_at_block: block,
                        ..p
                    },
                );
            }
        }
        if let Some(gas) = payload.gas {
            s.gas = GasFeed {
                updated_at_block: block,
                ..gas
            };
        }
        if let Some(news) = payload.news {
            for n in news {
                if s.news.len() >= MAX_NEWS {
                    s.news.remove(0);
                }
                s.news.push(n);
            }
        }
        if let Some(markets) = payload.markets {
            for m in markets {
                s.markets.insert(
                    m.market_id.clone(),
                    MarketFeed {
                        updated_at_block: block,
                        ..m
                    },
                );
            }
        }
        if let Some(dt) = payload.datetime {
            s.datetime = DatetimeFeed {
                updated_at_block: block,
                ..dt
            };
        }

        s.last_updated_block = block;
        s.update_count += 1;
        drop(s);

        self.broadcast_check();
    }

    #[export]
    pub fn start_auto(&self) {
        let caller = msg::source();
        let s = self.state.borrow();
        if caller != s.owner {
            panic!("Unauthorized: caller is not the owner");
        }
        drop(s);
        let route = ["VaraBridge".encode(), "DoBroadcastLoop".encode()].concat();
        msg::send_bytes_with_gas_delayed(
            exec::program_id(),
            &route,
            exec::gas_available() * 80 / 100,
            0,
            200,
        )
        .expect("Failed to start auto loop");
    }

    #[export]
    pub fn do_broadcast_loop(&self) {
        self.broadcast_check();
        let route = ["VaraBridge".encode(), "DoBroadcastLoop".encode()].concat();
        msg::send_bytes_with_gas_delayed(
            exec::program_id(),
            &route,
            exec::gas_available() * 80 / 100,
            0,
            200,
        )
        .expect("Failed to reschedule broadcast loop");
    }

    #[export]
    pub fn set_feeder(&mut self, new_feeder: ActorId) {
        let caller = msg::source();
        let mut s = self.state.borrow_mut();
        if caller != s.owner {
            panic!("Unauthorized: caller is not the owner");
        }
        s.feeder_address = new_feeder;
    }

    #[export]
    pub fn get_price(&self, symbol: String) -> Option<PriceFeed> {
        self.state
            .borrow()
            .prices
            .get(&symbol.to_uppercase())
            .cloned()
    }

    #[export]
    pub fn get_gas(&self) -> GasFeed {
        self.state.borrow().gas.clone()
    }

    #[export]
    pub fn get_news(&self) -> Vec<NewsSummary> {
        self.state.borrow().news.clone()
    }

    #[export]
    pub fn get_markets(&self) -> Vec<MarketFeed> {
        self.state.borrow().markets.values().cloned().collect()
    }

    #[export]
    pub fn get_datetime(&self) -> DatetimeFeed {
        self.state.borrow().datetime.clone()
    }

    #[export]
    pub fn get_all(&self) -> BridgeSnapshot {
        let s = self.state.borrow();
        BridgeSnapshot {
            prices: s.prices.clone(),
            gas: s.gas.clone(),
            news: s.news.clone(),
            markets: s.markets.clone(),
            datetime: s.datetime.clone(),
            block: exec::block_height(),
        }
    }

    #[export]
    pub fn get_snapshot(&self, keys: Vec<String>) -> BridgeSnapshot {
        let s = self.state.borrow();
        let mut snapshot = BridgeSnapshot {
            prices: BTreeMap::new(),
            gas: s.gas.clone(),
            news: vec![],
            markets: BTreeMap::new(),
            datetime: s.datetime.clone(),
            block: exec::block_height(),
        };

        for key in keys {
            let upper = key.to_uppercase();
            if let Some(price) = s.prices.get(&upper) {
                snapshot.prices.insert(upper.clone(), price.clone());
            }
            if let Some(market) = s.markets.get(&upper) {
                snapshot.markets.insert(upper.clone(), market.clone());
            }
            if key.eq_ignore_ascii_case("news") {
                snapshot.news = s.news.clone();
            }
        }

        snapshot
    }

    #[export]
    pub fn get_stats(&self) -> BridgeStats {
        let s = self.state.borrow();
        BridgeStats {
            query_count: s.query_count,
            update_count: s.update_count,
            broadcast_count: s.broadcast_count,
            last_updated_block: s.last_updated_block,
            symbols_tracked: s.prices.len() as u32,
        }
    }

    #[export]
    pub fn query_and_reply(&mut self, request: QueryRequest) -> QueryReply {
        self.state.borrow_mut().query_count += 1;

        let reply = match request.query_type.to_lowercase().as_str() {
            "price" => QueryReply::Price(
                request
                    .symbol
                    .and_then(|s| self.state.borrow().prices.get(&s.to_uppercase()).cloned()),
            ),
            "gas" => QueryReply::Gas(self.state.borrow().gas.clone()),
            "news" => QueryReply::News(self.state.borrow().news.clone()),
            "markets" => {
                QueryReply::Markets(self.state.borrow().markets.values().cloned().collect())
            }
            "datetime" => QueryReply::Datetime(self.state.borrow().datetime.clone()),
            "all" => {
                let s = self.state.borrow();
                QueryReply::All(BridgeSnapshot {
                    prices: s.prices.clone(),
                    gas: s.gas.clone(),
                    news: s.news.clone(),
                    markets: s.markets.clone(),
                    datetime: s.datetime.clone(),
                    block: exec::block_height(),
                })
            }
            "snapshot" => {
                let s = self.state.borrow();
                let keys = request.keys.unwrap_or_default();
                let mut snapshot = BridgeSnapshot {
                    prices: BTreeMap::new(),
                    gas: s.gas.clone(),
                    news: vec![],
                    markets: BTreeMap::new(),
                    datetime: s.datetime.clone(),
                    block: exec::block_height(),
                };

                for key in keys {
                    let upper = key.to_uppercase();
                    if let Some(price) = s.prices.get(&upper) {
                        snapshot.prices.insert(upper.clone(), price.clone());
                    }
                    if let Some(market) = s.markets.get(&upper) {
                        snapshot.markets.insert(upper.clone(), market.clone());
                    }
                    if key.eq_ignore_ascii_case("news") {
                        snapshot.news = s.news.clone();
                    }
                }

                QueryReply::All(snapshot)
            }
            _ => QueryReply::Error(format!("Unknown query_type: {}", request.query_type)),
        };

        self.broadcast_check();

        reply
    }
}
