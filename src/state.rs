use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, Timestamp};

//Auction structure
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Auction {
    pub nft: String,
    pub current_bid: Coin,
    pub buyout_price: Coin,
    pub owner: Addr,
    pub end_auction: Timestamp,
}

//Contract owner that will receive rewards from Archway inflation module when they are withdrawn from this one.
//When platform is live this will be a proxy contract address that will have a method for all rewards receivers to claim their proportional rewards.

pub const OWNER: Item<Vec<Addr>> = Item::new("owner");

//Current auctions that are open and/or unclaimed

pub const AUCTIONS: Item<Vec<Auction>> = Item::new("open_auctions");
