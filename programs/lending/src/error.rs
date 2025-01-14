use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds to withdraw")]
    InsufficientFunds,
    #[msg("Over Repaying Borrowed amount")]
    OverRepay,
}
