use anchor_lang::prelude::*;

mod state;
mod instructions;
mod constants;

declare_id!("He6UwBtt2bHyi4EsvYMSq6BrepQFWxMdSircNLkNij7w");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
