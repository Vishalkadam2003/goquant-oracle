use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod instructions;
pub mod utils;

use instructions::*;

declare_id!("YourProgramPubkeyHere111111111111111111111111");

#[program]
pub mod oracle_program {
    use super::*;

    pub fn get_pyth_price(
        ctx: Context<GetPythPrice>,
        price_feed: Pubkey,
    ) -> Result<state::PriceData> {
        instructions::get_pyth_price::handler(ctx, price_feed)
    }

    pub fn get_switchboard_price(
        ctx: Context<GetSwitchboardPrice>,
        aggregator: Pubkey,
    ) -> Result<state::PriceData> {
        instructions::get_switchboard_price::handler(ctx, aggregator)
    }

    pub fn validate_price_consensus(
        ctx: Context<ValidatePrice>,
        prices: Vec<state::PriceData>,
    ) -> Result<u64> {
        instructions::validate_price_consensus::handler(ctx, prices)
    }
}
