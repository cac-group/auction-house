use cosmwasm_std::{Addr, DepsMut, Response, StdResult};
use cw2::set_contract_version;

use crate::state::{AUCTIONS, OWNERS};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(deps: DepsMut, sender: Addr) -> StdResult<Response> {
    //Set name and version of auction house contract
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    deps.api.addr_validate(&sender.clone().into_string())?;

    let mut owners = OWNERS.load(deps.storage)?;

    owners.push(sender.clone());
    //The instantiation of this contract will also be the initial owner of it.
    OWNERS.save(deps.storage, &owners)?;

    AUCTIONS.save(deps.storage, &Vec::new())?;

    let resp = Response::new()
        .add_attribute("action", "Instantiating Action House")
        .add_attribute("Owner", sender);

    Ok(resp)
}

pub mod query {
    use archway_bindings::{
        types::rewards::{ContractMetadataResponse, RewardsRecordsResponse},
        ArchwayQuery, PageRequest,
    };
    use cosmwasm_std::{Deps, Env, StdResult};
    use cw_utils::NativeBalance;

    use crate::{
        msg::{OpenAuctionsResp, OutstandingRewardsResponse},
        state::AUCTIONS,
    };

    //We return the current auctions that are still open and/or unclaimed.
    pub fn open_auctions(deps: Deps<ArchwayQuery>) -> StdResult<OpenAuctionsResp> {
        let auctions = AUCTIONS.load(deps.storage)?;

        Ok(OpenAuctionsResp { auctions })
    }

    //We get the owner address and rewards address
    pub fn contract_metadata(
        deps: Deps<ArchwayQuery>,
        env: Env,
    ) -> StdResult<ContractMetadataResponse> {
        let req = ArchwayQuery::contract_metadata(env.contract.address).into();
        deps.querier.query(&req)
    }

    //Check unclaimed rewards
    pub fn outstanding_rewards(
        deps: Deps<ArchwayQuery>,
        env: Env,
    ) -> StdResult<OutstandingRewardsResponse> {
        let rewards_address = env.contract.address;
        let req = ArchwayQuery::rewards_records_with_pagination(
            rewards_address,
            PageRequest::new().count_total(),
        )
        .into();

        let response: RewardsRecordsResponse = deps.querier.query(&req)?;
        let rewards_coins = response
            .records
            .iter()
            .flat_map(|r| r.rewards.iter().cloned())
            .collect();
        let mut rewards_balance = NativeBalance(rewards_coins);
        rewards_balance.normalize();

        let total_records = response.pagination.and_then(|p| p.total).unwrap_or(0);

        Ok(OutstandingRewardsResponse {
            rewards_balance: rewards_balance.into_vec(),
            total_records,
        })
    }
}

pub mod exec {
    use archway_bindings::{ArchwayMsg, ArchwayQuery, ArchwayResult};
    use cosmwasm_std::{coin, Addr, BankMsg, Coin, DepsMut, Response, Timestamp};

    use crate::{
        error::ContractError,
        state::{Auction, AUCTIONS, OWNERS},
    };

    //Any of the owners an modify where the rewards accumulated by the contract will be sent to when they are withdrawn.
    pub fn update_rewards_address(
        deps: DepsMut<ArchwayQuery>,
        sender: Addr,
        rewards_address: Addr,
    ) -> ArchwayResult<ContractError> {
        deps.api.addr_validate(&sender.clone().into_string())?;

        let owners = OWNERS.load(deps.storage)?;

        if !owners.contains(&sender) {
            return Err(ContractError::Unauthorized);
        }

        let msg = ArchwayMsg::update_rewards_address(rewards_address);

        let res = Response::new()
            .add_message(msg)
            .add_attribute("method", "update_rewards_address");

        Ok(res)
    }

    //Any of the owners can withdraw the rewards to the reward address set up (This can be a wallet or ideally a contract that distributes rewards accordingly
    //if there are multiple rewards receivers (optionally using a ratio)).
    pub fn withdraw_rewards(
        deps: DepsMut<ArchwayQuery>,
        sender: Addr,
    ) -> ArchwayResult<ContractError> {
        deps.api.addr_validate(&sender.clone().into_string())?;

        let owners = OWNERS.load(deps.storage)?;

        if !owners.contains(&sender) {
            return Err(ContractError::Unauthorized);
        }

        let msg = ArchwayMsg::withdraw_rewards_by_limit(0);

        let res = Response::new()
            .add_message(msg)
            .add_attribute("method", "withdraw_rewards");

        Ok(res)
    }

