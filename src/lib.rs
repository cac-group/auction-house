use archway_bindings::{ArchwayQuery, ArchwayResult};
use contract::exec::{update_rewards_address, withdraw_rewards, add_owner, remove_owner, create_auction, bid};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use error::ContractError;
use msg::{ExecMsg, InstantiateMsg};

mod contract;
pub mod error;
pub mod msg;
mod state;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, info.sender)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ArchwayQuery>, env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use contract::query;
    use msg::QueryMsg::*;

    match msg {
        OpenAuctions {} => to_binary(&query::open_auctions(deps)?),
        Metadata {} => to_binary(&query::contract_metadata(deps, env)?),
        OutstandingRewards {} => to_binary(&query::outstanding_rewards(deps, env)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<ArchwayQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecMsg,
) -> ArchwayResult<ContractError> {
    match msg {
        ExecMsg::UpdateRewardsAddress { address } => {
            update_rewards_address(deps, info.sender, address.unwrap_or(env.contract.address))
        }
        ExecMsg::WithdrawRewards {} => withdraw_rewards(deps, info.sender),
        ExecMsg::AddOwner { new_owner } => {
            add_owner(deps, info.sender, new_owner)
        },
        ExecMsg::RemoveOwner { old_owner } => {
            remove_owner(deps, info.sender, old_owner)
        },
        ExecMsg::CreateAuction { nft, min_bid, buyout, denom } => {
            create_auction(deps, info.sender, env.block.time.seconds() ,nft, min_bid, buyout, denom)
        }
        ExecMsg::Bid { nft } => {
            bid(deps, info.sender, info.funds, nft)
        }
    }
}
