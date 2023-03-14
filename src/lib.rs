use archway_bindings::ArchwayQuery;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use msg::InstantiateMsg;

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
