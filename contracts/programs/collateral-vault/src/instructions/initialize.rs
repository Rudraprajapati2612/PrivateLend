use anchor_lang::prelude::*;

use crate::{events::VaultInitialize, state::Vault};

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = Vault::LEN,
        seeds = [Vault::SEED_PREFIX],
        bump
    )]
    pub vault: Account<'info, Vault>,
    
    /// CHECK: Vault token account
    pub vault_token_account: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx:Context<InitializeVault>,
    collateral_mint : Pubkey
)->Result<()>{
    let vault = &mut ctx.accounts.vault;

    let clock = Clock::get()?;

    vault.authority = ctx.accounts.authority.key();
    vault.collateral_mint = collateral_mint;
    vault.vault_token_account = ctx.accounts.vault_token_account.key();
    vault.total_locked = 0;
    vault.total_locked_all_time=0;
    vault.total_released=0;
    vault.total_liquidated=0;
    vault.active_locks_count = 0;
    vault.total_locks_count=0;
    vault.created_at = clock.unix_timestamp;
    vault.bump = ctx.bumps.vault;

    emit!(VaultInitialize {
        vault: vault.key(),
        authority: vault.authority,
        collateral_mint,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Vault initialized");

    Ok(())
}