    //Any owner can add another owner that will have permissions to modify the contract metadata and be able to withdraw rewards.
    pub fn add_owner(
        deps: DepsMut<ArchwayQuery>,
        sender: Addr,
        new_owner: Addr,
    ) -> ArchwayResult<ContractError> {
        deps.api.addr_validate(&sender.clone().into_string())?;
        deps.api.addr_validate(&new_owner.clone().into_string())?;

        let mut owners = OWNERS.load(deps.storage)?;

        if !owners.contains(&sender) {
            return Err(ContractError::Unauthorized);
        }

        if !owners.contains(&new_owner.clone()) {
            owners.push(new_owner)
        }

        OWNERS.save(deps.storage, &owners)?;

        let res = Response::new().add_attribute("method", "add_owner");

        Ok(res)
    }

    //Any owner can remove another owner to withdraw his permissions as long as he is not the last owner.
    pub fn remove_owner(
        deps: DepsMut<ArchwayQuery>,
        sender: Addr,
        old_owner: Addr,
    ) -> ArchwayResult<ContractError> {
        deps.api.addr_validate(&sender.clone().into_string())?;
        deps.api.addr_validate(&old_owner.clone().into_string())?;

        let mut owners = OWNERS.load(deps.storage)?;

        if !owners.contains(&sender) {
            return Err(ContractError::Unauthorized);
        }

        owners.retain(|value| value.to_string() != old_owner.to_string());

        if owners.is_empty() {
            return Err(ContractError::NoOwner);
        }

        OWNERS.save(deps.storage, &owners)?;

        let res = Response::new().add_attribute("method", "remove_owner");

        Ok(res)
    }

    pub fn create_auction(
        deps: DepsMut<ArchwayQuery>,
        sender: Addr,
        blocktime: u64,
        nft: String,
        min_bid: u64,
        buyout: u64,
        denom: String,
    ) -> ArchwayResult<ContractError> {
        //TODO: Check if contract has the NFT that was sent before, we can't create an auction of the

        let mut auctions = AUCTIONS.load(deps.storage)?;

        if auctions.iter().any(|auction| auction.nft == nft) {
            return Err(ContractError::AuctionExists);
        }

        //TODO (FOR PRODUCTION): Make a list of allowed denoms to create auctions

        let three_days = Timestamp::from_seconds(72 * 60 * 60);
        //We create an auction with a default time limit of 72h (In the future we will make this time modifiable)
        let new_auction = Auction {
            nft,
            current_bid: None,
            current_bidder: None,
            min_bid: coin(min_bid.into(), denom.clone()),
            buyout_price: coin(buyout.into(), denom),
            owner: sender,
            end_auction: three_days.plus_seconds(blocktime),
        };

        //Store the new auction in the contract state
        auctions.push(new_auction);

        AUCTIONS.save(deps.storage, &auctions)?;

        let res = Response::new().add_attribute("method", "create_auction");

        Ok(res)
    }

