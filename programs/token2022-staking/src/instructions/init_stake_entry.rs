use {
    crate::{errors::*, state::*, utils::*},
    anchor_lang::prelude::*,
    anchor_spl::{associated_token::AssociatedToken, token_interface},
    std::mem::size_of,
};

pub fn handler(ctx: Context<InitializeStakeEntry>) -> Result<()> {
    check_token_program(ctx.accounts.token_program.key());

    // initialize user stake entry state
    let user_entry = &mut ctx.accounts.user_stake_entry;
    user_entry.user = ctx.accounts.user.key();
    user_entry.user_stake_token_account = ctx.accounts.user_stake_token_account.key();
    user_entry.bump = ctx.bumps.user_stake_entry;
    user_entry.balance = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeStakeEntry<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
    init,
    seeds = [user.key().as_ref(), pool_state.token_mint.key().as_ref(), STAKE_ENTRY_SEED.as_bytes()],
    bump,
    payer = user,
    space = 8 + size_of::<StakeEntry>()
)]
    pub user_stake_entry: Account<'info, StakeEntry>,
    #[account(
    init,
    associated_token::mint = staking_token_mint,
    associated_token::authority = user,
    associated_token::token_program = token_program,
    payer = user,
)]
    pub user_stake_token_account: InterfaceAccount<'info, token_interface::TokenAccount>,
    #[account(
    constraint = staking_token_mint.key() == pool_state.staking_token_mint
    @ StakeError::InvalidStakingTokenMint,
    mint::token_program = token_program
)]
    pub staking_token_mint: InterfaceAccount<'info, token_interface::Mint>,
    //这样写就不需要前端传入pool_state,直接可以通过PDA找到地址
    #[account(
        seeds = [pool_state.token_mint.key().as_ref(), STAKE_POOL_STATE_SEED.as_bytes()],
        bump = pool_state.bump
    )]
    pub pool_state: Account<'info, PoolState>,
    pub token_program: Interface<'info, token_interface::TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
