use cosmwasm_std::{Addr, DepsMut, Response, StdResult};
use cw2::set_contract_version;

use crate::state::{AUCTIONS, OWNER};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(deps: DepsMut, sender: Addr) -> StdResult<Response> {
    //Set name and version of auction house contract
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    deps.api.addr_validate(&sender.clone().into_string())?;

    let mut owners = OWNER.load(deps.storage)?;

    owners.push(sender.clone());
    //The instantiation of this contract will also be the initial owner of it.
    OWNER.save(deps.storage, &owners)?;

    AUCTIONS.save(deps.storage, &Vec::new())?;

    let resp = Response::new()
        .add_attribute("action", "Instantiating Action House")
        .add_attribute("Owner", sender);

    Ok(resp)
}

pub mod query {
    use archway_bindings::{types::rewards::{ContractMetadataResponse, RewardsRecordsResponse}, ArchwayQuery, PageRequest};
    use cosmwasm_std::{Deps, Env, StdResult};
    use cw_utils::NativeBalance;

    use crate::{msg::{OpenAuctionsResp, OutstandingRewardsResponse}, state::AUCTIONS};

    //We return the current auctions that are still open and/or unclaimed.
    pub fn open_auctions(deps: Deps<ArchwayQuery>) -> StdResult<OpenAuctionsResp> {
        let auctions = AUCTIONS.load(deps.storage)?;

        Ok(OpenAuctionsResp { auctions })
    }

    pub fn contract_metadata(
        deps: Deps<ArchwayQuery>,
        env: Env,
    ) -> StdResult<ContractMetadataResponse> {
        let req = ArchwayQuery::contract_metadata(env.contract.address).into();
        deps.querier.query(&req)
    }

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
    use archway_bindings::{ArchwayMsg, ArchwayResult, ArchwayQuery};
    use cosmwasm_std::{Addr, Response, DepsMut};

    use crate::{error::ContractError, state::OWNER};


    pub fn update_rewards_address(deps: DepsMut<ArchwayQuery>, sender: Addr, rewards_address: Addr) -> ArchwayResult<ContractError> {

        deps.api.addr_validate(&sender.clone().into_string())?;

        let owners = OWNER.load(deps.storage)?;

        if !owners.contains(&sender){
            return Err(ContractError::Unauthorized)
        }

        let msg = ArchwayMsg::update_rewards_address(rewards_address);
    
        let res = Response::new()
            .add_message(msg)
            .add_attribute("method", "update_rewards_address");
    
        Ok(res)
    }

    pub fn withdraw_rewards(deps: DepsMut<ArchwayQuery>, sender: Addr) -> ArchwayResult<ContractError> {

        deps.api.addr_validate(&sender.clone().into_string())?;

        let owners = OWNER.load(deps.storage)?;

        if !owners.contains(&sender){
            return Err(ContractError::Unauthorized)
        }

        let msg = ArchwayMsg::withdraw_rewards_by_limit(0);
    
        let res = Response::new()
            .add_message(msg)
            .add_attribute("method", "withdraw_rewards");
    
        Ok(res)
    }

    
}
