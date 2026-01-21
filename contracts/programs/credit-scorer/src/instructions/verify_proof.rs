// programs/credit-scorer/src/instructions/verify_proof.rs
use anchor_lang::prelude::*;
use crate::events::*;
use crate::utils::*;

pub fn handler(
    ctx: Context<VerifyProof>,
    proof_data: Vec<u8>,
    public_inputs: Vec<u8>,
    threshold: u16,
) -> Result<bool> {
    let clock = Clock::get()?;
    
    msg!("🔍 Verifying Noir ZK proof...");
    msg!("Threshold: {}", threshold);
    msg!("Proof size: {} bytes", proof_data.len());
    
    // Verify proof structure
    let is_valid = verify_noir_proof_structure(
        &proof_data,
        &public_inputs,
        threshold,
    )?;
    
    if is_valid {
        msg!(" Proof verified successfully");
        msg!("Wallet meets threshold: score >= {}", threshold);
    } else {
        msg!(" Proof verification failed");
    }
    
    emit!(ProofVerified {
        wallet: ctx.accounts.wallet.key(),
        threshold,
        is_valid,
        timestamp: clock.unix_timestamp,
    });
    
    Ok(is_valid)
}

#[derive(Accounts)]
pub struct VerifyProof<'info> {
    pub wallet: Signer<'info>,
}