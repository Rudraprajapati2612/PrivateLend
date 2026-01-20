use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{
    errors::LoanManagerError, 
    events::{LoanFullyRepaid, PartialRepayment}, 
    instructions::{LenderAccount, LendingPool}, 
    state::{Loan, LoanStatus, PrivateAmount}, 
    utils::{calculate_required_collateral, verify_nullifier_unused, verify_privacy_commitment}
};

#[derive(Accounts)]
#[instruction(
    loan_id: u64,
    repayment_commitment: [u8; 32],
    repayment_nullifier: [u8; 32],
    actual_repayment_amount: u64,
    repayment_secret: [u8; 32],
)]
pub struct RepayLoan<'info> {
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
    
    /// Pool account
    #[account(
        mut,
        constraint = pool.key() == loan.pool,
    )]
    pub pool: Account<'info, LendingPool>,
    
    /// Lender's account
    #[account(
        mut,
        constraint = lender_account.lender == loan.lender,
        constraint = lender_account.pool == pool.key(),
    )]
    pub lender_account: Account<'info, LenderAccount>,
    
    /// Borrower's token account (source for repayment)
    #[account(
        mut,
        constraint = borrower_token_account.owner == borrower.key(),
        constraint = borrower_token_account.mint == pool.token_mint,
    )]
    pub borrower_token_account: Account<'info, TokenAccount>,
    
    /// Borrower's collateral account (destination if fully repaid)
    #[account(
        mut,
        constraint = borrower_collateral_account.owner == borrower.key(),
        constraint = borrower_collateral_account.mint == loan.collateral_mint,
    )]
    pub borrower_collateral_account: Account<'info, TokenAccount>,
    
    /// Pool token account (destination for repayment)
    #[account(
        mut,
        constraint = pool_token_account.key() == pool.pool_token_account,
    )]
    pub pool_token_account: Account<'info, TokenAccount>,
    
    /// Collateral vault (source if fully repaid)
    #[account(
        mut,
        constraint = collateral_vault.mint == loan.collateral_mint,
    )]
    pub collateral_vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<RepayLoan>,
    loan_id: u64,
    repayment_commitment: [u8; 32],
    repayment_nullifier: [u8; 32],
    actual_repayment_amount: u64,
    repayment_secret: [u8; 32],
) -> Result<()> {
    let loan = &mut ctx.accounts.loan;
    let pool = &mut ctx.accounts.pool;
    let lender_account = &mut ctx.accounts.lender_account;
    let clock = Clock::get()?;

    msg!("💳 Processing loan repayment for loan #{}...", loan_id);

    // ✅ STEP 1: Validate loan status
    require!(
        loan.status == LoanStatus::Active || loan.status == LoanStatus::PartiallyRepaid,
        LoanManagerError::InvalidLoanStatus
    );

    // ✅ STEP 2: Validate borrower
    require!(
        loan.borrower == ctx.accounts.borrower.key(),
        LoanManagerError::UnauthorizedBorrower
    );

    // ✅ STEP 3: Verify privacy commitment
    msg!("🔐 Verifying repayment commitment...");

    let repayment_valid = verify_privacy_commitment(
        &repayment_commitment,
        
        &repayment_secret,
        actual_repayment_amount
    )?;

    require!(repayment_valid, LoanManagerError::InvalidCommitment);
    msg!("✅ Repayment commitment verified");

    // ✅ STEP 4: Verify nullifier not used
    verify_nullifier_unused(&repayment_nullifier, &loan.used_nullifier)?;
    msg!("✅ Nullifier verified (no double-spend)");

    // ✅ STEP 5: Calculate loan amounts
    let estimated_principal = (loan.principal_range.0 + loan.principal_range.1) / 2;

    let time_elapsed = clock
        .unix_timestamp
        .checked_sub(loan.start_time)
        .ok_or(error!(LoanManagerError::ArithmeticUnderflow))?;

    let seconds_per_year = 365 * 24 * 60 * 60;

    // ✅ FIX: Calculate interest correctly
    let interest_accrued = (estimated_principal as u128)
        .checked_mul(loan.interest_rate as u128)
        .and_then(|v| v.checked_mul(time_elapsed as u128))
        .and_then(|v| v.checked_div(seconds_per_year as u128))
        .and_then(|v| v.checked_div(10000))
        .and_then(|v| u64::try_from(v).ok())
        .ok_or(error!(LoanManagerError::ArithmeticOverflow))?;

    let total_owed = estimated_principal
        .checked_add(interest_accrued)
        .ok_or(error!(LoanManagerError::ArithmeticOverflow))?;

    msg!("📊 Loan calculation:");
    msg!("  Principal: ~{}", estimated_principal);
    msg!("  Interest accrued: {}", interest_accrued);
    msg!("  Total owed: {}", total_owed);
    msg!("  Repayment amount: {}", actual_repayment_amount);

    // ✅ STEP 6: Validate repayment amount
    require!(
        actual_repayment_amount <= total_owed,
        LoanManagerError::RepaymentExceedsBalance
    );
    msg!("✅ Repayment amount validated");

    // ✅ STEP 7: Transfer repayment from borrower to pool
    msg!("💸 Transferring repayment...");

    let cpi_accounts = Transfer {
        from: ctx.accounts.borrower_token_account.to_account_info(),
        to: ctx.accounts.pool_token_account.to_account_info(),
        authority: ctx.accounts.borrower.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::transfer(cpi_ctx, actual_repayment_amount)?;

    msg!("✅ Repayment transferred: {} tokens", actual_repayment_amount);

    // ✅ STEP 8: Mark nullifier as used
    loan.used_nullifier.push(repayment_nullifier);

    // ✅ STEP 9: Update repayment commitment
    loan.repaid_commitment = PrivateAmount::new(repayment_commitment, repayment_nullifier);

    // ✅ STEP 10: Calculate interest and principal portions
    let interest_portion = actual_repayment_amount.min(interest_accrued);
    let principal_portion = actual_repayment_amount.saturating_sub(interest_portion);

    msg!("📊 Repayment breakdown:");
    msg!("  Interest portion: {}", interest_portion);
    msg!("  Principal portion: {}", principal_portion);

    // ✅ STEP 11: Update pool reserves (happens for both full and partial)
    pool.available_liquidity = pool
        .available_liquidity
        .checked_add(principal_portion)
        .ok_or(error!(LoanManagerError::ArithmeticOverflow))?;

    pool.cumalative_interest = pool
        .cumalative_interest
        .checked_add(interest_portion)
        .ok_or(error!(LoanManagerError::ArithmeticOverflow))?;

    msg!("✅ Pool updated");
    msg!("  New available liquidity: {}", pool.available_liquidity);
    msg!("  Total interest earned: {}", pool.cumalative_interest);

    // ✅ STEP 12: Calculate remaining balance
    let remaining_balance = total_owed.saturating_sub(actual_repayment_amount);

    msg!("📊 Remaining balance: {}", remaining_balance);

    // ✅ STEP 13: Check if fully repaid or partial
    if remaining_balance == 0 || actual_repayment_amount >= total_owed {
        // ═══════════════════════════════════════════
        // FULLY REPAID PATH
        // ═══════════════════════════════════════════
        msg!("🎉 Loan fully repaid!");

        loan.status = LoanStatus::Repaid;

        // Release collateral back to borrower
        msg!("🔓 Releasing collateral...");

        let estimated_collateral = (loan.principal_range.0 + loan.principal_range.1) / 2;
        let required_ratio = loan.required_collateral_ratio;
        let collateral_to_release =
            calculate_required_collateral(estimated_collateral, required_ratio)?;

        // Use loan PDA as signer for vault
        let loan_id_bytes = loan.loan_id.to_le_bytes();
        let seeds = &[Loan::SEED_PREFIX, loan_id_bytes.as_ref(), &[loan.bump]];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.collateral_vault.to_account_info(),
            to: ctx.accounts.borrower_collateral_account.to_account_info(),
            authority: loan.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        token::transfer(cpi_ctx, collateral_to_release)?;

        msg!("✅ Collateral released: {} tokens", collateral_to_release);

        // Update lender account
        lender_account.lent_amount = lender_account
            .lent_amount
            .checked_sub(estimated_principal)
            .ok_or(error!(LoanManagerError::ArithmeticUnderflow))?;

        lender_account.available_amount = lender_account
            .available_amount
            .checked_add(estimated_principal)
            .ok_or(error!(LoanManagerError::ArithmeticOverflow))?;

        lender_account.interest_earned = lender_account
            .interest_earned
            .checked_add(interest_portion)
            .ok_or(error!(LoanManagerError::ArithmeticOverflow))?;

        msg!("✅ Lender account updated");

        // Update pool stats
        pool.active_loan_count = pool.active_loan_count.saturating_sub(1);
        pool.total_borrowed = pool
            .total_borrowed
            .checked_sub(estimated_principal)
            .ok_or(error!(LoanManagerError::ArithmeticUnderflow))?;

        // Emit full repayment event
        emit!(LoanFullyRepaid {
            loan_id: loan.loan_id,
            borrower: loan.borrower,
            lender: loan.lender,
            total_amount_paid: 0,     // Hidden for privacy
            principal: 0,              // Hidden for privacy
            interest_paid: 0,          // Hidden for privacy
            collateral_released: 0,    // Hidden for privacy
            timestamp: clock.unix_timestamp,
        });

        msg!("🎊 LOAN FULLY REPAID!");
        msg!("🔓 Collateral returned to borrower");
    } else {
        // ═══════════════════════════════════════════
        // PARTIAL REPAYMENT PATH
        // ═══════════════════════════════════════════
        msg!("📝 Partial repayment recorded");

        loan.status = LoanStatus::PartiallyRepaid;

        // ✅ Track total repaid amount
        loan.total_repaid = loan
            .total_repaid
            .checked_add(actual_repayment_amount)
            .ok_or(error!(LoanManagerError::ArithmeticOverflow))?;

        msg!("Total repaid so far: {}", loan.total_repaid);

        // ✅ Update lender account (partial repayment)
        lender_account.lent_amount = lender_account
            .lent_amount
            .checked_sub(principal_portion)
            .ok_or(error!(LoanManagerError::ArithmeticUnderflow))?;

        lender_account.available_amount = lender_account
            .available_amount
            .checked_add(principal_portion)
            .ok_or(error!(LoanManagerError::ArithmeticOverflow))?;

        lender_account.interest_earned = lender_account
            .interest_earned
            .checked_add(interest_portion)
            .ok_or(error!(LoanManagerError::ArithmeticOverflow))?;

        msg!("✅ Lender account updated");
        msg!("  Available: +{}", principal_portion);
        msg!("  Lent: -{}", principal_portion);
        msg!("  Interest earned: +{}", interest_portion);

        // Update pool stats (partial)
        pool.total_borrowed = pool
            .total_borrowed
            .checked_sub(principal_portion)
            .ok_or(error!(LoanManagerError::ArithmeticUnderflow))?;

        msg!("✅ Pool total borrowed updated: -{}", principal_portion);

        // Calculate repayment percentage
        let repayment_percentage = ((loan.total_repaid as u128)
            .checked_mul(100)
            .and_then(|v| v.checked_div(total_owed as u128))
            .and_then(|v| u16::try_from(v).ok())
            .ok_or(error!(LoanManagerError::ArithmeticOverflow))?)
            as u16;

        // Emit partial repayment event
        emit!(PartialRepayment {
            loan_id: loan.loan_id,
            borrower: loan.borrower,
            amount_paid: 0,          // Hidden for privacy
            total_paid: 0,           // Hidden for privacy
            remaining_balance: 0,    // Hidden for privacy
            repayment_percentage,
            timestamp: clock.unix_timestamp,
        });

        msg!("✅ PARTIAL REPAYMENT ACCEPTED");
        msg!("Progress: {}%", repayment_percentage);
        msg!("Remaining: {} tokens", remaining_balance);
    }

    Ok(())
}