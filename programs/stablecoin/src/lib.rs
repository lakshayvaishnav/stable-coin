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
}
