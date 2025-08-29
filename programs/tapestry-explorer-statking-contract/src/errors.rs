use anchor_lang::prelude::*;

pub use TapestrySackingError::*;

#[error_code]
pub enum TapestrySackingError {
    #[msg("Contract is stopped")]
    ContractIsStopped,
    #[msg("reward token mint error")]
    DepositRewardTokenMintError,
    #[msg("Invester token mint error")]
    DepositInvesterTokenMintError,
    #[msg("Invester token owner error")]
    DepositInvesterTokenOwnerError,
    #[msg("Invester token Amount error")]
    DepositInvesterTokenAmountError,
    #[msg("You have no withdrawable amount")]
    YouHaveNoWithdrawableAmount,
    #[msg("Need more time to deposit.")]
    NeedMoreTimeToDeposit,
    #[msg("ValueTooSmall")]
    ValueTooSmall,
    #[msg("ValueTooLarge")]
    ValueTooLarge,
    #[msg("ValueInvalid")]
    ValueInvalid,
    #[msg("IncorrectConfigAccount")]
    IncorrectConfigAccount,
    #[msg("IncorrectAuthority")]
    IncorrectAuthority,
    #[msg("Overflow or underflow occured")]
    OverflowOrUnderflowOccurred,
    #[msg("Amount is invalid")]
    InvalidAmount,
    #[msg("Arithmetic Error")]
    ArithmeticError,
    #[msg("Invalid Parameter")]
    InvalidParameter,
    #[msg("Invalid Argument")]
    InvalidArgument,
}
