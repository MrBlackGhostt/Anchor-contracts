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
        let counter = &mut ctx.accounts.counter;

        counter.no = 0;

        msg!("PDA {} count: {}", counter.key(), counter.no);
        Ok(())
    }

    pub fn commit(ctx: Context<IncrementAndCommit>) -> Result<()> {
        ctx.accounts.commit()?;
        Ok(())
    }
    /// Delegate the account to the delegation program
    /// Set specific validator based on ER, see https://docs.magicblock.gg/pages/get-started/how-integrate-your-program/local-setup
    pub fn delegate(ctx: Context<DelegateInput>) -> Result<()> {
        ctx.accounts.delegate()?;
        Ok(())
    }

    ///Increment the counter
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.increment()?;
        Ok(())
    }

    /// Increment the counter and manually commit the account in the Ephemeral Rollup session.
    pub fn increment_and_commit(ctx: Context<IncrementAndCommit>) -> Result<()> {
        // ...
        ctx.accounts.increment_and_commit()?; // why we put the ? here did not the increment send OK at last v
        ctx.accounts.commit()?; //CPI to ER  sdk
        Ok(())
    }

    pub fn increment_and_undelegate(ctx: Context<IncrementAndCommit>) -> Result<()> {
        ctx.accounts.increment_and_undelegate()?;
        Ok(())
    }

    /// Undelegate the account from the delegation program
    pub fn undelegate(ctx: Context<IncrementAndCommit>) -> Result<()> {
        ctx.accounts.undelegate()?;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init_if_needed, payer=signer, seeds=[TEST_SEED], bump, space=8 + 8 )]
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
    pub payer: Signer<'info>,
    //Check by the delegate program
    //What is this checking exactly
    pub validator: Option<AccountInfo<'info>>,
    //the pda to delegate
    // this tell the program this account is eligibal for ownership transfer
    /// CHECK: PDA derived from seeds, validated by delegation CPI
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
    #[account(mut, seeds=[TEST_SEED], bump   )]
    pub counter: Account<'info, Counter>,
    //magic_context required by ER
    /// CHECK:` doc comment explaining why no checks through types are necessary
    pub magic_context: AccountInfo<'info>,
    //magic_program required by ER
    /// CHECK:` doc comment explaining why no checks through types are necessary
    pub magic_program: AccountInfo<'info>,
}

impl IncrementAndCommit<'_> {
    ///Incremet the counter and manual commit the account in the ER.
    pub fn increment_and_commit(&mut self) -> Result<()> {
        self.counter.no += 1;
        msg!(" count: {}", self.counter.no);

        self.counter.exit(&crate::ID)?;

        commit_accounts(
            &self.payer,
            vec![&self.counter.to_account_info()],
            &self.magic_context,
            &self.magic_program,
        )?;

        Ok(())
    }

    /// Manual commit the account in the ER.
    pub fn commit(&mut self) -> Result<()> {
        commit_accounts(
            &self.payer,
            vec![&self.counter.to_account_info()],
            &self.magic_context,
            &self.magic_program,
        )?;
        Ok(())
    }
    /// Undelegate the account from the delegation program
    pub fn undelegate(&mut self) -> Result<()> {
        commit_and_undelegate_accounts(
            &self.payer, // for the commit account and undelegate the payer has to pay the fee
            vec![&self.counter.to_account_info()],
            &self.magic_context,
            &self.magic_program,
        )?;
        Ok(())
    }

    /// Increment the counter + manual commit the account in the ER.
    pub fn increment_and_undelegate(&mut self) -> Result<()> {
        let counter = &mut self.counter;
        counter.no += 1;
        msg!("PDA {} count: {}", counter.key(), counter.no);

        commit_and_undelegate_accounts(
            &self.payer,
            vec![&self.counter.to_account_info()],
            &self.magic_context,
            &self.magic_program,
        )?;

        Ok(())
    }
}
