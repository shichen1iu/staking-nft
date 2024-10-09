pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("4ZUMbat3p3YL1QjoAmc6jAyenvJWUz1cL3uuZ6mtG8jj");

#[program]
pub mod token_22_staking {
    use super::*;

    
    pub fn init_pool(ctx: Context<InitializePool>) -> Result<()> {

        msg!("Creating staking pool...");
    
        init_pool::handler(ctx)
    }

    pub fn init_stake_entry(ctx: Context<InitializeStakeEntry>) -> Result<()> {

        msg!("Creating staking entry...");

        init_stake_entry::handler(ctx)
    }
    
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {

        msg!("Staking...");

        stake::handler(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {

        msg!("Unstaking...");

        unstake::handler(ctx)
    }
}