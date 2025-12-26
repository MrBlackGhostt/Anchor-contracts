use anchor_lang::prelude::*;

declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod valut {
    use anchor_lang::system_program::{transfer, Transfer};
    use super::*;

    pub fn deposit(ctx: Context<InitializeVault>, amount: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        require_eq!(ctx.accounts.vault.lamports(),0,VaultError::VaultAlreadyExists );

        require_gt!(amount, Rent::get()?.minimum_balance(0), VaultError::InvalidAmount);
        let signer = ctx.accounts.user.to_account_info();
    let cpi_program = ctx.accounts.system_program.to_account_info();
let cpi_account= ctx.accounts.vault.to_account_info();

        transfer(
            CpiContext::new(cpi_program, // which program the cpi happen
            Transfer{
                from: signer,
                to:cpi_account
            },
        ), amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<InitializeVault>, amount: u64) -> Result<()>{
        
require_gt!(ctx.accounts.vault.lamports(), 0,VaultError::InvalidAmount); 
        require_gte!( ctx.accounts.vault.lamports(),amount, VaultError::InvalidAmount);

let signer_key = ctx.accounts.user.key();
let account_from = ctx.accounts.vault.to_account_info();
        let accounts_to  = ctx.accounts.user.to_account_info();

        let cpi_program = ctx.accounts.system_program.to_account_info();

     
let signer_key = ctx.accounts.user.key();
    let seeds: &[&[u8]] = &[
        b"vault",
        signer_key.as_ref(),
        &[ctx.bumps.vault],
    ];
    let signer_seeds: &[&[&[u8]]] = &[seeds];

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: account_from,
            to: accounts_to,
        },
        signer_seeds,
    );

    
transfer(cpi_context, amount)?;
        Ok(())
}
}

#[derive(Accounts)]
pub struct InitializeVault <'info>{
    #[account(mut)]
    user: Signer<'info>,
    #[account(mut, 
seeds=[b"vault", user.key().as_ref()], 
        bump
    )]
vault: SystemAccount<'info>,
    system_program: Program<'info, System>
}

#[error_code]
pub enum VaultError {
#[msg("The Vault is already exist")]
VaultAlreadyExists,
    #[msg("Invalid amount")]
    InvalidAmount
}
