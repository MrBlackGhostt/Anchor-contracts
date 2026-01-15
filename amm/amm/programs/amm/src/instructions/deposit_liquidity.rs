use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked, MintTo};
use fixed::types::I64F64;

use crate::state::{Amm,Pool};
use crate::constants::LIQUIDITY_SEED;
use crate::errors::Error::DepositeTooSmall;
use crate::constants::AUTHORITY_SEED;

pub fn deposite_liquidity(
    ctx:Context<DepositLiquidity>,
    amount_a:u64,
    amount_b:u64
) -> Result<()>{

    let mut amount_a =if amount_a > ctx.accounts.depositer_account_a.amount {
        ctx.accounts.depositer_account_a.amount 
    }else {
        amount_a
    };
    
    let mut amount_b = if amount_b > ctx.accounts.depositer_account_b.amount {
        ctx.accounts.depositer_account_b.amount
    }else{
        amount_b
    };

    let pool_a = &ctx.accounts.pool_account_a;
    let pool_b  = &ctx.accounts.pool_account_b;

let pool_creation = pool_a.amount == 0 && pool_b.amount == 0;

    (amount_a,amount_b) = if !pool_creation{
    (amount_a, amount_b)
    }else {
            let constant = I64F64::from_num(pool_a.amount).checked_mul(I64F64::from_num(pool_b.amount)).unwrap(); 

            if amount_a > amount_b {
                (I64F64::from_num(constant).checked_div(I64F64::from_num(amount_b)).unwrap().to_num::<u64>(), amount_b)
            }else{
                (amount_a,I64F64::from_num(constant).checked_div(I64F64::from_num(amount_a)).unwrap().to_num::<u64>() )
            }
        };

    //amount of liquidity needed
    let mut liquidity = I64F64::from_num(amount_a).checked_mul(I64F64::from_num(amount_b)).unwrap().sqrt().to_num::<u64>();

    if pool_creation {
        if liquidity < MINIMUM_LIQUIDITY{
            return  err!(DepositeTooSmall);
        }
    }else {
        liquidity -= MINIMUM_LIQUIDITY  // this is to lock it as initial liquidity
    }

let cpi_account  = TransferChecked{
    mint: ctx.accounts.mint_a.to_account_info(),
        from: ctx.accounts.depositer_account_a.to_account_info(),
        to: ctx.accounts.pool_account_a.to_account_info(),
    authority: ctx.accounts.depositer.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();

    let cpi_context = CpiContext::new(cpi_program, cpi_account);

    token_interface::transfer_checked(cpi_context, amount_a, ctx.accounts.mint_a.decimals)?;

//  Transfer Token B from depositer_account_b to pool_account_b
    let cpi_account  = TransferChecked{
    mint: ctx.accounts.mint_b.to_account_info(),
        from: ctx.accounts.depositer_account_b.to_account_info(),
        to: ctx.accounts.pool_account_b.to_account_info(),
    authority: ctx.accounts.depositer.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();

    let cpi_context = CpiContext::new(cpi_program, cpi_account);

    token_interface::transfer_checked(cpi_context, amount_a, ctx.accounts.mint_b.decimals)?;

///// add the liquidity token tradfer to the deposite
   let authority_bump = ctx.bumps.pool_authority; 
    let mint_seed = &[&ctx.accounts.amm.key().to_bytes(), &ctx.accounts.mint_a.key().to_bytes(), &ctx.accounts.mint_b.key().to_bytes(),AUTHORITY_SEED, &[authority_bump]];

let signer_seeds =&[&mint_seed[..]]; 

        let cpi_accounts = MintTo{
            mint: ctx.accounts.mint_liquidity.to_account_info(),
            to: ctx.accounts.depositer_mint_liquidity_account.to_account_info(), //Depositere ATA
        //for the mint_liquidity
            authority: ctx.accounts.pool_authority.to_account_info()
    };

    let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts).with_signer(signer_seeds);

    token_interface::mint_to(cpi_context, liquidity)?;
    Ok(())
}




#[derive(Accounts)]
pub struct DepositLiquidity <'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(seeds=[amm.id.as_ref()], bump)]
    pub amm: AccountInfo<'info, Amm>,



#[account(seeds=[pool.amm.as_ref(), pool.mint_a.key().as_ref(), pool.mint_b.key().as_ref()], bump, has_one= mint_a, has_one=mint_b)]
    pub pool : Box<Account<'info, Pool>>,

    #[account(seeds=[
        pool.amm.as_ref(), mint_a.key().as_ref(), mint_b.key().as_ref(), AUTHORITY_SEED
    ], bump)]
    pub pool_authority: AccountInfo<'info>,

#[account(seeds=[amm.key().as_ref(), 
             mint_a.key().as_ref(), 
             mint_b.key().as_ref(), 
             LIQUIDITY_SEED.as_ref() ],bump)]
    pub mint_liquidity: Box<InterfaceAccount<'info, Mint>>, 

pub mint_a: Box<InterfaceAccount<'info,Mint>>,
pub mint_b: Box<InterfaceAccount<'info,Mint>>,

    //Pool account of token B
    #[account(seeds=[associated_token::mint=mint_a, associated_token::authority= pool_authority], bump)]
    pub pool_account_a: InterfaceAccount<'info,TokenAccount>,

    //Pool account of token B
    #[account(seeds=[associated_token::mint=mint_a, associated_token::authority= pool_authority, associated_token::token_program= token_program], bump)]
    pub pool_account_b: InterfaceAccount<'info,TokenAccount>,

#[account(mut)]
    depositer: AccountInfo<'info>,

    #[account(mut, 
    associated_token::mint= mint_a,
    associated_token::authority=depositer)]
    pub depositer_account_a:  InterfaceAccount<'info, TokenAccount>,
    //Depositer for account B
    #[account(mut, 
    associated_token::mint= mint_a,
    associated_token::authority=depositer)]
    pub depositer_account_b:  InterfaceAccount<'info, TokenAccount>,
    

    #[account(init, payer= depositer, associated_token::mint= mint_liquidity, associated_token::authority= depositer, associated_token::token_program = token_program)]
    pub depositer_mint_liquidity_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program: Program<'info,AssociatedToken>,
    pub system_program: Program<'info,System>

}
