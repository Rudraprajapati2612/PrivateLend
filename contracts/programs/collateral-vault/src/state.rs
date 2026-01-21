
use anchor_lang::prelude::*;


#[account]

pub struct  Vault{
    pub authority : Pubkey,
    pub collateral_mint :Pubkey,
    pub vault_token_account : Pubkey,
    pub total_locked : u64,
    pub total_locked_all_time : u64,
    pub total_released : u64,
    pub total_liquidated : u64,
    pub active_locks_count : u32,
    pub total_locks_count : u32,
    pub created_at : i64,
    pub bump : u8
}

impl Vault {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 4 + 4 + 8 + 1;
    pub const SEED_PREFIX: &'static [u8] = b"vault";
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum  LockStatus {
    Locked,
    Released,
    Liquidated
}

#[account]

pub struct  CollateralLock{ 
    pub loan_id : u64,
    pub borrower : Pubkey,
    pub lender : Pubkey,
    pub vault : Pubkey,
    pub amount : u64,
    pub commitment : [u8;32],
    pub status : LockStatus,
    pub locked_at : i64,
    pub released_at : i64,
    pub initial_health_factor : u16,
    pub current_health_factor : u16,
    pub bump : u8
}


impl CollateralLock {
    pub const LEN: usize = 8 + 8 + 32 + 32 + 32 + 8 + 32 + 2 + 8 + 8 + 2 + 2 + 1;
    pub const SEED_PREFIX: &'static [u8] = b"collateral_lock";
    
    pub fn is_active(&self) -> bool {
        self.status == LockStatus::Locked
    }
    
    pub fn can_liquidate(&self, threshold: u16) -> bool {
        self.is_active() && self.current_health_factor <= threshold
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]

pub struct  VaultStats{
    pub total_locked : u64,
    pub total_released : u64,
    pub total_liquidated : u64,
    pub active_locks : u32,
    pub utilization_rate : u16
}