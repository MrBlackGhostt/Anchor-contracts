use anchor_lang::prelude::*;
#[account]

pub struct UserStats {
    pub owner: Pubkey,
    pub amount: u64,
    pub stake_at: u64,
}

impl UserStats {
    pub const LEN: usize = 8 + 32 + 8 + 8;
}

#[account]
pub struct Vault {}
