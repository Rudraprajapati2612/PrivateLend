// programs/collateral-vault/src/constants.rs

/// Minimum collateral amount (1 token with 9 decimals)
pub const MIN_COLLATERAL_AMOUNT: u64 = 1_000_000_000;

/// Liquidation threshold (0.9x = 90)
pub const LIQUIDATION_THRESHOLD: u16 = 90;

/// Warning threshold (1.2x = 120)
pub const WARNING_THRESHOLD: u16 = 120;