use anchor_lang::prelude::*;

use crate::{
    errors::OracleError,
    state::{OracleConfig, PriceData},
    utils::{deviation_bps, median},
};

#[derive(Accounts)]
pub struct ValidatePrice<'info> {
    /// Oracle configuration for this symbol
    pub config: Account<'info, OracleConfig>,

    /// Clock sysvar to get current timestamp
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<ValidatePrice>, prices: Vec<PriceData>) -> Result<u64> {
    let cfg = &ctx.accounts.config;
    let now_ts = ctx.accounts.clock.unix_timestamp;

    // 1) Filter valid prices: not stale, confidence ok, consistent exponent.
    let mut valid: Vec<PriceData> = Vec::new();

    for p in prices.iter() {
        // Staleness check
        if now_ts - p.timestamp > cfg.max_staleness {
            continue;
        }

        // Confidence check: here we assume confidence is provided in same units as price,
        // but config.max_confidence is in basis points of price.
        // Equivalent threshold: conf_bps = confidence / |price| * 10_000
        if p.price != 0 {
            let price_abs = (p.price as f64).abs();
            let conf_abs = p.confidence as f64;
            let conf_bps = ((conf_abs / price_abs) * 10_000.0).round() as u64;
            if conf_bps > cfg.max_confidence {
                continue;
            }
        }

        valid.push(p.clone());
    }

    require!(!valid.is_empty(), OracleError::NoValidSources);

    // 2) Ensure all exponents are the same to avoid mixing scales.
    let first_expo = valid[0].expo;
    for p in valid.iter() {
        require!(p.expo == first_expo, OracleError::InconsistentExponents);
    }

    // 3) Compute median price among valid sources.
    let prices_i64: Vec<i64> = valid.iter().map(|p| p.price).collect();
    let median_price = median(prices_i64);

    // 4) Remove outliers whose deviation from median exceeds max_deviation bps.
    let mut filtered: Vec<i64> = Vec::new();
    for p in valid.iter() {
        let dev = deviation_bps(p.price, median_price);
        if dev <= cfg.max_deviation {
            filtered.push(p.price);
        }
    }

    require!(!filtered.is_empty(), OracleError::NoValidSources);

    // 5) Recompute median from filtered prices (final consensus).
    let consensus_price = median(filtered);

    // 6) For simplicity, we return the raw integer price as u64.
    // In a real system, you might normalize by exponent or return (price, expo) separately.
    require!(consensus_price > 0, OracleError::InvalidOracleData);

    Ok(consensus_price as u64)
}
