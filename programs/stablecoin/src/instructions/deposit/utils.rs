use anchor_lang::{ prelude::*, system_program::{ transfer, Transfer } };
use anchor_spl::{ token_2022::mint_to, token_interface::{ Mint, MintTo, Token2022, TokenAccount } };

use crate::constants::SEED_MINT_ACCOUNT;

pub fn mint_tokens<'info>(
    token_account: &InterfaceAccount<'info, TokenAccount>,
    mint_account: &InterfaceAccount<'info, Mint>,
    token_program: &Program<'info, Token2022>,
    bump: u8,
    amount: u64
) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[SEED_MINT_ACCOUNT, &[bump]]];

    mint_to(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            MintTo {
                mint: mint_account.to_account_info(),
                to: token_account.to_account_info(),
                authority: mint_account.to_account_info(),
            },
            signer_seeds
        ),
        amount
    );

    Ok(())
}

pub fn deposit_sol<'info>(
    system_program: &Program<'info, System>,
    from: &Signer<'info>,
    to: &SystemAccount<'info>,
    amount: u64
) -> Result<()> {
    transfer(
        CpiContext::new(system_program.to_account_info(), Transfer {
            from: from.to_account_info(),
            to: to.to_account_info(),
        }),
        amount
    )
}
