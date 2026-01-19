use anchor_lang::prelude::*;


#[event]
pub struct Loanrequested {
    pub loan_id : u64,
    pub borrower : Pubkey,
    pub pool : Pubkey,
    pub amount : u64,
    pub collateral_amount : u64,
    pub duration : i64,
    pub credit_score : u16,
    pub require_collateral_ratio : u16,
    pub timestamp : i64
}

#[event]

pub struct  LoanApproved{
    pub loan_id : u64,
    pub borrower : Pubkey,
    pub lender  : Pubkey,
    pub pool : Pubkey,
    pub amount : u64,
    pub timestamp : i64
}
#[event]
pub struct LoanDisbursed {
    pub loan_id : u64,
    pub borrower : Pubkey,
    pub lender : Pubkey,
    pub amount : u64,
    pub collateral_locked : u64,
    pub timestamp : i64
}

#[event]

pub struct PartialRepayment{
    pub loan_id : u64,
    pub borrower : Pubkey,
    pub amount_paid : u64,
    pub total_paid : u64,
    pub remaining_balance : u64,
    pub repayment_percentage : u16,
    pub timestamp : i64
}

#[event]

pub struct  LoanFullyRepaid{
    pub loan_id : u64,
    pub borrower : Pubkey,
    pub lender : Pubkey,
    pub total_amount_paid : u64,
    pub principal : u64,
    pub interest_paid:u64,
    pub collateral_released : u64,
    pub timestamp : i64
}

#[event]

pub struct CollateralAdded{
    pub loan_id : u64,
    pub borrower : Pubkey,
    pub additional_amount : u64,
    pub new_total_collateral : u64,
    pub new_health_factore : u16,
    pub timestamp : i64
}


#[event]

pub struct  HealthFactorUpdated{
    pub loan_id : u64,
    pub borrower : Pubkey,
    pub old_health_factor : u16,
    pub new_health_factor : u16,
    pub collateral_value_usd : u64,
    pub timestamp : i64
}


#[event]

pub struct LoanDefaulted {
    pub loan_id : u64,
    pub borrowed : Pubkey,
    pub lender : Pubkey,
    pub principal : u64,
    pub collateral_seized : u64,
    pub timestamp : i64
}