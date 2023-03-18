use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Not owner, unauthorized")]
    Unauthorized,

    #[error("Must have atleast 1 owner")]
    NoOwner,

    #[error("Auction already exists")]
    AuctionExists,
}
