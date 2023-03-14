use cosmwasm_std::{DepsMut, Addr, Response, StdResult};
use cw2::set_contract_version;

use crate::state::OWNER;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(deps: DepsMut, sender: Addr) -> StdResult<Response> {

    //Set name and version of auction house contract
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    //The instantiation of this contract will also be the initial owner of it.
    OWNER.save(deps.storage, &sender)?;

    let resp = Response::new();

    Ok(resp)
}