use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;

use crate::state::Auction;

pub type Coins = Vec<cosmwasm_std::Coin>;

#[cw_serde]
#[derive(QueryResponses)]
//We will receive all auctions that are still open/unclaimed
pub enum QueryMsg {
    #[returns(OpenAuctionsResp)]
    OpenAuctions {},
}

#[cw_serde]
pub struct OpenAuctionsResp {
    pub auctions: Vec<Auction>,
}

#[cw_serde]
pub struct InstantiateMsg {}