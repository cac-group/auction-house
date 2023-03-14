
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};

//Auction structure
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]

pub struct Auction {
    pub nft: String,
    pub current_bid: Coin,
    pub buyout_price: Coin,
    pub owner: Addr,
}

//Contract owner that will receive rewards from Archway inflation module when they are withdrawn from this one.
pub const OWNER: Item<Addr> = Item::new("owner");

//Current auctions that are open and/or unclaimed

pub const AUCTIONS: Item<Vec<Auction>> = Item::new("open_auctions");