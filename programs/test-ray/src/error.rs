use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized owner.")]
    UnauthorizedOwner,

    #[msg("Invalid base token mint.")]
    InvalidBaseToken,

    #[msg("Pool has already been initialized.")]
    AlreadyInitialized,

    #[msg("Invalid amount provided.")]
    InvalidAmount,

    #[msg("Transfer failed due to insufficient balance.")]
    InsufficientBalance,

    #[msg("Token name cannot be empty.")]
    InvalidTokenName,

    #[msg("Token symbol cannot be empty.")]
    InvalidTokenSymbol,

    #[msg("Token URI cannot be empty.")]
    InvalidTokenUri,

    #[msg("Invalid token amount.")]
    InvalidTokenAmount,

    #[msg("Overflow occurred during calculation.")]
    Overflow,

    #[msg("LP amount must be greater than zero.")]
    InvalidLpAmount,

    #[msg("Insufficient LP balance in pool.")]
    InsufficientLpBalance,
}
