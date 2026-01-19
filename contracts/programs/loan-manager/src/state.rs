use anchor_lang::prelude::*;

/// Frontend encrypts with Arcium SDK, we store the ciphertext
// #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
// pub struct  EncryptedAmount{
//     pub ciphertext : [u8;32],

//     pub metadata : [u8;32]
// }

// impl  EncryptedAmount {
//     pub fn new(ciphertext : [u8;32],metadata : [u8;32]) ->Self{
//         Self { ciphertext, metadata }
//     }

//     pub fn default()->Self{
//         Self { ciphertext: [0u8;32], metadata: [0u8;32] }
//     }
// }
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub struct PrivateAmount{
    pub commitment : [u8;32],


    pub nullifier : [u8;32],

    pub amount_range_proof : Option<[u8;64]>
}


impl PrivateAmount {
    pub fn new(commitment: [u8; 32], nullifier: [u8; 32]) -> Self {
        Self {
            commitment,
            nullifier,
            amount_range_proof: None,
        }
    }
    
    pub fn default() -> Self {
        Self {
            commitment: [0u8; 32],
            nullifier: [0u8; 32],
            amount_range_proof: None,
        }
    }
}

#[account]

pub struct Loan {
    pub loan_id : u64,

    pub borrower : Pubkey,

    pub lender : Pubkey,

    pub pool : Pubkey,
    // Loan actual amount (encrypted)
    pub principal_commitment: PrivateAmount,
    pub collateral_commitment: PrivateAmount,
    pub repaid_commitment: PrivateAmount,

    pub principal_range: (u64, u64),

    pub collateral_mint : Pubkey,

    pub interest_rate : u16,
    // loan Duration 
    pub duration : i64,

    pub status : LoanStatus,

    pub start_time : i64,

    pub end_time : i64,
    // Total repaid till now 

    pub merkel_root : [u8;32],

    pub used_nullifier : Vec<[u8;32]>,
    // // interest earned between the last payment date and the current date
    // pub interest_accured : u64,

    pub amount_hash: [u8; 32],
    pub collateral_hash: [u8; 32],
       /// Health factor (100 = 1.0x, 200 = 2.0x)
    pub health_factor : u16, //check because borrower put collateral in different token instead of stable coin 
    // last health check time stamp
    pub last_health_check : i64,

    pub collateral_value_usd : u64,
    // if change in health factore means price of collateral is increased or decreased in that case warning_sent is used 
    pub warning_sent : bool,

    pub credit_score : u16,

    pub required_collateral_ratio : u16,

    pub bump : u8


}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum LoanStatus {
    Pending,

    Approved,

    Active,

    PartiallyRepaid,

    Disputed,

    Liquidated
}

impl Loan {
    pub const LEN: usize = 8 + // discriminator
    8 + // loan_id
    32 + // borrower
    32 + // lender
    32 + // pool
    64 + // principal (EncryptedAmount = 64 bytes)
    64 + // collateral_amount
    64 + // total_repaid
    32 + // collateral_mint
    2 + // interest_rate
    8 + // duration
    2 + // status
    8 + // start_time
    8 + // end_time
    32 + // merkle_root
    (4 + 32 * 10) + // used_nullifiers (max 10)
    32 + // amount_hash
    32 + // collateral_hash
    2 + // health_factor
    8 + // last_health_check
    8 + // collateral_value_usd
    1 + // warning_sent
    2 + // credit_score
    2 + // required_collateral_ratio
    1; // bump

}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]


// ok so this is used when proof in generated using clinet side before getting credit 
pub struct  NoirCreditProof{

    pub proof : Vec<u8>, //mathematical proof 

    pub public_inputs  :  Vec<u8>, // inpu of the score like 800

    pub score_commitment  : [u8;32],  //hidden score hash(800 + secret)

    pub timestamp  : i64
}

#[account]

pub struct LoanCounter{
    pub next_loan_id : u64,

    pub total_loans : u64,

    pub active_loans : u64,

    pub bump : u8 
}

impl LoanCounter{
    pub const  LEN : usize = 8+ //discriminator
        8+
        8+
        8+
        1;
}

impl Loan {
    pub const SEED_PREFIX : &'static [u8] = b"loan";
}

impl LoanCounter{
    pub const  SEED_PREFIX : &'static [u8] = b"loan_counter";
}
