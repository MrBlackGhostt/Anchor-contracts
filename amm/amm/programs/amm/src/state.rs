use anchor_lang::prelude::*;

#[account]
//Put default unimplemention of the struct with all fields set to their default values
#[derive(Default)]
pub struct Amm {
    pub admin: Pubkey,

    pub id: Pubkey,
    pub fee: u16,
}

impl Amm {
    pub const LEN: usize = 8 + 32 + 32 + 2;
}

#[account]
pub struct Pool {
    pub amm: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
}

impl Pool {
    pub const LEN: usize = 32 * 3;
}
