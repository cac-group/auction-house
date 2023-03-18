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

    #[error("Auction does not exist")]
    NoAuction,

    #[error("Auction already exists")]
    AuctionExists,

    #[error("No bid funds sent")]
    NoFunds,

    #[error("Bid is lower than minimum bid")]
    BidUnderMinimum,

    #[error("Bid is lower than current bid")]
    BidNotEnough,

    #[error("Buyout price not met")]
    PriceNotMet,
}
