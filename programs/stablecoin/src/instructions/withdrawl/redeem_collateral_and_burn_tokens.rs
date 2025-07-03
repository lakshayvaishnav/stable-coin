use anchor_lang::prelude::*;
use anchor_spl::token_interface::{ Mint, TokenAccount, Token2022 };
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use crate::{
    burn_tokens,
    check_health_factor,
    constants::{ SEED_COLLATERAL_ACCOUNT, SEED_CONFIG_ACCOUNT },
    redeem_collateral,
    state::{ Collateral, Config },
};

#[derive(Accounts)]
pub struct RedeemCollateralAndBurnTokens<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    pub price_update: Account<'info, PriceUpdateV2>,

    #[account(seeds = [SEED_CONFIG_ACCOUNT], bump = config_account.bump, has_one = mint_account)]
    pub config_account: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump,
        has_one = sol_account,
        has_one = token_account,
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(mut)]
    pub sol_account: SystemAccount<'info>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

/*
    ⚠️⚠️⚠️ attack vector : - 
    ✅ Correct Approach:
    The user should only specify how much collateral they want to redeem, and your program should:

    Fetch the current price of the collateral from your oracle (price_update),

    Determine how many tokens must be burned to withdraw that much collateral,

    Verify post-transaction health factor,

    Burn exactly that amount from their token account.

*/

pub fn process_redeem_collateral_and_burn_tokens(
    ctx: Context<RedeemCollateralAndBurnTokens>,
    amount_collateral: u64,
    amount_to_burn: u64
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.lmaport_balance = ctx.accounts.sol_account.lamports() - amount_collateral;
    collateral_account.amount_minted -= amount_to_burn;

    check_health_factor(
        &ctx.accounts.collateral_account,
        &ctx.accounts.price_update,
        &ctx.accounts.config_account
    )?;

    burn_tokens(
        &ctx.accounts.token_program,
        &ctx.accounts.token_account,
        &ctx.accounts.mint_account,
        &ctx.accounts.depositor,
        amount_to_burn
    )?;

    redeem_collateral(
        &ctx.accounts.depositor.key(),
        &ctx.accounts.sol_account,
        &ctx.accounts.depositor.to_account_info(),
        &ctx.accounts.system_program,
        amount_collateral,
        ctx.accounts.collateral_account.bump_sol_account
    )?;

    Ok(())
}
