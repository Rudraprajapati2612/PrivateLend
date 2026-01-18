// LendingPool Contract - Error Types
// Location: programs/lending-pool/src/errors.rs
// Purpose: Define all custom error messages

use anchor_lang::prelude::*;

#[error_code]
pub enum LendingPoolError {
    #[msg("Pool name exceeds maximum length of 50 characters")]
    PoolNameTooLong,
    
    #[msg("Pool is currently paused. No deposits or loans allowed.")]
    PoolPaused,
    
    #[msg("Pool is not paused")]
    PoolNotPaused,
    
    #[msg("Deposit amount is below minimum required. Minimum: 100 tokens")]
    DepositBelowMinimum,
    
    #[msg("Withdrawal amount is below minimum required. Minimum: 1 token")]
    WithdrawalBelowMinimum,
    
    #[msg("Insufficient available balance for withdrawal. Check your available vs lent amounts.")]
    InsufficientAvailableBalance,
    
    #[msg("Withdrawal would leave balance below minimum required (10 tokens)")]
    BelowMinimumBalance,
    
    #[msg("Pool has insufficient liquidity for this operation")]
    InsufficientPoolLiquidity,
    
    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    
    #[msg("Arithmetic underflow occurred")]
    ArithmeticUnderflow,
    
    #[msg("Only pool authority can perform this action")]
    UnauthorizedAccess,
    
    #[msg("Pool utilization rate too high. Wait for loan repayments.")]
    UtilizationTooHigh,
    
    #[msg("Invalid token mint for this pool")]
    InvalidTokenMint,
    
    #[msg("Cannot withdraw: funds are currently lent out to active loans")]
    FundsCurrentlyLent,
}