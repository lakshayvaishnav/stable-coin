#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;


pub mod state;
pub mod instructions;
pub mod constants;
pub mod error;

pub use error::*;
pub use instructions::*;
declare_id!("He6UwBtt2bHyi4EsvYMSq6BrepQFWxMdSircNLkNij7w");


#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }

    pub fn upate_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        process_update_config(ctx, min_health_factor)
    }

    pub fn deposit_collateral_and_mint_toknens(
        ctx: Context<DepositCollateralandMintTokens>,
        amount_collateral: u64,
        amount_to_mint: u64
    ) -> Result<()> {
        process_deposit_collateral_and_mint_tokens(ctx, amount_collateral, amount_to_mint)
    }

    pub fn redeem_collateral_and_burn_tokens(
        ctx: Context<RedeemCollateralAndBurnTokens>,
        amount_collateral: u64,
        amount_to_burn: u64
    ) -> Result<()> {
        process_redeem_collateral_and_burn_tokens(ctx, amount_collateral, amount_to_burn)
    }
}
