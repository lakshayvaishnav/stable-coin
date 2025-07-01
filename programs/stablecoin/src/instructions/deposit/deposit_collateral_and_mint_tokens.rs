use anchor_lang::prelude::*;

use crate::{constants::SEED_CONFIG_ACCOUNT, state::Config};


#[derive(Accounts)]
pub struct DepositCollateralandMintTokens<'info> {
    #[account(mut)]
    pub depositor : Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump,
        has_one= mint_account,
    )]
    pub config_account : Account<'info, Config>
}