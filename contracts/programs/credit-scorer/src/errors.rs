// programs/credit-scorer/src/errors.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum CreditScorerError {
    #[msg("Credit score below minimum threshold (600)")]
    ScoreTooLow,
    
    #[msg("Credit score above maximum (1000)")]
    ScoreTooHigh,
    
    #[msg("Credit score has expired. Request a new score.")]
    ScoreExpired,
    
    #[msg("Credit score not found")]
    ScoreNotFound,
    
    #[msg("Invalid oracle signature")]
    InvalidOracleSignature,
    
    #[msg("Unauthorized: Only oracle can submit scores")]
    UnauthorizedOracle,
    
    #[msg("Unauthorized: Only admin can perform this action")]
    UnauthorizedAdmin,
    
    #[msg("ZK proof verification failed")]
    InvalidProof,
    
    #[msg("Invalid public inputs")]
    InvalidPublicInputs,
    
    #[msg("Proof data too short")]
    ProofDataTooShort,
    
    #[msg("Score request already exists")]
    RequestAlreadyExists,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
}