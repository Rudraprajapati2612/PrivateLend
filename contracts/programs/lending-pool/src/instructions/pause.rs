
use anchor_lang::prelude::*;

use crate::states::*;
use crate::errors::*;
use crate::events::*;

#[derive(Accounts)]
pub struct PausePool<'info> {
    /// Pool authority (only they can pause)
    #[account(mut)]
    pub authority: Signer<'info>,

    /// Lending pool account
    #[account(
        mut,
        constraint = pool.authority == authority.key() @ LendingPoolError::UnauthorizedAccess,
    )]
    pub pool: Account<'info, LendingPool>,
}

#[derive(Accounts)]
pub struct UnpausePool<'info> {
    /// Pool authority (only they can unpause)
    #[account(mut)]
    pub authority: Signer<'info>,

    /// Lending pool account
    #[account(
        mut,
        constraint = pool.authority == authority.key() @ LendingPoolError::UnauthorizedAccess,
    )]
    pub pool: Account<'info, LendingPool>,
}

pub fn handler(
    ctx: Context<PausePool>,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let clock = Clock::get()?;

    // Check if already paused
    require!(!pool.paused, LendingPoolError::PoolPaused);

    // Pause the pool
    pool.paused = true;

    // Emit event
    emit!(PoolPaused {
        pool: pool.key(),
        authority: ctx.accounts.authority.key(),
        timestamp: clock.unix_timestamp,
    });

    msg!("⚠️  Pool paused successfully");
    msg!("No new deposits or loans allowed");
    msg!("Withdrawals still enabled");

    Ok(())
}

/// Instruction handler for unpausing the pool
pub fn unpause_handler(
    ctx: Context<UnpausePool>,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let clock = Clock::get()?;

    // Check if paused
    require!(pool.paused, LendingPoolError::PoolNotPaused);

    // Unpause the pool
    pool.paused = false;

    // Emit event
    emit!(PoolUnpaused {
        pool: pool.key(),
        authority: ctx.accounts.authority.key(),
        timestamp: clock.unix_timestamp,
    });

    msg!(" Pool unpaused successfully");
    msg!("Deposits and loans re-enabled");

    Ok(())
}

