use anchor_lang::{prelude::*, solana_program::hash};
use ed25519_dalek::{PublicKey, Signature, Verifier};
use sha2::{Digest, Sha256};

use crate::{constants::{COLLATERAL_EXCELLENT, COLLATERAL_FAIR, COLLATERAL_GOOD, COLLATERAL_POOR, COLLATERAL_VERY_GOOD, SCORE_EXCELLENT, SCORE_FAIR, SCORE_GOOD, SCORE_VERY_GOOD}, errors::CreditScorerError};

pub fn calculate_score_commitment(score:u16,secret:&[u8;32])->[u8;32]{
    let mut hasher = Sha256::new();
    hasher.update(score.to_be_bytes());
    hasher.update(secret);

    let result = hasher.finalize();

    let mut commitment = [0u8;32];
    commitment.copy_from_slice(&result[..32]);
    commitment

}

pub fn verify_oracle_signature(
    oracle_pubkey: &Pubkey,
    message: &[u8],
    signature: &[u8; 64],
)->Result<bool>{
    // Convert the Public Key to the Bytes 
    let pubkey_bytes = oracle_pubkey.to_bytes();

    let public_key = PublicKey::from_bytes(&pubkey_bytes)
    .map_err(|_| error!(CreditScorerError::InvalidOracleSignature))?;

    let sig  = Signature::from_bytes(signature)
    .map_err(|_| error!(CreditScorerError::InvalidOracleSignature))?;
    

    public_key
        .verify(message, &sig)
        .map_err(|_| error!(CreditScorerError::InvalidOracleSignature))?;
    
    Ok(true)
}

pub fn verify_noir_proof_structure(
    proof_data: &[u8],  //contain cryptography Proof generated with the ZK system 
    public_inputs: &[u8],
    threshold: u16,
)->Result<bool>{

    
    require!(proof_data.len()>=64,CreditScorerError::ProofDataTooShort);

    require!(public_inputs.len()>=2,CreditScorerError::InvalidPublicInputs);

    let public_threshold = u16::from_be_bytes([
        public_inputs[0],
        public_inputs[1]
    ]); 

    require!(public_threshold == threshold , CreditScorerError::InvalidProof);

        
    // Check Proof contain non zero 

    let has_data  = proof_data.iter().any(|&x|x!=0);

    require!(has_data,CreditScorerError::InvalidProof);

     
    msg!(" Noir proof structure validated");
    msg!("Threshold: {}", threshold);
    
    
    Ok(true)
}

/// Get collateral requirement based on credit score
pub fn get_collateral_requirement(score: u16) -> u16 {
    if score >= SCORE_EXCELLENT {
        COLLATERAL_EXCELLENT // 50%
    } else if score >= SCORE_VERY_GOOD {
        COLLATERAL_VERY_GOOD // 75%
    } else if score >= SCORE_GOOD {
        COLLATERAL_GOOD // 100%
    } else if score >= SCORE_FAIR {
        COLLATERAL_FAIR // 125%
    } else {
        COLLATERAL_POOR // 150%
    }
}

/// Check if score is valid (not expired)
pub fn is_score_valid(expires_at: i64, current_time: i64) -> bool {
    current_time < expires_at
}

pub fn get_score_tier(score: u16) -> &'static str {
    if score >= SCORE_EXCELLENT {
        "Excellent"
    } else if score >= SCORE_VERY_GOOD {
        "Very Good"
    } else if score >= SCORE_GOOD {
        "Good"
    } else if score >= SCORE_FAIR {
        "Fair"
    } else {
        "Poor"
    }
}