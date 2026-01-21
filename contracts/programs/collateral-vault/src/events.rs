use anchor_lang::prelude::*;

#[event]

pub struct VaultInitialize{
    pub vault : Pubkey,
    pub authority : Pubkey,
    pub collateral_mint : Pubkey,
    pub timestamp : i64
}


#[event]

pub struct  CollateralLocked{
    pub lock_id: Pubkey,
    pub loan_id : u64,
    pub borrower : Pubkey,
    pub amount : u64,
    pub timestamp : i64
}

#[event]

pub struct  CollateralReleased{
    pub lock_id: Pubkey,
    pub loan_id : u64,
    pub borrower : Pubkey,
    pub amount : u64,
    pub timestamp : i64
}

#[event]

pub struct  CollateralLiquidated{
    pub lock_id : Pubkey,
    pub loan_id : u64,
    pub borrower:Pubkey,
    pub lender:Pubkey,
    pub amount : u64,
    pub health_factor : u16,
    pub timestamp : i64
}
