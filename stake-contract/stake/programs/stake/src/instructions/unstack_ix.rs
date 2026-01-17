use anchor_lang::{
    accounts,
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{
    errors::VaultError,
    state::{UserStats, Vault},
};

#[derive(Accounts)]
pub struct UnStack<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut, seeds=[b"stake", signer.key().as_ref()], bump)]
    pub user_stats: Account<'info, UserStats>,

    #[account(mut, seeds=[b"vault", signer.key().as_ref()],bump, close= signer)]
    pub vault: Account<'info, Vault>,

    pub system_programs: Program<'info, System>,
}
impl<'info> UnStack<'info> {
    pub fn claim(ctx: Context<UnStack>) -> Result<()> {
        let vault = &ctx.accounts.vault;
        let mut user_stats = &mut ctx.accounts.user_stats;
        let signer = &ctx.accounts.signer;
        let bump = ctx.bumps.vault;

        let signer_seeds = &[b"signer", signer.key.as_ref(), &[bump]];
        require_gt!(
            vault.to_account_info().lamports(),
            0,
            VaultError::ClaimAmountInvalid
        );
        let signer_seeds_binding = &[&signer_seeds[..]];
        let clock = Clock::get()?.slot;

        let time_stacked = clock - user_stats.stake_at;

        let rewards_rate = 0.0001;

        let claimable_amount = (time_stacked as f64 * rewards_rate) as u64;

        let amount = vault.to_account_info().lamports();

        let ctx = CpiContext::new(
            ctx.accounts.system_programs.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.signer.to_account_info(),
            },
        )
        .with_signer(signer_seeds_binding);

        transfer(ctx, amount + claimable_amount * 1000000000);

        Ok(())
    }
}
