use anchor_lang::prelude::*;

#[error_code]
pub enum SparkError {
    #[msg("Pledge Amount Can't Be Zero")]
    PledgeAmountZero,
}
