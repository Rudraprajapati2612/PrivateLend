use anchor_lang::{prelude::*, solana_program::clock};

use crate::{constants::{MAX_CREDIT_SCORE, MIN_CREDIT_SCORE, SCORE_VALIDITY_PERIOD}, errors::CreditScorerError, events::ScoreSubmitted, state::{CreditScore, ScoreStatus, ScoringConfig}, utils::{calculate_score_commitment, get_collateral_requirement, get_score_tier}};

#[derive(Accounts)]
pub struct SubmitScore<'info> {
    #[account(mut)]
    pub oracle: Signer<'info>,
    
    #[account(
        mut,
        constraint = credit_score.status == ScoreStatus::Pending @ CreditScorerError::RequestAlreadyExists,
    )]
    pub credit_score: Account<'info, CreditScore>,
    
    #[account(mut)]
    pub config: Account<'info, ScoringConfig>,
}

pub fn handler(
    ctx: Context<SubmitScore>,
    credit_score: u16,
    score_data: Vec<u8>,
)->Result<()>{
    let score_account = &mut ctx.accounts.credit_score;
    let config = &mut ctx.accounts.config;
    let clock  = Clock::get()?;

// Check oracle is authorise 
    require!(
        ctx.accounts.oracle.key() == config.oracle_authority,
        CreditScorerError::UnauthorizedOracle
    );
    
    // Validate score range
    require!(
        credit_score >= MIN_CREDIT_SCORE,
        CreditScorerError::ScoreTooLow
    );
    require!(
        credit_score <= MAX_CREDIT_SCORE,
        CreditScorerError::ScoreTooHigh
    );

    let secret = [1u8;32];
    let commitment  = calculate_score_commitment(credit_score, &secret);

    score_account.score = credit_score;
    score_account.score_commitment = commitment;
    score_account.status = ScoreStatus::Completed;
    score_account.score_data = score_data;
    score_account.scored_at = clock.unix_timestamp;
    score_account.expires_at = clock.unix_timestamp + SCORE_VALIDITY_PERIOD;

    config.total_score_issued = config.total_score_issued
    .checked_add(1)
    .unwrap();

emit!(ScoreSubmitted {
    wallet: score_account.wallet,
    score: credit_score,
    commitment,
    expires_at: score_account.expires_at,
    timestamp: clock.unix_timestamp,
});

let tier = get_score_tier(credit_score);
let collateral_req = get_collateral_requirement(credit_score);

msg!(" Credit score submitted");
msg!("Wallet: {}", score_account.wallet);
msg!("Score: {} ({})", credit_score, tier);
msg!("Collateral required: {}%", collateral_req / 100);
msg!("Expires at: {}", score_account.expires_at);


    Ok(())
}