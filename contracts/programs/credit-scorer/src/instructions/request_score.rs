

use anchor_lang::prelude::*;

use crate::{events::ScoreRequested, state::{CreditScore, ScoringConfig}};

#[derive(Accounts)]

pub struct RequestScore<'info>{
    #[account(mut)]
    pub wallet : Signer<'info>,

    #[account(
        init,
        payer = wallet ,
        space = CreditScore::MAX_LEN,
        seeds = [CreditScore::SEED_PREFIX,
                wallet.key().as_ref()
        ],
        bump
    )]
    pub credit_score : Account<'info,CreditScore>,

    #[account(mut)]
    pub config : Account<'info,ScoringConfig>,

    pub system_program  : Program<'info,System>
}


pub fn handler(ctx:Context<RequestScore>)->Result<()>{
    let credit_score = &mut ctx.accounts.credit_score;
    let config = &mut ctx.accounts.config;
    let clock = Clock::get()?;

    // Initialize Credit score request 

    credit_score.wallet = ctx.accounts.wallet.key();
    credit_score.score = 0;
    credit_score.score_commitment = [0u8; 32];
    credit_score.status = crate::state::ScoreStatus::Pending;
    credit_score.score_data = vec![];
    credit_score.requested_at  = clock.unix_timestamp; 
    credit_score.scored_at = 0; 
    credit_score.expires_at = 0;
    credit_score.oracle_signature = [0u8;64];
    credit_score.bump = ctx.bumps.credit_score;

    // change state the request is +1

    config.total_requests = config.total_requests
    .checked_add(1)
    .unwrap();

    emit!(ScoreRequested {
        wallet: ctx.accounts.wallet.key(),
        credit_score_account: credit_score.key(),
        timestamp: clock.unix_timestamp,
    });
    
    msg!(" Credit score requested for wallet: {}", ctx.accounts.wallet.key());
    
    Ok(())
}