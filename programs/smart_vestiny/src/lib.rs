use anchor_lang::prelude::*;

pub mod constants;
pub mod state;

use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
pub use constants::*;
pub use state::*;

declare_id!("98vJxJanGrYXsw22T8ERBweXvPWLEQwsxbpXAjiFdeKf");

#[program]
pub mod smart_vestiny {
    use super::*;

    pub fn create_vesting_account(
        ctx: Context<CreateVestingAccount>,
        _company_name: String,
    ) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct CreateVestingAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = ANCHOR_DISCRIMINATOR + VestingAccount::INIT_SPACE,
        seeds = [company_name.as_ref(), owner.key().as_ref()],
        bump,
    )]
    pub vesting_account: Account<'info, VestingAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = owner,
        token::mint = mint,
        token::authority = treasury_token_account,
        seeds = [b"treasury_token_account", company_name.as_bytes()],
        bump,
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Interface<'info, TokenInterface>,
}
