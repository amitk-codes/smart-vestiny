use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{VestingAccount, ANCHOR_DISCRIMINATOR};

pub fn create_vesting_account_handler(
    ctx: Context<CreateVestingAccount>,
    company_name: String,
) -> Result<()> {
    *ctx.accounts.vesting_account = VestingAccount {
        owner: ctx.accounts.owner.key(),
        company_name,
        mint: ctx.accounts.mint.key(),
        treasury_token_account: ctx.accounts.treasury_token_account.key(),
        treasury_bump: ctx.bumps.treasury_token_account,
        bump: ctx.bumps.vesting_account,
    };
    Ok(())
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
