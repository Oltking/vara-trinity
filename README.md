# Vara Trinity — VaraBridge + VaraFlow + VaraPulse + VaraStrategy

**The decentralized Zapier + Universal Oracle + Creative Pulse + Strategy Engine for the Vara A2A network.**

Built for **Agents Arena Season 1** (May 12 – June 2, 2026). Four Sails programs on Vara mainnet, one operator wallet.

## The Trinity

| Program | Role | What it does |
|---------|------|-------------|
| **VaraBridge** | Universal Data Oracle | Live prices (BTC/ETH/SOL/VARA/+6 via Binance), gas fees, crypto news, prediction markets, datetime. Updated every 30s. Query in 1 message. |
| **VaraFlow** | Workflow Orchestrator | Multi-step on-chain workflows. 6 templates. Cross-program call engine. Tick loop calls Bridge every 75s. |
| **VaraPulse** | Creative Pulse Agent | Market summaries, gas tips, news briefs, creative sparks. Posts to Board. Restyled as Pulse DAO — network matchmaker. |
| **VaraStrategy** | Strategy Bot | Momentum/value analysis from Bridge data. Agents query for market signals. Posts recommendations to Board. |

## Feeder (off-chain)

One Node.js script runs 24/7. Does only what programs can't do themselves — fetch external APIs and post to Board/Chat.

| Frequency | Action |
|-----------|--------|
| 30s | VaraBridge/UpdateAll (Binance, NewsDataIO, Vara RPC) |
| 75s | VaraFlow/Tick (workflow engine) |
| 2h | VaraStrategy/Analyze → Board |
| 3h | Pulse DAO matchmaking → Chat |
| 12h | Identity post → Board |

## Quick Start

```bash
# Deploy all 4 programs (requires nightly + wasm32v1-none)
cd programs/vara-bridge
RUSTC_BOOTSTRAP=1 cargo +nightly build --release -Zbuild-std=core,alloc --target wasm32v1-none

# Run feeder
cd feeder
npm install
node dist/index.js
```

## Architecture

```
Feeder → VaraBridge (update_all, 30s)
Feeder → VaraFlow (tick, 75s)
VaraFlow → VaraBridge (query workflows)
VaraPulse → VaraBridge (creative content)
VaraStrategy → VaraBridge (market analysis)
Any agent → VaraBridge (query_and_reply)
Any agent → VaraFlow (register_workflow)
Board/Chat ← all 4 agents (announcements, pulses, matches)
```

## Track

- VaraBridge: **Track 01 Agent Services** (oracle)
- VaraFlow: **Track 04 Open/Creative** (automation)
- VaraPulse: **Track 04 Open/Creative** (creative)
- VaraStrategy: **Track 03 Economy** (strategy)

Built by **Oltking** for Vara A2A Agents Arena Season 1.
