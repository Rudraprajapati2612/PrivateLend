use anchor_lang::prelude::*;
#[event]
pub struct PoolInitialized{
    pub pool  : Pubkey,
    pub authority : Pubkey,
    pub token_mint : Pubkey,
    pub pool_name : String,
    pub timestamp : i64
}

#[event]
pub struct Deposited{
    pub lender : Pubkey,

    pub pool : Pubkey,

    pub amount : u64,

    pub new_available_amount : u64,

    pub new_pool_liquidity : u64,

    pub timestamp : i64
}

#[event]

pub struct Withdrawal{
    pub lender : Pubkey,
    pub pool : Pubkey,

    pub amount : u64,

    pub remaining_available : u64,
    // Fund is currentlly locked in active loan 
    pub remaining_lent : u64,

    pub timestamp : i64
}


#[event]
pub struct PoolPaused {
    pub pool: Pubkey,
    pub authority: Pubkey,
    pub timestamp: i64,
}


#[event]
pub struct PoolUnpaused {
    pub pool: Pubkey,
    pub authority: Pubkey,
    pub timestamp: i64,
}


#[event]

pub struct LiquidityLocked{
    pub pool : Pubkey,

    pub lender : Pubkey,

    pub principal : u64,

    pub interest : u64,

    pub loan_id : u64,

    pub timestamp : i64
}

/// Emitted when liquidity is released after loan repayment (called by LoanManager)
#[event]
pub struct LiquidityReleased {
    pub pool: Pubkey,
    pub lender: Pubkey,
    pub principal: u64,
    pub interest: u64,
    pub loan_id: u64,
    pub timestamp: i64,
}