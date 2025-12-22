use anchor_lang::prelude::*;
use anchor_lang::solana_program;
declare_id!("GBK2QeA6tHyNEfYDpW9Xa3MxtYLS7B3sypx6XNWKBHto");

#[program]
pub mod todo {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NewAccount {
    data: u64,
}
