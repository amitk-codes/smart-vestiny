use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("98vJxJanGrYXsw22T8ERBweXvPWLEQwsxbpXAjiFdeKf");

#[program]
pub mod smart_vestiny {

    use super::*;

    pub fn create_vesting_account(
        ctx: Context<CreateVestingAccount>,
        company_name: String,
    ) -> Result<()> {
        create_vesting_account_handler(ctx, company_name)?;
        Ok(())
    }

    pub fn create_employee_account(
        ctx: Context<CreateEmployeeAccount>,
        total_amount: u64,
        start_time: i64,
        end_time: i64,
        cliff_time: i64,
    ) -> Result<()> {
        create_employee_account_handler(ctx, total_amount, start_time, end_time, cliff_time)?;
        Ok(())
    }

    pub fn claim_tokens(ctx: Context<ClaimTokens>, company_name: String) -> Result<()> {
        claim_tokens_handler(ctx, company_name)?;
        Ok(())
    }
}
