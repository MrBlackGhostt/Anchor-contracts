use crate::errors::VaultError;
use crate::state::{UserStats, Vault};

use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init_if_needed, payer= signer,space=UserStats::LEN , seeds=[b"stake", signer.key().as_ref()], bump)]
    pub user_stats: Account<'info, UserStats>,

    #[account(init_if_needed,payer=signer,space=0, seeds=[b"vault", signer.key().as_ref()], bump )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let user_stats = &mut ctx.accounts.user_stats;

        let vault = &ctx.accounts.vault.to_account_info();

        let user = &ctx.accounts.signer;

        let clock = Clock::get()?.slot;

        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: user.to_account_info(),
                to: vault.to_account_info(),
            },
        );
        transfer(cpi_context, amount * 1000000000)?;

        user_stats.owner = *user.key;
        user_stats.amount += amount;
        user_stats.stake_at = clock;
        Ok(())
    }
}
