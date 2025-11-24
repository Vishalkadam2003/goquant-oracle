use core::cmp::Ordering;

/// Check if price timestamp is stale w.r.t current time.
pub fn is_stale(price_ts: i64, max_staleness: i64, current_ts: i64) -> bool {
    current_ts - price_ts > max_staleness
}

/// Deviation between two prices in basis points (bps).
/// bps = |a - b| / mid * 10_000
pub fn deviation_bps(a: i64, b: i64) -> u64 {
    if a == b {
        return 0;
    }

    let a_f = a as f64;
    let b_f = b as f64;
    let diff = (a_f - b_f).abs();
    let mid = (a_f + b_f) / 2.0;

    if mid == 0.0 {
        return u64::MAX; // undefined, treat as huge deviation
    }

    ((diff / mid) * 10_000.0).round() as u64
}

/// Median of a non-empty vector of i64.
/// For even length, we take the lower middle to keep deterministic integer behavior.
pub fn median(mut values: Vec<i64>) -> i64 {
    values.sort_unstable();
    let len = values.len();
    values[(len - 1) / 2]
}
