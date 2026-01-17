use anchor_lang::prelude::*;

use crate::errors::Error::InvalidFee;
use crate::state::Amm;
//I did run :LSPRestart so the code get recommend me
// ERROR: Typo in function name to match lib.rs (or lib.rs is wrong).
// ERROR: 'admin' account used here, but lib.rs expects 'admin' Pubkey argument.
pub fn create_amm(ctx: Context<CreateAmm>, id: Pubkey, fee: u16) -> Result<()> {
    let amm = &mut ctx.accounts.amm;
    amm.id = id;
    amm.fee = fee;
    amm.admin = ctx.accounts.admin.key();
    Ok(())
}

#[derive(Accounts)]
#[instruction(id:Pubkey, fee:u16)]
pub struct CreateAmm<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(init ,payer=admin, space=Amm::LEN, seeds=[id.as_ref()], bump, constraint= fee < 10000 @ InvalidFee)]
    pub amm: Account<'info, Amm>,

    // ERROR: AccountInfo is generic. If this is the admin, usage should be clearer (Signer or Unchecked).

    // How we telling the account is only read only
    // ERROR: Comment is confusing. AccountInfo is read-only unless stated otherwise.
    pub system_program: Program<'info, System>,
}
