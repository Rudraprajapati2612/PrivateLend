// LoanManager Contract - Main Entry Point (Privacy-Enabled)
// Location: programs/loan-manager/src/lib.rs
// Anchor Version: 0.30.1

use anchor_lang::prelude::*;

// Program ID - Replace after first build
declare_id!("62cntT6xRY9yRPFENRwV8ZEnwkkTx84jGHKfVbHhv8fX");

// Import modules
pub mod state;
pub mod instructions;
pub mod errors;
pub mod events;
pub mod constants;
pub mod utils;

// Re-export
pub use state::*;
pub use instructions::*;
pub use errors::*;
pub use events::*;
pub use constants::*;
pub use utils::*;

#[program]
pub mod loan_manager {
    use super::*;

    /// Request a new loan with privacy commitments and ZK credit proof
    /// PRIVACY: Amounts stored as commitments, credit score verified via ZK
    pub fn request_loan(
        ctx: Context<RequestLoan>,
        // Privacy Cash commitments
        principal_commitment: [u8; 32],
        principal_nullifier: [u8; 32],
        principal_range: (u64, u64),
        collateral_commitment: [u8; 32],
        collateral_nullifier: [u8; 32],
        collateral_range: (u64, u64),
        duration_days: u16,
        // Noir ZK credit proof
        credit_proof: NoirCreditProof,
        credit_threshold: u16,
    ) -> Result<()> {
        instructions::request_loan::handler(
            ctx,
            principal_commitment,
            principal_nullifier,
            principal_range,
            collateral_commitment,
            collateral_nullifier,
            collateral_range,
            duration_days,
            credit_proof,
            credit_threshold,
        )
    }

    /// Approve a loan and match with lender
    /// CRITICAL: Handles Edge Case #1 (Race Condition - Atomic Liquidity Lock)
    pub fn approve_loan(
        ctx: Context<ApproveLoan>,
        loan_id: u64,
    ) -> Result<()> {
        instructions::approve_loan::handler(ctx, loan_id)
    }

    /// Disburse loan funds to borrower
    /// Transfers funds and locks collateral
    /// PRIVACY: Verifies commitments before disbursement
    pub fn disburse_loan(
        ctx: Context<DisburseLoan>,
        loan_id: u64,
        // Reveal amounts to verify commitments
        actual_principal: u64,
        actual_collateral: u64,
        principal_secret: [u8; 32],
        collateral_secret: [u8; 32],
    ) -> Result<()> {
        instructions::disburse_loan::handler(
            ctx,
            loan_id,
            actual_principal,
            actual_collateral,
            principal_secret,
            collateral_secret,
        )
    }

    /// Repay loan (full or partial)
    /// CRITICAL: Handles Edge Case #2 (Partial Repayment Tracking)
    /// PRIVACY: Uses repayment commitments
    pub fn repay_loan(
        ctx: Context<RepayLoan>,
        loan_id: u64,
        // Privacy commitment for repayment
        repayment_commitment: [u8; 32],
        repayment_nullifier: [u8; 32],
        // Reveal amount to process
        actual_repayment_amount: u64,
        repayment_secret: [u8; 32],
    ) -> Result<()> {
        instructions::repay_loan::handler(
            ctx,
            loan_id,
            repayment_commitment,
            repayment_nullifier,
            actual_repayment_amount,
            repayment_secret,
        )
    }

    /// Add additional collateral to existing loan
    /// Useful when health factor drops
    /// PRIVACY: Uses commitment for additional collateral
    pub fn add_collateral(
        ctx: Context<AddCollateral>,
        loan_id: u64,
        // Privacy commitment for additional collateral
        additional_commitment: [u8; 32],
        additional_nullifier: [u8; 32],
        // Reveal amount to process
        actual_additional_amount: u64,
        additional_secret: [u8; 32],
    ) -> Result<()> {
        instructions::add_collateral::handler(
            ctx,
            loan_id,
            additional_commitment,
            additional_nullifier,
            actual_additional_amount,
            additional_secret,
        )
    }

    /// Update loan health factor
    /// Called by health monitoring service
    pub fn update_health_factor(
        ctx: Context<UpdateHealthFactor>,
        loan_id: u64,
        collateral_value_usd: u64,
        health_factor: u16,
    ) -> Result<()> {
        instructions::update_health::handler(
            ctx,
            loan_id,
            collateral_value_usd,
            health_factor,
        )
    }
}