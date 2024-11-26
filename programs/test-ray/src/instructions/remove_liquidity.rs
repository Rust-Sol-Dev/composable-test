use crate::state::{Pool, SolPool};
use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
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

    #[account(mut)]
    pub lp_token_userAta: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

pub fn remove_liquidity(ctx: Context<RemoveLiquidity>, amount_lp: u64) -> Result<()> {
    msg!("add liquidity ix running");
    let pool = &mut ctx.accounts.pool;
    let solpool = &mut ctx.accounts.sol_pool;
    ////////////////////////    transfer lp token   /////////////////////

    let source_lp = &ctx.accounts.lp_token_userAta;
    let destination_lp = &ctx.accounts.lp_token_pool;
    let authority_lp = &ctx.accounts.owner;
    let token_program = &ctx.accounts.token_program;
    let token_amount = amount_lp * 1000000;

    let cpi_account = Transfer {
        from: source_lp.to_account_info().clone(),
        to: destination_lp.to_account_info().clone(),
        authority: authority_lp.to_account_info().clone(),
    };
    let cpi_program = token_program.to_account_info();

    let _res_base = token::transfer(CpiContext::new(cpi_program, cpi_account), token_amount);

    let source_sol = solpool.clone();
    let destination_sol = &ctx.accounts.owner;
    let amount_sol = *&mut solpool.amount * 1000000000;
    msg!("AMOUNT sol:{}", amount_sol);
    **source_sol.to_account_info().try_borrow_mut_lamports()? -= amount_sol;
    **destination_sol.try_borrow_mut_lamports()? += amount_sol;

    // let cpi_account = system_program::Transfer {
    //     from: source_sol.to_account_info().clone(),
    //     to: destination_sol.to_account_info().clone(),
    // };
    // let cpi_program = ctx.accounts.system_program.to_account_info();
    // let _res = system_program::transfer(CpiContext::new(cpi_program, cpi_account), amount_sol);

    ///////////////  transfer base token    //////////////
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

    let from = &ctx.accounts.base_token_pool;
    let to = &ctx.accounts.base_userAta;
    let authority_base = pool.clone();
    let token_amount_base = pool.amount_base_token * 1000000000;
    token::transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            token::Transfer {
                from: from.to_account_info(),
                to: to.to_account_info(),
                authority: authority_base.to_account_info(),
            },
            signer_seeds,
        ),
        token_amount_base,
    )?;

    pool.amount_base_token = 0;
    solpool.amount = 0;
    solpool.owner = ctx.accounts.owner.key();

    Ok(())
}
