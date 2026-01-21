
use anchor_lang::prelude::*;

use crate::{events::ScoringSystemInitialized, state::ScoringConfig};


#[derive(Accounts)]

pub struct Initialize<'info>{
    #[account(mut)]
    pub admin : Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = ScoringConfig::LEN,
        seeds = [ScoringConfig::SEED_PREFIX],
        bump
    )]
    pub config : Account<'info,ScoringConfig>,

    pub system_program : Program<'info,System>
}


pub fn handler(
    ctx:Context<Initialize>,
    oracle_authority : Pubkey
)->Result<()>{
    let config = &mut ctx.accounts.config;
    let clock = Clock::get()?;

    config.oracle_authority = oracle_authority;
    config.admin = ctx.accounts.admin.key();
    config.total_score_issued = 0;
    config.total_requests = 0;
    config.created_at = clock.unix_timestamp;
    config.bump = ctx.bumps.config;


    emit!(ScoringSystemInitialized {
        config: config.key(),
        oracle_authority,
        admin: config.admin,
        timestamp: clock.unix_timestamp,
    });
    
    msg!(" Credit scoring system initialized");
    msg!("Oracle authority: {}", oracle_authority);
    

    Ok(())
}