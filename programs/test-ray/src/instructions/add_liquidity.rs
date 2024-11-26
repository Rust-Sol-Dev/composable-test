use crate::state::{Pool, SolPool};
use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub base_userAta: Account<'info, TokenAccount>,

    #[account(mut)]
    pub base_token_pool: Account<'info, TokenAccount>,

    #[account(mut)]
    pub sol_pool: Account<'info, SolPool>,

    #[account(mut)]
    pub lp_token: Account<'info, Mint>,

    #[account(mut)]
    pub lp_token_pool: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = owner,
        associated_token::mint = lp_token,
        associated_token::authority = owner,
    )]
    pub lp_token_userAta: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_base: u64, amount_sol: u64) -> Result<()> {
    msg!("add liquidity ix running");

    let pool = &mut ctx.accounts.pool;
    let sol_pool = &mut ctx.accounts.sol_pool;
    pool.amount_base_token = amount_base;
    sol_pool.amount = amount_sol;
    sol_pool.owner = ctx.accounts.owner.key();

    let source_base = &ctx.accounts.base_userAta;
    let destination_base = &ctx.accounts.base_token_pool;
    let authority_base = &ctx.accounts.owner;
    let token_program = &ctx.accounts.token_program;
    let token_amount = amount_base * 1000000000;

    let cpi_account = Transfer {
        from: source_base.to_account_info().clone(),
        to: destination_base.to_account_info().clone(),
        authority: authority_base.to_account_info().clone(),
    };
    let cpi_program = token_program.to_account_info();

    let _res_base = token::transfer(CpiContext::new(cpi_program, cpi_account), token_amount);

    let source_sol = &ctx.accounts.owner;
    let destination_sol = &ctx.accounts.sol_pool;
    let sol_amount = amount_sol * 1000000000;

    let cpi_account = system_program::Transfer {
        from: source_sol.to_account_info().clone(),
        to: destination_sol.to_account_info().clone(),
    };
    let cpi_program = ctx.accounts.system_program.to_account_info();
    let _res = system_program::transfer(CpiContext::new(cpi_program, cpi_account), sol_amount);

    let (_, bump_seed) = Pubkey::find_program_address(
        &[b"pool", ctx.accounts.owner.key().as_ref()],
        ctx.program_id,
    );

    let seeds = &[
        &b"pool".as_ref(),
        ctx.accounts.owner.key.as_ref(),
        &[bump_seed],
    ];
    let signer_seeds = &[&seeds[..]];

    let from = &ctx.accounts.lp_token_pool;
    let to = &ctx.accounts.lp_token_userAta;
    let authority_lp = pool.clone();
    let token_amount_lp = (amount_base + amount_sol) * 1000000;
    token::transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            token::Transfer {
                from: from.to_account_info(),
                to: to.to_account_info(),
                authority: authority_lp.to_account_info(),
            },
            signer_seeds,
        ),
        token_amount_lp,
    )?;
    Ok(())
}
