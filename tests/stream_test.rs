use futures_util::StreamExt;
use polyedge::{PolyEdgeStream, StreamError, StreamOptions};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::test]
async fn test_rust_stream_receive_and_heartbeat() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        if let Ok((mut socket, _)) = listener.accept().await {
            let mut buf = [0u8; 1024];
            let _ = socket.read(&mut buf).await;

            let response = "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\n\r\n: heartbeat\n\nid: 999\nevent: tx\ndata: {\"tx_hash\":\"0xrust_test\",\"timestamp\":\"2026-10-09T00:00:00Z\",\"market\":{\"id\":1,\"question\":\"\",\"slug\":\"\",\"event_slug\":\"\",\"series_slug\":\"\",\"condition_id\":\"\",\"neg_risk\":false,\"sports_market_type\":\"\",\"start_date\":\"\",\"tags_slug\":[],\"outcomes\":[\"Yes\",\"No\"],\"token_ids\":[\"1\",\"2\"]},\"taker\":{\"fee\":\"0\",\"order\":{\"order_hash\":\"\",\"shares\":\"1\",\"signature_type\":1,\"timestamp\":\"\",\"usdc\":\"1\"},\"outcome\":\"Yes\",\"shares\":\"1\",\"side\":\"BUY\",\"token_ids_index\":0,\"usdc\":\"1\",\"user\":{\"address\":\"0x1\",\"name\":\"\"}}}\n\n";
            let _ = socket.write_all(response.as_bytes()).await;
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    });

    let stream = PolyEdgeStream::new(
        "test-stream",
        "test_key",
        Some(StreamOptions {
            stream_base_url: Some(format!("http://127.0.0.1:{}", port)),
            heartbeat_timeout: Some(Duration::from_secs(5)),
            initial_backoff: Some(Duration::from_millis(50)),
            max_backoff: Some(Duration::from_millis(200)),
        }),
    );

    let mut stream_iter = Box::pin(stream.subscribe());
    let item = stream_iter.next().await;
    assert!(item.is_some());
    let tx_res = item.unwrap();
    assert!(tx_res.is_ok());
    let tx = tx_res.unwrap();
    assert_eq!(tx.tx_hash.as_deref(), Some("0xrust_test"));
}

#[tokio::test]
async fn test_rust_stream_fatal_401() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        if let Ok((mut socket, _)) = listener.accept().await {
            let mut buf = [0u8; 1024];
            let _ = socket.read(&mut buf).await;

            let response = "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\nContent-Length: 25\r\n\r\n{\"error\":\"Unauthorized\"}";
            let _ = socket.write_all(response.as_bytes()).await;
        }
    });

    let stream = PolyEdgeStream::new(
        "fatal-stream",
        "bad_key",
        Some(StreamOptions {
            stream_base_url: Some(format!("http://127.0.0.1:{}", port)),
            heartbeat_timeout: Some(Duration::from_secs(5)),
            initial_backoff: Some(Duration::from_millis(50)),
            max_backoff: Some(Duration::from_millis(200)),
        }),
    );

    let mut stream_iter = Box::pin(stream.subscribe());
    let item = stream_iter.next().await;
    assert!(item.is_some());
    let err = item.unwrap().unwrap_err();
    assert!(matches!(err, StreamError::Unauthorized));
    assert!(err.is_fatal());
}

