// LoanManager Contract - Update Health Factor Instruction
// Location: programs/loan-manager/src/instructions/update_health.rs
// Purpose: Update loan health factor based on collateral price changes

use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;
use crate::events::*;
use crate::constants::*;

pub fn handler(
    ctx: Context<UpdateHealthFactor>,
    loan_id: u64,
    collateral_value_usd: u64,  // Current USD value of collateral
    new_health_factor: u16,      // Calculated health factor (100 = 1.0x)
) -> Result<()> {
    let loan = &mut ctx.accounts.loan;
    let clock = Clock::get()?;
    
    msg!("📊 Updating health factor for loan #{}...", loan_id);
    
    // Validate loan is active
    require!(
        loan.status == LoanStatus::Active || loan.status == LoanStatus::PartiallyRepaid,
        LoanManagerError::InvalidLoanStatus
    );
    
    // Store old values for event
    let old_health_factor = loan.health_factor;
    let old_collateral_value = loan.collateral_value_usd;
    
    msg!("📊 Current state:");
    msg!("  Old health factor: {}.{}x", old_health_factor / 100, old_health_factor % 100);
    msg!("  Old collateral value: ${}", old_collateral_value);
    
    // Update loan data
    loan.collateral_value_usd = collateral_value_usd;
    loan.health_factor = new_health_factor;
    loan.last_health_check = clock.unix_timestamp;
    
    msg!("📊 New state:");
    msg!("  New health factor: {}.{}x", new_health_factor / 100, new_health_factor % 100);
    msg!("  New collateral value: ${}", collateral_value_usd);
    
    // Check health factor status and set warning flags
    if new_health_factor <= HEALTH_FACTOR_LIQUIDATION {
        // CRITICAL: Below liquidation threshold
        msg!("🚨 CRITICAL: Health factor at liquidation level!");
        msg!("   Threshold: {}.{}x", HEALTH_FACTOR_LIQUIDATION / 100, HEALTH_FACTOR_LIQUIDATION % 100);
        
        // In production: Trigger liquidation process
        loan.status = LoanStatus::Defaulted;
        
    } else if new_health_factor <= HEALTH_FACTOR_CRITICAL {
        // DANGER: At critical level
        msg!("⚠️  DANGER: Health factor at critical level!");
        msg!("   Threshold: {}.{}x", HEALTH_FACTOR_CRITICAL / 100, HEALTH_FACTOR_CRITICAL % 100);
        msg!("   Action required: Add collateral immediately");
        
        loan.warning_sent = true;
        
    } else if new_health_factor <= HEALTH_FACTOR_WARNING {
        // WARNING: Approaching danger zone
        msg!("⚠️  WARNING: Health factor low!");
        msg!("   Threshold: {}.{}x", HEALTH_FACTOR_WARNING / 100, HEALTH_FACTOR_WARNING % 100);
        msg!("   Recommendation: Consider adding collateral");
        
        loan.warning_sent = true;
        
    } else {
        // HEALTHY: Above warning threshold
        msg!("✅ HEALTHY: Health factor acceptable");
        
        // Clear warning if it was previously set
        if loan.warning_sent {
            loan.warning_sent = false;
            msg!("✅ Previous warning cleared");
        }
    }
    
    // Calculate health factor change
    let factor_change = if new_health_factor > old_health_factor {
        new_health_factor - old_health_factor
    } else {
        old_health_factor - new_health_factor
    };
    
    msg!("📈 Health factor change: {}% ({})", 
        factor_change,
        if new_health_factor > old_health_factor { "improved" } else { "declined" }
    );
    
    // Emit event
    emit!(HealthFactorUpdated {
        loan_id: loan.loan_id,
        borrower: loan.borrower,
        old_health_factor,
        new_health_factor,
        collateral_value_usd,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("✅ Health factor updated successfully!");
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    loan_id: u64,
    collateral_value_usd: u64,
    new_health_factor: u16,
)]
pub struct UpdateHealthFactor<'info> {
    /// Health monitor service (authorized to update health)
    /// In production: This would be a specific authorized pubkey
    pub monitor: Signer<'info>,
    
    /// Loan account
    #[account(
        mut,
        seeds = [Loan::SEED_PREFIX, loan_id.to_le_bytes().as_ref()],
        bump = loan.bump,
    )]
    pub loan: Account<'info, Loan>,
}