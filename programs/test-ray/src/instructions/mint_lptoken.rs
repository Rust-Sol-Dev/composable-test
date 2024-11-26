use crate::consts::*;
use crate::error::*;
use crate::state::Pool;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct MintLptoken<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        seeds = [METADATA_SEED.as_ref(), token_metadata_program.key().as_ref(), lp_token.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    /// CHECK:
    pub metadata_account: AccountInfo<'info>,

    // Create new LP_Token
    #[account(
        init_if_needed,
        payer = owner,
        mint::decimals = 6,
        mint::authority = owner,
    )]
    pub lp_token: Account<'info, Mint>,

    // Create LP TokenAccount
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = lp_token,
        associated_token::authority = pool,
    )]
    pub lp_token_pool: Account<'info, TokenAccount>,

    pub token_metadata_program: Program<'info, Metadata>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn mint_lptoken(
    ctx: Context<MintLptoken>,
    lp_token_name: String,
    lp_token_symbol: String,
    lp_token_uri: String,
    lp_token_amount: u64,
) -> Result<()> {
    msg!("Mint LP Token Instruction Executing!");

    // Create Metadata Account
    create_metadata_accounts_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.lp_token.to_account_info(),
                mint_authority: ctx.accounts.owner.to_account_info(),
                update_authority: ctx.accounts.owner.to_account_info(),
                payer: ctx.accounts.owner.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        DataV2 {
            name: lp_token_name.clone(),
            symbol: lp_token_symbol.clone(),
            uri: lp_token_uri.clone(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        false, // Is mutable
        true,  // Update authority is signer
        None,
    )?;
    msg!("Metadata account created successfully.");

    // Mint LP tokens to the associated token account
    let mint_amount = lp_token_amount
        .checked_mul(10u64.pow(ctx.accounts.lp_token.decimals as u32))
        .ok_or(CustomError::Overflow)?;

    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.lp_token.to_account_info(),
                to: ctx.accounts.lp_token_pool.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        mint_amount,
    )?;
    msg!(
        "{} LP tokens minted to {}",
        lp_token_amount,
        ctx.accounts.lp_token_pool.key()
    );

    // Update pool state
    let pool = &mut ctx.accounts.pool;
    pool.amount_lp_token = pool
        .amount_lp_token
        .checked_add(lp_token_amount)
        .ok_or(CustomError::Overflow)?;

    msg!(
        "Pool updated with new LP token amount: {}",
        pool.amount_lp_token
    );

    Ok(())
}
