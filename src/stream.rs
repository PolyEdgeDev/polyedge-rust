use crate::types::LiveTransaction;
use async_stream::stream;
use futures_util::{Stream, StreamExt};
use rand::Rng;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StreamError {
    #[error("PolyEdge fatal error: unauthorized (HTTP 401)")]
    Unauthorized,
    #[error("PolyEdge fatal error: forbidden or quota exceeded (HTTP 403)")]
    Forbidden,
    #[error("PolyEdge fatal error: stream not found (HTTP 404)")]
    NotFound,
    #[error("PolyEdge fatal error: bad request (HTTP 400)")]
    BadRequest,
    #[error("SSE watchdog timed out after {0:?} with no bytes or heartbeat received")]
    WatchdogTimeout(Duration),
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Stream error: {0}")]
    Custom(String),
}

impl StreamError {
    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            StreamError::Unauthorized
                | StreamError::Forbidden
                | StreamError::NotFound
                | StreamError::BadRequest
        )
    }
}

pub struct StreamOptions {
    pub stream_base_url: Option<String>,
    pub heartbeat_timeout: Option<Duration>,
    pub initial_backoff: Option<Duration>,
    pub max_backoff: Option<Duration>,
}

pub struct PolyEdgeStream {
    stream_id: String,
    api_key: String,
    stream_base_url: String,
    heartbeat_timeout: Duration,
    initial_backoff: Duration,
    max_backoff: Duration,
    last_event_id: Arc<AtomicU64>,
}

impl PolyEdgeStream {
    pub fn new(stream_id: impl Into<String>, api_key: impl Into<String>, options: Option<StreamOptions>) -> Self {
        let opts = options.unwrap_or(StreamOptions {
            stream_base_url: None,
            heartbeat_timeout: None,
            initial_backoff: None,
            max_backoff: None,
        });

        Self {
            stream_id: stream_id.into(),
            api_key: api_key.into(),
            stream_base_url: opts
                .stream_base_url
                .unwrap_or_else(|| "https://stream.polyedge.dev".to_string())
                .trim_end_matches('/')
                .to_string(),
            heartbeat_timeout: opts.heartbeat_timeout.unwrap_or(Duration::from_secs(45)),
            initial_backoff: opts.initial_backoff.unwrap_or(Duration::from_secs(1)),
            max_backoff: opts.max_backoff.unwrap_or(Duration::from_secs(30)),
            last_event_id: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn last_event_id(&self) -> u64 {
        self.last_event_id.load(Ordering::SeqCst)
    }

    pub fn subscribe(self) -> impl Stream<Item = Result<LiveTransaction, StreamError>> {
        let stream_id = self.stream_id.clone();
        let api_key = self.api_key.clone();
        let base_url = self.stream_base_url.clone();
        let heartbeat_timeout = self.heartbeat_timeout;
        let initial_backoff = self.initial_backoff;
        let max_backoff = self.max_backoff;
        let last_event_id_atomic = self.last_event_id.clone();

        let client = reqwest::Client::builder()
            .no_proxy()
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        stream! {
            let mut backoff = initial_backoff;

            loop {
                let last_id = last_event_id_atomic.load(Ordering::SeqCst);
                let mut url = format!("{}/streams/{}", base_url, stream_id);
                if last_id > 0 {
                    url.push_str(&format!("?lastEventId={}", last_id));
                }

                let mut req = client
                    .get(&url)
                    .header("Accept", "text/event-stream")
                    .header("X-PolyEdge-Key", &api_key)
                    .header("Cache-Control", "no-cache");

                if last_id > 0 {
                    req = req.header("Last-Event-ID", last_id.to_string());
                }

                let resp_res = req.send().await;
                let resp = match resp_res {
                    Ok(r) => r,
                    Err(e) => {
                        yield Err(StreamError::Network(e));
                        let jitter = backoff.as_secs_f64() * (0.8 + rand::thread_rng().gen_range(0.0..0.4));
                        tokio::time::sleep(Duration::from_secs_f64(jitter)).await;
                        backoff = std::cmp::min(max_backoff, backoff.mul_f64(2.0));
                        continue;
                    }
                };

                let status = resp.status();
                if !status.is_success() {
                    let err = match status.as_u16() {
                        400 => StreamError::BadRequest,
                        401 => StreamError::Unauthorized,
                        403 => StreamError::Forbidden,
                        404 => StreamError::NotFound,
                        code => StreamError::Custom(format!("Unexpected HTTP status {}", code)),
                    };

                    if err.is_fatal() {
                        yield Err(err);
                        return; // Fast-fail on fatal errors
                    }

                    yield Err(err);
                    let jitter = backoff.as_secs_f64() * (0.8 + rand::thread_rng().gen_range(0.0..0.4));
                    tokio::time::sleep(Duration::from_secs_f64(jitter)).await;
                    backoff = std::cmp::min(max_backoff, backoff.mul_f64(2.0));
                    continue;
                }

                // Reset backoff upon successful connection
                backoff = initial_backoff;

                let mut byte_stream = resp.bytes_stream();
                let mut buffer = String::new();

                loop {
                    // Watchdog: timeout if no bytes/heartbeat arrive within heartbeat_timeout
                    let next_chunk = tokio::time::timeout(heartbeat_timeout, byte_stream.next()).await;

                    let chunk = match next_chunk {
                        Ok(Some(Ok(bytes))) => bytes,
                        Ok(Some(Err(err))) => {
                            yield Err(StreamError::Network(err));
                            break; // Reconnect
                        }
                        Ok(None) => {
                            // Stream EOS
                            break;
                        }
                        Err(_) => {
                            // Watchdog timed out!
                            yield Err(StreamError::WatchdogTimeout(heartbeat_timeout));
                            break; // Reconnect
                        }
                    };

                    let chunk_str = String::from_utf8_lossy(&chunk);
                    buffer.push_str(&chunk_str);

                    while let Some(pos) = buffer.find("\n\n") {
                        let block = buffer[..pos].to_string();
                        buffer = buffer[pos + 2..].to_string();

                        let trimmed = block.trim();
                        if trimmed.is_empty() || trimmed.starts_with(':') {
                            // Server heartbeat or empty comment
                            continue;
                        }

                        let mut cur_id = None;
                        let mut cur_event = "message".to_string();
                        let mut cur_data = String::new();

                        for line in block.lines() {
                            let line = line.trim();
                            if line.starts_with(':') {
                                continue;
                            } else if let Some(id_str) = line.strip_prefix("id:") {
                                if let Ok(parsed_id) = id_str.trim().parse::<u64>() {
                                    cur_id = Some(parsed_id);
                                }
                            } else if let Some(ev) = line.strip_prefix("event:") {
                                cur_event = ev.trim().to_string();
                            } else if let Some(d) = line.strip_prefix("data:") {
                                cur_data.push_str(d.trim());
                            }
                        }

                        if let Some(id) = cur_id {
                            last_event_id_atomic.store(id, Ordering::SeqCst);
                        }

                        if cur_event == "tx" && !cur_data.is_empty() {
                            match serde_json::from_str::<LiveTransaction>(&cur_data) {
                                Ok(tx) => yield Ok(tx),
                                Err(err) => yield Err(StreamError::Json(err)),
                            }
                        }
                    }
                }

                // If broke out of read loop, apply backoff and reconnect
                let jitter = backoff.as_secs_f64() * (0.8 + rand::thread_rng().gen_range(0.0..0.4));
                tokio::time::sleep(Duration::from_secs_f64(jitter)).await;
                backoff = std::cmp::min(max_backoff, backoff.mul_f64(2.0));
            }
        }
    }
}
