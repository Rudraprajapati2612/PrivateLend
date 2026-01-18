

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{constants::MIN_DEPOSIT_AMOUNT, errors::LendingPoolError, events::Deposited, states::{LenderAccount, LendingPool}};
#[derive(Accounts)]
pub struct Deposit<'info>{
    #[account(mut)]
    pub lender : Signer<'info>,

    #[account(mut)]
    pub pool : Account<'info,LendingPool>,


    #[account(
        init_if_needed,
        payer = lender,
        space = LenderAccount::LEN,
        seeds = [
            LenderAccount::SEED_PREFIX,
            pool.key().as_ref(),
            lender.key().as_ref(),
        ],
        bump

    )]
    pub lender_account :  Account<'info,LenderAccount>,

    #[account(
        constraint = lender_token_account.mint == pool.token_mint,
        constraint = lender_token_account.key() == lender.key()
    )]
    pub lender_token_account : Account<'info,TokenAccount>,


    #[account(
        mut ,
        constraint = pool_token_account.key() == pool.pool_token_account
    )]
    pub pool_token_account : Account<'info,TokenAccount>,

    // System program
    pub system_program: Program<'info, System>,

    /// Token program
    pub token_program: Program<'info, Token>,
}



impl <'info> Deposit <'info> {
    pub fn initialize_lender_account (&mut self , bump:u8) ->Result<()>{
        let lender_account = &mut self.lender_account;

        //  initialize If there is a new account 

        if lender_account.lender == Pubkey::default() {
            lender_account.lender = self.lender.key();
            lender_account.pool = self.pool.key();
            lender_account.deposited_amount = 0;
            lender_account.available_amount = 0;
            lender_account.interest_earned = 0;
            lender_account.total_withdrawals = 0;
            lender_account.last_deposited_time = 0;
            lender_account.bump = bump
        }
        Ok(())
    }
}


pub fn handler(ctx:Context<Deposit>,amount:u64)->Result<()>{
    let pool = &mut ctx.accounts.pool;

    let lender_account = &mut ctx.accounts.lender_account;

    let clock = Clock::get()?;

    require!(!pool.paused,LendingPoolError::PoolPaused);

    require!(amount>MIN_DEPOSIT_AMOUNT,LendingPoolError::DepositBelowMinimum);

    // transfer token from lender account to pool account 

    let cpi_account = Transfer{
        from: ctx.accounts.lender_token_account.to_account_info(),
        to : ctx.accounts.pool_token_account.to_account_info(),
        authority : ctx.accounts.lender.to_account_info()
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_account);

    token::transfer(cpi_ctx, amount)?;


    // Update Lender account 

    lender_account.deposited_amount = lender_account
    .deposited_amount
    .checked_add(amount)
    .ok_or(LendingPoolError::ArithmeticOverflow)?;

    lender_account.available_amount = lender_account
        .available_amount
        .checked_add(amount)
        .ok_or(LendingPoolError::ArithmeticOverflow)?;

    lender_account.last_deposited_time = clock.unix_timestamp;

    // Update Pool state 

    pool.total_deposits = pool.total_deposits.checked_add(amount).ok_or(LendingPoolError::ArithmeticOverflow)?;

    pool.available_liquidity = pool.available_liquidity.checked_add(amount).ok_or(LendingPoolError::ArithmeticOverflow)?;

    emit!(Deposited {
        lender: ctx.accounts.lender.key(),
        pool: pool.key(),
        amount,
        new_available_amount: lender_account.available_amount,
        new_pool_liquidity: pool.available_liquidity,
        timestamp: clock.unix_timestamp,
    });

    msg!(" Deposit successful");
    msg!("Amount: {} tokens", amount);
    msg!("Your available balance: {}", lender_account.available_amount);
    msg!("Pool liquidity: {}", pool.available_liquidity);
    Ok(())
}