#[tokio::test]
async fn test_rust_stream_watchdog_and_resume() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let conns = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let last_event_id_received = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

    let conns_clone = conns.clone();
    let last_event_id_clone = last_event_id_received.clone();

    tokio::spawn(async move {
        while let Ok((mut socket, _)) = listener.accept().await {
            let conn_num = conns_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let mut buf = [0u8; 2048];
            let n = socket.read(&mut buf).await.unwrap_or(0);
            let req_str = String::from_utf8_lossy(&buf[..n]);

            let last_id = req_str
                .lines()
                .find(|l| l.to_lowercase().starts_with("last-event-id:"))
                .map(|l| l[14..].trim().to_string());
            last_event_id_clone.lock().unwrap().push(last_id);

            if conn_num == 0 {
                // First connection: send message with id 444, then stall to trigger watchdog
                let res = "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\n\r\nid: 444\nevent: tx\ndata: {\"tx_hash\":\"0xfirst\",\"timestamp\":\"\",\"market\":{\"id\":1,\"question\":\"\",\"slug\":\"\",\"event_slug\":\"\",\"series_slug\":\"\",\"condition_id\":\"\",\"neg_risk\":false,\"sports_market_type\":\"\",\"start_date\":\"\",\"tags_slug\":[],\"outcomes\":[\"Yes\",\"No\"],\"token_ids\":[\"1\",\"2\"]},\"taker\":{\"fee\":\"0\",\"order\":{\"order_hash\":\"\",\"shares\":\"1\",\"signature_type\":1,\"timestamp\":\"\",\"usdc\":\"1\"},\"outcome\":\"Yes\",\"shares\":\"1\",\"side\":\"BUY\",\"token_ids_index\":0,\"usdc\":\"1\",\"user\":{\"address\":\"0x1\",\"name\":\"\"}}}\n\n";
                let _ = socket.write_all(res.as_bytes()).await;
                tokio::time::sleep(Duration::from_millis(1500)).await;
            } else {
                // Second connection: send heartbeat and second tx
                let res = "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\n\r\n: heartbeat\n\nid: 445\nevent: tx\ndata: {\"tx_hash\":\"0xsecond\",\"timestamp\":\"\",\"market\":{\"id\":1,\"question\":\"\",\"slug\":\"\",\"event_slug\":\"\",\"series_slug\":\"\",\"condition_id\":\"\",\"neg_risk\":false,\"sports_market_type\":\"\",\"start_date\":\"\",\"tags_slug\":[],\"outcomes\":[\"Yes\",\"No\"],\"token_ids\":[\"1\",\"2\"]},\"taker\":{\"fee\":\"0\",\"order\":{\"order_hash\":\"\",\"shares\":\"1\",\"signature_type\":1,\"timestamp\":\"\",\"usdc\":\"1\"},\"outcome\":\"Yes\",\"shares\":\"1\",\"side\":\"BUY\",\"token_ids_index\":0,\"usdc\":\"1\",\"user\":{\"address\":\"0x1\",\"name\":\"\"}}}\n\n";
                let _ = socket.write_all(res.as_bytes()).await;
            }
        }
    });

    let stream = PolyEdgeStream::new(
        "watchdog-stream",
        "test_key",
        Some(StreamOptions {
            stream_base_url: Some(format!("http://127.0.0.1:{}", port)),
            heartbeat_timeout: Some(Duration::from_millis(300)), // 300ms watchdog
            initial_backoff: Some(Duration::from_millis(50)),
            max_backoff: Some(Duration::from_millis(200)),
        }),
    );

    let mut stream_iter = Box::pin(stream.subscribe());
    let mut received = Vec::new();

    while let Some(item) = stream_iter.next().await {
        match item {
            Ok(tx) => {
                received.push(tx.tx_hash);
                if received.len() == 2 {
                    break;
                }
            }
            Err(StreamError::WatchdogTimeout(_)) => {
                // Expected watchdog event on first disconnect
            }
            Err(e) => {
                eprintln!("Non-fatal stream notice: {:?}", e);
            }
        }
    }

    assert_eq!(received.len(), 2);
    assert_eq!(received[0].as_deref(), Some("0xfirst"));
    assert_eq!(received[1].as_deref(), Some("0xsecond"));

    let history = last_event_id_received.lock().unwrap();
    assert!(history.len() >= 2);
    assert_eq!(history[0], None);
    assert_eq!(history[1], Some("444".to_string()));
}

