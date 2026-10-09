# PolyEdge Rust SDK

[![Crates.io](https://img.shields.io/crates/v/polyedge.svg?color=blue)](https://crates.io/crates/polyedge)
[![Docs.rs](https://docs.rs/polyedge/badge.svg)](https://docs.rs/polyedge)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://img.shields.io/badge/docs-polyedge.dev-cyan)](https://polyedge.dev/docs)
[![Benchmark](https://img.shields.io/badge/benchmark-100%2B_nodes-green)](https://github.com/PolyEdgeDev/polyedge-stream-benchmark)

Official Rust SDK for **[PolyEdge](https://polyedge.dev)** — **Ultra-Low-Latency Polymarket Mempool Trade Streaming & Real-Time On-Chain Analytics API.**

> Engineered for Polymarket copy-trading bots, professional traders, and prediction market builders. Capture pending trades pre-block and track smart money with institutional-grade on-chain analytics.

---

## Key Features

- **4 First-Class Namespaces**: Aligned with OpenAPI 3.1:
  - `client.streams()`: Real-time SSE trade streaming, sessions & filter management.
  - `client.analytics()`: Polymarket trade attribution, trader leaderboards, market settlement audit.
  - `client.account()`: User profile, API keys, request telemetry, and ledger.
  - `client.subscription()`: Pricing tier catalog, upgrade quotes, and promo codes.
- **Async Streaming with Tokio & Reqwest**:
  - **45s Keep-Alive Watchdog**: Native read-timeout detection automatically cleans up dead socket half-open connections.
  - **Atomic `Last-Event-ID` Resumption**: Seamlessly captures buffered trades on reconnection.
  - **Fail-Fast for Fatal Errors**: Immediate non-retry bailout on HTTP 401/403/404.
  - **Standard Futures Stream**: Implements `futures_util::Stream<Item = Result<LiveTransaction, StreamError>>`.
- **Pre-Block Polymarket Trade Streaming**: Peered with 100+ Polygon nodes to capture pending Polymarket trades pre-block. The World's Fastest — [verify yourself](https://github.com/PolyEdgeDev/polyedge-stream-benchmark).

---

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
polyedge = "1.0.0"
tokio = { version = "1", features = ["full"] }
futures-util = "0.3"
```

Or via `cargo add`:

```bash
cargo add polyedge tokio --features full futures-util
```

---

## Quick Start

### 1. Initialize Client & Query Analytics

```rust
use polyedge::PolyEdgeClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("POLYEDGE_API_KEY").expect("POLYEDGE_API_KEY is required");
    let client = PolyEdgeClient::new(&api_key);

    // 1. Query Top Traders Leaderboard
    let leaderboard = client.analytics().get_leaderboard(Some(5), None).await?;
    println!("Total active traders: {:?}", leaderboard.total_count);

    if let Some(traders) = leaderboard.traders {
        for (i, t) in traders.iter().enumerate() {
            let addr = t.profile.as_ref().and_then(|p| p.address.as_deref()).unwrap_or("0x...");
            let name = t.profile.as_ref().and_then(|p| p.name.as_deref()).unwrap_or("Anon");
            let pnl_usd = (t.total_pnl.unwrap_or(0) as f64) / 1e6;
            println!("#{i} {addr} ({name}) - PnL: ${pnl_usd:.2}");
        }
    }

    Ok(())
}
```

### 2. Real-Time Mempool SSE Trade Stream

```rust
use futures_util::StreamExt;
use polyedge::PolyEdgeClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("POLYEDGE_API_KEY").expect("POLYEDGE_API_KEY is required");
    let client = PolyEdgeClient::new(&api_key);

    // Connect to active stream
    let stream = client.streams().connect("your_stream_id");
    let mut stream_iter = Box::pin(stream.subscribe());

    println!("Listening for real-time Polymarket mempool trades...");

    while let Some(item) = stream_iter.next().await {
        match item {
            Ok(tx) => {
                let tx_hash = tx.tx_hash.as_deref().unwrap_or("");
                let short_hash = if tx_hash.len() > 18 { &tx_hash[..18] } else { tx_hash };
                let question = tx.market.as_ref().and_then(|m| m.question.as_deref()).unwrap_or("");
                let taker = tx.taker.as_ref();
                let outcome = taker.and_then(|t| t.outcome.as_deref()).unwrap_or("");
                let side = taker.and_then(|t| t.side.as_ref());
                let shares = taker.and_then(|t| t.shares.as_deref()).unwrap_or("0");
                let makers_count = tx.makers.as_ref().map(|m| m.len()).unwrap_or(0);

                println!("⚡ [Live Trade] Tx: {short_hash}...");
                println!("   Market: \"{question}\"");
                println!("   Taker: {outcome} ({side:?}) | Shares: {shares}");
                println!("   Matched Makers: {makers_count} orders");
            }
            Err(e) => {
                eprintln!("Stream notice/reconnect: {e:?}");
            }
        }
    }

    Ok(())
}
```

---

## Complete API Service Reference (28 Methods)

### 1. Streams Service (`client.streams()`)

| Method | HTTP | Description |
| :--- | :--- | :--- |
| `connect(stream_id)` | `GET /streams/{id}` (SSE) | Connect to ultra-low-latency real-time SSE mempool trade stream |
| `list().await` | `GET /streams` | List all configured data streams |
| `create(&req).await` | `POST /streams` | Provision a new targeted filter stream (tags, addresses, series) |
| `get(id).await` | `GET /streams/{id}` | Get stream metadata and configuration by ID |
| `update_metadata(id, &req).await` | `PUT /streams/{id}/meta` | Update stream name and description |
| `update_subscription(id, &req).await` | `PUT /streams/{id}/subscription` | Hot-reload market filters (addresses, tags, series) on active stream |
| `delete(id).await` | `DELETE /streams/{id}` | Delete stream by ID |
| `get_active_sessions().await` | `GET /sessions` | List all active live SSE connections across streams |
| `get_session_history(limit, offset).await` | `GET /sessions/history` | Query historical SSE connection logs and durations |

### 2. Analytics Service (`client.analytics()`)

| Method | HTTP | Description |
| :--- | :--- | :--- |
| `get_leaderboard(limit, offset).await` | `GET /v2/analytics/leaderboard` | Top profitable traders ranked by realized PnL, volume, and ROI |
| `get_trader(address).await` | `GET /v2/traders/{address}` | Comprehensive trader intelligence profile, taker tier, and top tags |
| `get_trader_hourly_stats(address).await`| `GET /v2/traders/{address}/hourly_stats` | 24-hour hourly trading PnL and volume breakdown |
| `get_trader_markets(addr, limit, offset).await` | `GET /v2/traders/{address}/markets` | Historical market positions and settled outcomes by trader |
| `get_trader_market_orders(addr, id, ...).await` | `GET /v2/traders/{address}/markets/{id}/orders` | Order fill details for a specific trader in a specific market |
| `get_market(id).await` | `GET /v2/markets/{id}` | Prediction market detail, top 20 earners, and volume attribution |
| `get_deposits(limit, offset).await` | `GET /v2/analytics/deposits` | 60-day aggregated trader deposit summaries (>= 100 pUSD) |

### 3. Account Service (`client.account()`)

| Method | HTTP | Description |
| :--- | :--- | :--- |
| `get_profile().await` | `GET /v2/user/me` | Current user profile, active tier, and deposit balance |
| `list_keys().await` | `GET /v2/user/keys` | List all active API keys and statuses |
| `create_key(&req).await` | `POST /v2/user/keys` | Generate a new API key with custom name |
| `update_key(key, &req).await` | `PATCH /v2/user/keys/{key}` | Enable, disable, or rename an API key |
| `delete_key(key).await` | `DELETE /v2/user/keys/{key}` | Revoke and delete an API key |
| `get_ledger(limit, offset).await` | `GET /v2/user/ledger` | Balance ledger transaction records |
| `get_telemetry().await` | `GET /v2/user/telemetry` | Daily request count and per-endpoint usage telemetry |
| `withdraw(&req).await` | `POST /v2/user/withdraw` | Request balance withdrawal |

### 4. Subscription Service (`client.subscription()`)

| Method | HTTP | Description |
| :--- | :--- | :--- |
| `list_tiers().await` | `GET /v2/tiers` | Public tier catalog, feature allowances, and pricing specifications |
| `get_quote(&req).await` | `POST /v2/user/subscribe/quote` | Calculate quote for plan upgrade or billing cycle change |
| `subscribe(&req).await` | `POST /v2/user/subscribe` | Purchase or upgrade tier subscription |
| `validate_promo_code(&req).await` | `POST /v2/user/promo-codes/validate` | Validate promotional discount code |

---

## License

[MIT](LICENSE) © 2026 [PolyEdge Labs](https://polyedge.dev)