    pub fn bid(
        deps: DepsMut<ArchwayQuery>,
        sender: Addr,
        funds: Vec<Coin>,
        nft: String,
    ) -> ArchwayResult<ContractError> {
        let mut auctions = AUCTIONS.load(deps.storage)?;

        //We check if the auction we want to bid on exists (get the position in the auction array)
        let auction_position = auctions.iter().position(|auction| auction.nft == nft);

        if auction_position.is_none() {
            return Err(ContractError::NoAuction);
        }

        let auction_denom = auctions[auction_position.unwrap()].min_bid.clone().denom;
        //We check if the bidder sent the funds wanted by the auction creator (and that they correspond to the right denom)
        if funds
            .iter()
            .find(|coin| coin.denom == auctions[auction_position.unwrap()].min_bid.denom)
            == None
        {
            return Err(ContractError::NoFunds);
        }

        let new_bid_amount = funds
            .iter()
            .find(|coin| coin.denom == auction_denom)
            .unwrap()
            .amount
            .u128();

        if new_bid_amount < auctions[auction_position.unwrap()].min_bid.amount.into() {
            return Err(ContractError::BidUnderMinimum);
        }

        //We check if there is already a bidder and if our bid is higher than his. If that's the case, we update the current bidder with the new one
        //and return the funds to the old bidder.
        let resp;

        if auctions[auction_position.unwrap()].current_bidder.is_some() {
            //If the new bid is lower than current bid then we throw an error.
            if new_bid_amount
                <= auctions[auction_position.unwrap()]
                    .current_bid
                    .clone()
                    .unwrap()
                    .amount
                    .into()
            {
                return Err(ContractError::BidNotEnough);
            }

            //We create the return funds message of previous bidder.
            let return_funds_msg = BankMsg::Send {
                to_address: auctions[auction_position.unwrap()]
                    .current_bidder
                    .clone()
                    .unwrap()
                    .into_string(),
                amount: vec![auctions[auction_position.unwrap()]
                    .current_bid
                    .clone()
                    .unwrap()],
            };

            resp = Response::new()
                .add_message(return_funds_msg)
                .add_attribute("method", "bid_with_refund")
                .add_attribute("new_bidder", sender.clone())
                .add_attribute("old_bidder", auctions[auction_position.unwrap()].current_bidder.clone().unwrap());
            
        } else {
            resp = Response::new()
                .add_attribute("method", "bid")
                .add_attribute("bidder", sender.clone());
        }

        //We update the new current highest offer in the contract state.

        auctions[auction_position.unwrap()].current_bidder = Some(sender);
        auctions[auction_position.unwrap()].current_bid = Some(coin(new_bid_amount, auction_denom));

        AUCTIONS.save(deps.storage, &auctions)?;

        Ok(resp)
    }

    pub fn buyout(
        deps: DepsMut<ArchwayQuery>,
        sender: Addr,
        funds: Vec<Coin>,
        nft: String,
    ) -> ArchwayResult<ContractError> {
        let mut auctions = AUCTIONS.load(deps.storage)?;

        //We check if the auction we want to buyout exists (get the position in the auction array)
        let auction_position = auctions.iter().position(|auction| auction.nft == nft);

        if auction_position.is_none() {
            return Err(ContractError::NoAuction);
        }

        let auction_denom = auctions[auction_position.unwrap()].min_bid.clone().denom;
        //We check if the buyer sent the funds wanted by the auction creator (and that they correspond to the right denom)
        if funds
            .iter()
            .find(|coin| coin.denom == auctions[auction_position.unwrap()].min_bid.denom)
            == None
        {
            return Err(ContractError::NoFunds);
        }

        let buyout_amount = funds
            .iter()
            .find(|coin| coin.denom == auction_denom)
            .unwrap()
            .amount
            .u128();

        if buyout_amount < auctions[auction_position.unwrap()].buyout_price.amount.into() {
            return Err(ContractError::PriceNotMet);
        }

        //We check if there is already a bidder. If that's the case, we send his funds back because he lost the auction.
        let resp;

        if auctions[auction_position.unwrap()].current_bidder.is_some() {
            //We create the return funds message of previous bidder.
            let return_funds_msg = BankMsg::Send {
                to_address: auctions[auction_position.unwrap()]
                    .current_bidder
                    .clone()
                    .unwrap()
                    .into_string(),
                amount: vec![auctions[auction_position.unwrap()]
                    .current_bid
                    .clone()
                    .unwrap()],
            };

            resp = Response::new()
                .add_message(return_funds_msg)
                .add_attribute("method", "buyout_with_refund")
                .add_attribute("buyer", sender.clone())
                .add_attribute("old_bidder", auctions[auction_position.unwrap()].current_bidder.clone().unwrap());
            
        } else {
            resp = Response::new()
                .add_attribute("method", "buyout")
                .add_attribute("buyer", sender.clone());
        }

        //We remove the auction from the auction from the auctions array

        auctions.retain(|auction| auction.nft != nft);

        AUCTIONS.save(deps.storage, &auctions)?;

        //TODO: SEND NFT TO BUYER.

        Ok(resp)
    }
}
