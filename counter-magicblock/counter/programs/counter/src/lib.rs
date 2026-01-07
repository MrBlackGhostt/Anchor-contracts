use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::ephem::{commit_accounts, commit_and_undelegate_accounts};
use ephemeral_rollups_sdk::{
    anchor::{delegate, ephemeral},
    cpi::DelegateConfig,
};
declare_id!("HEs7dPwbM38d9hgDVzdx1fXExgogRsWZYGJefPMN8JXf");

const TEST_SEED: &[u8] = b"count";

#[ephemeral]
#[program]
pub mod counter_magicblock {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let count = &mut ctx.accounts.counter;

        count.no = 0;

        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    /// Delegate the account to the delegation program
    /// Set specific validator based on ER, see https://docs.magicblock.gg/pages/get-started/how-integrate-your-program/local-setup
    pub fn delegate(ctx: Context<DelegateInput>) -> Result<()> {
        ctx.accounts.delegate()?;
        Ok(())
    }

    /// Increment the counter and manually commit the account in the Ephemeral Rollup session.
    pub fn increment_and_commit(ctx: Context<IncrementAndCommit>) -> Result<()> {
        // ...
        ctx.accounts.increment()?; // why we put the ? here did not the increment send OK at last v
        ctx.accounts.commit_account()?; //CPI to ER  sdk
        Ok(())
    }

    /// Undelegate the account from the delegation program
    pub fn undelegate(ctx: Context<IncrementAndCommit>) -> Result<()> {
        ctx.accounts.exit_rollups()?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init, payer=signer, seeds=[TEST_SEED], bump, space=8 + 8 )]
    pub counter: Account<'info, Counter>,
    system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    pub no: u64,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut,  seeds=[TEST_SEED], bump  )]
    pub counter: Account<'info, Counter>,
}

impl Increment<'_> {
    pub fn increment(&mut self) -> Result<()> {
        let count = &mut self.counter;

        count.no += 1;
        Ok(())
    }
}

#[delegate]
#[derive(Accounts)]
pub struct DelegateInput<'info> {
    #[account(mut)] // ✅ Also safe: payer mut validated by Signer
    pub payer: Signer<'info>,

    /// CHECK: ER validator pubkey from MagicBlock local setup
    pub validator: Option<AccountInfo<'info>>,

    /// CHECK: PDA derived from [TEST_SEED], validated by delegate_pda CPI
    #[account(mut, del)]
    pub pda: AccountInfo<'info>,
}

impl DelegateInput<'_> {
    pub fn delegate(&self) -> Result<()> {
        self.delegate_pda(
            // this is the fun which transfer the ownership of the counter
            &self.payer,
            &[TEST_SEED],
            DelegateConfig {
                ..Default::default()
            },
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct IncrementAndCommit<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, seeds=[TEST_SEED], bump)]
    pub counter: Account<'info, Counter>,

    /// CHECK: MagicBlock ER session context (validated by commit_accounts CPI)
    pub magic_context: AccountInfo<'info>,

    /// CHECK: MagicBlock ER program account (fixed deployment address)
    pub magic_program: AccountInfo<'info>,
}

impl IncrementAndCommit<'_> {
    pub fn increment(&mut self) -> Result<()> {
        self.counter.no += 1;
        Ok(())
    }
    pub fn commit_account(&mut self) -> Result<()> {
        // why we give the mut the self where we
        // updating the value
        commit_accounts(
            &self.payer,
            vec![&self.counter.to_account_info()],
            &self.magic_context,
            &self.magic_program,
        )?;
        Ok(())
    }

    pub fn exit_rollups(&mut self) -> Result<()> {
        commit_and_undelegate_accounts(
            &self.payer, // for the commit account and undelegate the payer has to pay the fee
            vec![&self.counter.to_account_info()],
            &self.magic_context,
            &self.magic_program,
        )?;
        Ok(())
    }
}
