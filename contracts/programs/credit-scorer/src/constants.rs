// programs/credit-scorer/src/constants.rs

/// Minimum credit score (600)
pub const MIN_CREDIT_SCORE: u16 = 600;

/// Maximum credit score (1000)
pub const MAX_CREDIT_SCORE: u16 = 1000;

/// Score validity period (12 hours in seconds)
pub const SCORE_VALIDITY_PERIOD: i64 = 12 * 60 * 60;

/// Credit score tiers
pub const SCORE_EXCELLENT: u16 = 900; // 900+
pub const SCORE_VERY_GOOD: u16 = 800; // 800-899
pub const SCORE_GOOD: u16 = 700;      // 700-799
pub const SCORE_FAIR: u16 = 600;      // 600-699

/// Collateral ratios for each tier (basis points)
pub const COLLATERAL_EXCELLENT: u16 = 5000;  // 50%
pub const COLLATERAL_VERY_GOOD: u16 = 7500;  // 75%
pub const COLLATERAL_GOOD: u16 = 10000;      // 100%
pub const COLLATERAL_FAIR: u16 = 12500;      // 125%
pub const COLLATERAL_POOR: u16 = 15000;      // 150%