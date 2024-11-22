use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VestingAccount {
    pub owner: Pubkey,

    #[max_len(50)]
    pub company_name: String,
    pub mint: Pubkey,
    pub treasury_token_account: Pubkey,
    pub treasury_bump: u8,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct EmployeeAccount {
    pub beneficiary: Pubkey,
    pub total_amount: u64,
    pub total_withdrawal_account: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub cliff_time: i64, // the period after which the employee can withdraw the tokens
    pub vesting_account: Pubkey, // storing the vesting_account as well to have an reference
    pub bump: u8,
}
