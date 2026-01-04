use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenInterface, TokenAccount,TransferChecked };
use anchor_spl::associated_token::AssociatedToken;
use token_program::program::TokenProgram;


declare_id!("7zWbJyToagqhZMuYELVF761c7HgCmcT8k9HzUn48H55Z");

#[program]
pub mod valut {
    use anchor_lang::system_program::{transfer, Transfer};
    use super::*;

pub fn create_valut_pda(ctx:Context<CreateVaultPda>) ->         Result<()>{
        msg!("Create the vault pda");
    
Ok(())
    }
// Create the pda for the vault and transfer_token form user_pda to vault pda 
pub fn transfer_token(ctx: Context<TransferToken>) -> Result<()> {
    let cpi_program = ctx.accounts.token_program.to_account_info(); // ✅

    let accounts_for_ata = TransferChecked {
        mint: ctx.accounts.mint_account.to_account_info(),
        from: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),          // ✅ user is owner of `user_token_account`
        to: ctx.accounts.vault_token_account.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(cpi_program, accounts_for_ata);

    let decimals = ctx.accounts.mint_account.decimals;
    let amount   = ctx.accounts.user_token_account.amount;

    token_interface::transfer_checked(cpi_ctx, amount, decimals)?;
    Ok(())
}


    // WithDraw token from valut ata to user ata
    pub fn withdraw(ctx: Context<WithDraw>, amount: u64 ) -> Result<()>{
        
    require_gt!(ctx.accounts.vault_token_ata.amount, 0,VaultError::InvalidAmount); 
    require_gte!( ctx.accounts.vault_token_ata.amount,amount, VaultError::InvalidAmount);


    let signer_key = ctx.accounts.signer.key();
    let account_from = ctx.accounts.vault_token_ata.to_account_info();
    let accounts_to  = ctx.accounts.user_token_ata.to_account_info();

    let cpi_program = ctx.accounts.token_program.to_account_info();

     
    let signer_key = ctx.accounts.signer.key();
    let seeds: &[&[u8]] = &[
        b"vault",
        signer_key.as_ref(),
        &[ctx.bumps.vault_pda],
    ];
let authority = account_from.clone();
    let accounts_for_cpi = TransferChecked{
            mint: ctx.accounts.mint_account.to_account_info(),
            from: account_from,
            authority: authority,
            to: accounts_to
        };
            
    let cpi_context = CpiContext::new(cpi_program, accounts_for_cpi);

    let decimals = ctx.accounts.mint_account.decimals;
    token_interface::transfer_checked(cpi_context, amount, decimals)?;
     

    
        Ok(())
}

}


//PDA OF the vault for the user
#[derive(Accounts)]
pub struct CreateVaultPda <'info>{
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init_if_needed,payer=signer, space=8 + VaultAccount::INIT_SPACE , seeds=[b"vault", signer.key().as_ref()], bump)]
    vault_pda:Account<'info, VaultAccount>,
    system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct VaultAccount {
    pub owner: Pubkey,      // 32 bytes
    pub bump: u8,           // 1 byte
}


#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump
    )]
    pub vault_pda: Account<'info, VaultAccount>,

    #[account(
        mut,
        token::mint = mint_account,
        token::authority = signer,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint_account,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

    pub mint_account: InterfaceAccount<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}



//withdraw struct
#[derive(Accounts)]
pub struct WithDraw<'info>{
     
    #[account(mut)]
    signer: Signer<'info>,
    //vault pda 
    #[account(mut, 
        seeds=[b"vault", signer.key().as_ref()], 
        bump
    )]
    vault_pda: SystemAccount<'info>,
    
    // user token ata
    pub user_token_ata: InterfaceAccount<'info, TokenAccount>,
    // valut token ata 
    pub vault_token_ata: InterfaceAccount<'info, TokenAccount>,

    pub mint_account: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>

}

#[error_code]
pub enum VaultError {
#[msg("The Vault is already exist")]
VaultAlreadyExists,
    #[msg("Invalid amount")]
    InvalidAmount
}
