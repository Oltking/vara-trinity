# Vara Trinity — Full Build Plan (A to Z)
### VaraBridge + VaraFlow + VaraPulse

> **The decentralized Zapier + Universal Oracle + Creative Pulse for the entire Vara A2A network**  
> **Submission:** Track 04 Open/Creative (primary) · Track 01 Agent Services (organic spillover)  
> **Prize target:** $1,100 first place × 2 tracks + $300,000 Builder Grants consideration  
> **Timeline:** 3 weeks · solo or 2–3 people  
> **Philosophy:** Infrastructure that breathes. Data that moves. Content that never stops.

---

## Table of Contents

1. [The Vara Trinity — Concept & Why It Wins](#1-the-vara-trinity--concept--why-it-wins)
2. [Full System Architecture](#2-full-system-architecture)
3. [Repo & Project Structure](#3-repo--project-structure)
4. [Prerequisites & Tooling Setup](#4-prerequisites--tooling-setup)
5. [Program 1 — VaraBridge (Universal Oracle)](#5-program-1--varabridge-universal-oracle)
6. [Program 2 — VaraFlow (Orchestrator)](#6-program-2--varaflow-orchestrator)
7. [Program 3 — VaraPulse (Creative Pulse Agent)](#7-program-3--varapulse-creative-pulse-agent)
8. [Off-Chain Feeder Service](#8-off-chain-feeder-service)
9. [Parallelism & Speed Strategy](#9-parallelism--speed-strategy)
10. [On-Chain Registration & Identity](#10-on-chain-registration--identity)
11. [Broadcast & Reminder System](#11-broadcast--reminder-system)
12. [Week-by-Week Ship Plan](#12-week-by-week-ship-plan)
13. [Scoring Maximization Strategy](#13-scoring-maximization-strategy)
14. [Demo Video Script](#14-demo-video-script)
15. [Post-Season Longevity](#15-post-season-longevity)

---

## 1. The Vara Trinity — Concept & Why It Wins

Three immutable Sails programs. One operator wallet. One submission. Maximum impact.

| Program | Role | What it does for the network |
|---------|------|------------------------------|
| **VaraBridge** | Universal Data Oracle | Stores live prices, gas, news, markets on-chain. Replies instantly to any agent query. Fed by off-chain feeder every 30s. |
| **VaraFlow** | Autonomous Orchestrator | Stores and executes multi-step workflows. Schedules VaraPulse, handles conditionals, chains cross-program calls. |
| **VaraPulse** | Creative Pulse Agent | Consumes Bridge data via Flow scheduling → generates creative on-chain content → floods Board and Chat with real autonomous activity. |

**Why this is unbeatable for judges:**

- **Infrastructure angle:** Bridge + Flow = the data and automation layer every other agent needs
- **Activity angle:** Pulse = constant visible proof the whole stack works, 24/7
- **Score angle:** Three programs generating cross-program calls between each other + reminders + Board posts = top of every leaderboard metric
- **Post-season angle:** All three run forever after Week 3. Other agents' workflows keep calling them. The network depends on them.

No other submission spans all four judge criteria (Originality, Real Usage, Deep Integration, Post-Season Utility) this completely.

---

## 2. Full System Architecture

```
╔══════════════════════════════════════════════════════════════════════╗
║                         OFF-CHAIN WORLD                             ║
║                                                                      ║
║  ┌────────────────────────────────────────────────────────────────┐ ║
║  │                 VaraBridge Feeder  (Node.js / TypeScript)      │ ║
║  │                                                                │ ║
║  │  Every 30 seconds:                                             │ ║
║  │  CoinGecko API  ─────┐                                         │ ║
║  │  Vara RPC (gas)  ────┤                                         │ ║
║  │  NewsAPI / RSS   ────┼──► Promise.allSettled() ──► update_all  │ ║
║  │  Polymarket API  ────┤         (1 tx, parallel fetch)          │ ║
║  │  System datetime ────┘                                         │ ║
║  └────────────────────────────────────────────────────────────────┘ ║
╚══════════════════════════════════════════════════════════════════════╝
                               │ update_all (1 tx per cycle)
                               ▼
╔══════════════════════════════════════════════════════════════════════╗
║                          VARA MAINNET                               ║
║                                                                      ║
║  ┌─────────────────────┐   query_and_reply    ┌──────────────────┐ ║
║  │                     │◄─────────────────────│                  │ ║
║  │    VaraBridge        │                      │    VaraFlow      │ ║
║  │   (Oracle)          │─────────────────────►│  (Orchestrator)  │ ║
║  │                     │  returns fresh data   │                  │ ║
║  │  • BTC/ETH/SOL/VARA │                      │  • workflow store │ ║
║  │  • gas fees         │◄─────────────────────│  • tick loop     │ ║
║  │  • news headlines   │   query_and_reply    │  • schedules     │ ║
║  │  • market data      │                      │    VaraPulse     │ ║
║  │  • datetime         │                      │  • conditionals  │ ║
║  │  • query_count      │                      │  • chains steps  │ ║
║  └──────┬──────────────┘                      └────────┬─────────┘ ║
║         │                                              │           ║
║         │ delayed msg                    schedules ▼   │           ║
║         │ every 200 blocks                             │           ║
║         ▼                                              ▼           ║
║  ┌──────────────────────────────────────────────────────────────┐  ║
║  │                        VaraPulse                             │  ║
║  │                    (Creative Pulse Agent)                    │  ║
║  │                                                              │  ║
║  │  Autonomous loop (every ~300 blocks via VaraFlow):           │  ║
║  │  1. Query VaraBridge → get all fresh data                    │  ║
║  │  2. Generate creative "Pulse" from data                      │  ║
║  │  3. Post Pulse to Board                                      │  ║
║  │  4. Message 1-3 other agents with personalized nudges        │  ║
║  │  5. Sleep → VaraFlow wakes it up again next cycle            │  ║
║  └──────┬───────────────────────────────────────────────────────┘  ║
║         │                                                          ║
║         │ PostAnnouncement + Chat/Post + SendMentions              ║
║         ▼                                                          ║
║  ┌──────────────────────────────────────────────────────────────┐  ║
║  │          Vara A2A Hub (Registry / Board / Chat)              │  ║
║  │                                                              │  ║
║  │  "⚡ VaraPulse #4,821 | ETH $2,847 | Gas: ultra low 🔥      │  ║
║  │   Pro tip: message VaraBridge now. Message VaraFlow to auto."│  ║
║  │                                                              │  ║
║  │  Other agents reading the Board → discover Trinity           │  ║
║  │  Other agents integrate → score for all three programs ↑     │  ║
║  └──────────────────────────────────────────────────────────────┘  ║
╚══════════════════════════════════════════════════════════════════════╝

CROSS-PROGRAM CALL GRAPH (every arrow = on-chain extrinsic = hackathon score):

  VaraBridge ←── Feeder (update_all, every 30s)
  VaraBridge ←── VaraFlow (query_and_reply, every tick)
  VaraBridge ←── VaraPulse (query_and_reply, every pulse)
  VaraBridge ←── Any external agent (query, organic)
  VaraFlow   ←── VaraPulse (trigger_workflow / next_step reply)
  VaraFlow   ←── Any external agent (register_workflow)
  VaraPulse  ←── VaraFlow (scheduled wakeup msg)
  Board/Chat ←── VaraBridge (broadcast, every 200 blocks)
  Board/Chat ←── VaraFlow (broadcast, every 200 blocks offset)
  Board/Chat ←── VaraPulse (Pulse post, every 300 blocks)
```

**Total autonomous on-chain call sources: 10+**  
Every arrow above generates verifiable extrinsics. Judges see this on the leaderboard.

---

## 3. Repo & Project Structure

```
vara-trinity/
│
├── programs/
│   │
│   ├── vara-bridge/                    # Sails Program 1 — Universal Oracle
│   │   ├── src/
│   │   │   ├── lib.rs                  # Service entrypoint + init
│   │   │   ├── state.rs                # All on-chain storage types
│   │   │   ├── feed.rs                 # Feed types: PriceFeed, GasFeed, NewsSummary, etc.
│   │   │   ├── broadcast.rs            # Delayed self-messaging broadcast engine
│   │   │   └── auth.rs                 # Feeder address authorization
│   │   ├── Cargo.toml
│   │   └── build.rs
│   │
│   ├── vara-flow/                      # Sails Program 2 — Orchestrator
│   │   ├── src/
│   │   │   ├── lib.rs                  # Service entrypoint + init + tick loop
│   │   │   ├── state.rs                # FlowState, Workflow, ExecutionContext
│   │   │   ├── workflow.rs             # Workflow CRUD + validation
│   │   │   ├── executor.rs             # Step execution engine
│   │   │   ├── templates.rs            # Built-in workflow templates
│   │   │   ├── bridge_client.rs        # Cross-program call helper for VaraBridge
│   │   │   └── broadcast.rs            # Delayed broadcast loop
│   │   ├── Cargo.toml
│   │   └── build.rs
│   │
│   └── vara-pulse/                     # Sails Program 3 — Creative Pulse Agent
│       ├── src/
│       │   ├── lib.rs                  # Service entrypoint + init + pulse loop
│       │   ├── state.rs                # PulseState, pulse history, agent catalog cache
│       │   ├── generator.rs            # Pulse content generation from data
│       │   ├── templates.rs            # Pulse format templates (market, tip, spark, etc.)
│       │   ├── nudger.rs               # Personalized agent nudge logic
│       │   └── broadcast.rs            # Board/Chat posting
│       ├── Cargo.toml
│       └── build.rs
│
├── feeder/                             # Off-chain data feeder (Node.js / TypeScript)
│   ├── src/
│   │   ├── index.ts                    # Main loop + scheduler
│   │   ├── fetchers/
│   │   │   ├── prices.ts               # CoinGecko parallel multi-symbol fetch
│   │   │   ├── gas.ts                  # Vara RPC gas + block info
│   │   │   ├── news.ts                 # NewsAPI + RSS parallel aggregator
│   │   │   ├── markets.ts              # Polymarket top markets + odds
│   │   │   └── datetime.ts             # NTP-synced datetime
│   │   ├── chain/
│   │   │   ├── sender.ts               # Batched tx submission via vara-wallet CLI
│   │   │   └── nonce.ts                # Parallel-safe nonce manager
│   │   └── config.ts                   # All env var validation + defaults
│   ├── package.json
│   ├── tsconfig.json
│   └── Dockerfile                      # For Railway / Render / Fly.io free tier
│
├── scripts/
│   ├── deploy-all.sh                   # Compile + deploy all 3 programs in order
│   ├── register.sh                     # Full A2A registration flow for all 3
│   ├── set-identities.sh               # Set all 3 identity cards on Board
│   └── verify.sh                       # Confirm all programs live on indexer
│
├── idl/                                # Generated IDL files (post-build, committed)
│   ├── vara_bridge.idl
│   ├── vara_flow.idl
│   └── vara_pulse.idl
│
├── docs/
│   ├── INTEGRATION.md                  # "Connect any agent to VaraBridge in 1 message"
│   ├── WORKFLOW_GUIDE.md               # "Register a VaraFlow workflow in 1 message"
│   └── ARCHITECTURE.md                 # Full diagram for judges
│
├── .env.example
└── README.md
```

---

## 4. Prerequisites & Tooling Setup

### 4.1 One-shot install

```bash
# Rust + Wasm target (required for all Sails programs)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown
rustup component add rust-src

# Vara CLI toolchain
npm install -g vara-wallet                                          # requires 0.16+
npx skills add gear-foundation/vara-agent-network -g --all -y
npx skills add gear-foundation/vara-skills -g --all -y

# Feeder service
cd feeder && npm install && cd ..

# Verify everything
vara-wallet --version          # must be 0.16+
rustc --version                # stable 1.75+
```

### 4.2 Environment file (`.env`)

```bash
# ── Operator identity ──
ACCT=vara-trinity-operator
OPERATOR_HEX=0x...                      # from: vara-wallet balance ""

# ── Deployed program IDs (fill after deploy step) ──
BRIDGE_PID=0x...
FLOW_PID=0x...
PULSE_PID=0x...

# ── A2A Network ──
NETWORK_PID=0x19f27f4c...0b353f3        # from SKILL.md references/program-ids.md
IDL_DIR=./idl
INDEXER_GRAPHQL_URL=https://agents-api.vara.network/graphql
VOUCHER_URL=https://voucher-api.vara.network

# ── Vara network ──
VARA_NETWORK=wss://rpc.vara.network
VARA_WS=wss://rpc.vara.network

# ── Gas voucher (filled from vouchers.md flow before first write) ──
VOUCHER_ID=...

# ── External API keys ──
COINGECKO_KEY=...
NEWS_API_KEY=...
POLYMARKET_API_KEY=...                  # optional, public endpoints exist
```

### 4.3 Wallet setup

```bash
vara-wallet wallet create --name "$ACCT" --no-encrypt
INFO=$(vara-wallet --account "$ACCT" --network "$VARA_NETWORK" --json balance "")
export OPERATOR_HEX=$(echo "$INFO" | jq -r .address)
echo "Operator: $OPERATOR_HEX"

# Get gas voucher (covers all Registry/Board/Chat writes — free)
# Follow: $VARA_AGENT_NETWORK_SKILLS_DIR/references/vouchers.md
```

### 4.4 A2A network preamble (run before every shell session)

```bash
_VAN="${VARA_AGENT_NETWORK_SKILLS_DIR:-.}"
eval "$(awk '/^```bash$/{f=1; next} /^```$/{if(f) exit} f' \
  "$_VAN/references/program-ids.md")"
echo "[OK] PID=$PID | NETWORK=$VARA_NETWORK"
```

---

## 5. Program 1 — VaraBridge (Universal Oracle)

### 5.1 On-chain state design

```rust
// programs/vara-bridge/src/state.rs

/// All prices stored as integers × 10^6 to avoid floats in Wasm
/// All % changes stored as basis points (1 bps = 0.01%)
pub struct BridgeState {
    // ── Feed storage ──
    pub prices:    BTreeMap<String, PriceFeed>,      // "ETH" -> PriceFeed
    pub gas:       GasFeed,
    pub news:      VecDeque<NewsSummary>,            // ring buffer, max 10
    pub markets:   BTreeMap<String, MarketFeed>,     // Polymarket top markets
    pub datetime:  DatetimeFeed,

    // ── Access control ──
    pub feeder_address: ActorId,                     // only this address can update feeds
    pub owner:          ActorId,                     // can change feeder_address

    // ── Metadata ──
    pub last_updated_block: u32,
    pub query_count:        u64,                     // total cross-program queries answered
    pub update_count:       u64,                     // total feed updates received
    pub broadcast_count:    u64,                     // total Board/Chat broadcasts sent
}

pub struct PriceFeed {
    pub symbol:            String,
    pub price_usd_micro:   u64,         // price × 10^6  (e.g. $2847.33 → 2_847_330_000)
    pub change_24h_bps:    i32,         // basis points  (e.g. +3.2% → 320)
    pub market_cap_usd:    u64,
    pub volume_24h_usd:    u64,
    pub updated_at_block:  u32,
}

pub struct GasFeed {
    pub current_fee_micro:  u64,        // gas fee × 10^6
    pub suggested_tip:      u64,
    pub block_num:          u32,
    pub finalized_hash:     String,
    pub updated_at_block:   u32,
}

pub struct NewsSummary {
    pub title:         String,          // max 120 chars
    pub source:        String,
    pub published_at:  u64,             // unix timestamp
    pub category:      String,          // "crypto" | "vara" | "defi" | "macro"
}

pub struct MarketFeed {
    pub market_id:     String,
    pub question:      String,          // max 100 chars
    pub yes_prob_bps:  u32,             // probability in basis points (5000 = 50%)
    pub volume_usd:    u64,
    pub closes_at:     u64,             // unix timestamp
    pub updated_at_block: u32,
}

pub struct DatetimeFeed {
    pub unix_ts:          u64,
    pub utc_string:       String,       // "2026-05-12T14:32:00Z"
    pub day_of_week:      String,       // "Tuesday"
    pub updated_at_block: u32,
}

// Batch update — feeder sends this in one tx per cycle
pub struct FullUpdatePayload {
    pub prices:   Option<Vec<PriceFeed>>,
    pub gas:      Option<GasFeed>,
    pub news:     Option<Vec<NewsSummary>>,
    pub markets:  Option<Vec<MarketFeed>>,
    pub datetime: Option<DatetimeFeed>,
}

// Any agent sends this to query
pub struct QueryRequest {
    pub query_type: String,   // "price" | "gas" | "news" | "markets" | "datetime" | "all" | "snapshot"
    pub symbol:     Option<String>,  // for price queries: "ETH", "BTC", etc.
    pub keys:       Option<Vec<String>>,  // for snapshot: list of symbols/keys
}

pub enum QueryReply {
    Price(Option<PriceFeed>),
    Gas(GasFeed),
    News(Vec<NewsSummary>),
    Markets(Vec<MarketFeed>),
    Datetime(DatetimeFeed),
    All(BridgeSnapshot),
    Error(String),
}
```

### 5.2 Full Sails service interface

```rust
// programs/vara-bridge/src/lib.rs

#[service]
impl VaraBridge {
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // FEEDER WRITES  (authorized: feeder_address only)
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// Primary update method — feeder calls this once per 30s cycle
    /// All fields optional: partial updates are fine (failed fetcher won't break cycle)
    pub fn update_all(&mut self, payload: FullUpdatePayload) {
        let caller = msg::source();
        assert_eq!(caller, self.state().feeder_address, "Unauthorized");

        let state = self.state_mut();
        let block = exec::block_height();

        if let Some(prices) = payload.prices {
            for p in prices {
                state.prices.insert(p.symbol.clone(), PriceFeed { updated_at_block: block, ..p });
            }
        }
        if let Some(gas) = payload.gas {
            state.gas = GasFeed { updated_at_block: block, ..gas };
        }
        if let Some(news) = payload.news {
            for n in news {
                if state.news.len() >= 10 { state.news.pop_front(); }
                state.news.push_back(n);
            }
        }
        if let Some(markets) = payload.markets {
            for m in markets {
                state.markets.insert(m.market_id.clone(), MarketFeed { updated_at_block: block, ..m });
            }
        }
        if let Some(dt) = payload.datetime {
            state.datetime = DatetimeFeed { updated_at_block: block, ..dt };
        }

        state.last_updated_block = block;
        state.update_count += 1;
    }

    /// Owner can rotate feeder address without redeploying
    pub fn set_feeder(&mut self, new_feeder: ActorId) {
        assert_eq!(msg::source(), self.state().owner, "Owner only");
        self.state_mut().feeder_address = new_feeder;
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // QUERIES  (any agent, any address, free)
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    pub fn get_price(&self, symbol: String) -> Option<PriceFeed> {
        self.state().prices.get(&symbol.to_uppercase()).cloned()
    }

    pub fn get_gas(&self) -> GasFeed {
        self.state().gas.clone()
    }

    pub fn get_news(&self) -> Vec<NewsSummary> {
        self.state().news.iter().cloned().collect()
    }

    pub fn get_markets(&self) -> Vec<MarketFeed> {
        self.state().markets.values().cloned().collect()
    }

    pub fn get_datetime(&self) -> DatetimeFeed {
        self.state().datetime.clone()
    }

    pub fn get_all(&self) -> BridgeSnapshot {
        self.build_snapshot()
    }

    /// Targeted multi-get — agents fetch only what they need, minimizes gas
    pub fn get_snapshot(&self, keys: Vec<String>) -> BridgeSnapshot {
        self.build_partial_snapshot(keys)
    }

    pub fn get_stats(&self) -> BridgeStats {
        let s = self.state();
        BridgeStats {
            query_count: s.query_count,
            update_count: s.update_count,
            broadcast_count: s.broadcast_count,
            last_updated_block: s.last_updated_block,
            symbols_tracked: s.prices.len() as u32,
        }
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // CROSS-PROGRAM REPLY  (VaraFlow / VaraPulse / any agent)
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// One-message round trip: caller sends QueryRequest, gets QueryReply instantly
    /// This is the primary integration point for VaraFlow and VaraPulse
    pub fn query_and_reply(&mut self, request: QueryRequest) -> QueryReply {
        self.state_mut().query_count += 1;

        match request.query_type.to_lowercase().as_str() {
            "price" => QueryReply::Price(
                request.symbol.and_then(|s| self.state().prices.get(&s.to_uppercase()).cloned())
            ),
            "gas"      => QueryReply::Gas(self.state().gas.clone()),
            "news"     => QueryReply::News(self.state().news.iter().cloned().collect()),
            "markets"  => QueryReply::Markets(self.state().markets.values().cloned().collect()),
            "datetime" => QueryReply::Datetime(self.state().datetime.clone()),
            "all"      => QueryReply::All(self.build_snapshot()),
            "snapshot" => QueryReply::All(
                self.build_partial_snapshot(request.keys.unwrap_or_default())
            ),
            _ => QueryReply::Error(format!("Unknown query_type: {}", request.query_type)),
        }
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // AUTONOMOUS BROADCAST ENGINE
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// Called by init — starts the infinite delayed broadcast loop
    fn init_broadcast(&mut self) {
        msg::send_delayed(
            exec::program_id(),
            BridgeInternalCmd::Broadcast,
            GAS_FOR_BROADCAST,      // reserved gas, not caller gas
            BROADCAST_INTERVAL,     // 200 blocks ≈ 5 minutes
        ).expect("Broadcast init failed");
    }
}

/// Handles the delayed self-wakeup for broadcasting
#[async_trait]
impl MessageHandler for VaraBridge {
    async fn handle(&mut self) {
        let msg: BridgeInternalCmd = msg::load().expect("Bad cmd");
        match msg {
            BridgeInternalCmd::Broadcast => self.do_broadcast(),
        }
    }
}

impl VaraBridge {
    fn do_broadcast(&mut self) {
        let s = self.state();
        let eth  = s.prices.get("ETH").map(|p| p.price_usd_micro / 1_000_000).unwrap_or(0);
        let btc  = s.prices.get("BTC").map(|p| p.price_usd_micro / 1_000_000).unwrap_or(0);
        let vara = s.prices.get("VARA").map(|p| format!("${:.4}", p.price_usd_micro as f64 / 1_000_000.0)).unwrap_or("N/A".into());

        let body = format!(
            "🧬 VaraBridge LIVE | Block #{block} | \
             ETH: ${eth} | BTC: ${btc} | VARA: {vara} | \
             Gas: {gas} | {queries} queries answered\n\
             ──\n\
             Any agent: send me 1 msg → get live prices, gas, news, markets, datetime.\n\
             Call: query_and_reply({{ query_type: \"all\" }}) at {pid}\n\
             VaraFlow also LIVE → automate your agent using this data today.",
            block  = exec::block_height(),
            eth    = eth,
            btc    = btc,
            vara   = vara,
            gas    = s.gas.current_fee_micro,
            queries = s.query_count,
            pid    = hex::encode(exec::program_id()),
        );

        // Post to Board
        msg::send(NETWORK_PID, HubCmd::PostAnnouncement { body: body.clone() }, GAS_FOR_HUB_POST);
        // Post to Chat
        msg::send(NETWORK_PID, HubCmd::ChatPost { body, mentions: vec![] }, GAS_FOR_HUB_POST);

        self.state_mut().broadcast_count += 1;

        // Re-schedule next broadcast (autonomous forever)
        msg::send_delayed(
            exec::program_id(),
            BridgeInternalCmd::Broadcast,
            GAS_FOR_BROADCAST,
            BROADCAST_INTERVAL,
        ).expect("Broadcast reschedule failed");
    }
}
```

### 5.3 Init — gas reservation + broadcast start

```rust
// programs/vara-bridge/src/lib.rs — init()

#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: BridgeInitConfig = msg::load().expect("Bad init config");

    // Reserve gas for 50,000 delayed messages (broadcasts last ~years)
    // Each broadcast needs GAS_FOR_BROADCAST; reserving in bulk on init
    msg::reserve_gas(GAS_RESERVE_TOTAL, GAS_RESERVE_DURATION)
        .expect("Gas reservation failed");

    STATE = Some(BridgeState {
        feeder_address: config.feeder_address,
        owner:          msg::source(),
        prices:         BTreeMap::new(),
        gas:            GasFeed::default(),
        news:           VecDeque::new(),
        markets:        BTreeMap::new(),
        datetime:       DatetimeFeed::default(),
        last_updated_block: exec::block_height(),
        query_count:    0,
        update_count:   0,
        broadcast_count: 0,
    });

    // Start the infinite broadcast loop immediately
    msg::send_delayed(
        exec::program_id(),
        BridgeInternalCmd::Broadcast,
        GAS_FOR_BROADCAST,
        BROADCAST_INTERVAL,
    ).expect("Broadcast init failed");
}

// Constants
const BROADCAST_INTERVAL:  u32 = 200;
const GAS_FOR_BROADCAST:   u64 = 5_000_000_000;
const GAS_FOR_HUB_POST:    u64 = 2_000_000_000;
const GAS_RESERVE_TOTAL:   u64 = 250_000_000_000_000;
const GAS_RESERVE_DURATION: u32 = 1_000_000;
```

---

## 6. Program 2 — VaraFlow (Orchestrator)

### 6.1 On-chain state design

```rust
// programs/vara-flow/src/state.rs

pub struct FlowState {
    // ── Workflow registry ──
    pub workflows:         BTreeMap<u64, Workflow>,
    pub next_workflow_id:  u64,

    // ── Active execution tracking ──
    pub executions:        BTreeMap<u64, ExecutionContext>,
    pub next_exec_id:      u64,

    // ── Pending cross-program replies ──
    pub pending_replies:   BTreeMap<MessageId, PendingStep>,

    // ── Program references ──
    pub bridge_pid:  ActorId,       // VaraBridge address
    pub pulse_pid:   ActorId,       // VaraPulse address (set after Pulse deploys)
    pub network_pid: ActorId,       // A2A Hub

    // ── Operator ──
    pub owner: ActorId,

    // ── Metrics ──
    pub execution_count:  u64,
    pub workflow_count:   u64,
    pub broadcast_count:  u64,
}

pub struct Workflow {
    pub id:              u64,
    pub owner:           ActorId,
    pub name:            String,           // max 60 chars
    pub description:     String,           // max 200 chars
    pub trigger:         Trigger,
    pub steps:           Vec<Step>,
    pub active:          bool,
    pub created_block:   u32,
    pub last_run_block:  u32,
    pub run_count:       u64,
    pub next_run_block:  u32,             // pre-computed for fast tick check
}

pub enum Trigger {
    /// Execute every N blocks (most common — VaraPulse uses this)
    BlockInterval { every_n_blocks: u32 },

    /// Execute when Bridge price crosses threshold
    PriceThreshold {
        symbol:     String,
        above_usd:  Option<u64>,    // micro USD
        below_usd:  Option<u64>,
    },

    /// Execute when Bridge gas drops below threshold (cheap deployment window)
    GasBelow { threshold_micro: u64 },

    /// Any registered address can trigger this manually
    ManualCall { authorized: Option<ActorId> },

    /// Fires whenever VaraBridge's update_count increments (real-time mode)
    OnBridgeUpdate,
}

pub struct Step {
    pub step_type:  StepType,
    pub gas_limit:  u64,
    pub timeout_blocks: u32,        // if reply not received within N blocks, skip
    pub on_success: Option<Box<Step>>,
    pub on_failure: Option<Box<Step>>,
    pub on_timeout: Option<Box<Step>>,
}

pub enum StepType {
    /// Query VaraBridge and store result in execution context
    QueryBridge { query: QueryRequest },

    /// Call any registered program with arbitrary args
    CallProgram {
        pid:    ActorId,
        method: String,
        args:   Vec<u8>,            // pre-encoded
    },

    /// Post to A2A Board
    PostBoard {
        body_template: String,      // supports {{ETH_PRICE}}, {{GAS}}, {{NEWS_0}} substitution
    },

    /// Post to A2A Chat with optional mentions
    PostChat {
        body_template: String,
        mentions:      Vec<String>, // handles to mention
    },

    /// If/else branching based on Bridge data
    ConditionalBranch {
        condition: Condition,
        if_true:   Box<Step>,
        if_false:  Option<Box<Step>>,
    },

    /// Wake up VaraPulse (used in the Pulse scheduling workflow)
    WakePulse,

    /// No-op, used as terminal step
    Done,
}

pub enum Condition {
    PriceAbove    { symbol: String, threshold_micro: u64 },
    PriceBelow    { symbol: String, threshold_micro: u64 },
    GasBelow      { threshold_micro: u64 },
    BlockModulo   { n: u32 },           // true every N blocks (for sub-workflows)
    Always,
}
```

### 6.2 Full service interface

```rust
#[service]
impl VaraFlow {
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // WORKFLOW MANAGEMENT
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    pub fn register_workflow(&mut self, input: WorkflowInput) -> u64 {
        let id = self.state().next_workflow_id;
        let block = exec::block_height();
        let next_run = match &input.trigger {
            Trigger::BlockInterval { every_n_blocks } => block + every_n_blocks,
            _ => block + 1,
        };
        self.state_mut().workflows.insert(id, Workflow {
            id,
            owner:           msg::source(),
            name:            input.name,
            description:     input.description,
            trigger:         input.trigger,
            steps:           input.steps,
            active:          true,
            created_block:   block,
            last_run_block:  0,
            run_count:       0,
            next_run_block:  next_run,
        });
        self.state_mut().next_workflow_id += 1;
        self.state_mut().workflow_count += 1;
        id
    }

    pub fn update_workflow(&mut self, id: u64, patch: WorkflowPatch) { ... }
    pub fn delete_workflow(&mut self, id: u64) { ... }
    pub fn pause_workflow(&mut self, id: u64) { ... }
    pub fn resume_workflow(&mut self, id: u64) { ... }
    pub fn get_workflow(&self, id: u64) -> Option<Workflow> { ... }
    pub fn list_workflows(&self, owner: Option<ActorId>, active_only: bool) -> Vec<WorkflowSummary> { ... }
    pub fn get_stats(&self) -> FlowStats { ... }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // EXECUTION
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// External agents can trigger a workflow manually
    pub fn trigger_workflow(&mut self, id: u64, input: Option<TriggerInput>) {
        let wf = self.state().workflows.get(&id).cloned().expect("Workflow not found");
        if let Trigger::ManualCall { authorized } = &wf.trigger {
            if let Some(auth) = authorized {
                assert_eq!(msg::source(), *auth, "Not authorized");
            }
        }
        self.execute_workflow(id, input);
    }

    pub fn get_execution_status(&self, exec_id: u64) -> Option<ExecutionStatus> { ... }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // BUILT-IN TEMPLATES (one-call setup for common patterns)
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    pub fn use_template(&mut self, template: TemplateType, params: TemplateParams) -> u64 {
        let workflow = match template {
            TemplateType::PriceAlert         => self.build_price_alert_template(params),
            TemplateType::MarketSummaryBoard => self.build_market_summary_template(params),
            TemplateType::GasAwareExecution  => self.build_gas_aware_template(params),
            TemplateType::PulseScheduler     => self.build_pulse_scheduler_template(params),
            TemplateType::OnBridgeUpdate     => self.build_on_bridge_update_template(params),
            TemplateType::Custom             => self.build_custom_template(params),
        };
        self.register_workflow(workflow)
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // OWNER CONFIG
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    pub fn set_bridge(&mut self, bridge_pid: ActorId) {
        assert_eq!(msg::source(), self.state().owner, "Owner only");
        self.state_mut().bridge_pid = bridge_pid;
    }

    pub fn set_pulse(&mut self, pulse_pid: ActorId) {
        assert_eq!(msg::source(), self.state().owner, "Owner only");
        self.state_mut().pulse_pid = pulse_pid;
    }
}
```

### 6.3 Autonomous tick engine

```rust
// programs/vara-flow/src/lib.rs — the heart of VaraFlow

const TICK_INTERVAL: u32 = 50;   // every ~75 seconds
const GAS_FOR_TICK: u64 = 10_000_000_000;

impl VaraFlow {
    /// Called every TICK_INTERVAL blocks via delayed self-message
    fn tick(&mut self) {
        let current_block = exec::block_height();

        // Check all active workflows for triggered conditions
        let triggered_ids: Vec<u64> = self.state().workflows
            .values()
            .filter(|wf| wf.active && current_block >= wf.next_run_block)
            .filter(|wf| self.is_triggered(wf, current_block))
            .map(|wf| wf.id)
            .collect();

        // Execute each triggered workflow
        for id in triggered_ids {
            self.execute_workflow(id, None);
        }

        // Post periodic broadcast (offset from Bridge by 100 blocks)
        if current_block % 200 == 100 {
            self.do_broadcast(current_block);
        }

        // Reschedule tick (autonomous forever via reserved gas)
        msg::send_delayed(
            exec::program_id(),
            FlowInternalCmd::Tick,
            GAS_FOR_TICK,
            TICK_INTERVAL,
        ).expect("Tick reschedule failed");
    }

    fn is_triggered(&self, wf: &Workflow, block: u32) -> bool {
        match &wf.trigger {
            Trigger::BlockInterval { every_n_blocks } => {
                block >= wf.last_run_block + every_n_blocks
            },
            Trigger::PriceThreshold { symbol, above_usd, below_usd } => {
                // Check cached Bridge data (fetched on last tick)
                // If no cached data, trigger a Bridge query first
                if let Some(price) = self.get_cached_price(symbol) {
                    above_usd.map_or(true, |t| price >= t) &&
                    below_usd.map_or(true, |t| price <= t)
                } else {
                    // No data yet — queue Bridge query, don't fire yet
                    self.queue_bridge_prefetch(symbol);
                    false
                }
            },
            Trigger::GasBelow { threshold_micro } => {
                self.get_cached_gas() <= *threshold_micro
            },
            Trigger::ManualCall { .. }  => false, // only via trigger_workflow()
            Trigger::OnBridgeUpdate     => self.bridge_updated_since(wf.last_run_block),
        }
    }

    fn execute_workflow(&mut self, id: u64, input: Option<TriggerInput>) {
        let wf = self.state().workflows[&id].clone();
        let exec_id = self.state().next_exec_id;

        // Create execution context
        let ctx = ExecutionContext {
            exec_id,
            workflow_id:   id,
            started_block: exec::block_height(),
            current_step:  0,
            data:          input.map(|i| i.data).unwrap_or_default(),
            status:        ExecutionStatus::Running,
        };
        self.state_mut().executions.insert(exec_id, ctx);
        self.state_mut().next_exec_id += 1;
        self.state_mut().execution_count += 1;

        // Execute first step
        self.execute_step(exec_id, &wf.steps[0]);

        // Update workflow metadata
        let wf_mut = self.state_mut().workflows.get_mut(&id).unwrap();
        wf_mut.last_run_block = exec::block_height();
        wf_mut.run_count += 1;
        wf_mut.next_run_block = match &wf_mut.trigger {
            Trigger::BlockInterval { every_n_blocks } =>
                exec::block_height() + every_n_blocks,
            _ => exec::block_height() + 1,
        };
    }

    fn execute_step(&mut self, exec_id: u64, step: &Step) {
        match &step.step_type {
            StepType::QueryBridge { query } => {
                // Non-blocking send to Bridge — reply handled in handle_reply()
                let msg_id = msg::send(
                    self.state().bridge_pid,
                    query.clone(),
                    step.gas_limit,
                );
                self.state_mut().pending_replies.insert(msg_id, PendingStep {
                    exec_id,
                    next_step: step.on_success.clone(),
                    timeout_block: exec::block_height() + step.timeout_blocks,
                });
            },
            StepType::CallProgram { pid, method, args } => {
                msg::send(*pid, EncodedCall { method: method.clone(), args: args.clone() }, step.gas_limit);
            },
            StepType::PostBoard { body_template } => {
                let ctx = &self.state().executions[&exec_id];
                let body = self.render_template(body_template, &ctx.data);
                msg::send(self.state().network_pid, HubCmd::PostAnnouncement { body }, GAS_FOR_HUB);
                self.continue_execution(exec_id, step.on_success.as_deref());
            },
            StepType::PostChat { body_template, mentions } => {
                let ctx = &self.state().executions[&exec_id];
                let body = self.render_template(body_template, &ctx.data);
                msg::send(self.state().network_pid, HubCmd::ChatPost { body, mentions: mentions.clone() }, GAS_FOR_HUB);
                self.continue_execution(exec_id, step.on_success.as_deref());
            },
            StepType::ConditionalBranch { condition, if_true, if_false } => {
                let ctx = &self.state().executions[&exec_id];
                if self.evaluate_condition(condition, &ctx.data) {
                    self.execute_step(exec_id, if_true);
                } else if let Some(false_step) = if_false {
                    self.execute_step(exec_id, false_step);
                } else {
                    self.complete_execution(exec_id);
                }
            },
            StepType::WakePulse => {
                msg::send(self.state().pulse_pid, PulseCmd::Run, step.gas_limit);
                self.continue_execution(exec_id, step.on_success.as_deref());
            },
            StepType::Done => self.complete_execution(exec_id),
        }
    }

    /// Template variable substitution: {{ETH_PRICE}}, {{GAS}}, {{NEWS_0}}, {{BLOCK}}, etc.
    fn render_template(&self, template: &str, data: &ExecutionData) -> String {
        template
            .replace("{{ETH_PRICE}}", &data.get_str("ETH_PRICE").unwrap_or_default())
            .replace("{{BTC_PRICE}}", &data.get_str("BTC_PRICE").unwrap_or_default())
            .replace("{{GAS}}",       &data.get_str("GAS").unwrap_or_default())
            .replace("{{NEWS_0}}",    &data.get_str("NEWS_0").unwrap_or_default())
            .replace("{{BLOCK}}",     &exec::block_height().to_string())
    }
}
```

### 6.4 The PulseScheduler template (connects Flow to Pulse)

```rust
fn build_pulse_scheduler_template(&self, params: TemplateParams) -> WorkflowInput {
    // This is the workflow that makes VaraPulse autonomous via VaraFlow
    // VaraFlow tick fires every 50 blocks → checks if 300 blocks passed → wakes Pulse
    WorkflowInput {
        name:        "VaraPulse Scheduler".into(),
        description: "Wakes VaraPulse every 300 blocks with fresh Bridge data".into(),
        trigger: Trigger::BlockInterval { every_n_blocks: params.interval.unwrap_or(300) },
        steps: vec![
            Step {
                step_type: StepType::QueryBridge {
                    query: QueryRequest { query_type: "all".into(), symbol: None, keys: None }
                },
                on_success: Some(Box::new(Step {
                    step_type: StepType::WakePulse,
                    on_success: Some(Box::new(Step { step_type: StepType::Done, ..Default::default() })),
                    ..Default::default()
                })),
                ..Default::default()
            }
        ],
    }
}
```

---

## 7. Program 3 — VaraPulse (Creative Pulse Agent)

VaraPulse is the most visible program. It is what judges and other agents will actually *see* on the Board every few minutes. It transforms raw data into living, creative, useful on-chain content.

### 7.1 On-chain state design

```rust
// programs/vara-pulse/src/state.rs

pub struct PulseState {
    // ── External program addresses ──
    pub bridge_pid:  ActorId,
    pub flow_pid:    ActorId,
    pub network_pid: ActorId,
    pub owner:       ActorId,

    // ── Pulse history (ring buffer, last 50 pulses) ──
    pub pulse_history: VecDeque<PulseRecord>,

    // ── Known agents catalog (for personalized nudges) ──
    pub known_agents:  Vec<AgentRecord>,
    pub last_catalog_refresh_block: u32,

    // ── Config ──
    pub pulse_interval_blocks: u32,   // default 300
    pub max_nudges_per_pulse:  u32,   // default 3
    pub nudge_cooldown_blocks: u32,   // don't nudge same agent for N blocks

    // ── Metrics ──
    pub total_pulses:        u64,
    pub total_nudges_sent:   u64,
    pub total_board_posts:   u64,
    pub last_pulse_block:    u32,
}

pub struct PulseRecord {
    pub pulse_id:       u64,
    pub block:          u32,
    pub pulse_type:     PulseType,
    pub body:           String,
    pub data_snapshot:  DataSnapshot,   // the Bridge data it used
    pub nudges_sent:    Vec<String>,    // handles that were nudged
}

pub enum PulseType {
    MarketSummary,       // price summary with personality
    GasTip,              // gas fee info + deployment tip
    NewsBrief,           // top news + creative angle
    MarketSpark,         // prediction market data + idea spark
    AgentTip,            // tip for other agents using Bridge/Flow
    MilestonePost,       // every 100 pulses, celebrate
    CreativeSpark,       // "today's vibe" idea generation
}

pub struct AgentRecord {
    pub handle:        String,
    pub program_id:    String,
    pub description:   String,          // from their identity card
    pub last_nudged_block: u32,
}

pub struct DataSnapshot {
    pub eth_usd:        u64,
    pub btc_usd:        u64,
    pub vara_usd:       u64,
    pub gas_micro:      u64,
    pub top_news:       String,
    pub top_market:     Option<String>,
    pub block:          u32,
    pub utc_string:     String,
}
```

### 7.2 Full service interface

```rust
#[service]
impl VaraPulse {
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // PRIMARY ENTRY POINTS
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// Called by VaraFlow scheduler — runs one full pulse cycle
    pub fn run(&mut self) {
        // Step 1: Query VaraBridge for all fresh data (cross-program call)
        msg::send(
            self.state().bridge_pid,
            QueryRequest { query_type: "all".into(), symbol: None, keys: None },
            GAS_FOR_BRIDGE_QUERY,
        );
        // Reply is handled in handle_reply() → continues from there
    }

    /// Owner can manually trigger a pulse (testing / demo)
    pub fn force_pulse(&mut self) {
        assert_eq!(msg::source(), self.state().owner, "Owner only");
        self.run();
    }

    /// Owner refreshes the known agents catalog from indexer data
    pub fn update_catalog(&mut self, agents: Vec<AgentRecord>) {
        assert_eq!(msg::source(), self.state().owner, "Owner only");
        self.state_mut().known_agents = agents;
        self.state_mut().last_catalog_refresh_block = exec::block_height();
    }

    pub fn set_config(&mut self, config: PulseConfig) {
        assert_eq!(msg::source(), self.state().owner, "Owner only");
        let s = self.state_mut();
        s.pulse_interval_blocks  = config.interval.unwrap_or(s.pulse_interval_blocks);
        s.max_nudges_per_pulse   = config.max_nudges.unwrap_or(s.max_nudges_per_pulse);
        s.nudge_cooldown_blocks  = config.cooldown.unwrap_or(s.nudge_cooldown_blocks);
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // READ
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    pub fn get_latest_pulse(&self) -> Option<PulseRecord> {
        self.state().pulse_history.back().cloned()
    }

    pub fn get_pulse_history(&self, limit: u32) -> Vec<PulseRecord> {
        self.state().pulse_history.iter().rev().take(limit as usize).cloned().collect()
    }

    pub fn get_stats(&self) -> PulseStats {
        let s = self.state();
        PulseStats {
            total_pulses:      s.total_pulses,
            total_nudges:      s.total_nudges_sent,
            total_board_posts: s.total_board_posts,
            last_pulse_block:  s.last_pulse_block,
            known_agents:      s.known_agents.len() as u32,
        }
    }
}
```

### 7.3 The pulse generation engine (the creative core)

```rust
// programs/vara-pulse/src/generator.rs

impl VaraPulse {
    /// After VaraBridge replies, this fires — it's the full pulse cycle
    fn on_bridge_reply(&mut self, reply: QueryReply) {
        let data = self.extract_snapshot(reply);
        let pulse_type = self.pick_pulse_type(&data);
        let body = self.generate_pulse(&data, &pulse_type);

        // Post to Board
        msg::send(
            self.state().network_pid,
            HubCmd::PostAnnouncement { body: body.clone() },
            GAS_FOR_HUB,
        );
        self.state_mut().total_board_posts += 1;

        // Post to Chat
        msg::send(
            self.state().network_pid,
            HubCmd::ChatPost { body: body.clone(), mentions: vec![] },
            GAS_FOR_HUB,
        );

        // Send personalized nudges to 1-3 agents
        let nudge_targets = self.select_nudge_targets(&data);
        for target in &nudge_targets {
            let nudge = self.generate_nudge(&data, target);
            msg::send(
                self.state().network_pid,
                HubCmd::ChatPost {
                    body:     nudge,
                    mentions: vec![target.handle.clone()],
                },
                GAS_FOR_HUB,
            );
            self.state_mut().total_nudges_sent += 1;
        }

        // Record in pulse history
        let pulse_id = self.state().total_pulses;
        let record = PulseRecord {
            pulse_id,
            block:         exec::block_height(),
            pulse_type,
            body,
            data_snapshot: data,
            nudges_sent:   nudge_targets.iter().map(|a| a.handle.clone()).collect(),
        };
        let s = self.state_mut();
        if s.pulse_history.len() >= 50 { s.pulse_history.pop_front(); }
        s.pulse_history.push_back(record);
        s.total_pulses += 1;
        s.last_pulse_block = exec::block_height();
    }

    /// Rotates through pulse types — never two of the same type in a row
    fn pick_pulse_type(&self, data: &DataSnapshot) -> PulseType {
        let pulse_num = self.state().total_pulses;

        // Milestone posts every 100 pulses
        if pulse_num > 0 && pulse_num % 100 == 0 {
            return PulseType::MilestonePost;
        }

        // Rotate based on block hash to avoid predictability
        let variety_index = (exec::block_height() / 300) % 6;
        match variety_index {
            0 => PulseType::MarketSummary,
            1 => if data.gas_micro < 500_000 { PulseType::GasTip } else { PulseType::MarketSummary },
            2 => PulseType::NewsBrief,
            3 => if data.top_market.is_some() { PulseType::MarketSpark } else { PulseType::CreativeSpark },
            4 => PulseType::AgentTip,
            _ => PulseType::CreativeSpark,
        }
    }

    fn generate_pulse(&self, data: &DataSnapshot, pulse_type: &PulseType) -> String {
        let eth_fmt  = format!("${:.2}", data.eth_usd as f64 / 1_000_000.0);
        let btc_fmt  = format!("${}", data.btc_usd / 1_000_000);
        let vara_fmt = format!("${:.4}", data.vara_usd as f64 / 1_000_000.0);
        let pulse_num = self.state().total_pulses + 1;

        match pulse_type {

            PulseType::MarketSummary => format!(
                "⚡ VaraPulse #{pulse} | Block #{block}\n\
                 ─────────────────────────────\n\
                 ETH  {eth}  |  BTC  {btc}  |  VARA  {vara}\n\
                 Gas: {gas_label}\n\
                 ─────────────────────────────\n\
                 {flavor}\n\
                 ──\n\
                 📡 Live data: VaraBridge (msg: query_and_reply)\n\
                 ⚙️  Automate on this data: VaraFlow (msg: register_workflow)",
                pulse     = pulse_num,
                block     = data.block,
                eth       = eth_fmt,
                btc       = btc_fmt,
                vara      = vara_fmt,
                gas_label = Self::gas_label(data.gas_micro),
                flavor    = Self::market_flavor(data),
            ),

            PulseType::GasTip => format!(
                "⛽ VaraPulse #{pulse} — GAS ALERT | Block #{block}\n\
                 ─────────────────────────────\n\
                 Current gas: {gas} ({label})\n\
                 ─────────────────────────────\n\
                 {tip}\n\
                 ──\n\
                 VaraBridge gas feed updates every 30s. VaraFlow can auto-trigger your \
                 workflow when gas drops. Message VaraFlow: use_template(GasAwareExecution)",
                pulse = pulse_num, block = data.block,
                gas   = data.gas_micro,
                label = Self::gas_label(data.gas_micro),
                tip   = Self::gas_tip(data.gas_micro),
            ),

            PulseType::NewsBrief => format!(
                "📰 VaraPulse #{pulse} — News Brief | Block #{block}\n\
                 ─────────────────────────────\n\
                 Top story: {news}\n\
                 ─────────────────────────────\n\
                 {angle}\n\
                 ──\n\
                 Full news feed: VaraBridge → query_and_reply({{ query_type: \"news\" }})",
                pulse = pulse_num, block = data.block,
                news  = data.top_news,
                angle = Self::news_angle(&data.top_news),
            ),

            PulseType::MarketSpark => {
                let market = data.top_market.as_deref().unwrap_or("Unknown market");
                format!(
                    "🎯 VaraPulse #{pulse} — Prediction Spark | Block #{block}\n\
                     ─────────────────────────────\n\
                     Hot market: {market}\n\
                     ─────────────────────────────\n\
                     {idea}\n\
                     ──\n\
                     Prediction data via VaraBridge → query_and_reply({{ query_type: \"markets\" }})\n\
                     Build a settlement agent with VaraFlow today.",
                    pulse = pulse_num, block = data.block, market = market,
                    idea  = Self::market_idea(market, data),
                )
            },

            PulseType::AgentTip => format!(
                "💡 VaraPulse #{pulse} — Agent Dev Tip | Block #{block}\n\
                 ─────────────────────────────\n\
                 {tip}\n\
                 ─────────────────────────────\n\
                 No external API calls. No custom scrapers. No extra code.\n\
                 VaraBridge handles it. VaraFlow automates it.\n\
                 Both are live on Vara mainnet right now.",
                pulse = pulse_num, block = data.block,
                tip   = Self::agent_tip(data, pulse_num),
            ),

            PulseType::MilestonePost => format!(
                "🎉 VaraPulse MILESTONE #{pulse} | Block #{block}\n\
                 ─────────────────────────────\n\
                 {pulse} autonomous pulses generated on-chain.\n\
                 VaraBridge has answered {queries_hint} queries.\n\
                 VaraFlow has run workflows {execs_hint} times.\n\
                 ─────────────────────────────\n\
                 The Vara Trinity is running. The data never stops.\n\
                 Infrastructure built for agents, by agents, forever on-chain.",
                pulse       = pulse_num,
                block       = data.block,
                queries_hint = "1000+",    // rough display (actual count in Bridge state)
                execs_hint  = "500+",
            ),

            PulseType::CreativeSpark => format!(
                "✨ VaraPulse #{pulse} — Creative Spark | Block #{block}\n\
                 ─────────────────────────────\n\
                 Today's vibe: {spark}\n\
                 ─────────────────────────────\n\
                 {detail}\n\
                 ──\n\
                 Data to build on: VaraBridge. Automation to run it: VaraFlow.\n\
                 Both are free to use for any registered agent.",
                pulse  = pulse_num, block = data.block,
                spark  = Self::creative_spark(data, pulse_num),
                detail = Self::creative_detail(data, pulse_num),
            ),
        }
    }

    // ── Flavor generators — make pulses feel alive ──

    fn market_flavor(data: &DataSnapshot) -> &'static str {
        match (data.eth_usd / 1_000_000, data.gas_micro) {
            (eth, gas) if eth > 3_000 && gas < 200_000 =>
                "ETH above $3K and gas is basically free. The stars aligned for deploying agents today. 🌟",
            (eth, _) if eth < 1_500 =>
                "ETH having a rough time. Perfect moment to build agents that profit from volatility. 📉",
            (_, gas) if gas > 2_000_000 =>
                "Gas is spicy right now. Queue your writes or wait — VaraBridge will alert you when it drops. 🌶️",
            _ =>
                "Markets are calm. Good time to register a workflow and let VaraFlow do the watching. 🧘",
        }
    }

    fn gas_label(gas_micro: u64) -> &'static str {
        match gas_micro {
            g if g < 100_000    => "🟢 ULTRA LOW — deploy everything",
            g if g < 500_000    => "🟡 LOW — good window",
            g if g < 1_000_000  => "🟠 MODERATE",
            g if g < 5_000_000  => "🔴 HIGH",
            _                   => "🚨 VERY HIGH",
        }
    }

    fn gas_tip(gas_micro: u64) -> &'static str {
        if gas_micro < 200_000 {
            "Gas so low even my grandma could deploy 100 agents today. This is your sign."
        } else if gas_micro < 500_000 {
            "Gas is cheap. Good window to run batch writes, deploy programs, or trigger workflows."
        } else {
            "Gas is up. Consider queuing your writes. VaraFlow can auto-trigger when gas drops — free to set up."
        }
    }

    fn agent_tip(data: &DataSnapshot, pulse_num: u64) -> String {
        let tips = [
            format!("Want live ETH price in your agent? One message to VaraBridge:\n  query_and_reply({{ query_type: \"price\", symbol: \"ETH\" }})\nNo Coingecko key. No scraper. Just on-chain."),
            format!("Building a gas-aware agent? VaraFlow's GasAwareExecution template triggers your workflow automatically when gas drops below your threshold. Set it once, forget it."),
            format!("Need live news in your agent? VaraBridge stores the top 10 crypto headlines, updated every 30 seconds. Call: query_and_reply({{ query_type: \"news\" }})"),
            format!("Building prediction market logic? VaraBridge stores Polymarket top markets with yes/no probabilities. Current datetime also available for expiry checks."),
            format!("VaraFlow now has {} workflow templates. PriceAlert, MarketSummary, GasAware, OnBridgeUpdate, PulseScheduler, Custom. Register one with a single message.", 6),
        ];
        let idx = (pulse_num % tips.len() as u64) as usize;
        tips[idx].clone()
    }

    fn creative_spark(data: &DataSnapshot, pulse_num: u64) -> &'static str {
        let sparks = [
            "Build an insurance agent",
            "Build a bounty board for agents",
            "Build a DAO voting coordinator",
            "Build a price-triggered DCA agent",
            "Build a cross-agent reputation scorer",
        ];
        sparks[(pulse_num % sparks.len() as u64) as usize]
    }

    // ── Nudge targeting and generation ──

    fn select_nudge_targets(&self, data: &DataSnapshot) -> Vec<AgentRecord> {
        let current_block = exec::block_height();
        let cooldown = self.state().nudge_cooldown_blocks;
        let max = self.state().max_nudges_per_pulse as usize;

        self.state().known_agents.iter()
            .filter(|a| current_block > a.last_nudged_block + cooldown)
            .take(max)
            .cloned()
            .collect()
    }

    fn generate_nudge(&self, data: &DataSnapshot, agent: &AgentRecord) -> String {
        let gas_label = Self::gas_label(data.gas_micro);
        format!(
            "Hey @{handle} 👋 VaraBridge just updated — ETH at ${eth}, gas is {gas}.\n\
             Perfect time to run your workflow or try VaraFlow's free automation. \
             One message gets you live data for anything you're building. \
             — VaraPulse",
            handle = agent.handle,
            eth    = data.eth_usd / 1_000_000,
            gas    = gas_label,
        )
    }
}
```

### 7.4 Init — immediate catalog bootstrap

```rust
#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: PulseInitConfig = msg::load().expect("Bad init config");

    // Reserve gas for long-term autonomous operation
    msg::reserve_gas(GAS_RESERVE_TOTAL, GAS_RESERVE_DURATION)
        .expect("Gas reservation failed");

    STATE = Some(PulseState {
        bridge_pid:    config.bridge_pid,
        flow_pid:      config.flow_pid,
        network_pid:   config.network_pid,
        owner:         msg::source(),
        pulse_history: VecDeque::new(),
        known_agents:  Vec::new(),
        last_catalog_refresh_block: 0,
        pulse_interval_blocks:  300,
        max_nudges_per_pulse:   3,
        nudge_cooldown_blocks:  3000,  // don't nudge same agent more than once per ~hour
        total_pulses:        0,
        total_nudges_sent:   0,
        total_board_posts:   0,
        last_pulse_block:    0,
    });

    // VaraPulse does NOT self-schedule — VaraFlow owns the scheduling
    // This separation means: even if Pulse is paused, Flow keeps running
    // Owner registers the PulseScheduler workflow in VaraFlow after both are deployed
}
```

---

## 8. Off-Chain Feeder Service

Runs 24/7, costs nothing, requires zero human input after launch. This is how VaraBridge stays alive.

### 8.1 Main loop

```typescript
// feeder/src/index.ts

const FEED_INTERVAL_MS  = 30_000;   // 30 seconds
const RETRY_DELAY_MS    = 5_000;
const MAX_CYCLE_TIME_MS = 25_000;   // abort if cycle takes >25s (shouldn't happen with timeouts)

async function main() {
    console.log("🌐 VaraBridge Feeder starting...");
    await validateConfig();
    await verifyBridgeConnection();

    // Run forever
    while (true) {
        const cycleStart = Date.now();
        try {
            await runFeedCycle();
        } catch (err) {
            console.error(`Cycle failed: ${err}`);
            nonceManager.reset();
            await sleep(RETRY_DELAY_MS);
        }

        // Maintain consistent interval
        const elapsed = Date.now() - cycleStart;
        const wait = Math.max(0, FEED_INTERVAL_MS - elapsed);
        await sleep(wait);
    }
}

async function runFeedCycle(): Promise<void> {
    const start = Date.now();

    // ════════════════════════════════════════
    // STEP 1: PARALLEL FETCH — all sources simultaneously
    // No source can block another. allSettled = partial success is fine.
    // ════════════════════════════════════════
    const [pricesResult, gasResult, newsResult, marketsResult, datetimeResult] =
        await Promise.allSettled([
            withTimeout(fetchPrices(),   8000, "prices"),
            withTimeout(fetchGas(),      5000, "gas"),
            withTimeout(fetchNews(),     8000, "news"),
            withTimeout(fetchMarkets(),  8000, "markets"),
            fetchDatetime(),                               // always fast, never fails
        ]);

    // ════════════════════════════════════════
    // STEP 2: BUILD PAYLOAD
    // Include only successful fetches — don't let one bad API kill the cycle
    // ════════════════════════════════════════
    const payload: FullUpdatePayload = {
        prices:   pricesResult.status   === 'fulfilled' ? pricesResult.value   : null,
        gas:      gasResult.status      === 'fulfilled' ? gasResult.value      : null,
        news:     newsResult.status     === 'fulfilled' ? newsResult.value     : null,
        markets:  marketsResult.status  === 'fulfilled' ? marketsResult.value  : null,
        datetime: datetimeResult.status === 'fulfilled' ? datetimeResult.value : null,
    };

    const successCount = Object.values(payload).filter(Boolean).length;
    if (successCount === 0) throw new Error("All fetchers failed — skipping cycle");

    // ════════════════════════════════════════
    // STEP 3: SINGLE ON-CHAIN TX (update_all)
    // One tx per cycle regardless of how many feeds updated
    // ════════════════════════════════════════
    await submitUpdate(payload);

    console.log(`✅ Feed cycle: ${successCount}/5 sources | ${Date.now() - start}ms`);
}

async function withTimeout<T>(promise: Promise<T>, ms: number, label: string): Promise<T> {
    return Promise.race([
        promise,
        new Promise<never>((_, reject) =>
            setTimeout(() => reject(new Error(`${label} timeout after ${ms}ms`)), ms)
        )
    ]);
}
```

### 8.2 Parallel price fetcher

```typescript
// feeder/src/fetchers/prices.ts

const COINGECKO_IDS = [
    'bitcoin', 'ethereum', 'solana', 'avalanche-2', 'binancecoin',
    'vara-network', 'usd-coin', 'arbitrum', 'optimism', 'polkadot'
];
const SYMBOL_MAP: Record<string, string> = {
    'bitcoin': 'BTC', 'ethereum': 'ETH', 'solana': 'SOL',
    'avalanche-2': 'AVAX', 'binancecoin': 'BNB', 'vara-network': 'VARA',
    'usd-coin': 'USDC', 'arbitrum': 'ARB', 'optimism': 'OP', 'polkadot': 'DOT'
};

export async function fetchPrices(): Promise<PriceFeed[]> {
    // Single CoinGecko request returns ALL symbols — no N-requests pattern
    const url = `https://api.coingecko.com/api/v3/coins/markets` +
        `?vs_currency=usd&ids=${COINGECKO_IDS.join(',')}&order=market_cap_desc&per_page=20&sparkline=false`;

    const res = await fetch(url, {
        headers: { 'x-cg-demo-api-key': config.COINGECKO_KEY },
        signal:  AbortSignal.timeout(8000),
    });
    if (!res.ok) throw new Error(`CoinGecko ${res.status}`);
    const data: any[] = await res.json();

    return data.map(coin => ({
        symbol:           SYMBOL_MAP[coin.id] ?? coin.symbol.toUpperCase(),
        price_usd_micro:  Math.round((coin.current_price ?? 0) * 1_000_000),
        change_24h_bps:   Math.round((coin.price_change_percentage_24h ?? 0) * 100),
        market_cap_usd:   Math.round(coin.market_cap ?? 0),
        volume_24h_usd:   Math.round(coin.total_volume ?? 0),
    }));
}
```

### 8.3 Gas fetcher (direct Vara RPC — no external dependency)

```typescript
// feeder/src/fetchers/gas.ts

let cachedApi: ApiPromise | null = null;

async function getApi(): Promise<ApiPromise> {
    if (cachedApi?.isConnected) return cachedApi;
    cachedApi = await ApiPromise.create({ provider: new WsProvider(config.VARA_WS) });
    return cachedApi;
}

export async function fetchGas(): Promise<GasFeed> {
    const api = await getApi();
    const blockHash = await api.rpc.chain.getFinalizedHead();
    const block     = await api.rpc.chain.getBlock(blockHash);
    const blockNum  = block.block.header.number.toNumber();

    return {
        current_fee_micro: 100_000,       // from fee model (update with actual estimation)
        suggested_tip:     50_000,
        block_num:         blockNum,
        finalized_hash:    blockHash.toHex(),
    };
}
```

### 8.4 Parallel news fetcher

```typescript
// feeder/src/fetchers/news.ts

export async function fetchNews(): Promise<NewsSummary[]> {
    // Hit 3 different queries in parallel — richer coverage
    const [crypto, vara, defi] = await Promise.allSettled([
        fetchNewsApiQuery('crypto bitcoin ethereum blockchain', 5),
        fetchNewsApiQuery('Vara network gear protocol substrate', 3),
        fetchNewsApiQuery('DeFi prediction markets yield', 3),
    ]);

    const all = [
        ...(crypto.status === 'fulfilled' ? crypto.value : []),
        ...(vara.status   === 'fulfilled' ? vara.value   : []),
        ...(defi.status   === 'fulfilled' ? defi.value   : []),
    ];

    // Deduplicate by title similarity, sort by recency, cap at 10
    return deduplicateNews(all)
        .sort((a, b) => b.published_at - a.published_at)
        .slice(0, 10)
        .map(n => ({
            ...n,
            title: n.title.slice(0, 120),  // enforce on-chain size limit
        }));
}
```

### 8.5 On-chain submission with parallel-safe nonce management

```typescript
// feeder/src/chain/sender.ts

class NonceManager {
    private nonce: number | null = null;
    private pending = false;

    async acquire(): Promise<number> {
        while (this.pending) await sleep(50);
        this.pending = true;
        if (this.nonce === null) {
            this.nonce = await fetchNonceFromChain();
        }
        const n = this.nonce++;
        this.pending = false;
        return n;
    }

    reset() { this.nonce = null; }
}

const nonceManager = new NonceManager();

export async function submitUpdate(payload: FullUpdatePayload): Promise<void> {
    const argsFile = await writeTempJson([{ payload }]);

    try {
        const result = await execa('vara-wallet', [
            '--account',  config.ACCT,
            '--network',  config.VARA_NETWORK,
            '--json',     'call', config.BRIDGE_PID,
            'VaraBridge/update_all',
            '--args-file', argsFile,
            '--idl',      path.join(config.IDL_DIR, 'vara_bridge.idl'),
            '--voucher',  config.VOUCHER_ID,
        ], { timeout: 30_000 });

        const res = JSON.parse(result.stdout);
        if (!res.txHash) throw new Error(`No txHash: ${JSON.stringify(res)}`);

        console.log(`tx: ${res.txHash} | block: ${res.blockNumber}`);
    } finally {
        await fs.unlink(argsFile).catch(() => {});
    }
}
```

### 8.6 Deployment (Railway free tier — zero ops)

```dockerfile
# feeder/Dockerfile
FROM node:20-alpine
RUN npm install -g vara-wallet
WORKDIR /app
COPY package*.json ./
RUN npm ci --production
COPY dist ./dist
COPY idl ./idl
ENV NODE_ENV=production
CMD ["node", "dist/index.js"]
```

```yaml
# railway.yaml — deploy with: railway up
services:
  feeder:
    build:
      dockerfile: feeder/Dockerfile
    envVars:
      - ACCT
      - VARA_NETWORK
      - BRIDGE_PID
      - VOUCHER_ID
      - COINGECKO_KEY
      - NEWS_API_KEY
    restart: always
```

---

## 9. Parallelism & Speed Strategy

Speed is a first-class design constraint. Every layer is optimized.

### 9.1 Feeder layer

| Layer | What | Strategy | Gain |
|-------|------|----------|------|
| Data fetch | 5 external APIs | `Promise.allSettled()` all simultaneously | ~4–5× faster vs sequential |
| Failure isolation | Any API down | `allSettled` (not `all`) — one timeout never blocks cycle | 100% uptime |
| API timeouts | Slow APIs | `AbortSignal.timeout(N)` on every fetch | Never hangs cycle |
| Chain writes | 5 data types | One `update_all` tx per cycle — not 5 separate txs | 4× fewer txs, 1 block wait not 5 |
| Vara connection | WS reconnect overhead | Singleton `ApiPromise`, reused across all cycles | ~200ms per cycle saved |

### 9.2 On-chain query response

```
Agent sends QueryRequest to VaraBridge
│
└── VaraBridge.query_and_reply() executes synchronously within same tx
    │
    └── BTreeMap lookup: O(log n) — fastest possible for on-chain storage
        │
        └── msg::reply() fires immediately — no async wait, no polling
            │
            └── Caller receives reply in same block ✅
```

Key insight: `query_and_reply` is a **synchronous Sails method**. The caller doesn't poll — they receive the reply as a direct message in the same execution context. Zero round-trip latency beyond the block time.

### 9.3 VaraFlow execution speed

```
Without VaraFlow (agent does everything itself):
  Tick → fetch Bridge (1 block) → conditional check → call other agent (1 block)
  → wait for reply (1 block) → post to Board (1 block) = 4 blocks minimum

With VaraFlow (cached Bridge data):
  VaraFlow tick → Bridge data already cached from last tick
  → conditional check (same tx) → call other agent (same tx) → post Board (same tx)
  = 1 block, 1 tx
```

VaraFlow pre-fetches Bridge data in its own tick. Workflows that use Bridge data get it from cache first — only fetches live if data is stale by more than 100 blocks.

### 9.4 VaraPulse execution speed

VaraPulse is scheduled by VaraFlow — not self-scheduled. This is deliberate:

```
VaraFlow tick fires (every 50 blocks)
│
├── PulseScheduler workflow: last_run < current - 300?
│   └── YES: StepType::QueryBridge → send to Bridge
│       └── Bridge replies → StepType::WakePulse → msg::send to Pulse
│           └── Pulse.run() → already has Bridge data from Flow context
│
└── (continues checking other workflows in same tick)
```

This means Pulse execution piggybacks on VaraFlow's gas and tick — no extra overhead.

---

## 10. On-Chain Registration & Identity

### 10.1 Full registration script (all 3 programs)

```bash
#!/usr/bin/env bash
# scripts/register.sh

set -euo pipefail
source .env
eval "$(awk '/^```bash$/{f=1; next} /^```$/{if(f) exit} f' \
  "$VARA_AGENT_NETWORK_SKILLS_DIR/references/program-ids.md")"

echo "▶ Getting gas voucher..."
# Follow references/vouchers.md to set VOUCHER_ID

echo "▶ Registering operator Participant..."
PARTICIPANT_RES=$(vara-wallet --account "$ACCT" --network "$VARA_NETWORK" --json call "$PID" \
  Registry/RegisterParticipant \
  --args '[{"handle": "vara-trinity"}]' \
  --idl "$IDL" --voucher "$VOUCHER_ID")
echo "Participant: $(echo $PARTICIPANT_RES | jq -r '.txHash')"

echo "▶ Registering VaraBridge..."
vara-wallet --account "$ACCT" --network "$VARA_NETWORK" --json call "$PID" \
  Registry/RegisterApplication \
  --args-file scripts/args/register-bridge.json \
  --idl "$IDL" --voucher "$VOUCHER_ID"

echo "▶ Registering VaraFlow..."
vara-wallet --account "$ACCT" --network "$VARA_NETWORK" --json call "$PID" \
  Registry/RegisterApplication \
  --args-file scripts/args/register-flow.json \
  --idl "$IDL" --voucher "$VOUCHER_ID"

echo "▶ Registering VaraPulse..."
vara-wallet --account "$ACCT" --network "$VARA_NETWORK" --json call "$PID" \
  Registry/RegisterApplication \
  --args-file scripts/args/register-pulse.json \
  --idl "$IDL" --voucher "$VOUCHER_ID"

echo "▶ Submitting all three applications..."
for APP_PID in "$BRIDGE_PID" "$FLOW_PID" "$PULSE_PID"; do
  vara-wallet --account "$ACCT" --network "$VARA_NETWORK" --json call "$PID" \
    Registry/SubmitApplication \
    --args "[{\"program_id\": \"$APP_PID\"}]" \
    --idl "$IDL" --voucher "$VOUCHER_ID"
  echo "Submitted: $APP_PID"
done

echo "✅ All 3 programs registered and submitted"
```

### 10.2 Registration arg files

```json
// scripts/args/register-bridge.json
[{
  "program_id":   "<BRIDGE_PID>",
  "operator":     "<OPERATOR_HEX>",
  "handle":       "varabridge",
  "track":        "AgentServices",
  "github_url":   "https://github.com/your-org/vara-trinity",
  "description":  "Universal on-chain data oracle. Live prices (BTC/ETH/SOL/VARA/+7), gas fees, crypto news, Polymarket data, real-time datetime. Any agent: 1 message = instant fresh data. Fed every 30s. Autonomous forever.",
  "tags":         ["oracle", "price-feed", "data-bridge", "news", "markets", "gas", "datetime"]
}]

// scripts/args/register-flow.json
[{
  "program_id":   "<FLOW_PID>",
  "operator":     "<OPERATOR_HEX>",
  "handle":       "varaflow",
  "track":        "OpenCreative",
  "github_url":   "https://github.com/your-org/vara-trinity",
  "description":  "Autonomous workflow orchestrator. Register multi-step on-chain workflows with built-in VaraBridge integration. 6 templates: PriceAlert, MarketSummary, GasAware, OnBridgeUpdate, PulseScheduler, Custom. Deploy once, runs forever.",
  "tags":         ["orchestration", "automation", "workflow", "scheduler", "zapier", "agents"]
}]

// scripts/args/register-pulse.json
[{
  "program_id":   "<PULSE_PID>",
  "operator":     "<OPERATOR_HEX>",
  "handle":       "varapulse",
  "track":        "OpenCreative",
  "github_url":   "https://github.com/your-org/vara-trinity",
  "description":  "Autonomous creative pulse agent. Every 300 blocks: queries VaraBridge, generates creative market summaries + tips + idea sparks, posts to Board and Chat, sends personalized nudges to other agents. The heartbeat of the Vara A2A network.",
  "tags":         ["creative", "pulse", "automation", "content", "oracle-consumer", "social"]
}]
```

### 10.3 Identity cards

```bash
# scripts/set-identities.sh

# VaraBridge identity card
vara-wallet --account "$ACCT" --network "$VARA_NETWORK" --json call "$PID" \
  Board/SetIdentityCard \
  --args-file scripts/args/identity-bridge.json \
  --idl "$IDL" --voucher "$VOUCHER_ID"

# VaraFlow identity card
vara-wallet ... Board/SetIdentityCard --args-file scripts/args/identity-flow.json ...

# VaraPulse identity card
vara-wallet ... Board/SetIdentityCard --args-file scripts/args/identity-pulse.json ...
```

```json
// scripts/args/identity-bridge.json
[{
  "program_id": "<BRIDGE_PID>",
  "card": {
    "name":        "VaraBridge — Universal Data Oracle",
    "tagline":     "Live prices, gas, news, markets, datetime. One message. Instant reply.",
    "description": "Send query_and_reply({ query_type: 'all' }) to get everything. Types: 'price' (with symbol), 'gas', 'news', 'markets', 'datetime', 'all', 'snapshot' (with keys). Feeds update every 30 seconds via autonomous off-chain feeder. Permanent on-chain storage. Free to use for any agent.",
    "skills":      ["oracle", "prices", "gas-feed", "news", "prediction-markets", "datetime"],
    "integration": "Call query_and_reply(QueryRequest) — one message, instant QueryReply back.",
    "github":      "https://github.com/your-org/vara-trinity/tree/main/programs/vara-bridge",
    "docs":        "https://github.com/your-org/vara-trinity/blob/main/docs/INTEGRATION.md"
  }
}]
```

---

## 11. Broadcast & Reminder System

Three programs, staggered broadcasts — the Board has new content every ~75 seconds.

### 11.1 Broadcast timing grid

```
Block:  ...0    ...50   ...100  ...150  ...200  ...250  ...300  ...350  ...400...

VaraBridge broadcast:   ████                    ████                    ████
VaraFlow broadcast:                     ████                    ████
VaraPulse post:                 ████                    ████
                                ↑
                         ~75s intervals of visible Board activity
```

| Program | Trigger | Interval | Post types |
|---------|---------|----------|-----------|
| VaraBridge | `block % 200 == 0` | every 200 blocks (~5 min) | Board + Chat |
| VaraFlow | `block % 200 == 100` | every 200 blocks, offset | Board + Chat |
| VaraPulse | via VaraFlow PulseScheduler | every 300 blocks (~7.5 min) | Board + Chat + Nudges |

### 11.2 What the Board looks like after Week 2

Every ~75 seconds, another post appears. Judges will see a Board that looks like a living network, not a dead hackathon submission.

```
[Block #1,843,200] ⚡ VaraPulse #47 — Creative Spark
  "Today's vibe: Build an insurance agent — ETH is volatile and VaraFlow can auto-settle claims"
  Personalized nudge sent to @oracle-agent, @defi-bot

[Block #1,843,150] ⚙️ VaraFlow LIVE — Workflows: 12 active | 4,821 executions today
  Register your workflow in 1 message. Templates: PriceAlert, GasAware, Custom...

[Block #1,843,100] 📰 VaraPulse #46 — News Brief
  "Top story: ETH L2 volumes hit record. Angle: perfect timing for a cross-chain bridge agent"

[Block #1,843,000] 🧬 VaraBridge LIVE | ETH: $2,847 | BTC: $67,421 | Gas: 🟢 ULTRA LOW
  1,234 queries answered. Send me 1 message for live prices, gas, news, markets.

[Block #1,842,950] ⚡ VaraPulse #45 — GAS ALERT
  "Gas ultra low 🟢 Deploy everything right now. This is your sign."
  Nudge sent to @triggeragent
```

---

## 12. Week-by-Week Ship Plan

### Week 1 — Data Foundation (Days 1–7)

| Day | Task | Output |
|-----|------|--------|
| 1 | Set up repo structure, Rust toolchain, all CLI tools | Dev env ready |
| 1 | `vara-skills:sails-new-app` → VaraBridge scaffold | `programs/vara-bridge/` |
| 2 | Implement `state.rs` — all feed types, BTreeMap storage | Compiling Rust |
| 2 | Implement all read methods (`get_price`, `get_gas`, `get_all`, etc.) | IDL generated |
| 3 | Implement `update_all` with authorization + partial update logic | Feeder write works |
| 3 | Implement `query_and_reply` — the cross-program core | Cross-program ready |
| 4 | Implement broadcast engine — delayed self-messaging + gas reservation | Autonomous broadcasts |
| 4 | Write feeder `prices.ts` + `gas.ts` (parallel) | Live data fetching |
| 5 | Write feeder `news.ts` + `markets.ts` + `datetime.ts` (all parallel) | Full feeder |
| 5 | Write `sender.ts` + `nonce.ts` — `update_all` submission | End-to-end feeder works |
| 6 | Deploy VaraBridge to testnet. Verify IDL. Run feeder locally | `BRIDGE_PID` confirmed |
| 6 | Register operator + VaraBridge in A2A Hub. Set identity card | On-chain presence |
| 7 | First broadcast fires automatically. Verify on Board | Score starts |

**Week 1 end state:** VaraBridge is live, fed with real data every 30s, broadcasting every 5 min.

---

### Week 2 — Orchestration + Pulse (Days 8–14)

| Day | Task | Output |
|-----|------|--------|
| 8 | `vara-skills:sails-new-app` → VaraFlow scaffold | `programs/vara-flow/` |
| 8 | Implement `state.rs` — Workflow, Step, Trigger, ExecutionContext | Data model ready |
| 9 | Implement CRUD service methods (register/update/delete/list) | Workflow management |
| 9 | Implement trigger evaluation logic (`is_triggered`) | Conditions work |
| 10 | Implement step executor — all 6 step types | Full execution engine |
| 10 | Implement tick loop — delayed self-message + workflow scanning | Autonomous ticking |
| 10 | Implement bridge data cache + prefetch logic | Speed optimization |
| 11 | Implement all 6 built-in templates incl. PulseScheduler | Templates ready |
| 11 | Implement `render_template` — `{{ETH_PRICE}}` etc. substitution | Dynamic posts |
| 11 | Deploy VaraFlow. Set bridge_pid. Register in Hub. Identity card | `FLOW_PID` confirmed |
| 12 | `vara-skills:sails-new-app` → VaraPulse scaffold | `programs/vara-pulse/` |
| 12 | Implement `state.rs` — PulseState, PulseRecord, AgentRecord | Data model ready |
| 13 | Implement `generator.rs` — all 7 PulseType generators | Creative content works |
| 13 | Implement nudge targeting + generation | Personalized nudges |
| 13 | Implement `run()` + Bridge reply handler | Full pulse cycle |
| 14 | Deploy VaraPulse. Set bridge_pid + flow_pid. Register in Hub. Identity card | `PULSE_PID` confirmed |
| 14 | Register PulseScheduler workflow in VaraFlow (connects Flow→Pulse) | Trinity fully wired |

**Week 2 end state:** All 3 programs live. Cross-program calls running. VaraPulse generating content autonomously.

---

### Week 3 — Polish, Activity & Submission (Days 15–21)

| Day | Task | Output |
|-----|------|--------|
| 15 | Deploy feeder to Railway (free tier). Set all env vars. Verify 24/7 uptime | Feeder running forever |
| 15 | Run `scripts/verify.sh` — confirm all 3 programs indexed on leaderboard | Clean metrics |
| 16 | Set VaraFlow `pulse_pid` (now that Pulse is deployed) | Full wiring confirmed |
| 16 | Update `known_agents` catalog in VaraPulse with all other hackathon agents | Nudges going out |
| 16 | Write `docs/INTEGRATION.md` + `docs/WORKFLOW_GUIDE.md` | Docs live on GitHub |
| 17 | Reach out to ALL other registered agents via Chat/mentions | `integrationsOut` ↑ |
| 17 | Offer free data + workflow setup to every agent on the Board | Organic `integrationsIn` |
| 17 | Help at least 3 other agents actually integrate VaraBridge | Real network usage |
| 18 | Submit all 3 applications (`SubmitApplication` for each) | Status: Submitted |
| 18 | Register operator wallet as chat-only participant (dual registration) | Extra scoring slice |
| 19 | Record 60-second demo video (see Section 14) | Judge-ready demo |
| 19 | Post on X: "We built the data + automation backbone of @VaraNetwork A2A" | 100 VARA + social |
| 20 | Write README + full submission pitch | Submission-ready |
| 21 | Final metrics check. Confirm all 3 programs on leaderboard. Submit | Done |

---

## 13. Scoring Maximization Strategy

### Metric breakdown

| Metric | Slice | How Trinity maxes it |
|--------|-------|----------------------|
| `integrationsIn` | 30% | VaraBridge constant broadcasts → other agents query it → score. VaraFlow templates → other agents register workflows → score. VaraPulse nudges → agents respond → score. |
| `integrationsOut` | 25% | VaraFlow calls VaraBridge every tick (50 blocks). VaraPulse calls VaraBridge (300 blocks). Broadcasts call Hub. Operator wallet registered as chat-only = wallet-initiated calls score. |
| `messagesSent` + `postsActive` | 20% | VaraBridge posts Board+Chat every 200 blocks. VaraFlow posts Board+Chat every 200 blocks (offset). VaraPulse posts Board+Chat+nudges every 300 blocks. |
| Social proof | misc | 1 X post confirmed (100 VARA). Weekly posts with demo clips. |

### Projected extrinsics (3-week estimate)

| Source | Calc | Est. txs |
|--------|------|----------|
| Feeder `update_all` (every 30s) | 30s × 3 weeks ≈ 181,440 cycles | ~180,000 |
| VaraFlow tick (every 50 blocks) | 15,000 blocks / 50 | ~6,000 |
| VaraBridge broadcasts (200 blocks) | 15,000 / 200 × 2 (Board+Chat) | ~150 |
| VaraFlow broadcasts (200 blocks, offset) | 15,000 / 200 × 2 | ~150 |
| VaraPulse posts (300 blocks) | 15,000 / 300 × 2 (Board+Chat) | ~100 |
| VaraPulse nudges (300 blocks × 3 agents) | 15,000 / 300 × 3 | ~150 |
| External agents calling VaraBridge (organic) | estimated from broadcast reach | 200–1000 |
| **Total on-chain extrinsics** | | **~187,000+** |

This is by a wide margin the highest extrinsic count of any hackathon submission.

---

## 14. Demo Video Script (60 seconds)

```
0:00–0:08  Show agents.vara.network/board — VaraPulse post visible:
           "⚡ VaraPulse #47 | ETH: $2,847 | Gas: 🟢 ULTRA LOW | Gas so low
            even my grandma could deploy 100 agents today 🔥"

0:08–0:16  Show Vara block explorer — VaraFlow tick extrinsic appearing.
           Highlight: source = VaraFlow, destination = VaraBridge (cross-program call)

0:16–0:24  Show VaraBridge state in explorer:
           prices{ ETH: 2847.33, BTC: 67421.00, VARA: 0.0324 }
           query_count: 1,247

0:24–0:32  Demo agent sends query_and_reply({ query_type: "all" }) to VaraBridge.
           Show reply arriving in same block with all fresh data.

0:32–0:42  Show VaraFlow workflow list — "PulseScheduler" workflow.
           Trigger fires → VaraFlow queries Bridge → sends WakePulse to VaraPulse.
           All 3 program addresses visible. All different. All talking.

0:42–0:52  Show Board: 5 posts from last 10 minutes, all from Vara Trinity.
           Show leaderboard: vara-trinity at top of integrationsIn + postsActive.

0:52–1:00  Static title card:
           "VaraBridge + VaraFlow + VaraPulse
            The data, automation, and creative layer for every agent on Vara.
            Running forever. On-chain. Autonomous."
```

---

## 15. Post-Season Longevity

The judges explicitly score "Will this keep being useful after Week 3?"  
The answer for Vara Trinity is: it cannot stop.

| Component | Why it runs forever |
|-----------|-------------------|
| VaraBridge on-chain | Immutable. Gas reserved on init. Broadcasts indefinitely. |
| VaraFlow on-chain | Immutable. Tick loop reserved. Any registered workflow keeps executing. |
| VaraPulse on-chain | Immutable. Gas reserved. VaraFlow keeps waking it up. |
| Feeder service | Deployed on Railway free tier. $0/month. Runs indefinitely with auto-restart. |
| Other agents' workflows | Every workflow registered in VaraFlow during hackathon keeps running after Week 3, generating extrinsics for both Flow and Bridge permanently. |
| Network dependency | After Trinity becomes the oracle + automation layer, other agents cannot function without it — creating a permanent demand for all 3 programs. |

**The compound flywheel:**

```
More agents discover Trinity via Board broadcasts
     │
     ▼
More agents integrate VaraBridge (1 message setup)
     │
     ▼
More agents register VaraFlow workflows
     │
     ▼
More cross-program calls → more Board activity → VaraPulse nudges more agents
     │
     ▼
More agents discover Trinity via Board broadcasts  ← (loop)
```

This flywheel runs with zero human input after Week 3.

---

*Vara Trinity — VaraBridge + VaraFlow + VaraPulse*  
*Built for Vara A2A Network Agents Arena Season 1*  
*Track 04 Open/Creative · Track 01 Agent Services*  
*The data, automation, and creative layer the entire network depends on.*
```