use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::{token_interface::{self,Mint, TokenInterface, TokenAccount,MintTo, TransferChecked}};

//TODO write the instruction for the TransferToken
//anchor_spl provide code which is compatible types for working with both token programs
//
declare_id!("EPTmSLWCpkChWxm4mpRHvRkSfagrF3M6c77qUTmLVsiL");

#[program]
pub mod token_program {

    use super::*;
    pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.accounts.mint);
        Ok(())
    }
    pub fn create_token_account(ctx:Context<CreateTokenAccount>) -> Result<()>{
        msg!("The TokenACcount {:?}" , ctx.accounts.token_account);
        Ok(())
    }

    pub fn mint_token(ctx: Context<MintToken>, amount:u64) -> Result<()>{
        let cpi_account = MintTo{
            mint: ctx.accounts.mint.to_account_info(),
            to:ctx.accounts.token_account.to_account_info(),
            authority:ctx.accounts.user.to_account_info()
        };
        let cpi_program_id = ctx.accounts.token_program.to_account_info();

        let cpi_content = CpiContext::new(cpi_program_id,cpi_account);

        token_interface::mint_to(cpi_content, amount)?;

        Ok(())
    }
    pub fn transfer_token(ctx: Context<TransferToken>, amount:u64) -> Result<()>{
        let decimal = ctx.accounts.mint.decimals;

        let cpi_account = TransferChecked{ 
            mint:ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.from_token_account.to_account_info(),
            to:ctx.accounts.to_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info()
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context =  CpiContext::new(cpi_program, cpi_account) ;
        token_interface::transfer_checked(cpi_context, amount, decimal)?;
        Ok(())

    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub  struct CreateMint <'info>{
      #[account(mut)] 
    pub user: Signer<'info>,
    #[account(
        init,
        payer  = user,
        mint::decimals = 6, 
        mint::authority = user.key(),
        mint::freeze_authority = user.key()
    )] //Create the Mint account here 
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CreateTokenAccount<'info>{
    #[account(mut)]
pub user:Signer<'info>,
    #[account(init,
        payer= user,
        associated_token::mint= mint, 
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
pub mint: InterfaceAccount<'info, Mint>,//The InterfaceAccount type is a wrapper that allows the account to work with both the Token Program and Token Extension Program.
    pub token_program:Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info,    AssociatedToken>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct MintToken <'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
#[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>
}

#[derive(Accounts)]
pub struct TransferToken <'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub from_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub to_account: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info,TokenInterface>,

}
