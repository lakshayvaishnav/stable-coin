use anchor_lang::{ prelude::*, system_program::{ transfer, Transfer } };
use anchor_spl::{ token_2022::{ self, Burn }, token_interface::{ Mint, Token2022, TokenAccount } };

use crate::constants::SEED_SOL_ACCOUNT;

// function to burn tokens
pub fn burn_tokens<'info>(
    token_program: &Program<'info, Token2022>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    amount_to_burn: u64
) -> Result<()> {
    // cpi context for the burn ixn
    let cpi_accounts = Burn {
        authority: authority.to_account_info(),
        from: token_account.to_account_info(),
        mint: mint.to_account_info(),
    };

    let cpi_context = CpiContext::new(token_program.to_account_info(), cpi_accounts);
    token_2022::burn(cpi_context, amount_to_burn);
    Ok(())
}

// function to withdraw tokens
pub fn redeem_collateral<'info>(
    depositor_key: &Pubkey,
    sol_account: &SystemAccount<'info>,
    to: &AccountInfo<'info>,
    system_program: &Program<'info, System>,
    amount: u64,
    bump: u8
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[SEED_SOL_ACCOUNT, depositor_key.as_ref(), &[bump]]];
    let cpi_accounts = Transfer {
        from: sol_account.to_account_info(),
        to: to.to_account_info(),
    };

    transfer(
        CpiContext::new_with_signer(system_program.to_account_info(), cpi_accounts, signer_seeds),
        amount
    )?;
    Ok(())
}
