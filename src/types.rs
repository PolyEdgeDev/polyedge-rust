use serde::{Deserialize, Serialize};

/// Trade execution side (BUY = 0, SELL = 1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}

/// Data model for ActiveSessionInfo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActiveSessionInfo {
    /// Masked API credential used to authenticate the stream connection
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// Remote IP address of the connected client
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    /// ISO 8601 timestamp when the SSE connection was established
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connected_at: Option<String>,
    /// Total active connection uptime in seconds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    /// Total number of live matched order events delivered over this connection
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pushed_tx_count: Option<i64>,
    /// Unique persistent stream identifier (UUID)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    /// HTTP User-Agent identifier of the client library or application
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

/// Data model for CreateAPIKeyRequest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAPIKeyRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Data model for DeleteKeyResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteKeyResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Data model for DeleteStreamResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteStreamResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Data model for DepositItem
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepositItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deposit_count: Option<i64>,
    /// Earliest deposit time in 60-day window (RFC3339)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_deposit_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_trade_at: Option<String>,
    /// Most recent deposit time (RFC3339)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_deposit_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_trade_at: Option<String>,
    /// 6 decimals micro-USD integer as string (e.g. "100000000")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_deposit: Option<String>,
}

/// Data model for DepositQueryResult
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepositQueryResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DepositItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Data model for ErrorResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Data model for HistoryMarket
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HistoryMarket {
    /// Market thumbnail image URL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Unique numeric prediction market identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Raw JSON array of outcome display names, e.g. ["Yes", "No"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcomes: Option<[String; 2]>,
    /// Full title question describing the market condition
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    /// Timestamp when market officially settled (RFC 3339 UTC), or null if active
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    /// Resolution outcome (-2: unresolved, -1: void/invalid, 0-100: payout percentage for Outcome 0, e.g. 100 = Outcome 0 won, 0 = Outcome 1 won, 50 = 50/50 split)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<i64>,
    /// URL slug of the recurring sports league or tournament series
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series_slug: Option<String>,
    /// URL-friendly slug identifying the market on Polymarket
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

/// Data model for HourlyStat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HourlyStat {
    /// Aggregate capital cost basis in micro-USD (6 decimals)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_basis: Option<i64>,
    /// Total trading fees incurred in micro-USD (6 decimals)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee: Option<i64>,
    /// 1-hour UTC bucket timestamp (ISO 8601)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hour: Option<String>,
    /// Maker trading volume in micro-USD (6 decimals)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maker_volume: Option<i64>,
    /// Number of distinct prediction markets active or settled in this hour
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_count: Option<i64>,
    /// Realized net PnL in micro-USD (6 decimals)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pnl: Option<i64>,
    /// Taker trading volume in micro-USD (6 decimals)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taker_volume: Option<i64>,
    /// Number of profitable markets settled in this hour
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub win_count: Option<i64>,
}

/// Data model for LeaderboardResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaderboardResponse {
    /// Total count of matching traders for pagination
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// Ranked array of trader performance summaries
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traders: Option<Vec<TraderSummary>>,
}

/// Data model for LiveOrder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LiveOrder {
    /// Execution and relayer fee paid in micro-USDC
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    /// Underlying signed off-chain CLOB limit order metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<OrderInfo>,
    /// Market outcome label (e.g. "Yes", "No", "Up", "Down")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    /// Filled outcome shares amount represented as a 10^6 fixed-point decimal string
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shares: Option<String>,
    /// Order execution direction: "BUY" or "SELL"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    /// 0-based index matching market.token_ids and market.outcomes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_ids_index: Option<u8>,
    /// Filled USDC notional amount in micro-USDC (10^6 scale) decimal string
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usdc: Option<String>,
    /// Trader account identity (wallet address and optional display pseudonym)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<MonitorTrader>,
}

