use crate::models::{PriceData, ConsensusPrice};

const MAX_STALENESS_SECS: i64 = 30;
const MAX_DEVIATION_BPS: f64 = 100.0; // 1% deviation allowed

pub struct PriceAggregator;

impl PriceAggregator {
    pub fn aggregate(symbol: &str, prices: Vec<PriceData>) -> Option<ConsensusPrice> {
        if prices.is_empty() {
            return None;
        }

        let now = chrono::Utc::now().timestamp();

        // 1) Filter out stale prices
        let mut fresh: Vec<PriceData> = prices
            .into_iter()
            .filter(|p| now - p.timestamp <= MAX_STALENESS_SECS)
            .collect();

        if fresh.is_empty() {
            return None;
        }

        // 2) Sort by price and get median
        fresh.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        let len = fresh.len();
        let median_price = if len % 2 == 1 {
            fresh[len / 2].price
        } else {
            let a = fresh[len / 2 - 1].price;
            let b = fresh[len / 2].price;
            (a + b) / 2.0
        };

        // 3) Filter outliers by deviation in bps
        let filtered: Vec<PriceData> = fresh
            .into_iter()
            .filter(|p| deviation_bps(p.price, median_price) <= MAX_DEVIATION_BPS)
            .collect();

        if filtered.is_empty() {
            return None;
        }

        // 4) Recompute median on filtered
        let mut sorted = filtered.clone();
        sorted.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        let len2 = sorted.len();
        let consensus_price = if len2 % 2 == 1 {
            sorted[len2 / 2].price
        } else {
            let a = sorted[len2 / 2 - 1].price;
            let b = sorted[len2 / 2].price;
            (a + b) / 2.0
        };

        // Average timestamp of used prices
        let avg_ts = (sorted.iter().map(|p| p.timestamp).sum::<i64>() as f64 / len2 as f64)
            .round() as i64;

        Some(ConsensusPrice {
            symbol: symbol.to_string(),
            price: consensus_price,
            timestamp: avg_ts,
            sources_used: len2,
        })
    }
}

fn deviation_bps(a: f64, b: f64) -> f64 {
    if a == b {
        return 0.0;
    }
    let diff = (a - b).abs();
    let mid = (a + b) / 2.0;
    if mid == 0.0 {
        return f64::MAX;
    }
    (diff / mid) * 10_000.0
}
