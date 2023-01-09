use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {
    #[msg("Invalid Player Pool Owner")]
    InvalidPlayerPool,
    #[msg("Invalid Admin to Withdraw")]
    InvalidAdmin,
    #[msg("Invalid Claim to Withdraw Reward")]
    InvalidClaim,
    #[msg("Invalid Reward Vault to receive")]
    InvalidRewardVault,
    #[msg("Insufficient Reward SOL Balance")]
    InsufficientRewardVault,
    #[msg("Insufficient User SOL Balance")]
    InsufficientUserBalance,
    #[msg("Invalid Deposit Amount")]
    InvalidDeposit,
}