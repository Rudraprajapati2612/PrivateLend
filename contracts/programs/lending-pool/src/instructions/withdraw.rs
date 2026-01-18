use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{constants::{MIN_REMAINING_BALANCE, MIN_WITHDRAWAL_AMOUNT}, errors::LendingPoolError, events::Withdrawal, states::{LenderAccount, LendingPool}};

#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(mut)]
    pub lender : Signer<'info>,

    #[account(
        mut,
        seeds = [
            LendingPool::SEED_PREFIX,
            pool.token_mint.as_ref(),
        ],
        bump = pool.bump,
    )]
    pub pool : Account<'info,LendingPool>,

    #[account(
        mut,
        seeds = [
            LenderAccount::SEED_PREFIX,
            pool.key().as_ref(),
            lender.key().as_ref(),
        ],
        bump = lender_account.bump,
        constraint= lender_account.lender == lender.key(),
        constraint = lender_account.pool == pool.key()
    )]

    pub lender_account : Account<'info,LenderAccount>,

    #[account(
        constraint = lender_token_account.mint == pool.token_mint, //it checks for this pool contain same type of token like usdc then only usdc 
        constraint = lender_token_account.owner == lender.key() // and owner is who call this withdraw and lender is signer also 
    )]
    pub lender_token_account : Account<'info,TokenAccount>,

    #[account(
        mut ,
        constraint = pool_token_account.key() == pool.pool_token_account
    )]
    pub pool_token_account : Account<'info,TokenAccount>,

    pub token_program: Program<'info, Token>,
}


pub fn handler(ctx:Context<Withdraw>,amount:u64)->Result<()>{
    let pool = &mut ctx.accounts.pool;
    let lender_account = &mut ctx.accounts.lender_account;
    let clock = Clock::get()?;
    // Min amoutn to withdraw is 1USDC 
    require!(amount>=MIN_WITHDRAWAL_AMOUNT,LendingPoolError::WithdrawalBelowMinimum);
    // available amount shows how much amount is remaning after giving to the lent if given 
    require!(lender_account.available_amount>=amount,LendingPoolError::InsufficientAvailableBalance);

    // Check pool has sufficient balane 

    require!(pool.available_liquidity>=amount,LendingPoolError::InsufficientPoolLiquidity);

    let remaning_available = lender_account.available_amount.checked_sub(amount).ok_or(LendingPoolError::BelowMinimumBalance)?;

    if remaning_available > 0{
        require!(remaning_available>=MIN_REMAINING_BALANCE,LendingPoolError::BelowMinimumBalance);
    }

    let pool_key = pool.key();
    let seeds = &[
        LendingPool::SEED_PREFIX,
        pool.token_mint.as_ref(),
        &[pool.bump],
    ];
    let signer_seeds = &[&seeds[..]];


    let cpi_account = Transfer{
        from : ctx.accounts.pool_token_account.to_account_info(),
        to : ctx.accounts.lender_token_account.to_account_info(),
        authority : pool.to_account_info()
    };

    let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),
                                         cpi_account, signer_seeds);

    token::transfer(cpi_ctx, amount)?;


    // update lender account 

    lender_account.available_amount = lender_account.available_amount.checked_sub(amount).ok_or(LendingPoolError::ArithmeticOverflow)?;

    lender_account.total_withdrawals = lender_account
    .total_withdrawals
    .checked_add(amount)
    .ok_or(LendingPoolError::ArithmeticOverflow)?;

    // Update pool state
    pool.available_liquidity = pool
        .available_liquidity
        .checked_sub(amount)
        .ok_or(LendingPoolError::ArithmeticUnderflow)?;
    
    pool.total_deposits = pool
        .total_deposits
        .checked_sub(amount)
        .ok_or(LendingPoolError::ArithmeticUnderflow)?;
    
    emit!(Withdrawal{
        lender: ctx.accounts.lender.key(),
        pool: pool.key(),
        amount,
        remaining_available: lender_account.available_amount,
        remaining_lent: lender_account.lent_amount,
        timestamp: clock.unix_timestamp,
    });
    
    msg!(" Withdrawal successful");
    msg!("Amount withdrawn: {} tokens", amount);
    msg!("Remaining available: {}", lender_account.available_amount);
    msg!("Currently lent out: {}", lender_account.lent_amount);

    if lender_account.lent_amount > 0 {
        msg!(" You have {} tokens lent out in active loans", lender_account.lent_amount);
        msg!("These will become available as loans are repaid");
    }


    Ok(())
}