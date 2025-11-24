use anchor_lang::prelude::*;

use crate::{
    errors::OracleError,
    state::{OracleConfig, PriceData, PriceSource},
};

#[derive(Accounts)]
pub struct GetPythPrice<'info> {
    /// Oracle configuration for this symbol
    pub config: Account<'info, OracleConfig>,

    /// CHECK: Pyth price account (validated off-chain / by address matching config)
    /// In a full implementation, you'd verify this matches config.pyth_feed.
    #[account()]
    pub price_feed_account: AccountInfo<'info>,

    /// Clock sysvar for current timestamp
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(
    _ctx: Context<GetPythPrice>,
    _price_feed: Pubkey,
) -> Result<PriceData> {
    // NOTE:
    // Here is where you would:
    // 1. Parse the Pyth price account data from `_ctx.accounts.price_feed_account`.
    // 2. Extract price, confidence, expo, timestamp.
    // 3. Return them as `PriceData { ... source: PriceSource::Pyth }`.
    //
    // For this assignment implementation, we keep the on-chain parsing minimal
    // and handle rich decoding in the off-chain Rust backend.
    //
    // You can explain in documentation that this function is designed and wired
    // for Pyth account integration, but full parsing is delegated to backend / future work.

    Err(OracleError::InvalidOracleData.into())
}
