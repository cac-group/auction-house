#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

mod contract;
pub mod msg;
pub mod error;
mod state;
