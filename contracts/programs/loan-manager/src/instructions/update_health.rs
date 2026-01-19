use anchor_lang::prelude::*;

use crate::state::Loan;


#[derive(Accounts)]
#[instruction(
    loan_id: u64,
    collateral_value_usd: u64,
    new_health_factor: u16,
)]
pub struct  UpdateHealthFactor<'info>{
    pub monitor : Signer<'info>,

    #[account(
        mut,
        seeds=[Loan::SEED_PREFIX,loan_id.to_le_bytes().as_ref()],
        bump=loan.bump
    )]

    pub loan : Account<'info,Loan>
}


pub fn handler(ctx:Context<UpdateHealthFactor>) ->Result<()>{
    
    Ok(())
}