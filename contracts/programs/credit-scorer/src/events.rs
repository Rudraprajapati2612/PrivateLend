// programs/credit-scorer/src/events.rs
use anchor_lang::prelude::*;

#[event]
pub struct ScoringSystemInitialized {
    pub config: Pubkey,
    pub oracle_authority: Pubkey,
    pub admin: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ScoreRequested {
    pub wallet: Pubkey,
    pub credit_score_account: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ScoreSubmitted {
    pub wallet: Pubkey,
    pub score: u16,
    pub commitment: [u8; 32],
    pub expires_at: i64,
    pub timestamp: i64,
}

#[event]
pub struct ProofVerified {
    pub wallet: Pubkey,
    pub threshold: u16,
    pub is_valid: bool,
    pub timestamp: i64,
}

#[event]
pub struct ScoreUsed {
    pub wallet: Pubkey,
    pub score: u16,
    pub usage_count: u32,
    pub timestamp: i64,
}