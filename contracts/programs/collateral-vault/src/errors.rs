// programs/collateral-vault/src/errors.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Collateral amount too low")]
    AmountTooLow,
    
    #[msg("Collateral lock not found")]
    LockNotFound,
    
    #[msg("Collateral already released")]
    AlreadyReleased,
    
    #[msg("Collateral already liquidated")]
    AlreadyLiquidated,
    
    #[msg("Cannot liquidate: health factor too high")]
    CannotLiquidate,
    
    #[msg("Unauthorized access")]
    Unauthorized,
    
    #[msg("Invalid token mint")]
    InvalidMint,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[msg("Arithmetic underflow")]
    ArithmeticUnderflow,
}