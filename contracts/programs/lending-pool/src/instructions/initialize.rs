use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{constants::MAX_POOL_NAME_LENGTH, errors::LendingPoolError, events::PoolInitialized, states::LendingPool};
#[derive(Accounts)]
pub struct InitializePool<'info>{
    #[account(mut)]
    pub authority : Signer<'info>,
    #[account(
        init ,
        payer = authority,
        space = LendingPool::LEN,
        seeds = [LendingPool::SEED_PREFIX,token_mint.key().as_ref()],
        bump
    )]
    pub pool : Account<'info,LendingPool>,


    pub token_mint : Account<'info,Mint>,

    #[account(
        constraint = pool_token_account.mint == token_mint.key(),
        constraint = pool_token_account.owner == pool.key()
    )]
    pub pool_token_account : Account<'info,TokenAccount>,

    pub system_program: Program<'info, System>,

    /// Token program
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx:Context<InitializePool>,pool_name : String)->Result<()>{
    require!(pool_name.len()<=MAX_POOL_NAME_LENGTH,LendingPoolError::PoolNameTooLong);
    let pool_key = ctx.accounts.pool.key();
    let pool = &mut ctx.accounts.pool;
    let clock = Clock::get()?;

    pool.authority = ctx.accounts.authority.key();

    pool.token_mint = ctx.accounts.token_mint.key();

    pool.pool_token_account = ctx.accounts.pool_token_account.key();

    pool.pool_name = pool_name.clone();

    pool.total_deposits = 0;
    pool.available_liquidity = 0;
    pool.total_borrowed = 0;
    pool.cumulative_interest = 0;
    pool.active_loans_count = 0;
    pool.total_loans_count = 0;
    pool.paused = false;
    pool.created_at = clock.unix_timestamp;
    pool.bump = ctx.bumps.pool;


    emit!(PoolInitialized{
        pool: pool_key,
        authority: pool.authority,
        token_mint: pool.token_mint,
        pool_name,
        timestamp: clock.unix_timestamp,
    });

    msg!("Pool initialized successfully");
    msg!("Pool address: {}", pool.key());
    msg!("Token mint: {}", pool.token_mint);
    Ok(())
}