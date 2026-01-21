
use anchor_lang::{prelude::*};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{errors::VaultError, events::CollateralReleased, state::{CollateralLock, Vault}};


#[derive(Accounts)]

pub struct ReleaseCollateral<'info>{

    #[account(mut)]
    pub collateral_lock : Account<'info,CollateralLock>,

    #[account(mut)]
    pub vault : Account<'info,Vault>,

    #[account(mut)]
    pub borrower_collateral_account :Account<'info,TokenAccount>,
    #[account(mut)]
    pub vault_token_account : Account<'info,TokenAccount>,

    pub token_program  :  Program<'info,Token>
}

pub fn handler(ctx:Context<ReleaseCollateral>,loan_id : u64)->Result<()>{
    let lock = &mut ctx.accounts.collateral_lock;
    let vault = &mut ctx.accounts.vault;
    let clock = Clock::get()?;

    require!(lock.is_active(),VaultError::AlreadyReleased);

    // Transfer collateral back to borrower 

    let vault_bump = vault.bump;

    let seeds = [Vault::SEED_PREFIX,&[vault_bump]];

    let signer_seeds = &[&seeds[..]];

    let cpi_account = Transfer{
        from : ctx.accounts.vault_token_account.to_account_info(),
        to : ctx.accounts.borrower_collateral_account.to_account_info(),
        authority : vault.to_account_info()
    } ;

    let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_account, signer_seeds);

    token::transfer(cpi_ctx, lock.amount)?;

    lock.status = crate::state::LockStatus::Released;
    lock.released_at = clock.unix_timestamp;

    // Update Vualt state 
    vault.total_locked = vault.total_locked.checked_sub(lock.amount).unwrap();
    vault.total_released = vault.total_released.checked_add(lock.amount).unwrap();
    vault.active_locks_count = vault.active_locks_count.saturating_sub(1);
    
    emit!(CollateralReleased {
        lock_id: lock.key(),
        loan_id: lock.loan_id,
        borrower: lock.borrower,
        amount: lock.amount,
        timestamp: clock.unix_timestamp,
    });
    
    msg!(" Collateral released: {}", lock.amount);
    Ok(())
}