use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceData {
    pub symbol: String,
    pub price: f64,
    pub confidence: f64,
    pub timestamp: i64,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusPrice {
    pub symbol: String,
    pub price: f64,
    pub timestamp: i64,
    pub sources_used: usize,
}
