use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Tokens can't be claimed yet. The Cliff time is not over")]
    CliffTimeIsNotOver,

    #[msg("The vesting period is invalid for this employee account")]
    InvalidVestingPeriod,

    #[msg("Encountered calculation error")]
    CalculationError,

    #[msg("Already claimed the vested tokens during this period")]
    AlreadyClaimedFully,
}
