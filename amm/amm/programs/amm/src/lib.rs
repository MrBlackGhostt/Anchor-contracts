use anchor_lang::prelude::*;

mod constants;
mod errors;
mod instructions;
mod state;

pub use errors::*;
pub use instructions::*;
declare_id!("D4bnsAoKbEFUFi6rJvuH5EVGCrCwTrVsKYLAsZvSq1Ni");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn crate_amm(ctx: Context<CreateAmm>, id: Pubkey, admin: Pubkey, fee: u16) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
