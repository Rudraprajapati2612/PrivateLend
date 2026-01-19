// LoanManager Contract - Constants
// Location: programs/loan-manager/src/constants.rs
// Purpose: Define all constant values

/// Minimum loan amount (100 USDC with 6 decimals)
pub const MIN_LOAN_AMOUNT: u64 = 100_000_000;

/// Maximum loan amount (1,000,000 USDC with 6 decimals)
pub const MAX_LOAN_AMOUNT: u64 = 1_000_000_000_000;

/// Minimum loan duration (1 day in seconds)
pub const MIN_LOAN_DURATION: i64 = 24 * 60 * 60;

/// Maximum loan duration (365 days in seconds)
pub const MAX_LOAN_DURATION: i64 = 365 * 24 * 60 * 60;

/// Default interest rate (8% APR in basis points)
pub const DEFAULT_INTEREST_RATE: u16 = 800;

/// Maximum credit proof age (12 hours in seconds)
/// Edge Case #5: Prevents stale credit proofs
pub const MAX_CREDIT_PROOF_AGE: i64 = 12 * 60 * 60;

/// Collateral ratios based on credit score (in basis points)
/// Score 900+: 50% collateral
pub const COLLATERAL_RATIO_EXCELLENT: u16 = 5000; // 50%

/// Score 800-899: 75% collateral
pub const COLLATERAL_RATIO_VERY_GOOD: u16 = 7500; // 75%

/// Score 700-799: 100% collateral
pub const COLLATERAL_RATIO_GOOD: u16 = 10000; // 100%

/// Score 600-699: 125% collateral
pub const COLLATERAL_RATIO_FAIR: u16 = 12500; // 125%

/// Score <600: 150% collateral (or reject)
pub const COLLATERAL_RATIO_POOR: u16 = 15000; // 150%

/// Minimum credit score to get a loan
pub const MIN_CREDIT_SCORE: u16 = 600;

/// Health factor thresholds
/// Warning level: 1.2x (120 in our representation)
pub const HEALTH_FACTOR_WARNING: u16 = 120;

/// Critical level: 1.0x (100 in our representation)
pub const HEALTH_FACTOR_CRITICAL: u16 = 100;

/// Liquidation level: 0.9x (90 in our representation)
pub const HEALTH_FACTOR_LIQUIDATION: u16 = 90;

/// Seconds in a year (for interest calculations)
pub const SECONDS_PER_YEAR: i64 = 365 * 24 * 60 * 60;

/// Basis points divisor
pub const BASIS_POINTS_DIVISOR: u64 = 10000;