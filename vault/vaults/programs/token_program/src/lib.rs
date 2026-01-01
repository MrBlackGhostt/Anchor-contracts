use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    self, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
};

declare_id!("9MGfDMfR7TbnQD27zxsssusAwkYLPQRGt7sR8ett8QFP");

#[program]
pub mod token_program {

    use super::*;
    pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
        msg!("Going to create the mint");
        Ok(())
    }
    pub fn create_token_account(ctx: Context<CreateTokenAccount>) -> Result<()> {
        msg!(
            "Created Token Account: {:?}",
            ctx.accounts.token_account.key()
        );
        Ok(())
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        let mint_account = ctx.accounts.mint.to_account_info();
        let to_account = ctx.accounts.token_account.to_account_info();

        let authority = ctx.accounts.signer.to_account_info();

        let cpi_account = MintTo {
            mint: mint_account,
            to: to_account,
            authority: authority,
        };

        let program = ctx.accounts.token_program.to_account_info();

        let cpi_context = CpiContext::new(program, cpi_account);

        token_interface::mint_to(cpi_context, amount)?;
        Ok(())
    }
    pub fn transfer_token(ctx: Context<TransferToken>, amount: u64) -> Result<()> {
        let cpi_accounts = TransferChecked {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.token_account.to_account_info(),
            to: ctx.accounts.to_token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        let decimal = ctx.accounts.mint.decimals;
        token_interface::transfer_checked(cpi_ctx, amount, decimal)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(init, payer= signer,
    mint::decimals = 1,
        
    mint::authority = signer.key(),
    mint::freeze_authority = signer.key()
    )]
    pub mint: InterfaceAccount<'info, Mint>, // The InterfaceAccount tell anchor when deserilize
    // happen ckech who is the owner of the program as we tell in the Interface the TokenInterface
    // so anchor check the program owner is token_program or not
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(init, payer= signer,
    associated_token::mint= mint,
    associated_token::authority = signer,
        associated_token::token_program= token_program
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
      #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut,
    associated_token::mint = mint,
    associated_token::authority = signer,
    associated_token:: token_program = token_program)]
    pub token_account: InterfaceAccount<'info, TokenAccount>, //token-interface::transferChecked
    #[account( init_if_needed,payer=signer, associated_token::mint = mint,
    associated_token::authority = signer,
    associated_token:: token_program = token_program )]
    pub to_token_account: InterfaceAccount<'info, TokenAccount>,
     pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}
