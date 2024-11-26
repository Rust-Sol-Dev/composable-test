use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;

#[account]
// Pool Info
pub struct Pool {
    pub pool_owner: Pubkey,
    pub base_token: Pubkey,
    pub lp_token: Pubkey,
    pub amount_base_token: u64,
    pub amount_lp_token: u64,
}

#[account]
// SOL PDA Info
pub struct SolPool {
    pub amount: u64,
    pub owner: Pubkey,
}
