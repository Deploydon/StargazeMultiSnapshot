use cosmwasm_schema::{cw_serde};
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    CollectionOwners{
        collection: Addr, 
        iters: u32,
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

//collection owners response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollectionOwnersResponse {
    pub owners: Vec<OwnerInfo>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerInfo {
    // The token id for the NFT
    pub id: String,
    // The owner of the ID, as a string
    pub owner: String,
}