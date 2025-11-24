use anchor_lang::prelude::*;

use crate::{
    errors::OracleError,
    state::{OracleConfig, PriceData, PriceSource},
};

#[derive(Accounts)]
pub struct GetSwitchboardPrice<'info> {
    /// Oracle configuration for this symbol
    pub config: Account<'info, OracleConfig>,

    /// CHECK: Switchboard aggregator account
    #[account()]
    pub aggregator_account: AccountInfo<'info>,

    /// Clock sysvar for current timestamp
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(
    _ctx: Context<GetSwitchboardPrice>,
    _aggregator: Pubkey,
) -> Result<PriceData> {
    // NOTE:
    // Here is where you would:
    // 1. Parse Switchboard aggregator data.
    // 2. Extract consensus result, confidence, timestamp, exponent/scale.
    // 3. Return as `PriceData { ... source: PriceSource::Switchboard }`.
    //
    // As with Pyth, for this assignment we keep on-chain parsing minimal,
    // focusing on the consensus validation logic and off-chain integration.

    Err(OracleError::InvalidOracleData.into())
}
