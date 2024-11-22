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
