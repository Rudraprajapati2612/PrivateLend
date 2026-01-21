use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{constants::MIN_COLLATERAL_AMOUNT, errors::VaultError, events::CollateralLocked, state::{CollateralLock, Vault}};


#[derive(Accounts)]
#[instruction(loan_id: u64)]
pub struct LockCollateral<'info>{

    #[account(mut)]
    pub borrower : Signer<'info>,
    /// Check : lender Public Key 
    pub lender : UncheckedAccount<'info>,

    #[account(
        init,
        payer = borrower,
        space = CollateralLock::LEN,
        seeds = [CollateralLock::SEED_PREFIX,loan_id.to_le_bytes().as_ref()],
        bump 
    )]
    pub collateral_lock : Account<'info,CollateralLock>,

    #[account(mut)]
    pub vault : Account<'info,Vault>, 


    #[account(mut)]
    pub borrower_collateral_account : Account<'info,TokenAccount>,

    #[account(mut)]
    pub vault_token_account : Account<'info,TokenAccount>,

    pub token_program : Program<'info,Token>,

    pub system_program : Program<'info,System>

}

pub fn handler(
    ctx:Context<LockCollateral>,
    loan_id : u64,
    amount  : u64,
    collateral_commitment : [u8;32]
)->Result<()>{
    
    let lock = &mut ctx.accounts.collateral_lock;
    let vault_key = ctx.accounts.vault.key();
    let vault = &mut ctx.accounts.vault;

    let clock = Clock::get()?;

    // Check that amount is greater thatn require collateral amount 
    require!(amount>=MIN_COLLATERAL_AMOUNT,VaultError::AmountTooLow);

    // Transfer Collateral to Vault 

    let cpi_account = Transfer{
        from    : ctx.accounts.borrower_collateral_account.to_account_info(),
        to   : ctx.accounts.vault_token_account.to_account_info(),
        authority : ctx.accounts.borrower.to_account_info()
    };

    let cpi_ctx =  CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_account);

    token::transfer(cpi_ctx, amount)?;

    //  Initalize Lock 

    lock.loan_id = loan_id;
    lock.borrower = ctx.accounts.borrower.key();
    lock.lender = ctx.accounts.lender.key();
    lock.vault = vault_key;
    lock.amount = amount;
    lock.commitment = collateral_commitment;
    lock.status = crate::state::LockStatus::Locked;
    lock.locked_at = clock.unix_timestamp;
    lock.released_at = 0;
    lock.initial_health_factor=100;
    lock.current_health_factor =100;
    lock.bump = ctx.bumps.collateral_lock;

    // Update Vault State 

    vault.total_locked = vault.total_locked.checked_add(amount).ok_or(error!(VaultError::ArithmeticOverflow))?;
    vault.total_locked_all_time = vault.total_locked_all_time.checked_add(amount).unwrap();
    vault.active_locks_count = vault.active_locks_count.checked_add(1).unwrap();
    vault.total_locks_count = vault.total_locks_count.checked_add(1).unwrap();
    emit!(CollateralLocked {
        lock_id: lock.key(),
        loan_id,
        borrower: lock.borrower,
        amount,
        timestamp: clock.unix_timestamp,
    });
    msg!("Collateral locked: {}", amount);
    Ok(())
}