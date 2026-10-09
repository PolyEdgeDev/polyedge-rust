use futures_util::StreamExt;
use polyedge::PolyEdgeClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = match env::var("POLYEDGE_API_KEY") {
        Ok(k) => k,
        Err(_) => {
            eprintln!("POLYEDGE_API_KEY environment variable is required");
            std::process::exit(1);
        }
    };

    println!("==================================================");
    println!("🚀 [Rust SDK] Running Live Production E2E Test");
    println!("==================================================");

    let client = PolyEdgeClient::new(&api_key);

    // 1. Account Service
    println!("\n[1/5] Testing Account.get_profile()...");
    let profile = client.account().get_profile().await?;
    println!("  -> User Email: {:?}", profile.email);
    println!("  -> Tier: {:?}, Status: {:?}", profile.tier, profile.status);

    // 2. Analytics Service
    println!("\n[2/5] Testing Analytics.get_leaderboard(limit: 3)...");
    let leaderboard = client.analytics().get_leaderboard(Some(3), None).await?;
    println!("  -> Total Traders Count: {:?}", leaderboard.total_count);
    if let Some(traders) = leaderboard.traders {
        for (i, t) in traders.iter().take(3).enumerate() {
            println!(
                "     #{} {:?} ({:?}) - PnL micro-USD: {:?}",
                i + 1,
                t.profile.as_ref().and_then(|p| p.address.as_ref()),
                t.profile.as_ref().and_then(|p| p.name.as_ref()),
                t.total_pnl
            );
        }
    }

    // 3. Subscription Service
    println!("\n[3/5] Testing Subscription.list_tiers()...");
    let tiers = client.subscription().list_tiers().await?;
    println!("  -> Public Tiers Available: {} tiers found", tiers.len());

    // 4. Streams Service: List Streams
    println!("\n[4/5] Testing Streams.list()...");
    let streams_res = client.streams().list().await?;
    let streams_list = streams_res.data.unwrap_or_default();
    if streams_list.is_empty() {
        eprintln!("No available streams found for this account");
        std::process::exit(1);
    }
    let active_stream = &streams_list[0];
    let stream_id = active_stream.id.clone().unwrap_or_default();
    println!("  -> Found active stream: {} (name: {:?})", stream_id, active_stream.name);

    // 5. Streams Service: Live SSE Stream Connect
    println!("\n[5/5] Testing Live SSE Stream Connect ({})...", stream_id);
    let stream = client.streams().connect(&stream_id);
    let mut stream_iter = Box::pin(stream.subscribe());
    let mut received_count = 0;

    let timeout_duration = std::time::Duration::from_secs(15);
    let start = std::time::Instant::now();

    while let Some(item) = stream_iter.next().await {
        if start.elapsed() > timeout_duration {
            eprintln!("Timed out waiting for real-time SSE trade events");
            std::process::exit(1);
        }

        match item {
            Ok(tx) => {
                received_count += 1;
                let tx_hash = tx.tx_hash.unwrap_or_default();
                let short_hash = if tx_hash.len() > 18 { &tx_hash[..18] } else { &tx_hash };
                let question = tx.market.and_then(|m| m.question).unwrap_or_default();
                let taker = tx.taker;
                let outcome = taker.as_ref().and_then(|t| t.outcome.as_ref()).cloned().unwrap_or_default();
                let side = taker.as_ref().and_then(|t| t.side.as_ref()).cloned();
                let shares = taker.as_ref().and_then(|t| t.shares.as_ref()).cloned().unwrap_or_default();
                let makers_count = tx.makers.map(|m| m.len()).unwrap_or(0);

                println!("  ⚡ [Live Event] Tx: {}...", short_hash);
                println!("     Market: \"{}\"", question);
                println!("     Taker: {} ({:?}) | Shares: {}", outcome, side, shares);
                println!("     Makers: {} matched orders", makers_count);

                if received_count >= 2 {
                    break;
                }
            }
            Err(e) => {
                eprintln!("  Notice: {:?}", e);
            }
        }
    }

    println!("\n==================================================");
    println!("✅ [Rust SDK] All Live E2E tests passed successfully!");
    println!("==================================================");

    Ok(())
}
