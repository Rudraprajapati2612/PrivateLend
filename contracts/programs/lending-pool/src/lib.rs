

use anchor_lang::prelude::*;


declare_id!("8EJ5aeJVwQuZjftHywYjA1KUVGjaKwLYwUTJE8iLQbhr");

// Import modules
pub mod states;
pub mod instructions;
pub mod errors;
pub mod events;
pub mod constants;


pub use states::*;
pub use instructions::*;
pub use errors::*;
pub use events::*;
pub use constants::*;

#[program]
pub mod lending_pool {
    use super::*;

   
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        pool_name: String,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, pool_name)
    }

    /// Deposit tokens into the lending pool
    /// 
    /// # Arguments
    /// * `ctx` - Context with lender, pool, and token accounts
    /// * `amount` - Amount to deposit (in base units)
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
    ) -> Result<()> {
        instructions::deposit::handler(ctx, amount)
    }

    /// Withdraw tokens from the lending pool
    /// 
    /// # Arguments
    /// * `ctx` - Context with lender, pool, and token accounts
    /// * `amount` - Amount to withdraw (in base units)
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64,
    ) -> Result<()> {
        instructions::withdraw::handler(ctx, amount)
    }

    /// Emergency pause pool (only authority can call)
    /// 
    /// # Arguments
    /// * `ctx` - Context with authority and pool accounts
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn pause_pool(
        ctx: Context<PausePool>,
    ) -> Result<()> {
        instructions::pause::handler(ctx)
    }

    /// Unpause pool (only authority can call)
    /// 
    /// # Arguments
    /// * `ctx` - Context with authority and pool accounts
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn unpause_pool(
        ctx: Context<UnpausePool>,
    ) -> Result<()> {
        instructions::pause::unpause_handler(ctx)
    }
}