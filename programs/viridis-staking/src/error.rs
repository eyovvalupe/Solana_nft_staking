use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Tokens are not staked")]
    NotStaked,
    #[msg("No tokens to stake")]
    NoTokens,
    #[msg("Invalid stake period")]
    InvalidStakePeriod,
    #[msg("Stake period not met")]
    StakePeriodNotMet,
    #[msg("Calculation error")]
    CalculationError,
    #[msg("No stakes found")]
    NoStakes,
    #[msg("Invalid stake index")]
    InvalidStakeIndex,
    #[msg("Stake has already been destaked")]
    AlreadyDestaked,
}