/// Data model for LiveTransaction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LiveTransaction {
    /// List of matched maker limit orders filled in this transaction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub makers: Option<Vec<LiveOrder>>,
    /// Associated Polymarket prediction market condition and metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market: Option<Market>,
    /// Taker order execution details that initiated the match against the CLOB
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taker: Option<LiveOrder>,
    /// ISO 8601 UTC timestamp (millisecond precision) indicating when the pending transaction was detected in the mempool
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Canonical Ethereum/Polygon on-chain transaction hash confirming the match execution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
}

/// Data model for Market
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Market {
    /// 32-byte hexadecimal condition ID from Gnosis Conditional Tokens
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_id: Option<String>,
    /// URL slug of the parent event containing this market
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_slug: Option<String>,
    /// Sub-category item title within a grouped multi-market event
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_item_title: Option<String>,
    /// Polymarket numeric prediction market identifier (e.g. 3688221)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// True if market operates under multi-outcome negative risk adapter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neg_risk: Option<bool>,
    /// Array of market outcome labels: [Outcome 1, Outcome 2] (e.g. ["Yes", "No"])
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcomes: Option<[String; 2]>,
    /// Full question describing the prediction market condition
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    /// URL slug of the recurring sports league or tournament series
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series_slug: Option<String>,
    /// URL-friendly slug identifying the market on Polymarket
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Sports market classification (e.g. moneyline, spread, over/under)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sports_market_type: Option<String>,
    /// ISO 8601 UTC timestamp when trading began
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Canonical categorical tags for content filtering
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags_slug: Option<Vec<String>>,
    /// ERC-1155 token IDs: [Token 1, Token 2]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_ids: Option<[String; 2]>,
}

/// Data model for MarketDetailResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketDetailResponse {
    /// Complete standardized market metadata matching exporter format
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market: Option<MarketMetadata>,
    /// Ranked list of top 20 profitable traders in this market
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_earners: Option<Vec<MarketEarner>>,
    /// Total protocol and liquidity fees collected in micro-USD (6 decimals)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_fees: Option<i64>,
    /// Total positive realized PnL distributed to winners in micro-USD (6 decimals)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_pnl_distributed: Option<i64>,
    /// Total count of distinct wallet addresses that traded this market
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_traders: Option<i64>,
    /// Cumulative trading volume across all participants in micro-USD (6 decimals)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<i64>,
    /// Total number of distinct traders with positive realized PnL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_winners: Option<i64>,
}

/// Data model for MarketEarner
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketEarner {
    /// Executed fill orders for this earner (included when include_orders=true)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<UserOrder>>,
    /// Realized net profit and loss in micro-USD (6 decimals, e.g. 1000000 = 1 USD)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pnl: Option<i64>,
    /// Public identity dossier of the earner
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<TraderIdentity>,
    /// Rank position among top earners in this market (1-based)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    /// Return on investment percentage (e.g. 25.5 for 25.5%)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roi: Option<f64>,
    /// Total traded volume in micro-USD (6 decimals)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<i64>,
}

/// Data model for MarketMetadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketMetadata {
    /// 0x-prefixed 66-character CTF condition identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_id: Option<String>,
    /// Timestamp when market was indexed in database (RFC 3339 UTC)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Extended details and market resolution criteria
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Estimated resolution date or scheduled closing time (RFC 3339 UTC)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// URL slug of the parent event containing this market
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_slug: Option<String>,
    /// Detailed fee schedule JSON specification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_schedule: Option<serde_json::Value>,
    /// Fee model classification (e.g. "dynamic", "fixed")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_type: Option<String>,
    /// Whether trading fees are activated on this market
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fees_enabled: Option<bool>,
    /// Threshold for numerical interval outcomes if applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_item_threshold: Option<i64>,
    /// Sub-category item title within a grouped multi-market event
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_item_title: Option<String>,
    /// Market thumbnail image URL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Unique numeric prediction market identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// True if market operates under multi-outcome negative risk adapter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neg_risk: Option<bool>,
    /// Raw JSON array of outcome display names, e.g. ["Yes", "No"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcomes: Option<[String; 2]>,
    /// Full title question describing the market condition
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    /// Source URL or oracle specification for resolution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution_source: Option<String>,
    /// Timestamp when the market officially settled (RFC 3339 UTC), or null if active
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    /// Resolution outcome (-2: unresolved, -1: void/invalid, 0-100: payout percentage for Outcome 0, e.g. 100 = Outcome 0 won, 0 = Outcome 1 won, 50 = 50/50 split)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<i64>,
    /// URL slug of the recurring sports league or tournament series
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series_slug: Option<String>,
    /// URL-friendly slug identifying the market on Polymarket
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Sports market classification type if applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sports_market_type: Option<String>,
    /// Timestamp when trading began (RFC 3339 UTC)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Raw JSON array of topic tag URL slugs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags_slug: Option<Vec<String>>,
    /// Array of 2 ERC-1155 token IDs corresponding to outcomes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_ids: Option<[String; 2]>,
}

