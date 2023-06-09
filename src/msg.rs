use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Addr;

use crate::state::Auction;

pub type Coins = Vec<cosmwasm_std::Coin>;

#[cw_serde]
#[derive(QueryResponses)]
//We will receive all auctions that are still open/unclaimed
pub enum QueryMsg {
    #[returns(OpenAuctionsResp)]
    OpenAuctions {},
    #[returns(OutstandingRewardsResponse)]
    OutstandingRewards {},
    #[returns(archway_bindings::types::rewards::ContractMetadataResponse)]
    Metadata {},
}

#[cw_serde]
pub struct OpenAuctionsResp {
    pub auctions: Vec<Auction>,
}

#[cw_serde]
pub struct OutstandingRewardsResponse {
    pub rewards_balance: Coins,
    pub total_records: u64,
}

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecMsg {
    WithdrawRewards {},
    UpdateRewardsAddress {
        address: Option<Addr>,
    },
    AddOwner {
        new_owner: Addr,
    },
    RemoveOwner {
        old_owner: Addr,
    },
    CreateAuction {
        nft_id: String,
        nft_contract: String,
        min_bid: u64,
        buyout: u64,
        denom: String,
    },
    Bid {
        nft_id: String,
    },
    Buyout {
        nft_id: String,
    },
    Close {
        nft_id: String,
    }
}
