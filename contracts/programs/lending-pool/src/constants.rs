// LendingPool Contract - Constants
// Location: programs/lending-pool/src/constants.rs
// Purpose: Define all constant values used across the contract

/// Minimum deposit amount (100 USDC with 6 decimals)
/// Prevents spam deposits and dust
pub const MIN_DEPOSIT_AMOUNT: u64 = 100_000_000; // 100 tokens (assuming 6 decimals)

/// Minimum withdrawal amount (1 USDC with 6 decimals)
/// Prevents dust withdrawals
pub const MIN_WITHDRAWAL_AMOUNT: u64 = 1_000_000; // 1 token

/// Minimum balance that must remain after withdrawal (10 USDC)
/// Prevents complete withdrawal, keeps account active
pub const MIN_REMAINING_BALANCE: u64 = 10_000_000; // 10 tokens

/// Maximum pool name length in characters
pub const MAX_POOL_NAME_LENGTH: usize = 50;

/// Default interest rate for loans (8% APR)
/// Stored as basis points: 800 = 8.00%
pub const DEFAULT_INTEREST_RATE_BPS: u16 = 800;

/// Seconds in a year (for interest calculations)
pub const SECONDS_PER_YEAR: i64 = 365 * 24 * 60 * 60;

/// Maximum utilization rate allowed (95%)
/// If pool reaches this, no more loans can be approved
pub const MAX_UTILIZATION_RATE: u64 = 95;