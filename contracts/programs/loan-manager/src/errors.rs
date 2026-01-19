// programs/loan-manager/src/errors.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum LoanManagerError {
    #[msg("Loan amount is below minimum required (100 tokens)")]
    LoanAmountTooLow,
    
    #[msg("Loan amount exceeds maximum allowed (1,000,000 tokens)")]
    LoanAmountTooHigh,
    
    #[msg("Loan duration is below minimum (1 day)")]
    DurationTooShort,
    
    #[msg("Loan duration exceeds maximum (365 days)")]
    DurationTooLong,
    
    #[msg("Credit score is below minimum required (600)")]
    CreditScoreTooLow,
    
    #[msg("Credit proof has expired (max age: 12 hours)")]
    CreditProofExpired,
    
    #[msg("Invalid credit proof signature or structure")]
    InvalidCreditProofSignature,
    
    #[msg("Insufficient collateral for this credit tier")]
    InsufficientCollateral,
    
    #[msg("Pool has insufficient liquidity")]
    InsufficientPoolLiquidity,
    
    #[msg("Invalid loan status for this operation")]
    InvalidLoanStatus,
    
    #[msg("Repayment amount exceeds remaining balance")]
    RepaymentExceedsBalance,
    
    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    
    #[msg("Arithmetic underflow occurred")]
    ArithmeticUnderflow,
    
    #[msg("Only the borrower can perform this action")]
    UnauthorizedBorrower,
    
    #[msg("Only the lender can perform this action")]
    UnauthorizedLender,
    
    #[msg("Loan has not been approved yet")]
    LoanNotApproved,
    
    #[msg("Loan is already approved")]
    LoanAlreadyApproved,
    
    #[msg("Loan repayment is overdue")]
    LoanOverdue,
    
    #[msg("Invalid commitment hash")]
    InvalidCommitment,
    
    #[msg("Nullifier has already been used (prevents double-spend)")]
    NullifierAlreadyUsed,
    
    #[msg("Invalid range proof")]
    InvalidRangeProof,
    
    #[msg("Merkle proof verification failed")]
    InvalidMerkleProof,
}