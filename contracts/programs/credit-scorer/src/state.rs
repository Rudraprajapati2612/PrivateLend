use anchor_lang::{prelude::*, solana_program::pubkey::PubkeyError};

#[account]

pub struct ScoringConfig {
    pub oracle_authority : Pubkey,

    pub admin : Pubkey,

    pub total_score_issued : u64,

    pub total_requests : u64,

    pub created_at : i64,

    pub bump : u8
}



impl ScoringConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // oracle_authority
        32 + // admin
        8 + // total_scores_issued
        8 + // total_requests
        8 + // created_at
        1; // bump
    
    pub const SEED_PREFIX: &'static [u8] = b"scoring_config";
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]

pub enum ScoreStatus {
    Pending,
    Completed,
    Expired
}

#[account]

pub struct  CreditScore{
    pub wallet : Pubkey ,

    pub score : u16,

    pub score_commitment : [u8;32],

    pub status : ScoreStatus,

    pub score_data : Vec<u8>,

    pub requested_at : i64,

    pub scored_at : i64,
    pub expires_at : i64,

    pub oracle_signature : [u8;64],
    // Number of Times the Score was used 
    pub usage_count : u32,

    pub bump : u8
}


impl CreditScore {
    pub const MAX_LEN: usize = 8 + // discriminator
        32 + // wallet
        2 + // score
        32 + // score_commitment
        2 + // status
        (4 + 200) + // score_data (vec: 4 bytes length + 200 bytes data)
        8 + // requested_at
        8 + // scored_at
        8 + // expires_at
        64 + // oracle_signature
        4 + // usage_count
        1; // bump
    
    pub const SEED_PREFIX: &'static [u8] = b"credit_score";
    
    /// Check if score is valid (not expired)
    pub fn is_valid(&self, current_time: i64) -> bool {
        self.status == ScoreStatus::Completed && current_time < self.expires_at
    }
    
    /// Check if user score is greater than required score 
    pub fn meets_threshold(&self, threshold: u16) -> bool {
        self.score >= threshold
    }
    
    /// Increment usage count
    pub fn use_score(&mut self) {
        self.usage_count = self.usage_count.saturating_add(1);
    }

}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]

pub struct ProofVerification{
    pub is_valid : bool,
    pub threshold : u16,
    pub verified_at : i64
}
