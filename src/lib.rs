use cosmwasm_std::{DepsMut, Env, MessageInfo, StdResult, Response};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use msg::InstantiateMsg;

mod contract;
pub mod msg;
pub mod error;
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