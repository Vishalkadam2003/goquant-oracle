use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum PriceSource {
    Pyth,
    Switchboard,
    Internal,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PriceData {
    pub price: i64,        // raw oracle price (fixed point)
    pub confidence: u64,   // confidence interval (same scale as price)
    pub expo: i32,         // exponent (e.g., -8 for 1e-8)
    pub timestamp: i64,    // unix timestamp (seconds)
    pub source: PriceSource,
}

#[account]
pub struct OracleConfig {
    pub symbol: String,
    pub pyth_feed: Pubkey,
    pub switchboard_aggregator: Pubkey,
    pub max_staleness: i64,   // seconds
    pub max_confidence: u64,  // basis points
    pub max_deviation: u64,   // basis points
}