/// Data model for MarketSettlementDetail
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketSettlementDetail {
    /// ISO 8601 UTC timestamp of the trader's last trade in this market
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_trade_at: Option<String>,
    /// Concise standardized market metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market: Option<HistoryMarket>,
    /// Detailed order fill events (present only when include_orders=true)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<UserOrder>>,
    /// Realized net PnL in micro-USD (6 decimals)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pnl: Option<i64>,
    /// Return on investment percentage in this market
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roi: Option<f64>,
    /// Total traded volume in this market in micro-USD (6 decimals)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<i64>,
}

/// Data model for MonitorTrader
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MonitorTrader {
    /// Canonical 42-character hexadecimal Ethereum/Polygon wallet address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Polymarket public username
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Data model for OrderInfo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderInfo {
    /// Cryptographic EIP-712 order hash identifying the unique off-chain order
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_hash: Option<String>,
    /// Original clob order shares capacity in 10^6 scale
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shares: Option<String>,
    /// Signature scheme encoding (0: Direct EOA, 1: Magic / Proxy Wallet, 2: Gnosis Safe, 3: Deposit Wallet (ERC-1271))
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<i64>,
    /// Unix epoch seconds when the order was signed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Original clob order notional value in micro-USDC (10^6 scale)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usdc: Option<String>,
}

/// Data model for QuoteSubscriptionRequest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuoteSubscriptionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub promo_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

/// Data model for QuoteSubscriptionResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuoteSubscriptionResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_to_pay: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_afford: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_balance: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_billing_cycle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_license: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_tier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_days: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_price: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prorated_credit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

/// Data model for SubscribeRequest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubscribeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub promo_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

/// Data model for SubscribeResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubscribeResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<UserSubscription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserProfile>,
}

/// Data model for TagStat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagStat {
    /// Passive maker volume in micro-USD (6 decimals)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maker_volume: Option<i64>,
    /// Number of markets traded in this tag
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub markets_count: Option<i64>,
    /// Realized net PnL in micro-USD (6 decimals)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pnl: Option<i64>,
    /// Return on investment percentage for this tag
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roi: Option<f64>,
    /// Category tag URL slug (e.g. "crypto", "politics", "sports")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Aggressive taker volume in micro-USD (6 decimals)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taker_volume: Option<i64>,
    /// Aggregated trading volume in micro-USD (6 decimals)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<i64>,
    /// Win rate percentage for this tag (0.0 to 100.0)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub win_rate: Option<f64>,
    /// Number of profitable markets in this tag
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wins_count: Option<i64>,
}

