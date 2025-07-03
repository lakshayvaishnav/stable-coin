use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ Mint, Token2022, TokenAccount },
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    check_health_factor,
    constants::{ SEED_COLLATERAL_ACCOUNT, SEED_CONFIG_ACCOUNT, SEED_SOL_ACCOUNT },
    deposit_sol,
    mint_tokens,
    state::{ Collateral, Config },
};

#[derive(Accounts)]
pub struct DepositCollateralandMintTokens<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(seeds = [SEED_CONFIG_ACCOUNT], bump, has_one = mint_account)]
    pub config_account: Account<'info, Config>,

    // mint of the stable coin
    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    // collateral info of depositor...
    #[account(
        init_if_needed,
        payer = depositor,
        space = 8 + Collateral::INIT_SPACE,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump
    )]
    pub collateral_account: Account<'info, Collateral>,

    // sol account of the user
    #[account(
        mut,
        seeds = [SEED_SOL_ACCOUNT, depositor.key().as_ref()],
        bump,
    )]
    // ⚠️⚠️⚠️ edge case : this should'nt be a system account in future make it a vault or somehting...
    // and (gpt says need to do depsoti with the cpi you can't increase lamports directly...)
    pub sol_account: SystemAccount<'info>,

    // associated token account for the stable coin
    #[account(
        init_if_needed,
        payer = depositor,
        associated_token::mint = mint_account,
        associated_token::authority = depositor,
        associated_token::token_program = token_program
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub price_update: Account<'info, PriceUpdateV2>,
}

pub fn process_deposit_collateral_and_mint_tokens(
    ctx: Context<DepositCollateralandMintTokens>,
    amount_collateral: u64,
    amount_to_mint: u64
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_account;

    collateral_account.lmaport_balance = ctx.accounts.sol_account.lamports() + amount_collateral;
    collateral_account.amount_minted += amount_to_mint;

    if !collateral_account.is_initialized {
        collateral_account.is_initialized = true;
        collateral_account.depositor = collateral_account.depositor.key();
        collateral_account.sol_account = ctx.accounts.sol_account.key();
        collateral_account.token_account = ctx.accounts.token_account.key();
        collateral_account.bump = ctx.bumps.collateral_account;
        collateral_account.bump_sol_account = ctx.bumps.sol_account;
    }

    check_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.price_update,
        &ctx.accounts.config_account
    )?;

    deposit_sol(
        &ctx.accounts.system_program,
        &ctx.accounts.depositor,
        &ctx.accounts.sol_account,
        amount_collateral
    )?;

    mint_tokens(
        &ctx.accounts.token_account,
        &ctx.accounts.mint_account,
        &ctx.accounts.token_program,
        ctx.accounts.config_account.bump_mint_account,
        amount_to_mint
    )?;

    Ok(())
}
