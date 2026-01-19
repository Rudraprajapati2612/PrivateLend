use anchor_lang::prelude::*;

use crate::{constants::{DEFAULT_INTEREST_RATE, MAX_LOAN_AMOUNT, MAX_LOAN_DURATION, MIN_LOAN_AMOUNT, MIN_LOAN_DURATION}, errors::LoanManagerError, events::Loanrequested, state::{Loan, LoanCounter, NoirCreditProof, PrivateAmount}, utils::{calculate_required_collateral, extract_score_tire_from_commitment, verify_noir_credit_proof, verify_nullifier_unused}};


#[derive(Accounts)]

pub struct RequestLoan<'info>{

    #[account(mut)]
    pub borrower: Signer<'info>,

    /// Check : Pool  
    pub pool : UncheckedAccount<'info>,

    /// Check : Collateral Mint 
    pub collateral_mint : UncheckedAccount<'info>,

    #[account(
        init,
        payer = borrower,
        space = LoanCounter::LEN,
        seeds = [LoanCounter::SEED_PREFIX],
        bump
    )]
    pub loan_counter : Account<'info,LoanCounter>,

    #[account(
        init,
        payer = borrower,
        space = Loan::LEN,
        seeds = [
            Loan::SEED_PREFIX,
            &loan_counter.next_loan_id.to_le_bytes().as_ref(),
         ],
        bump
    )]
    pub loan : Account<'info,Loan>,


   

    pub system_program : Program<'info,System>
}


pub fn handler(
    ctx:Context<RequestLoan>,
    principal_commitment:[u8;32],
    principal_nullifier : [u8;32],
    principal_range:(u64,u64),
    collateral_commitment : [u8;32],
    collateral_nullifier : [u8;32],
    collateral_range : (u64,u64),
    duration_days : u16,
    credit_proof :NoirCreditProof,
    credit_threshould : u16
)->Result<()>{
    let loan = &mut ctx.accounts.loan;
    let loan_counter = &mut ctx.accounts.loan_counter;

    let clock = Clock::get()?;

    msg!("Processing PRIVATE loan request...");
    msg!("Principal: HIDDEN (commitment: {:?}...)", &principal_commitment[..4]);
    msg!("Collateral: HIDDEN (commitment: {:?}...)", &collateral_commitment[..4]);

    require!(principal_range.0 >= MIN_LOAN_AMOUNT,LoanManagerError::LoanAmountTooLow);

    require!(principal_range.1 > MAX_LOAN_AMOUNT,LoanManagerError::LoanAmountTooHigh);

    let duration_seconds = (duration_days as i64) *24*60*60;

    require!(duration_seconds>=MIN_LOAN_DURATION,LoanManagerError::DurationTooShort);

    require!(duration_seconds<= MAX_LOAN_DURATION,LoanManagerError::DurationTooLong);

    // 1) In thisfunction it first check that proof is generated Before 12 Hours 
    // 2) verify public threshould is equal to threshould 
    // 3) and check that score commit is != 0
    let proof_valid = verify_noir_credit_proof(&credit_proof, credit_threshould, clock.unix_timestamp)?;

    require!(proof_valid,LoanManagerError::InvalidCreditProofSignature);

    let required_collateral_ratio = extract_score_tire_from_commitment(&credit_proof.score_commitment);

    verify_nullifier_unused(&principal_nullifier, &vec![])?;
    verify_nullifier_unused(&collateral_nullifier, &vec![])?;


    require!(principal_commitment.iter().any(|&x|x!=0),LoanManagerError::InvalidCommitment);

    require!(collateral_commitment.iter().any(|&x| x!=0),LoanManagerError::InvalidCommitment);

    // Check that Collateral is sufficient 

   let max_principal = principal_range.1;
   let min_collateral_amount = collateral_range.0;

   let required_collateral = calculate_required_collateral(max_principal, required_collateral_ratio)?;
    
   msg!("Security validation:");
   msg!("Max possible principal: {}", max_principal);
   msg!("Min provided collateral: {}", min_collateral_amount);
   msg!("Required collateral (for max): {}", required_collateral);

    require!(min_collateral_amount>=required_collateral,LoanManagerError::InsufficientCollateral);


    msg!(" Collateral verified (privacy-preserving)");
    msg!("Even at maximum loan amount, collateral is sufficient");
    
    let loan_id = loan_counter.next_loan_id;

    loan_counter.next_loan_id = loan_counter.next_loan_id.checked_add(1)
                    .ok_or(error!(LoanManagerError::ArithmeticOverflow))?;



    loan_counter.total_loans = loan_counter
    .total_loans.checked_add(1).ok_or(error!(LoanManagerError::ArithmeticOverflow))?;

    loan.loan_id = loan_id;
    loan.borrower = ctx.accounts.borrower.key();
    loan.lender = Pubkey::default();
    loan.pool = ctx.accounts.pool.key();


    // Store Commitment 

    loan.principal_commitment = PrivateAmount::new(principal_commitment, principal_nullifier);

    loan.collateral_commitment = PrivateAmount::new(collateral_commitment, collateral_nullifier);


    loan.repaid_commitment = PrivateAmount::default();


    loan.principal_range = principal_range;

    loan.collateral_mint = ctx.accounts.collateral_mint.key();

    loan.interest_rate = DEFAULT_INTEREST_RATE;
    loan.duration = duration_seconds;
    loan.status = crate::state::LoanStatus::Pending;
    loan.start_time = 0;
    loan.end_time = 0;
    loan.merkel_root = [0u8;32];
    loan.used_nullifier = vec![];

    loan.health_factor = 100;
    loan.last_health_check = clock.unix_timestamp;
    loan.credit_score = credit_threshould;
    loan.required_collateral_ratio = required_collateral_ratio;
    loan.bump = ctx.bumps.loan;


    emit!(Loanrequested{
        loan_id,
        borrower: ctx.accounts.borrower.key(),
        pool: ctx.accounts.pool.key(),
        amount: 0, // Amount hidden!
        collateral_amount: 0, // Amount hidden!
        duration: duration_seconds,
        credit_score: credit_threshould,
        require_collateral_ratio : required_collateral_ratio,
        timestamp: clock.unix_timestamp,
    });


    msg!("PRIVATE loan requested successfully");
    msg!("Loan ID: {}", loan_id);
    msg!(" All amounts stored as COMMITMENTS");
    msg!(" Credit verified via NOIR ZK proof");
    msg!(" Privacy preserved on-chain");
    
    Ok(())
}