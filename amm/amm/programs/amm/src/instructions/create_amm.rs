use anchor_lang::prelude::*;

use crate::errors::Error::InvalidFee;
use crate::state::Amm;
//I did run :LSPRestart so the code get recommend me
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
    pub signer: Signer<'info>,

    #[account(init ,payer=signer, space=Amm::LEN, seeds=[id.as_ref()], bump, constraint= fee < 10000 @ InvalidFee)]
    pub amm: Account<'info, Amm>,

    pub admin: AccountInfo<'info>,

    // How we telling the account is only read only
    pub system_program: Program<'info, System>,
}
