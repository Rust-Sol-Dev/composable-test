use crate::consts::*;
use crate::error::*;
use crate::state::{Pool, SolPool};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = owner,
        seeds = [POOL_SEED.as_ref(), owner.key().as_ref()],
        space = 8 + Pool::LEN,
        bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub base_token: Account<'info, Mint>,

    #[account(
        init,
        payer = owner,
        associated_token::mint = base_token,
        associated_token::authority = pool
    )]
    pub base_token_pool: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = owner,
        seeds = [SOL_POOL_SEED.as_ref(), owner.key().as_ref(), pool.key().as_ref()],
        space = 8 + SolPool::LEN,
        bump
    )]
    pub sol_pool: Account<'info, SolPool>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    msg!("Initialize Instruction Executing");
    // Set fields individually for Pool
    let pool = &mut ctx.accounts.pool;
    pool.pool_owner = ctx.accounts.owner.key();
    pool.base_token = ctx.accounts.base_token.key();
    pool.amount_base_token = 0;

    // Set fields individually for SolPool
    let sol_pool = &mut ctx.accounts.sol_pool;
    sol_pool.amount = 0;
    sol_pool.owner = ctx.accounts.owner.key();

    msg!("Pool and SOL Pool Initialized Successfully");
    Ok(())
}
