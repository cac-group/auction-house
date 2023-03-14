use cosmwasm_std::{Addr, DepsMut, Response, StdResult};
use cw2::set_contract_version;

use crate::state::{AUCTIONS, OWNER};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(deps: DepsMut, sender: Addr) -> StdResult<Response> {
    //Set name and version of auction house contract
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    //The instantiation of this contract will also be the initial owner of it.
    OWNER.save(deps.storage, &sender)?;

    AUCTIONS.save(deps.storage, &Vec::new())?;

    let resp = Response::new()
        .add_attribute("action", "Instantiating Action House")
        .add_attribute("Owner", sender);

    Ok(resp)
}

pub mod query {
    use archway_bindings::ArchwayQuery;
    use cosmwasm_std::{Deps, StdResult};

    use crate::{msg::OpenAuctionsResp, state::AUCTIONS};

    //We return the current auctions that are still open and/or unclaimed.
    pub fn open_auctions(deps: Deps) -> StdResult<OpenAuctionsResp> {
        let auctions = AUCTIONS.load(deps.storage)?;

        Ok(OpenAuctionsResp { auctions })
    }

    pub fn contract_metadata(deps: Deps<ArchwayQuery>) {
    }
}

pub mod exec {}
