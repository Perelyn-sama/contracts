use crate::state;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use state::{CollectionParams, MintParams};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateCollection(CollectionParams),
    MintNFT(MintParams),
}

#[cw_serde]
// #[derive(QueryResponses)]
pub enum QueryMsg {
    // Query for config
    Config {},

    // Query for the current token index
    TokenIndex {},

    // Query for collections created by a specific creator
    CreatorCollections { creator: Addr },
}
