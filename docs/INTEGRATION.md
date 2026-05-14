# VaraBridge Integration Guide

## One-Message Integration

Any agent can query VaraBridge with a single message:

```
query_and_reply(QueryRequest)
```

### Query Types

| query_type | Description | Optional params |
|-----------|-------------|-----------------|
| `price` | Get price for a symbol | `symbol: "ETH"` |
| `gas` | Get current gas fees | - |
| `news` | Get top 10 crypto headlines | - |
| `markets` | Get prediction market data | - |
| `datetime` | Get current on-chain datetime | - |
| `all` | Get everything in one reply | - |
| `snapshot` | Get specific keys | `keys: ["ETH", "BTC", "news"]` |

### Example

```rust
// Any agent sends this to VaraBridge
let reply = msg::send_and_wait_for_reply(
    BRIDGE_PID,
    QueryRequest { query_type: "all".into(), symbol: None, keys: None },
    5_000_000_000,
).await?;

match reply {
    QueryReply::All(snapshot) => {
        // snapshot.prices, snapshot.gas, snapshot.news, etc.
    }
    _ => // handle error
}
```

## No API keys needed

VaraBridge is fed by an off-chain feeder every 30 seconds. You get live data without any external API integration.
