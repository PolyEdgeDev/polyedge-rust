use crate::stream::{PolyEdgeStream, StreamOptions};
use crate::types::*;
use reqwest::Client as HttpClient;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("PolyEdge API error [{status}]: {body}")]
    Api { status: u16, body: String },
    #[error("HTTP request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serialization error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Service for Streams API calls.
pub struct StreamsService<'a> {
    client: &'a PolyEdgeClient,
}

impl<'a> StreamsService<'a> {
    /// Connects to real-time SSE order stream.
    pub fn connect(&self, stream_id: impl Into<String>) -> PolyEdgeStream {
        self.client.stream(stream_id)
    }

    /// List User Streams
    /// Returns custom and managed real-time trade streams configured by the user.
    pub async fn list(&self) -> Result<UserStreamsResponse, ClientError> {
        let path = "/streams".to_string();
        let url = format!("{}{}", self.client.stream_base_url, path);
        let resp = self.client.http_client.get(&url).header("X-PolyEdge-Key", &self.client.api_key).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Create Filtered Order Stream
    /// Provisions a new custom real-time stream with optional address, tag, or series filters.
    /// * `req` - Request payload
    pub async fn create(&self, req: &UserCreateStreamRequest) -> Result<UserStreamResponse, ClientError> {
        let path = "/streams".to_string();
        let url = format!("{}{}", self.client.stream_base_url, path);
        let resp = self.client.http_client.post(&url).header("X-PolyEdge-Key", &self.client.api_key).json(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Get Stream Metadata
    /// Fetches metadata, active sessions count, and filter configuration for a specific stream.
    /// * `id` - Stream unique ID
    pub async fn get(&self, id: &str) -> Result<UserStreamResponse, ClientError> {
        let path = format!("/streams/{}/meta", id);
        let url = format!("{}{}", self.client.stream_base_url, path);
        let resp = self.client.http_client.get(&url).header("X-PolyEdge-Key", &self.client.api_key).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Delete Stream
    /// Permanently removes a stream and gracefully disconnects all connected listener sessions.
    /// * `id` - Stream unique ID
    pub async fn delete(&self, id: &str) -> Result<DeleteStreamResponse, ClientError> {
        let path = format!("/streams/{}", id);
        let url = format!("{}{}", self.client.stream_base_url, path);
        let resp = self.client.http_client.delete(&url).header("X-PolyEdge-Key", &self.client.api_key).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Update Stream Metadata
    /// Updates operational attributes of the stream, such as nickname or enabled/disabled status.
    /// * `id` - Stream unique ID
    /// * `req` - Request payload
    pub async fn update_metadata(&self, id: &str, req: &UserUpdateStreamMetadataRequest) -> Result<UserStreamResponse, ClientError> {
        let path = format!("/streams/{}/meta", id);
        let url = format!("{}{}", self.client.stream_base_url, path);
        let resp = self.client.http_client.put(&url).header("X-PolyEdge-Key", &self.client.api_key).json(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Update Stream Filter Subscription
    /// Dynamically mutates monitored wallet addresses, market tags, or series slugs without disconnecting SSE clients.
    /// * `id` - Stream unique ID
    /// * `req` - Request payload
    pub async fn update_subscription(&self, id: &str, req: &UpdateSubscriptionRequest) -> Result<UserStreamResponse, ClientError> {
        let path = format!("/streams/{}/subscription", id);
        let url = format!("{}{}", self.client.stream_base_url, path);
        let resp = self.client.http_client.put(&url).header("X-PolyEdge-Key", &self.client.api_key).json(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// List All User Active Sessions
    /// Returns all currently connected active SSE listeners for this account.
    pub async fn get_active_sessions(&self) -> Result<UserActiveSessionsResponse, ClientError> {
        let path = "/sessions".to_string();
        let url = format!("{}{}", self.client.stream_base_url, path);
        let resp = self.client.http_client.get(&url).header("X-PolyEdge-Key", &self.client.api_key).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// List User Stream Session History
    /// Returns historical SSE connection logs including duration and pushed transactions count.
    /// * `limit` - Max records to return
    /// * `offset` - Pagination offset
    pub async fn get_session_history(&self, limit: Option<i64>, offset: Option<i64>) -> Result<UserSessionHistoryResponse, ClientError> {
        let path = "/sessions/history".to_string();
        let mut url = format!("{}{}", self.client.stream_base_url, path);
        let mut query_pairs = Vec::new();
        if let Some(v) = limit { query_pairs.push(format!("limit={}", v)); }
        if let Some(v) = offset { query_pairs.push(format!("offset={}", v)); }
        if !query_pairs.is_empty() { url.push('?'); url.push_str(&query_pairs.join("&")); }
        let resp = self.client.http_client.get(&url).header("X-PolyEdge-Key", &self.client.api_key).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

}

/// Service for Analytics API calls.
pub struct AnalyticsService<'a> {
    client: &'a PolyEdgeClient,
}

impl<'a> AnalyticsService<'a> {
    /// 60-Day Trader Deposit Analytics
    /// Returns 60-day aggregated deposit summary and individual transaction breakdown for top traders.
    /// * `limit` - Max records to return
    /// * `offset` - Pagination offset
    pub async fn get_deposits(&self, limit: Option<i64>, offset: Option<i64>) -> Result<DepositQueryResult, ClientError> {
        let path = "/v2/analytics/deposits".to_string();
        let mut url = format!("{}{}", self.client.api_base_url, path);
        let mut query_pairs = Vec::new();
        if let Some(v) = limit { query_pairs.push(format!("limit={}", v)); }
        if let Some(v) = offset { query_pairs.push(format!("offset={}", v)); }
        if !query_pairs.is_empty() { url.push('?'); url.push_str(&query_pairs.join("&")); }
        let resp = self.client.http_client.get(&url).header("X-PolyEdge-Key", &self.client.api_key).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Top Traders PnL Leaderboard
    /// Queries ranked traders across timeframes with PnL, volume, win rate, and performance metrics.
    /// * `limit` - Max traders to return
    /// * `offset` - Pagination offset
    pub async fn get_leaderboard(&self, limit: Option<i64>, offset: Option<i64>) -> Result<LeaderboardResponse, ClientError> {
        let path = "/v2/analytics/leaderboard".to_string();
        let mut url = format!("{}{}", self.client.api_base_url, path);
        let mut query_pairs = Vec::new();
        if let Some(v) = limit { query_pairs.push(format!("limit={}", v)); }
        if let Some(v) = offset { query_pairs.push(format!("offset={}", v)); }
        if !query_pairs.is_empty() { url.push('?'); url.push_str(&query_pairs.join("&")); }
        let resp = self.client.http_client.get(&url).header("X-PolyEdge-Key", &self.client.api_key).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Get Prediction Market Detail
    /// Fetches standardized market metadata matching on-chain condition IDs and NegRisk parameters.
    /// * `id` - Market numeric ID or condition ID
    pub async fn get_market(&self, id: &str) -> Result<MarketDetailResponse, ClientError> {
        let path = format!("/v2/markets/{}", id);
        let url = format!("{}{}", self.client.api_base_url, path);
        let resp = self.client.http_client.get(&url).header("X-PolyEdge-Key", &self.client.api_key).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Get Trader Intelligence Profile
    /// Fetches trader identity dossier, pUSD balance, total PnL, win rates, and ranking metrics.
    /// * `address` - Trader Polygon/Ethereum wallet address
    pub async fn get_trader(&self, address: &str) -> Result<TraderProfileResponse, ClientError> {
        let path = format!("/v2/traders/{}", address);
        let url = format!("{}{}", self.client.api_base_url, path);
        let resp = self.client.http_client.get(&url).header("X-PolyEdge-Key", &self.client.api_key).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Get Hourly PnL Equity Curve
    /// Provides 1-hour bucketed historical equity curves and trade metrics for a specific trader.
    /// * `address` - Trader Polygon/Ethereum wallet address
    pub async fn get_trader_hourly_stats(&self, address: &str) -> Result<TraderHourlyStatsResponse, ClientError> {
        let path = format!("/v2/traders/{}/hourly_stats", address);
        let url = format!("{}{}", self.client.api_base_url, path);
        let resp = self.client.http_client.get(&url).header("X-PolyEdge-Key", &self.client.api_key).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Get Trader Market Participation History
    /// Queries resolved and active prediction markets traded by the specified wallet address.
    /// * `address` - Trader Polygon/Ethereum wallet address
    /// * `limit` - Max markets to return
    /// * `offset` - Pagination offset
    pub async fn get_trader_markets(&self, address: &str, limit: Option<i64>, offset: Option<i64>) -> Result<TraderHistoryResponse, ClientError> {
        let path = format!("/v2/traders/{}/markets", address);
        let mut url = format!("{}{}", self.client.api_base_url, path);
        let mut query_pairs = Vec::new();
        if let Some(v) = limit { query_pairs.push(format!("limit={}", v)); }
        if let Some(v) = offset { query_pairs.push(format!("offset={}", v)); }
        if !query_pairs.is_empty() { url.push('?'); url.push_str(&query_pairs.join("&")); }
        let resp = self.client.http_client.get(&url).header("X-PolyEdge-Key", &self.client.api_key).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Get Trader Orders in Market
    /// Retrieves granular fill events, side, price, and token outcomes for a trader in a given market.
    /// * `address` - Trader wallet address
    /// * `id` - Market numeric ID
    /// * `limit` - Max orders to return
    /// * `offset` - Pagination offset
    pub async fn get_trader_market_orders(&self, address: &str, id: &str, limit: Option<i64>, offset: Option<i64>) -> Result<UserOrdersResponse, ClientError> {
        let path = format!("/v2/traders/{}/markets/{}/orders", address, id);
        let mut url = format!("{}{}", self.client.api_base_url, path);
        let mut query_pairs = Vec::new();
        if let Some(v) = limit { query_pairs.push(format!("limit={}", v)); }
        if let Some(v) = offset { query_pairs.push(format!("offset={}", v)); }
        if !query_pairs.is_empty() { url.push('?'); url.push_str(&query_pairs.join("&")); }
        let resp = self.client.http_client.get(&url).header("X-PolyEdge-Key", &self.client.api_key).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

}

/// Service for Account API calls.
pub struct AccountService<'a> {
    client: &'a PolyEdgeClient,
}

impl<'a> AccountService<'a> {
    /// Get Current Profile
    /// Retrieves the authenticated user profile, tier subscription status, and available USDC balance.
    pub async fn get_profile(&self) -> Result<UserProfile, ClientError> {
        let path = "/v2/user/me".to_string();
        let url = format!("{}{}", self.client.api_base_url, path);
        let resp = self.client.http_client.get(&url).header("X-PolyEdge-Key", &self.client.api_key).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// List API Keys
    /// Lists all active and revoked API keys issued to the authenticated account.
    pub async fn list_keys(&self) -> Result<UserKeysResponse, ClientError> {
        let path = "/v2/user/keys".to_string();
        let url = format!("{}{}", self.client.api_base_url, path);
        let resp = self.client.http_client.get(&url).header("X-PolyEdge-Key", &self.client.api_key).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Create API Key
    /// Generates a new authenticated API key token with optional memo label.
    /// * `req` - Request payload
    pub async fn create_key(&self, req: &CreateAPIKeyRequest) -> Result<UserAPIKey, ClientError> {
        let path = "/v2/user/keys".to_string();
        let url = format!("{}{}", self.client.api_base_url, path);
        let resp = self.client.http_client.post(&url).header("X-PolyEdge-Key", &self.client.api_key).json(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Update API Key
    /// Modifies label memo or operational status of an existing API key.
    /// * `key` - API key string
    /// * `req` - Request payload
    pub async fn update_key(&self, key: &str, req: &UserUpdateKeyRequest) -> Result<UserAPIKey, ClientError> {
        let path = format!("/v2/user/keys/{}", key);
        let url = format!("{}{}", self.client.api_base_url, path);
        let resp = self.client.http_client.patch(&url).header("X-PolyEdge-Key", &self.client.api_key).json(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Revoke API Key
    /// Permanently revokes and deactivates an API key token.
    /// * `key` - API key string
    pub async fn delete_key(&self, key: &str) -> Result<DeleteKeyResponse, ClientError> {
        let path = format!("/v2/user/keys/{}", key);
        let url = format!("{}{}", self.client.api_base_url, path);
        let resp = self.client.http_client.delete(&url).header("X-PolyEdge-Key", &self.client.api_key).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// List User Balance Ledger
    /// Lists chronological ledger entries (subscription billing charges, deposits, withdrawals).
    /// * `limit` - Max entries to return
    /// * `offset` - Pagination offset
    pub async fn get_ledger(&self, limit: Option<i64>, offset: Option<i64>) -> Result<UserLedgerResponse, ClientError> {
        let path = "/v2/user/ledger".to_string();
        let mut url = format!("{}{}", self.client.api_base_url, path);
        let mut query_pairs = Vec::new();
        if let Some(v) = limit { query_pairs.push(format!("limit={}", v)); }
        if let Some(v) = offset { query_pairs.push(format!("offset={}", v)); }
        if !query_pairs.is_empty() { url.push('?'); url.push_str(&query_pairs.join("&")); }
        let resp = self.client.http_client.get(&url).header("X-PolyEdge-Key", &self.client.api_key).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Get Account Telemetry & Limits
    /// Provides live usage statistics, quota limits, and remaining streaming bandwidth.
    pub async fn get_telemetry(&self) -> Result<UserTelemetryResponse, ClientError> {
        let path = "/v2/user/telemetry".to_string();
        let url = format!("{}{}", self.client.api_base_url, path);
        let resp = self.client.http_client.get(&url).header("X-PolyEdge-Key", &self.client.api_key).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Request Balance Withdrawal
    /// Submits a request to withdraw unspent USDC balance to a designated Polygon address.
    /// * `req` - Request payload
    pub async fn withdraw(&self, req: &UserWithdrawRequest) -> Result<UserWithdrawResponse, ClientError> {
        let path = "/v2/user/withdraw".to_string();
        let url = format!("{}{}", self.client.api_base_url, path);
        let resp = self.client.http_client.post(&url).header("X-PolyEdge-Key", &self.client.api_key).json(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

}

/// Service for Subscription API calls.
pub struct SubscriptionService<'a> {
    client: &'a PolyEdgeClient,
}

impl<'a> SubscriptionService<'a> {
    /// List Public Tiers Catalog
    /// Returns plan tiers (Free, Starter, Pro, Growth, Whale) with limits and pricing details.
    pub async fn list_tiers(&self) -> Result<Vec<Tier>, ClientError> {
        let path = "/v2/tiers".to_string();
        let url = format!("{}{}", self.client.api_base_url, path);
        let resp = self.client.http_client.get(&url).header("X-PolyEdge-Key", &self.client.api_key).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Get Subscription Quote
    /// Calculates prorated billing amounts for subscribing to or upgrading a tier plan.
    /// * `req` - Request payload
    pub async fn get_quote(&self, req: &QuoteSubscriptionRequest) -> Result<QuoteSubscriptionResponse, ClientError> {
        let path = "/v2/user/subscribe/quote".to_string();
        let url = format!("{}{}", self.client.api_base_url, path);
        let resp = self.client.http_client.post(&url).header("X-PolyEdge-Key", &self.client.api_key).json(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Purchase / Upgrade Tier Subscription
    /// Executes purchase, renewal, or upgrade of an active subscription tier plan.
    /// * `req` - Request payload
    pub async fn subscribe(&self, req: &SubscribeRequest) -> Result<SubscribeResponse, ClientError> {
        let path = "/v2/user/subscribe".to_string();
        let url = format!("{}{}", self.client.api_base_url, path);
        let resp = self.client.http_client.post(&url).header("X-PolyEdge-Key", &self.client.api_key).json(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

    /// Validate Promo Code
    /// Verifies validity, discount percentage, and applicable plans for a promotion coupon code.
    /// * `req` - Request payload
    pub async fn validate_promo_code(&self, req: &ValidatePromoCodeRequest) -> Result<ValidatePromoCodeResponse, ClientError> {
        let path = "/v2/user/promo-codes/validate".to_string();
        let url = format!("{}{}", self.client.api_base_url, path);
        let resp = self.client.http_client.post(&url).header("X-PolyEdge-Key", &self.client.api_key).json(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api { status, body });
        }
        Ok(resp.json().await?)
    }

}

/// PolyEdge Official API Client
pub struct PolyEdgeClient {
    api_key: String,
    api_base_url: String,
    stream_base_url: String,
    http_client: HttpClient,
}

impl PolyEdgeClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        let http_client = HttpClient::builder()
            .no_proxy()
            .build()
            .unwrap_or_else(|_| HttpClient::new());
        Self {
            api_key: api_key.into(),
            api_base_url: "https://api.polyedge.dev".to_string(),
            stream_base_url: "https://stream.polyedge.dev".to_string(),
            http_client,
        }
    }

    pub fn with_urls(mut self, api_base_url: impl Into<String>, stream_base_url: impl Into<String>) -> Self {
        self.api_base_url = api_base_url.into().trim_end_matches('/').to_string();
        self.stream_base_url = stream_base_url.into().trim_end_matches('/').to_string();
        self
    }

    /// Streams service handle
    pub fn streams(&self) -> StreamsService<'_> {
        StreamsService { client: self }
    }

    /// Analytics service handle
    pub fn analytics(&self) -> AnalyticsService<'_> {
        AnalyticsService { client: self }
    }

    /// Account service handle
    pub fn account(&self) -> AccountService<'_> {
        AccountService { client: self }
    }

    /// Subscription service handle
    pub fn subscription(&self) -> SubscriptionService<'_> {
        SubscriptionService { client: self }
    }

    /// Direct real-time SSE order stream factory
    pub fn stream(&self, stream_id: impl Into<String>) -> PolyEdgeStream {
        PolyEdgeStream::new(
            stream_id,
            self.api_key.clone(),
            Some(StreamOptions {
                stream_base_url: Some(self.stream_base_url.clone()),
                heartbeat_timeout: None,
                initial_backoff: None,
                max_backoff: None,
            }),
        )
    }
}