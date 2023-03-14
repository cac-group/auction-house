use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;

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