use anchor_lang::{accounts, prelude::*, };
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{errors::LoanManagerError, events::{CollateralAdded, LoanDisbursed}, instructions::{LenderAccount, LendingPool}, state::{Loan, LoanStatus}, utils::{verify_nullifier_unused, verify_privacy_commitment}};


#[derive(Accounts)]
#[instruction(
    loan_id: u64,
    actual_principal: u64,
    actual_collateral: u64,
    principal_secret: [u8; 32],
    collateral_secret: [u8; 32],
)]
pub struct DisburseLoan<'info>{

    #[account(mut)]
    pub borrower : Signer<'info>,


    #[account(
        mut,
        seeds = [Loan::SEED_PREFIX, loan_id.to_le_bytes().as_ref()],
        bump = loan.bump,
        constraint = loan.borrower == borrower.key() @ LoanManagerError::UnauthorizedBorrower,
        constraint = loan.status == LoanStatus::Approved @ LoanManagerError::LoanNotApproved,
    )]
    pub loan: Account<'info, Loan>,
    
    #[account(
        mut, 
        constraint = pool.key()==loan.pool 
    )]
    pub pool : Account<'info,LendingPool>,

    #[account(
        mut ,
        constraint  = lender_account.lender == loan.lender,
        constraint = lender_account.pool == pool.key()
    )]
    pub lender_account : Account<'info,LenderAccount>,

    #[account(
        mut,
        constraint = borrower_collateral_account.owner == borrower.key(),
        constraint = borrower_collateral_account.mint == loan.collateral_mint
    )]
    pub borrower_collateral_account : Account<'info,TokenAccount>,

    #[account(
        mut,
        constraint = collateral_vault.mint == loan.collateral_mint,
    )]
    pub collateral_vault: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = borrower_token_account.owner == borrower.key(),
        constraint = borrower_collateral_account.mint == pool.token_mint
    )]
    pub borrower_token_account : Account<'info,TokenAccount>,

    #[account(
        mut,
        constraint = pool_token_account.key() == pool.pool_token_account
    )]
    pub pool_token_account : Account<'info,TokenAccount>,

    pub token_program : Program<'info,Token>
}


 pub fn handler(
    ctx:Context<DisburseLoan>,
    loan_id : u64,
    actual_principal : u64,
    actual_collateral : u64,

    principal_secret : [u8;32],
    collateral_secret : [u8;32]
)->Result<()>{

    let loan = &mut ctx.accounts.loan;
    let pool = &mut ctx.accounts.pool;
    let lender_account = &mut ctx.accounts.lender_account;
    let clock = Clock::get()?;

    msg!("Loan Disburse is started");
    // validate loan status 

    require!(loan.status == LoanStatus::Approved,LoanManagerError::InvalidLoanStatus);

    // validate borrower

    require!(loan.borrower == ctx.accounts.borrower.key(),LoanManagerError::UnauthorizedBorrower);

    msg!(" Verifying privacy commitments..."); 

    // Verify principal Commutment 

    // let principal_valid = verify_privacy_commitment(&loan.principal_commitment.commitment, actual_principal, &principal_secret)?;
    let collateral_valid = verify_privacy_commitment(
        &loan.collateral_commitment.commitment,
        &collateral_secret,
        actual_collateral,
    )?;
    

    require!(collateral_valid, LoanManagerError::InvalidCommitment);
    msg!("Collateral commitment verified");
    msg!("Actual collateral: {}", actual_collateral);
    
    // Verify amounts are within declared ranges
    require!(
        actual_principal >= loan.principal_range.0 
        && actual_principal <= loan.principal_range.1,
        LoanManagerError::InvalidCommitment
    );

    verify_nullifier_unused(&loan.principal_commitment.nullifier, &loan.used_nullifier)?;

    verify_nullifier_unused(&loan.collateral_commitment.nullifier, &loan.used_nullifier)?;

    msg!(" Nullifiers verified (no double-spend)");
    
    // CRITICAL: NOW lock the ACTUAL amount (not estimate!)
    msg!(" Locking actual principal amount: {}", actual_principal);
    
    // Check pool has sufficient liquidity

    require!(pool.available_liquidity >= actual_principal,LoanManagerError::InsufficientPoolLiquidity);

    require!(lender_account.available_amount >=actual_principal,LoanManagerError::InsufficientPoolLiquidity);


    // reduce pool liquidity 

    pool.available_liquidity = pool.available_liquidity
                                .checked_sub(actual_principal)
                                .ok_or(error!(LoanManagerError::InsufficientPoolLiquidity))?;

    
    //  Increase pool total borrow 

    pool.total_borrowed = pool.total_borrowed
                          .checked_add(actual_principal)
                          .ok_or(error!(LoanManagerError::ArithmeticOverflow))?;
    // added loan count +1

    pool.active_loan_count = pool
    .active_loan_count
    .checked_add(1)
    .ok_or(error!(LoanManagerError::ArithmeticOverflow))?;

    msg!("Pool updated with ACTUAL amount");
    msg!("  Locked: {}", actual_principal);
    msg!("  New available: {}", pool.available_liquidity);
    msg!("  Total borrowed: {}", pool.total_borrowed);
    

    // decrease lender account avaiblae balance 
    lender_account.available_amount = lender_account
    .available_amount
    .checked_sub(actual_principal)
    .ok_or(error!(LoanManagerError::ArithmeticUnderflow))?;
    // increase lender accoutn lent amount 

    lender_account.lent_amount = lender_account
    .lent_amount
    .checked_add(actual_principal)
    .ok_or(error!(LoanManagerError::ArithmeticOverflow))?;

    msg!(" Lender account updated with ACTUAL amount");
    msg!("  Lender available: {}", lender_account.available_amount);
    msg!("  Lender lent: {}", lender_account.lent_amount);
    
    //  STEP 1: Lock collateral in vault
    msg!("Locking collateral...");

    // Transfer collateral from Borrower collateral accoutn to collateral vault 

    let cpi_accounts = Transfer {
        from: ctx.accounts.borrower_collateral_account.to_account_info(),
        to: ctx.accounts.collateral_vault.to_account_info(),
        authority: ctx.accounts.borrower.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    
    token::transfer(cpi_ctx, actual_collateral)?;
    
    msg!(" Collateral locked: {} tokens", actual_collateral);

    let collateral_nullifier  = loan.collateral_commitment.nullifier;
    loan.used_nullifier.push(collateral_nullifier);
    // transfer fund Loan fund from pool to borrower 

    msg!("Transfering Fund from pool to borrower");

    let pool_key = pool.key();

    let seeds = &[
                    b"lending_pool".as_ref(),
                    pool_key.as_ref(),
                    &[ctx.accounts.pool.bump]
    ]; 

    let signer_seeds = &[& seeds[..]];


    let cpi_account = Transfer{
        from : ctx.accounts.pool_token_account.to_account_info(),
        to : ctx.accounts.borrower_token_account.to_account_info(),
        authority : ctx.accounts.pool.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),
                 cpi_account,
                 signer_seeds);

    token::transfer(cpi_ctx, actual_principal)?;

    msg!("Funds transferred: {} tokens", actual_principal);

    let principal_nullifier = loan.principal_commitment.nullifier;

    loan.used_nullifier.push(principal_nullifier);

    // change loan status to active 

    loan.status = LoanStatus::Active;
    loan.start_time = clock.unix_timestamp;
    loan.end_time = clock.unix_timestamp * loan.duration;

    emit!(LoanDisbursed{
        loan_id : loan_id,
        borrower : loan.borrower,
        lender: loan.lender,
        amount : actual_principal,
        collateral_locked : actual_collateral,
        timestamp: clock.unix_timestamp
    });
    Ok(())
}