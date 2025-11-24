use anchor_lang::prelude::*;

#[error_code]
pub enum OracleError {
    #[msg("Price is stale")]
    StalePrice,

    #[msg("Confidence interval too high")]
    HighConfidence,

    #[msg("Price deviation too large")]
    PriceDeviation,

    #[msg("No valid price sources")]
    NoValidSources,

    #[msg("Invalid or missing oracle data")]
    InvalidOracleData,

    #[msg("Inconsistent price exponents between sources")]
    InconsistentExponents,
}
