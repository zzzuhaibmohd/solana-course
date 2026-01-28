use anchor_lang::prelude::*;

#[error_code]
pub enum Error {
    #[msg("Invalid pool fee")]
    InvalidFee,
    #[msg("Decimals mismatch")]
    DecimalsMismatch,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Amount out < min")]
    MinAmountOut,
}
