use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrorCode {
    #[msg("Invalid Price")]
    InvalidPrice,
    #[msg("health factor is less, need to liquidate the collateral")]
    BelowMinHealthFacotor,

}