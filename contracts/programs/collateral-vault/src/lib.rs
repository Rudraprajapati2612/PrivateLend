// programs/collateral-vault/src/lib.rs
use anchor_lang::prelude::*;

declare_id!("EM2iS7i794uhrGqodNjf8yPorTJSUdzziNuQqY23cAFA");

pub mod state;
pub mod instructions;
pub mod errors;
pub mod events;
pub mod constants;

pub use state::*;
pub use instructions::*;
pub use errors::*;
pub use events::*;
pub use constants::*;

#[program]
pub mod collateral_vault {
    use super::*;

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        collateral_mint: Pubkey,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, collateral_mint)
    }

    pub fn lock_collateral(
        ctx: Context<LockCollateral>,
        loan_id: u64,
        amount: u64,
        collateral_commitment: [u8; 32],
    ) -> Result<()> {
        instructions::lock::handler(ctx, loan_id, amount, collateral_commitment)
    }

    pub fn release_collateral(
        ctx: Context<ReleaseCollateral>,
        loan_id: u64,
    ) -> Result<()> {
        instructions::release::handler(ctx, loan_id)
    }

    pub fn liquidate_collateral(
        ctx: Context<LiquidateCollateral>,
        loan_id: u64,
    ) -> Result<()> {
        instructions::liquidate::handler(ctx, loan_id)
    }

    pub fn get_vault_stats(
        ctx: Context<GetVaultStats>,
    ) -> Result<VaultStats> {
        instructions::stats::handler(ctx)
    }
}