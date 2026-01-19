use anchor_lang::prelude::*;


#[account]

pub struct  LendingPool{
    // Authority to pause and resume the pool 
    pub authority : Pubkey,
    // address for the token that need to mint (USDC Address)
    pub token_mint : Pubkey,
    // account that hold all deposited token 
    pub pool_token_account: Pubkey,

    pub pool_name : String,
    // 
    pub total_deposits : u64,

    pub available_liquidity : u64,

    pub total_borrowed : u64,

    pub cumulative_interest : u64,

    pub active_loans_count : u32,

    pub total_loans_count : u32,
    pub paused : bool,

    pub created_at : i64,

    pub bump : u8
}

impl LendingPool {
    pub const LEN: usize = 8 + // discriminator
    32 + // authority
    32 + // token_mint
    32 + // pool_token_account
    (4 + 50) + // pool_name (max 50 chars)
    8 + // total_deposits
    8 + // available_liquidity
    8 + // total_borrowed
    8 + // cumulative_interest
    4 + // active_loans_count
    4 + // total_loans_count
    1 + // paused
    8 + // created_at
    1; // bump

    pub fn calculate_apy(&self) -> u64 {
        if self.total_deposits == 0 {
            return 0;
        }
        (self.cumulative_interest * 10000) / self.total_deposits
    }

    /// Calculate pool utilization rate
    /// Formula: (total_borrowed / total_deposits) * 100
    /// Returns: Utilization as percentage (0-100)
    pub fn calculate_utilization(&self) -> u64 {
        if self.total_deposits == 0 {
            return 0;
        }
        (self.total_borrowed * 100) / self.total_deposits
    }
    
}

#[account]
pub struct LenderAccount{
    // user 
    pub lender : Pubkey,
    //  pool address 
    pub pool : Pubkey,
    // Total amount deposited by the lender and (and dont decrease on withdrawal)
    pub deposited_amount : u64,

    // amount that is currently available to withdraw 
    pub  available_amount : u64,

    /// Amount currently lent out to active loans
    /// When loan repaid, this decreases and available_amount increases
    pub lent_amount : u64,

    pub interest_earned : u64,

    pub total_withdrawals : u64,

    pub last_deposited_time : i64,

    pub bump : u8


}

impl LenderAccount {
    pub const LEN: usize = 8 + // discriminator
    32 + // lender
    32 + // pool
    8 + // deposited_amount
    8 + // available_amount
    8 + // lent_amount
    8 + // interest_earned
    8 + // total_withdrawals
    8 + // last_deposit_time
    1; 

    pub fn total_balance(&self)->u64{
        self.available_amount + self.lent_amount
    }

    pub fn calculate_roi(&self) -> u64 {
        if self.deposited_amount == 0 {
            return 0;
        }
        (self.interest_earned * 100) / self.deposited_amount
    }
}


// seeds for PDA Derivation 

impl LendingPool {
    pub const SEED_PREFIX: &'static [u8] = b"lending_pool";
}

impl LenderAccount {
    pub const SEED_PREFIX: &'static [u8] = b"lender_account";
}