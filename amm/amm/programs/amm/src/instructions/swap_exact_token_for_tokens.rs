use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface;
use anchor_spl::{
    associated_token,
    token_interface::{Mint, TokenAccount, TokenInterface,TransferChecked,Transfer},
};
use fixed::types::I64F64;

use crate::constants::{AUTHORITY_SEED,LIQUIDITY_SEED};
use crate::errors::Error;
use crate::state::{Amm, Pool};
fn swap_exact_tokens_for_tokens(
    ctx: Context<SwapExactTokensForTokens>,
    swap_a: bool,
    input_amount: u64,
    min_output_amount: u64,
) -> Result<()> {
    let input = if swap_a && input_amount > ctx.accounts.depositer_account_a.amount {
        ctx.accounts.depositer_account_a.amount
    } else if !swap_a && input_amount > ctx.accounts.depositer_account_b.amount {
        ctx.accounts.depositer_account_b.amount
    } else {
        input_amount
    };

    let amm = &ctx.accounts.amm;
    let taxed_input = input - input * amm.fee as u64 / 10000;

    let pool_a = ctx.accounts.pool_account_a.amount;
    let pool_b = ctx.accounts.pool_account_b.amount;

    let output = if swap_a {
        I64F64::from_num(taxed_input)
            .checked_mul(I64F64::from_num(pool_b))
            .unwrap()
            .checked_div(
                I64F64::from_num(pool_a)
                    .checked_add(I64F64::from_num(taxed_input))
                    .unwrap(),
            )
            .unwrap()
    } else {
        I64F64::from_num(taxed_input)
            .checked_mul(I64F64::from_num(pool_a))
            .unwrap()
            .checked_div(
                I64F64::from_num(pool_b)
                    .checked_add(I64F64::from_num(taxed_input))
                    .unwrap(),
            )
            .unwrap()
    }
    .to_num::<u64>();

    if output < min_output_amount {
        return err!(Error::OutputTooSmall);
    }

    let invariant = pool_b * pool_a;

    let authority_bump = ctx.bumps.pool_authority;
    let authority_bump = ctx.bumps.pool_authority;
    let mint_seed = &[
        &ctx.accounts.amm.key().to_bytes(),
        &ctx.accounts.mint_a.key().to_bytes(),
        &ctx.accounts.mint_b.key().to_bytes(),
        AUTHORITY_SEED,
        &[authority_bump],
    ];

    let signer_seed = &[&mint_seed[..]];

let cpi_account = TransferChecked{
        mint: ctx.accounts.mint_a.to_account_info(),
        from: ctx.accounts.depositer_account_a.to_account_info(),
        to: ctx.accounts.pool_account_a.to_account_info(),
        authority: ctx.accounts.depositer.to_account_info()
    };

    if swap_a {
   let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_account);

    token_interface::transfer_checked(cpi_context, output, ctx.accounts.mint_a.decimals);

let cpi_account_mint_b = TransferChecked{
        mint: ctx.accounts.mint_b.to_account_info(),
        from: ctx.accounts.pool_account_b.to_account_info(),
        to: ctx.accounts.depositer_account_b.to_account_info(),
        authority: ctx.accounts.pool_authority.to_account_info()
    };
let cpi_constext_send_mint_b = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_account_mint_b).with_signer(signer_seed);

    token_interface::transfer_checked(cpi_constext_send_mint_b, output, ctx.accounts.mint_b.decimals)?;

    }
    else{
token_interface::transfer_checked(CpiContext::new(ctx.accounts.token_program.to_account_info(), TransferChecked{
           mint: ctx.accounts.mint_b.to_account_info(),
        from: ctx.accounts.depositer_account_b.to_account_info(),
        to: ctx.accounts.pool_account_b.to_account_info(),
        authority: ctx.accounts.depositer.to_account_info()
        }), input_amount, ctx.accounts.mint_b.decimals)?
    }
token_interface::transfer_checked(CpiContext::new(ctx.accounts.token_program.to_account_info(), TransferChecked{
           mint: ctx.accounts.mint_a.to_account_info(),
        from: ctx.accounts.pool_account_b.to_account_info(),
        to: ctx.accounts.depositer_account_b.to_account_info(),
        authority: ctx.accounts.pool_authority.to_account_info()
        }).with_signer(signer_seed), output, ctx.accounts.mint_b.decimals)?;
    
 msg!( 
     "Traded {} tokens ({} after fees) for {}", 
     input, 
     taxed_input, 
     output 
 );

     // Verify the invariant still holds 
 // Reload accounts because of the CPIs 
 // We tolerate if the new invariant is higher because it means a rounding error for LPs 
 ctx.accounts.pool_account_a.reload()?; 
 ctx.accounts.pool_account_b.reload()?; 
 if invariant > ctx.accounts.pool_account_a.amount * ctx.accounts.pool_account_a.amount { 
     return err!(Error::InvariantViolated); 
 }
     Ok(())
}

#[derive(Accounts)]
pub struct SwapExactTokensForTokens<'info> {
    #[account(mut)]
    pub depositer: Signer<'info>,

    pub amm: Account<'info, Amm>,

    #[account(seeds=[pool.amm.as_ref(), pool.mint_a.key().as_ref(), pool.mint_b.key().as_ref()], bump, has_one= mint_a, has_one=mint_b)]
    pub pool: Box<Account<'info, Pool>>,

#[account(seeds=[amm.key().as_ref(), 
             mint_a.key().as_ref(), 
             mint_b.key().as_ref(), 
             LIQUIDITY_SEED.as_ref() ],bump)]
    pub mint_liquidity: Box<InterfaceAccount<'info, Mint>>, 



    #[account(seeds=[
        pool.amm.as_ref(), mint_a.key().as_ref(), mint_b.key().as_ref(), AUTHORITY_SEED
    ], bump)]
    pub pool_authority: AccountInfo<'info>,

    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(associated_token::mint= mint_a, associated_token::authority=depositer, associated_token::token_program= token_program)]
    pub depositer_account_a: InterfaceAccount<'info, TokenAccount>,
    #[account(associated_token::mint=mint_b, associated_token::authority=depositer, associated_token::token_program= token_program)]
    pub depositer_account_b: InterfaceAccount<'info, TokenAccount>,

    //Pool account of token B
    #[account(seeds=[associated_token::mint =mint_a, associated_token::authority= pool_authority], bump)]
    pub pool_account_a: InterfaceAccount<'info, TokenAccount>,

    //Pool account of token B
    #[account(seeds=[associated_token::mint=mint_a, associated_token::authority= pool_authority, associated_token::token_program= token_program], bump)]
    pub pool_account_b: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
