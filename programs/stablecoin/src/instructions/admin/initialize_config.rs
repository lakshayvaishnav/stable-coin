use anchor_lang::prelude::*;
use anchor_spl::{ token_interface::{ Mint, Token2022 } };
use crate::{
    constants::{
        LIQUIDATION_BONUS,
        LIQUIDATION_THRESHOLD,
        MINT_DECIMALS,
        MIN_HEALTH_FACTOR,
        SEED_CONFIG_ACCOUNT,
        SEED_MINT_ACCOUNT,
    },
    state::Config,
};

// intialize config accounts
#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Config::INIT_SPACE,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        init,
        payer = authority,
        seeds = [SEED_MINT_ACCOUNT],
        bump,
        mint::decimals = MINT_DECIMALS,
        mint::authority = mint_account,
        mint::freeze_authority = mint_account,
        mint::token_program = token_program
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn process_initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
    let config_account = &mut ctx.accounts.config_account;

    config_account.authority = ctx.accounts.authority.key();
    config_account.mint_account = ctx.accounts.mint_account.key();
    config_account.liquidation_bonus = LIQUIDATION_BONUS;
    config_account.liquidation_threshold = LIQUIDATION_THRESHOLD;
    config_account.min_health_factor = MIN_HEALTH_FACTOR;
    config_account.bump = ctx.bumps.config_account;
    config_account.bump_mint_account = ctx.bumps.mint_account;

    msg!("Initialized config account : {:#?} ", config_account);

    Ok(())
}