/// Data model for Tier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Tier {
    /// Whether the tier is permitted to access REST API endpoints
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_rest_api: Option<bool>,
    /// Whether the tier is permitted to access real-time stream endpoints
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_stream: Option<bool>,
    /// Whether this tier supports a trial subscription
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_trial: Option<bool>,
    /// Record creation timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Unique tier identifier (e.g. "free", "starter", "individual", "builder")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Relative tier rank for upgrade comparison (0=free, 1=starter, etc.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    /// License type: "personal" or "builder"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    /// Maximum API queries allowed per 24-hour UTC window (0 = unlimited)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_daily_requests: Option<i64>,
    /// Maximum aggregated streaming seconds allowed per 24-hour UTC window
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_daily_stream_seconds: Option<i64>,
    /// Maximum wallet addresses that can be filtered per stream
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_filter_addrs: Option<i64>,
    /// Maximum market series allowed across active streams (0 = disabled)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_filter_series: Option<i64>,
    /// Maximum market tags allowed across active streams (0 = disabled)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_filter_tags: Option<i64>,
    /// Maximum concurrent API keys allowed per account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i64>,
    /// Maximum unfiltered streams allowed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_naked_streams: Option<i64>,
    /// Maximum API queries allowed per second
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_qps: Option<i64>,
    /// Maximum concurrent active SSE connections per account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_sessions: Option<i64>,
    /// Maximum persistent stream IDs the user can create
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_streams: Option<i64>,
    /// Monthly credit quota allocated to this tier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monthly_credits: Option<i64>,
    /// Display title of the tier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 6 decimals, micro-USD (e.g. 99_000_000 = 99 USD)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_monthly: Option<i64>,
    /// 6 decimals, micro-USD (e.g. 237_000_000 = 237 USD)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_quarterly: Option<i64>,
    /// 6 decimals, micro-USD for trial (e.g. 10_000_000 = 10 USD)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_trial: Option<i64>,
    /// 6 decimals, micro-USD (e.g. 499_000_000 = 499 USD)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_yearly: Option<i64>,
    /// Duration in days for trial subscription (e.g. 5)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trial_duration_days: Option<i64>,
    /// Last modification timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Data model for TraderHistoryResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TraderHistoryResponse {
    /// Paginated list of market settlement details
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<MarketSettlementDetail>>,
    /// Total count of matching market positions for pagination
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

/// Data model for TraderHourlyStatsResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TraderHourlyStatsResponse {
    /// Chronological array of 1-hour performance metrics
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<Vec<HourlyStat>>,
}

/// Data model for TraderIdentity
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TraderIdentity {
    /// Checksummed 0x-prefixed EVM wallet address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Custom display name or Polymarket profile username
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ISO 8601 UTC timestamp of Polymarket profile creation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_created_at: Option<String>,
    /// Public avatar image URL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<String>,
    /// Linked X (formerly Twitter) social handle
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_username: Option<String>,
}

/// Data model for TraderProfileResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TraderProfileResponse {
    /// ISO 8601 UTC timestamp of trader's most recent trade across the platform
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_trade_at: Option<String>,
    /// Public identity dossier of the trader
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<TraderIdentity>,
    /// Trader's pUSD balance in micro-pUSD (6 decimals, e.g. 1000000 = 1 pUSD)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pusd_balance: Option<i64>,
    /// Polymarket Taker Rebate Program tier level (0-6: 0=Tier 0, 1=Bronze, 2=Silver, 3=Gold, 4=Platinum, 5=Diamond, 6=Obsidian)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taker_tier: Option<i64>,
    /// Descriptive taker rebate tier name ("Tier 0", "Bronze", "Silver", "Gold", "Platinum", "Diamond", "Obsidian", or empty string if unsynced)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taker_tier_name: Option<String>,
    /// Performance breakdown across top 5 categories in last 30 days
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_tags: Option<Vec<TagStat>>,
    /// Rolling 30-day taker Weighted Volume (wV) in USD, calculated by Trade Size * (1 - Entry Price) * Category Weight * Bonuses
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weighted_volume: Option<f64>,
}

/// Data model for TraderSummary
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TraderSummary {
    /// ISO 8601 UTC timestamp of trader's most recent trade across the platform
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_trade_at: Option<String>,
    /// Number of distinct prediction markets traded in timeframe
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub markets_count: Option<i64>,
    /// Public identity dossier of the trader
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<TraderIdentity>,
    /// Leaderboard ranking position (1-based)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    /// Return on investment percentage over timeframe (e.g. 25.5 for 25.5%)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roi: Option<f64>,
    /// Realized net profit and loss in micro-USD (6 decimals, e.g. 1000000 = 1 USD)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_pnl: Option<i64>,
    /// Aggregated trading volume in micro-USD (6 decimals)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<i64>,
    /// Percentage of profitable closed positions (0.0 to 100.0)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub win_rate: Option<f64>,
    /// Number of profitable settled markets in timeframe
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wins_count: Option<i64>,
}

