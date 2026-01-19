use anchor_lang::prelude::*;

use crate::{errors::LoanManagerError, events::LoanApproved, state::{Loan, LoanStatus}};


#[account]

pub struct LendingPool{
    pub authority : Pubkey,
    pub token_mint : Pubkey,
    pub pool_token_account : Pubkey,
    pub pool_name : String,
    pub total_deposits : u64,
    pub available_liquidity : u64,
    pub total_borrowed : u64,
    pub cumalative_interest : u64,
    pub active_loan_count : u32,
    pub total_loan_count : u32,
    pub paused : bool,
    pub created_at : i64,
    pub bump : u8
}



#[account]

pub struct LenderAccount{
    pub lender : Pubkey,
    pub pool :  Pubkey,
    pub deposited_amount : u64,
    pub available_amount: u64,
    pub lent_amount : u64,
    pub interest_earned : u64,
    pub total_withdrawal : u64,
    pub last_deposited_time : i64,
    pub bump : u8
}


#[derive(Accounts)]
#[instruction(loan_id:u64)]
pub struct ApproveLoan<'info>{
    #[account(mut)]
    pub lender : Signer<'info>,

      /// Loan account
      #[account(
        mut,
        seeds = [Loan::SEED_PREFIX, loan_id.to_le_bytes().as_ref()],
        bump = loan.bump,
        constraint = loan.status == LoanStatus::Pending @ LoanManagerError::InvalidLoanStatus,
    )]
    pub loan: Account<'info, Loan>,

    #[account(
        mut,
        constraint = pool.key() == loan.pool @ LoanManagerError::InvalidLoanStatus
    )]
    pub pool : Account<'info,LendingPool>,

    #[account(
        mut,
        constraint = lender_account.lender == lender.key() @ LoanManagerError::UnauthorizedLender,
        constraint = lender_account.key() == pool.key() @ LoanManagerError::InvalidLoanStatus
    )]
    pub lender_account : Account<'info,LenderAccount>   
}


pub fn handler(
    ctx:Context<ApproveLoan>,loan_id:u64
)->Result<()>{
    let loan = &mut ctx.accounts.loan;
    let pool = &mut ctx.accounts.pool;
    let lender_account = &mut ctx.accounts.lender_account;
    let clock = Clock::get()?;


    msg!(" Approving loan #{}", loan_id);
    
    // Validate loan status
    require!(
        loan.status == LoanStatus::Pending,
        LoanManagerError::InvalidLoanStatus
    );

    let max_principal = loan.principal_range.1;

        
    msg!(" Loan principal range: {} - {}", 
        loan.principal_range.0, 
        loan.principal_range.1
    );
    msg!("📊 Maximum possible: {}", max_principal);
    msg!("💰 Pool available liquidity: {}", pool.available_liquidity);
    require!(pool.available_liquidity >= max_principal,LoanManagerError::InsufficientPoolLiquidity);

    // check lender has sufficient liquidity 

    require!(lender_account.available_amount >= max_principal , LoanManagerError::InsufficientPoolLiquidity);
    msg!("Liquidity check passed");
    

    loan.lender = ctx.accounts.lender.key();
    loan.status = LoanStatus::Approved;


    emit!(LoanApproved{
        loan_id: loan.loan_id,
        borrower: loan.borrower,
        lender: ctx.accounts.lender.key(),
        pool: loan.pool,
        amount: 0, // Amount hidden for privacy
        timestamp: clock.unix_timestamp,
    });


    msg!(" Loan approved successfully (Phase 1)");
    msg!("Loan ID: {}", loan_id);
    msg!("Next step: Borrower calls disburse_loan to lock actual amount");
    
    Ok(())
}