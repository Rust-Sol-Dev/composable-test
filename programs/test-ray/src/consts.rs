use crate::state::*;

// Seeds for PDA generation
pub const POOL_SEED: &[u8] = b"pool";
pub const SOL_POOL_SEED: &[u8] = b"sol_pool";
pub const METADATA_SEED: &[u8] = b"metadata";
// Space constants for accounts
// Adjust these based on the size of your structs
pub const POOL_ACCOUNT_SPACE: usize = 8 + Pool::LEN; // 8 bytes for discriminator + Pool struct size
pub const SOL_POOL_ACCOUNT_SPACE: usize = 8 + SolPool::LEN; // 8 bytes for discriminator + SolPool struct size

// Rent exempt buffer (optional, if struct sizes are dynamic)
pub const RENT_EXEMPT_BUFFER: usize = 8; // Additional bytes for future-proofing

// Define Pool structure length
impl Pool {
    pub const LEN: usize = 80    // pool_owner: Pubkey (32 bytes)
                           + 32  // base_token: Pubkey (32 bytes)
                           + 8; // amount_base_token: u64 (8 bytes)
}

// Define SolPool structure length
impl SolPool {
    pub const LEN: usize = 40   // owner: Pubkey (32 bytes)
                           + 8; // amount: u64 (8 bytes)
}

// Error messages (optional but recommended for clarity)
pub const ERR_UNAUTHORIZED: &str = "Unauthorized access.";
pub const ERR_INVALID_MINT: &str = "Invalid base token mint.";
pub const ERR_INSUFFICIENT_FUNDS: &str = "Insufficient funds.";
