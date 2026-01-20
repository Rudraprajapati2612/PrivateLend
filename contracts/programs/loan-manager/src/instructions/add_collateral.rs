
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use solana_program::clock;

use crate::{errors::LoanManagerError, events::CollateralAdded, state::{Loan, LoanStatus}, utils::{verify_nullifier_unused, verify_privacy_commitment}};


#[derive(Accounts)]
#[instruction(
    loan_id: u64,
    additional_commitment: [u8; 32],
    additional_nullifier: [u8; 32],
    actual_additional_amount: u64,
    additional_secret: [u8; 32],
)]
pub struct AddCollateral<'info> {
    /// Borrower adding collateral
    #[account(mut)]
    pub borrower: Signer<'info>,
    
    /// Loan account
    #[account(
        mut,
        seeds = [Loan::SEED_PREFIX, loan_id.to_le_bytes().as_ref()],
        bump = loan.bump,
        constraint = loan.borrower == borrower.key() @ LoanManagerError::UnauthorizedBorrower,
    )]
    pub loan: Account<'info, Loan>,
    
    /// Borrower's collateral account (source)
    #[account(
        mut,
        constraint = borrower_collateral_account.owner == borrower.key(),
        constraint = borrower_collateral_account.mint == loan.collateral_mint,
    )]
    pub borrower_collateral_account: Account<'info, TokenAccount>,
    
    /// Collateral vault (destination)
    #[account(
        mut,
        constraint = collateral_vault.mint == loan.collateral_mint,
    )]
    pub collateral_vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}



pub fn handler(
    ctx:Context<AddCollateral>,
    loan_id : u64,
    
    additional_commitment: [u8; 32],
    additional_nullifier: [u8; 32],
    // Reveal actual amount to process
    actual_additional_amount: u64,
    additional_secret: [u8; 32],
)->Result<()>{
    let loan = &mut ctx.accounts.loan;
    let clock= Clock::get()?;


    msg!(" Adding collateral to loan #{}...", loan_id);
    
    // Validate loan status (can add collateral to active or partially repaid loans)
    require!(
        loan.status == LoanStatus::Active || loan.status == LoanStatus::PartiallyRepaid,
        LoanManagerError::InvalidLoanStatus
    );
    
    // Validate borrower
    require!(
        loan.borrower == ctx.accounts.borrower.key(),
        LoanManagerError::UnauthorizedBorrower
    );

    msg!(" Verifying additional collateral commitment...");

    let commitment_valid = verify_privacy_commitment(&additional_commitment, &additional_secret, actual_additional_amount)?;

    require!(commitment_valid, LoanManagerError::InvalidCommitment);
    msg!(" Additional collateral commitment verified");
    
    // Verify nullifier hasn't been used
    verify_nullifier_unused(&additional_nullifier, &loan.used_nullifier)?;
    msg!(" Nullifier verified (no double-spend)");


    require!(
        actual_additional_amount > 0,
        LoanManagerError::InsufficientCollateral
    );
    
    msg!(" Adding {} tokens to collateral", actual_additional_amount);
    
    //  STEP 1: Transfer additional collateral to vault
    msg!(" Locking additional collateral...");
    
    let cpi_accounts = Transfer {
        from: ctx.accounts.borrower_collateral_account.to_account_info(),
        to: ctx.accounts.collateral_vault.to_account_info(),
        authority: ctx.accounts.borrower.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    token::transfer(cpi_ctx, actual_additional_amount)?;
    
    msg!("Additional collateral locked: {} tokens", actual_additional_amount);
    
    // Mark nullifier as used
    loan.used_nullifier.push(additional_nullifier);
    

    // Update the Loan collateral commitment 

    loan.principal_range.0 = loan.principal_range.0.checked_add(actual_additional_amount)
                            .ok_or(error!(LoanManagerError::ArithmeticOverflow))?;


    loan.principal_range.1 = loan.principal_range.1
        .checked_add(actual_additional_amount)
        .ok_or(error!(LoanManagerError::ArithmeticOverflow))?;

        let estimated_principal = (loan.principal_range.0 + loan.principal_range.1) / 2;
        let estimated_collateral = estimated_principal
        .checked_mul(loan.required_collateral_ratio as u64)
        .and_then(|v| v.checked_div(10000))
        .ok_or(error!(LoanManagerError::ArithmeticOverflow))?;

     // New collateral value (old + additional)
     let new_collateral_value = estimated_collateral
     .checked_add(actual_additional_amount)
     .ok_or(error!(LoanManagerError::ArithmeticOverflow))?;
    

     let new_health_factor = ((new_collateral_value as u128)
     .checked_mul(10000)
     .and_then(|v| v.checked_div(estimated_collateral as u128))
     .and_then(|v| v.checked_div(100))
     .and_then(|v| u16::try_from(v).ok())
     .ok_or(error!(LoanManagerError::ArithmeticOverflow))?) as u16;
 
 let old_health_factor = loan.health_factor;
 loan.health_factor = new_health_factor;

 msg!(" Health factor updated:");
 msg!("  Old: {}.{}x", old_health_factor / 100, old_health_factor % 100);
 msg!("  New: {}.{}x", new_health_factor / 100, new_health_factor % 100);
    
 if new_health_factor > 120 && loan.warning_sent {
    loan.warning_sent = false;
    msg!("✅ Health factor improved, warning cleared");
}
emit!(CollateralAdded {
    loan_id: loan.loan_id,
    borrower: loan.borrower,
    additional_amount: 0, // Hidden for privacy
    new_total_collateral: 0, // Hidden for privacy
    new_health_factore:new_health_factor,
    timestamp: clock.unix_timestamp,
});


msg!("✅ Collateral added successfully!");
    msg!("🔒 Additional {} tokens locked", actual_additional_amount);
    msg!("📊 New health factor: {}.{}x", new_health_factor / 100, new_health_factor % 100);
    
    Ok(())
}   