/// Data model for UpdateSubscriptionRequest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSubscriptionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// Data model for UserAPIKey
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserAPIKey {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

/// Data model for UserActiveSessionsResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserActiveSessionsResponse {
    /// Array of active connected client session metadata
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ActiveSessionInfo>>,
    /// Total concurrent active sessions currently running across all user streams
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

/// Data model for UserCreateStreamRequest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserCreateStreamRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// Data model for UserKeysResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserKeysResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UserAPIKey>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

/// Data model for UserLedgerEntry
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserLedgerEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance_after: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// Data model for UserLedgerResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserLedgerResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UserLedgerEntry>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

/// Data model for UserOrder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserOrder {
    /// Protocol trading fee deducted in micro-USD (6 decimals)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee: Option<i64>,
    /// True if executed against resting book liquidity as a taker
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_taker: Option<bool>,
    /// Prediction outcome label (e.g. "Yes", "No")
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    /// Filled outcome token shares (6 decimals)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shares: Option<i64>,
    /// Order execution side: "BUY" or "SELL"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    /// ISO 8601 UTC timestamp of order fill execution
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// Total collateral USDC transferred (6 decimals micro-USD)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usdc: Option<i64>,
}

/// Data model for UserOrdersResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserOrdersResponse {
    /// Chronological list of compacted order fill events
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<UserOrder>>,
}

/// Data model for UserProfile
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deposit_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deposit_balance: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_expired: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telegram_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier_expires_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Data model for UserSessionHistoryResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserSessionHistoryResponse {
    /// Array of historical stream session records
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UserSessionRecord>>,
    /// Total count of historical session records
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

/// Data model for UserSessionRecord
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserSessionRecord {
    /// Masked API credential used to authenticate the stream connection
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// Remote IP address of the connected client
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    /// Diagnostic termination reason (e.g. client_closed, admin_force_disconnect, quota_exceeded)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub close_reason: Option<String>,
    /// ISO 8601 timestamp when the SSE connection was established
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connected_at: Option<String>,
    /// ISO 8601 timestamp when the connection was closed, or null if currently active
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disconnected_at: Option<String>,
    /// Total connection duration in seconds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    /// Total number of live matched order events delivered over this connection
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pushed_tx_count: Option<i64>,
    /// Target stream unique identifier (UUID)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    /// HTTP User-Agent identifier of the client library or application
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

/// Data model for UserStreamResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStreamResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_sessions: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Data model for UserStreamsResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStreamsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UserStreamResponse>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

/// Data model for UserSubscription
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserSubscription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_paid: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_days: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_price: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_tier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub promo_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prorated_credit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Data model for UserTelemetryResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserTelemetryResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_requests: Option<i64>,
}

/// Data model for UserUpdateKeyRequest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserUpdateKeyRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 1: Active, 0: Disabled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

/// Data model for UserUpdateStreamMetadataRequest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserUpdateStreamMetadataRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Data model for UserWithdrawRequest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserWithdrawRequest {
    pub network: String,
    pub to_address: String,
    pub token: String,
}

/// Data model for UserWithdrawResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserWithdrawResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<UserProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub withdrawal: Option<Withdrawal>,
}

/// Data model for ValidatePromoCodeRequest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidatePromoCodeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

/// Data model for ValidatePromoCodeResponse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidatePromoCodeResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_to_pay: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_price: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prorated_credit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}

/// Data model for Withdrawal
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Withdrawal {
    /// Full balance amount withdrawn in micro-USD
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Withdrawal request creation timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// User's most recent deposit timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_deposit_at: Option<String>,
    /// "bsc" or "polygon"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Reason recorded upon rejection
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    /// Administrative review timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<String>,
    /// "pending", "completed", "rejected"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Destination EVM wallet address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_address: Option<String>,
    /// "USDC" or "USDT"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Transaction hash on destination network
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    /// Last modification timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}
