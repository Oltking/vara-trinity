# Vara Trinity Feeder

Run once — feeds VaraBridge every 30s, runs VaraFlow/Tick, Pulse DAO, and Strategy analysis autonomously.

## Setup

```bash
cd feeder
npm install
node dist/index.js
```

## What it does

| Frequency | Action |
|-----------|--------|
| 30s | VaraBridge/UpdateAll (Binance prices, gas, news, markets) |
| 75s | VaraFlow/Tick (workflow engine) |
| 2h | VaraStrategy/Analyze → Board |
| 3h | Pulse DAO matchmaking → Chat |
| 12h | Identity post → Board |

## Deploy to keep it running 24/7

**Railway (free):**
```bash
cd feeder
npm install
railway up
```

**Render (free):** connect GitHub repo with `feeder/` as root.

## Files

- `feeder/dist/` — compiled JavaScript (ready to run)
- `feeder/package.json` — dependencies
- `.env` — all config (wallet, PIDs, API keys)
- `agents_network_client.idl` — A2A Hub IDL for Board/Chat posts
