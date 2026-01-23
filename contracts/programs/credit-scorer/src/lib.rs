use anchor_lang::prelude::*;

declare_id!("2VkMEF537iSMtaxCXdh2xiyBop8mHAiB4GgjjPjXiEUv");

pub mod constants;
pub mod errors;
pub mod events;
pub mod  instructions;
pub mod  state;
pub mod utils;


pub use  constants::*;
pub use errors::*;
pub use events::*;
pub use instructions::*;
pub use state::*;
pub use utils::*;

#[program]
pub mod credit_scorer {
    use super::*;

    /// Initialize the credit scoring system
    pub fn initialize(
        ctx: Context<Initialize>,
        oracle_authority: Pubkey,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, oracle_authority)
    }

    /// Request a credit score for a wallet
    pub fn request_score(
        ctx: Context<RequestScore>,
    ) -> Result<()> {
        instructions::request_score::handler(ctx)
    }

    /// Submit credit score (oracle only)
    pub fn submit_score(
        ctx: Context<SubmitScore>,
        credit_score: u16,
        score_data: Vec<u8>,
    ) -> Result<()> {
        instructions::submit_score::handler(ctx, credit_score, score_data)
    }

    /// Verify Noir ZK proof
    pub fn verify_proof(
        ctx: Context<VerifyProof>,
        proof_data: Vec<u8>,
        public_inputs: Vec<u8>,
        threshold: u16,
    ) -> Result<bool> {
        instructions::verify_proof::handler(ctx, proof_data, public_inputs, threshold)
    }
}

