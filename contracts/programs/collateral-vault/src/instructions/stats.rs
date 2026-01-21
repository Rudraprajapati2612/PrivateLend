// programs/collateral-vault/src/instructions/stats.rs
use anchor_lang::prelude::*;
use crate::state::*;

pub fn handler(ctx: Context<GetVaultStats>) -> Result<VaultStats> {
    let vault = &ctx.accounts.vault;
    
    let utilization_rate = if vault.total_locked_all_time > 0 {
        ((vault.total_locked as u128 * 10000) / vault.total_locked_all_time as u128) as u16
    } else {
        0
    };
    
    Ok(VaultStats {
        total_locked: vault.total_locked,
        total_released: vault.total_released,
        total_liquidated: vault.total_liquidated,
        active_locks: vault.active_locks_count,
        utilization_rate,
    })
}

#[derive(Accounts)]
pub struct GetVaultStats<'info> {
    pub vault: Account<'info, Vault>,
}