use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::{CustomMsg, CosmosMsg};

#[cw_serde]
#[derive(QueryResponses)]
//We will receive all auctions that are still open/unclaimed
pub enum QueryMsg {
    #[returns(OpenResp)]
    OpenAuctions {},
}

#[cw_serde]
pub struct OpenResp {
    pub todo: i32,
}

#[cw_serde]
pub struct InstantiateMsg {
}

#[cw_serde]
pub enum ArchwayMsg {
    UpdateContractMetadata {
        owner_address: Option<String>,
        rewards_address: Option<String>,
    },
    WithdrawRewards {
        records_limit: Option<u64>,
        record_ids: Vec<u64>,
    },
}

impl CustomMsg for ArchwayMsg {}

impl From<ArchwayMsg> for CosmosMsg<ArchwayMsg> {
    fn from(msg: ArchwayMsg) -> Self {
        CosmosMsg::Custom(msg)
    }
}

impl ArchwayMsg {
    pub fn update_rewards_ownership(owner_address: impl Into<String>) -> Self {
        ArchwayMsg::UpdateContractMetadata {
            owner_address: Some(owner_address.into()),
            rewards_address: None,
        }
    }

    pub fn update_rewards_address(rewards_address: impl Into<String>) -> Self {
        ArchwayMsg::UpdateContractMetadata {
            owner_address: None,
            rewards_address: Some(rewards_address.into()),
        }
    }

    pub fn withdraw_rewards_by_limit(limit: u64) -> Self {
        ArchwayMsg::WithdrawRewards {
            records_limit: Some(limit),
            record_ids: vec![],
        }
    }

    pub fn withdraw_rewards_by_ids(record_ids: Vec<u64>) -> Self {
        ArchwayMsg::WithdrawRewards {
            records_limit: None,
            record_ids,
        }
    }
}