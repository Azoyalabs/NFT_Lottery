use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Never")]
    Never {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Cannot set to own account")]
    CannotSetOwnAccount {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("Allowance is expired")]
    Expired {},

    // payable
    #[error("Payable Contract")]
    RequiresFunds {},

    #[error("Not Payable Contract")]
    NotRequiresFunds {},

    #[error("Single Currency Accepted")]
    SingleCurrencyPayable {},

    #[error("Funds amount invalid")]
    InvalidFundsAmount {},

    #[error("Already Registered")]
    AlreadyRegistered {},

    #[error("Registrations Closed")]
    RegistrationsClosed {},

    #[error("Contract is Inactive")]
    ContractInactive {},


    // new version
    #[error("Lottery needs at least one admin")]
    LotteryRequiresAdmin {},
}
