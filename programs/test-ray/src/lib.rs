use anchor_lang::prelude::*;
pub mod consts;
pub mod error;
pub mod instructions;
pub mod state;
use crate::instructions::*;

declare_id!("3CH7VzEEc1487mMGfwuRLGmPXtVWaPjkciopd87JEkkX");

#[program]
pub mod test_ray {
    use super::*;

    /// Initializes the pool and sets up necessary accounts.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initializing Pool...");
        instructions::initialize(ctx)?;
        msg!("Pool initialized successfully.");
        Ok(())
    }

    /// Creates an LP token with metadata and mints it to the pool.
    pub fn mint_lptoken(
        ctx: Context<MintLptoken>,
        lp_token_name: String,
        lp_token_symbol: String,
        lp_token_uri: String,
        lp_token_amount: u64,
    ) -> Result<()> {
        msg!("Minting LP token: {} ({})", lp_token_name, lp_token_symbol);
        instructions::mint_lptoken(
            ctx,
            lp_token_name,
            lp_token_symbol,
            lp_token_uri,
            lp_token_amount,
        )?;
        msg!("LP token minted successfully.");
        Ok(())
    }

    /// Adds liquidity in SOL and base tokens.
    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        amount_base: u64,
        amount_quote: u64,
    ) -> Result<()> {
        msg!(
            "Adding liquidity: Base Token Amount: {}, SOL Amount: {}",
            amount_base,
            amount_quote
        );
        instructions::add_liquidity(ctx, amount_base, amount_quote)?;
        msg!("Liquidity added successfully.");
        Ok(())
    }

    /// Removes liquidity and returns assets to the user.
    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>, amount_lp: u64) -> Result<()> {
        msg!("Removing liquidity: LP Token Amount: {}", amount_lp);
        instructions::remove_liquidity(ctx, amount_lp)?;
        msg!("Liquidity removed successfully.");
        Ok(())
    }
}
