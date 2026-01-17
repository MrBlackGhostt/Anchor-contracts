use anchor_lang::prelude::*;
mod errors;
mod instructions;

pub mod state;
declare_id!("9PCD9HkNH3CnP2f9SimjNsj9Px5DJwf71cDU5XWcJVbP");

use crate::instructions::{claim_ix::ClaimStack, stake_ix::Stake, unstack_ix::UnStack};

#[program]
pub mod stake {
    use crate::instructions::{claim_ix::ClaimStack, unstack_ix::UnStack};

    use super::*;

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        Stake::stake(ctx, amount);
        Ok(())
    }

    pub fn claim_ix(ctx: Context<ClaimStack>, amount: u64) -> Result<()> {
        ClaimStack::claim(ctx, amount);
        Ok(())
    }
    pub fn stake_ix(ctx: Context<UnStack>) -> Result<()> {
        UnStack::claim(ctx);
        Ok(())
    }
}
