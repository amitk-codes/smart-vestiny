use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{CustomError, EmployeeAccount, VestingAccount};

pub fn claim_tokens_handler(ctx: Context<ClaimTokens>, _company_name: String) -> Result<()> {
    let employee_account = &mut ctx.accounts.employee_account;

    let now = Clock::get()?.unix_timestamp;
    if now < employee_account.cliff_time {
        return Err(CustomError::CliffTimeIsNotOver.into());
    }

    let time_since_start = now.saturating_sub(employee_account.start_time);
    let total_vesting_time = employee_account
        .end_time
        .saturating_sub(employee_account.start_time);

    if total_vesting_time == 0 {
        return Err(CustomError::InvalidVestingPeriod.into());
    }

    let vested_amount_till_now = if now >= employee_account.end_time {
        employee_account.total_amount
    } else {
        match employee_account
            .total_amount
            .checked_mul(time_since_start as u64)
        {
            Some(product) => product / total_vesting_time as u64,
            None => return Err(CustomError::CalculationError.into()),
        }
    };

    let claimable_amount =
        vested_amount_till_now.saturating_sub(employee_account.total_withdrawal_account);

    if claimable_amount == 0 {
        return Err(CustomError::AlreadyClaimedFully.into());
    };

    let vesting_accounts = TransferChecked {
        from: ctx.accounts.treasury_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.employee_token_account.to_account_info(),
        authority: ctx.accounts.treasury_token_account.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"treasury_token_account",
        ctx.accounts.vesting_account.company_name.as_ref(),
        &[ctx.accounts.vesting_account.treasury_bump],
    ]];

    let cpi_context = CpiContext::new(cpi_program, vesting_accounts).with_signer(signer_seeds);

    let decimals = ctx.accounts.mint.decimals;
    transfer_checked(cpi_context, claimable_amount, decimals)?;

    employee_account.total_withdrawal_account += claimable_amount;

    Ok(())
}

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub beneficiary: Signer<'info>,

    #[account(
        mut,
        seeds = [b"employee_account", beneficiary.key().as_ref(), vesting_account.key().as_ref()],
        bump = employee_account.bump,
        has_one = beneficiary,
        has_one = vesting_account,
    )]
    pub employee_account: Account<'info, EmployeeAccount>,

    #[account(
        mut,
        seeds = [company_name.as_ref(), vesting_account.owner.key().as_ref()],
        bump = vesting_account.bump,
        has_one = treasury_token_account,
        has_one = mint,
    )]
    pub vesting_account: Account<'info, VestingAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = beneficiary,
        associated_token::mint = mint,
        associated_token::authority = beneficiary,
        associated_token::token_program = token_program
    )]
    pub employee_token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
