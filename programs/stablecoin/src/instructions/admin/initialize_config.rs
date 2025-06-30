use anchor_lang::prelude::*;
use anchor_spl::{ token_interface::{Mint, Token2022}};
use crate::{ constants::{MINT_DECIMALS, SEED_CONFIG_ACCOUNT, SEED_MINT_ACCOUNT}, state::Config };

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
    pub token_program : Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}
