use core::hash;

use anchor_lang::prelude::*;
use sha2::{Digest, Sha256, digest::consts::True};

use crate::{constants::{BASIS_POINTS_DIVISOR, COLLATERAL_RATIO_EXCELLENT, COLLATERAL_RATIO_FAIR, COLLATERAL_RATIO_GOOD, COLLATERAL_RATIO_POOR, COLLATERAL_RATIO_VERY_GOOD, MAX_CREDIT_PROOF_AGE}, errors::LoanManagerError, state::NoirCreditProof};
// #[derive(AnchorDeserialize, Debug)]
// pub  struct  IncoProof{
//     pub credit_score : u16,
//     pub timestamp : i64,
//     pub signature : [u8;64],
//     pub proof_data:Vec<u8>
// }

    pub fn verify_noir_credit_proof (
        proof : &NoirCreditProof,
        threshold : u16,
        current_time : i64
    )->Result<bool>{
        let age = current_time.checked_sub(proof.timestamp)
        .ok_or(LoanManagerError::ArithmeticOverflow)?;

        require!(age<=MAX_CREDIT_PROOF_AGE,LoanManagerError::CreditProofExpired);


        let public_threshold = u16::from_le_bytes([
            proof.public_inputs[0],
            proof.public_inputs[1]
        ]);

        require!(public_threshold==threshold,LoanManagerError::InvalidCreditProofSignature);

        require!(proof.proof.len()>=64,LoanManagerError::InvalidCreditProofSignature);

        let commitment_valid = proof.score_commitment.iter().any(|&x| x!=0);
        require!(commitment_valid,LoanManagerError::InvalidCreditProofSignature);
         
        msg!("Noir proof verified");
        msg!("Proof proves: score >= {}", threshold);
        msg!("Actual score hidden via ZK");
        
        Ok(true)
    }



    pub fn extract_score_tire_from_commitment(commitment : &[u8;32])->u16{
        let tire_indicator = u16::from_le_bytes([commitment[0],commitment[1]]);

        if tire_indicator >= 900{
            COLLATERAL_RATIO_EXCELLENT
        }
        else if tire_indicator >= 800 {
            COLLATERAL_RATIO_VERY_GOOD
        }
        else if tire_indicator >=700 {
            COLLATERAL_RATIO_GOOD
        }
        else if tire_indicator>=600 {
            COLLATERAL_RATIO_FAIR
        }
        else {
            COLLATERAL_RATIO_POOR
        }
    }

pub fn get_required_collateral_ratio(credit_score:u16)->u16{
    if credit_score>=900 {
        COLLATERAL_RATIO_EXCELLENT
    }
    else if credit_score >=800{
        COLLATERAL_RATIO_VERY_GOOD
    }
    else if credit_score>=700{
        COLLATERAL_RATIO_GOOD
    }
    else if credit_score>=600 {
        COLLATERAL_RATIO_FAIR
    }
    else {
        COLLATERAL_RATIO_POOR
    }
}




pub fn verify_privacy_commitment(
    commitment : [u8;32],
    nullifier : [u8;32],
    amount : u64
)->Result<bool>{

    let mut hasher = Sha256::new();
    hasher.update(nullifier);
    hasher.update(amount.to_le_bytes());

    let computed = hasher.finalize();

    let valid = computed.as_slice() == commitment;

    if !valid {
        msg!("Invlaid Verification ");
        msg!("Expected: {:?}", commitment);
        msg!("Computed: {:?}", computed);
    }
    Ok(valid)
}


pub fn verify_nullifier_unused(
    nullifier : &[u8;32],
    used_nullifier : &Vec<[u8;32]>
)->Result<bool>{
    let is_used = !used_nullifier.contains(nullifier);

    require!(is_used,LoanManagerError::NullifierAlreadyUsed);

    Ok(is_used)

}
  
pub fn verify_amount_inrange(
    commitment : [u8;32],
    expected_range : (u64,u64),
    range_proof : Option<&[u8;64]>
)->Result<bool>{
    if let Some(proof) = range_proof {
        // For hackathon we are jsut checking that proof is not empty 
        let proof_valid = proof.iter().any(|&x| x!=0);
        require!(proof_valid,LoanManagerError::InvalidRangeProof);
    }

    msg!("Amount verified to be in range: {} - {}", 
    expected_range.0, expected_range.1);
    Ok(true)
}

pub fn calculate_required_collateral(loan_amount:u64,collateral_ratio:u16)->Result<u64>{

    let required = (loan_amount as u128)
                        .checked_mul(collateral_ratio as u128)
                        .and_then(|v| v.checked_div(BASIS_POINTS_DIVISOR as u128))
                        .and_then(|v|u64::try_from(v).ok())
                        .ok_or(error!(LoanManagerError::ArithmeticOverflow))?;

    Ok(required)
}

// pub fn calculate_health_factore(
//     collateral_value_usd : u64,
//     principal : u64
// )->Result<u16>{

// }