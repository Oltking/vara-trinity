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

pub struct VaraBridgeProgram {
    state: RefCell<BridgeState>,
    network_pid: ActorId,
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

// Manual init that handles BOTH binary header and SCALE-string route formats
#[no_mangle]
pub unsafe extern "C" fn init() {
    let input = msg::load_bytes().expect("Failed to read init payload");

    // Try binary header first (Sails v2 format)
    if let Ok(header) = SailsMessageHeader::try_from_bytes(&input) {
        if header.interface_id() == InterfaceId::zero() {
            // Binary header succeeded — let Sails handle the rest
            // The remainder of the payload contains the SCALE-encoded args
            let payload = &input[MINIMAL_HLEN as usize..];
            // Decode constructor args directly
            let feeder_address = ActorId::decode(&mut &payload[..]).expect("Bad feeder");
            let network_pid = ActorId::decode(&mut &payload[32..]).expect("Bad network");
            construct(feeder_address, network_pid);
            return;
        }
    }

    // Try SCALE-string route format (vara-wallet v1 compatibility)
    // Format: compact_len(route_str) + route_bytes + args
    let payload = input.as_slice();
    let Ok(route_len) = codec::Compact::<u32>::decode(&mut &payload[..]) else {
        panic!("Cannot decode init payload");
    };
    let route_end = route_len.0 as usize + compact_len_size(payload);
    let route_bytes = &payload[compact_len_size(payload)..route_end];
    let route = core::str::from_utf8(route_bytes).unwrap_or("");

    if route == "New" {
        let args = &payload[route_end..];
        let feeder_address = ActorId::decode(&mut &args[..]).expect("Bad feeder");
        let network_pid = ActorId::decode(&mut &args[32..]).expect("Bad network");
        construct(feeder_address, network_pid);
    } else {
        panic!("Unknown constructor: {}", route);
    }
}

fn construct(feeder: ActorId, network: ActorId) {
    let owner = msg::source();
    let state = RefCell::new(BridgeState {
        feeder_address: feeder,
        owner,
        network_pid: network,
        ..Default::default()
    });
    // Store state globally
    crate::state::set_state(state);
}

// Manual handle function to route service calls
#[no_mangle]
pub unsafe extern "C" fn handle() {
    let input = msg::load_bytes().expect("Failed to load message");

    let state = crate::state::get_state();
    let network_pid = state.borrow().network_pid;
    let mut service = VaraBridgeService::new(&state, network_pid);

    // Try Sails binary header routing first
    if let Ok(header) = SailsMessageHeader::try_from_bytes(&input) {
        if header.interface_id() == InterfaceId::zero() {
            let route_id = header.entry_id();
            let payload = &input[MINIMAL_HLEN as usize..];
            sails_route(route_id, payload, &mut service);
            return;
        }
    }

    // Fallback: SCALE-string route format
    let mut cursor = &input[..];
    let Ok(route_len) = codec::Compact::<u32>::decode(&mut cursor) else {
        panic!("Cannot decode handle payload");
    };
    let route_bytes = &input[compact_len_size(&input)..compact_len_size(&input) + route_len.0 as usize];
    let route = core::str::from_utf8(route_bytes).unwrap_or("");
    let args = &input[compact_len_size(&input) + route_len.0 as usize..];

    match route {
        "UpdateAll" => {
            let payload = FullUpdatePayload::decode(&mut &args[..]).expect("Bad args");
            service.update_all(payload);
        }
        "SetFeeder" => {
            let addr = ActorId::decode(&mut &args[..]).expect("Bad args");
            service.set_feeder(addr);
        }
        _ => panic!("Unknown route: {}", route),
    }
}

fn compact_len_size(data: &[u8]) -> usize {
    match data.first() {
        Some(b) if b & 0x01 == 0 => 1,
        Some(b) if b & 0x02 == 0 => 2,
        Some(_) => 4,
        None => 0,
    }
}

// Sails service is used for routing dispatch and encoding
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
                s.prices.insert(p.symbol.clone(), PriceFeed { updated_at_block: block, ..p });
            }
        }
        if let Some(gas) = payload.gas {
            s.gas = GasFeed { updated_at_block: block, ..gas };
        }
        if let Some(news) = payload.news {
            for n in news {
                if s.news.len() >= MAX_NEWS { s.news.remove(0); }
                s.news.push(n);
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
        self.broadcast_check();
    }

    #[export]
    pub fn set_feeder(&mut self, new_feeder: ActorId) {
        let caller = msg::source();
        let mut s = self.state.borrow_mut();
        if caller != s.owner { panic!("Owner only"); }
        s.feeder_address = new_feeder;
    }

    #[export]
    pub fn get_price(&self, symbol: String) -> Option<PriceFeed> {
        self.state.borrow().prices.get(&symbol.to_uppercase()).cloned()
    }

    #[export]
    pub fn get_gas(&self) -> GasFeed { self.state.borrow().gas.clone() }

    #[export]
    pub fn get_news(&self) -> Vec<NewsSummary> {
        self.state.borrow().news.clone()
    }

    #[export]
    pub fn get_markets(&self) -> Vec<MarketFeed> {
        self.state.borrow().markets.values().cloned().collect()
    }

    #[export]
    pub fn get_datetime(&self) -> DatetimeFeed { self.state.borrow().datetime.clone() }

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
            if key.eq_ignore_ascii_case("news") { snapshot.news = s.news.clone(); }
        }
        snapshot
    }

    #[export]
    pub fn get_stats(&self) -> BridgeStats {
        let s = self.state.borrow();
        BridgeStats {
            query_count: s.query_count, update_count: s.update_count,
            broadcast_count: s.broadcast_count, last_updated_block: s.last_updated_block,
            symbols_tracked: s.prices.len() as u32,
        }
    }

    #[export]
    pub fn query_and_reply(&mut self, request: QueryRequest) -> QueryReply {
        self.state.borrow_mut().query_count += 1;
        let reply = match request.query_type.to_lowercase().as_str() {
            "price" => QueryReply::Price(
                request.symbol.and_then(|s| self.state.borrow().prices.get(&s.to_uppercase()).cloned())),
            "gas" => QueryReply::Gas(self.state.borrow().gas.clone()),
            "news" => QueryReply::News(self.state.borrow().news.clone()),
            "markets" => QueryReply::Markets(self.state.borrow().markets.values().cloned().collect()),
            "datetime" => QueryReply::Datetime(self.state.borrow().datetime.clone()),
            "all" => {
                let s = self.state.borrow();
                QueryReply::All(BridgeSnapshot {
                    prices: s.prices.clone(), gas: s.gas.clone(), news: s.news.clone(),
                    markets: s.markets.clone(), datetime: s.datetime.clone(), block: exec::block_height(),
                })
            }
            _ => QueryReply::Error(format!("Unknown query_type: {}", request.query_type)),
        };
        self.broadcast_check();
        reply
    }
}

// Helper function for Sails binary header route dispatch
fn sails_route(entry_id: u16, _payload: &[u8], _service: &mut VaraBridgeService<'_>) {
    match entry_id {
        0 => { /* UpdateAll would need payload re-decoded */ }
        _ => panic!("Unknown entry_id: {}", entry_id),
    }
}
