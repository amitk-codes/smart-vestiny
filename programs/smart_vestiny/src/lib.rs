use anchor_lang::prelude::*;

declare_id!("98vJxJanGrYXsw22T8ERBweXvPWLEQwsxbpXAjiFdeKf");

#[program]
pub mod smart_vestiny {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
