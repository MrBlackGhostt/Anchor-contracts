use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint,TokenInterface, TokenAccount, TransferChecked};

use crate::constants::{AUTHORITY_SEED,LIQUIDITY_SEED};
use fixed::types::I64F64;

use crate::state::{Amm, Pool};

pub fn withdraw_liquidity(ctx: Context<WithdrawLiquidity>, amount: u64) -> Result<()> {
    let autority_bump = ctx.bumps.pool_authority;
    let authority_seeds = &[
        &ctx.accounts.pool.amm.to_bytes(),
        &ctx.accounts.mint_a.key().to_bytes(),
        &ctx.accounts.mint_b.key().to_bytes(),
        AUTHORITY_SEED,
        &[autority_bump],
    ];

    let signer_seed = &[&authority_seeds[..]];
    // Transfer tokens from the pool
    let amount_a = I64F64::from_num(amount)
        .checked_mul(I64F64::from_num(ctx.accounts.pool_account_a.amount))
        .unwrap()
        .checked_div(I64F64::from_num(
            ctx.accounts.mint_liquidity.supply + MINIMUM_LIQUIDITY,
        ))
        .unwrap()
        .floor()
        .to_num::<u64>();

token_interface::transfer_checked(cpicontext::new(ctx.accounts.token_program.to_account_info(), transferchecked{
        mint: ctx.accounts.mint_a.to_account_info(),
        from: ctx.accounts.pool_account_a.to_account_info(),
        to: ctx.accounts.depositer_account_a.to_account_info(),
        authority: ctx.accounts.pool_authority.to_account_info()
        }).with_signer(signer_seed), amount_a, ctx.accounts.mint_b.decimals)?;
    

 let amount_b = I64F64::from_num(amount) 
     .checked_mul(I64F64::from_num(ctx.accounts.pool_account_b.amount)) 
     .unwrap() 
     .checked_div(I64F64::from_num( 
         ctx.accounts.mint_liquidity.supply + MINIMUM_LIQUIDITY, 
     )) 
     .unwrap() 
     .floor() 
     .to_num::<u64>(); 

token_interface::transfer_checked(cpicontext::new(ctx.accounts.token_program.to_account_info(), transferchecked{
        mint: ctx.accounts.mint_b.to_account_info(),
        from: ctx.accounts.pool_account_b.to_account_info(),
        to: ctx.accounts.depositer_account_b.to_account_info(),
        authority: ctx.accounts.pool_authority.to_account_info()
        }).with_signer(signer_seed), amount_a, ctx.accounts.mint_b.decimals)?;
    


    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawLiquidity<'info> {
    #[account(mut)]
    pub depositer: Signer<'info>,

    pub amm: Account<'info, Amm>,
    #[account(seeds=[
        pool.amm.as_ref(), mint_a.key().as_ref(), mint_b.key().as_ref(), AUTHORITY_SEED
    ], bump)]
    pub pool_authority: AccountInfo<'info>,

    #[account(seeds=[pool.amm.as_ref(), pool.mint_a.key().as_ref(), pool.mint_b.key().as_ref()], bump, has_one= mint_a, has_one=mint_b)]
    pub pool: Box<Account<'info, Pool>>,

#[account(seeds=[amm.key().as_ref(), 
             mint_a.key().as_ref(), 
             mint_b.key().as_ref(), 
             LIQUIDITY_SEED.as_ref() ],bump)]
    pub mint_liquidity: Box<InterfaceAccount<'info, Mint>>, 



  //Pool account of token B
    #[account(seeds=[associated_token::mint =mint_a, associated_token::authority= pool_authority], bump)]
    pub pool_account_a: InterfaceAccount<'info, TokenAccount>,

    //Pool account of token B
    #[account(seeds=[associated_token::mint=mint_a, associated_token::authority= pool_authority, associated_token::token_program= token_program], bump)]
    pub pool_account_b: InterfaceAccount<'info, TokenAccount>,


    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(associated_token::mint= mint_a, associated_token::authority=depositer, associated_token::token_program= token_program)]
    pub depositer_account_a: InterfaceAccount<'info, TokenAccount>,


    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
