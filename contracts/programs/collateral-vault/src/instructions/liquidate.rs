use anchor_lang::prelude::*;
use anchor_spl::{token::{self, Token, TokenAccount, Transfer}, token_2022::spl_token_2022::extension::memo_transfer::instruction::RequiredMemoTransfersInstruction};

use crate::{constants::LIQUIDATION_THRESHOLD, errors::VaultError, events::CollateralLiquidated, state::{CollateralLock, LockStatus, Vault}};

#[derive(Accounts)]

pub struct LiquidatedCollateral<'info>{

    #[account(mut)]
    pub collateral_lock : Account<'info,CollateralLock>,
    #[account(mut)]
    pub vault : Account<'info,Vault>,
    #[account(mut)]
    pub lender_collateral_account : Account<'info,TokenAccount>,
    #[account(mut)]
    pub vault_token_accont : Account<'info,TokenAccount>,

    pub token_program : Program<'info,Token>
}

pub fn handler(
    ctx : Context<LiquidatedCollateral>,
    laon_id : u64
)->Result<()>{
    let lock = &mut ctx.accounts.collateral_lock;
    let vault = &mut ctx.accounts.vault;
    let clock = Clock::get()?;

    require!(lock.is_active(),VaultError::AlreadyLiquidated);

    require!(
        lock.can_liquidate(LIQUIDATION_THRESHOLD),VaultError::CannotLiquidate
    );

    // Transfer collateral to lender account 

    let vault_bump = vault.bump;
    let seeds = &[Vault::SEED_PREFIX, &[vault_bump]];
    let signer_seeds = &[&seeds[..]];
    
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_token_accont.to_account_info(),
        to: ctx.accounts.lender_collateral_account.to_account_info(),
        authority: vault.to_account_info(),
    };
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds
        ),
        lock.amount
    )?;
    
    // Update lock status
    lock.status = LockStatus::Liquidated;
    lock.released_at = clock.unix_timestamp;
    
    vault.total_locked = vault.total_locked.checked_sub(lock.amount).unwrap();
    vault.total_liquidated = vault.total_liquidated.checked_add(lock.amount).unwrap();
    vault.active_locks_count = vault.active_locks_count.saturating_sub(1);
    
    emit!(CollateralLiquidated {
        lock_id: lock.key(),
        loan_id: lock.loan_id,
        borrower: lock.borrower,
        lender: lock.lender,
        amount: lock.amount,
        health_factor: lock.current_health_factor,
        timestamp: clock.unix_timestamp,
    });

    msg!("Collateral liquidated: {}", lock.amount);
    Ok(())
}