use cosmwasm_schema::cw_serde;
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
    CollectionOwnersRange {
        collection: String,
        start: i32,
        end: i32,
    },
    CollectionOwnersPaged {
        collection: String,
    },
    AllCollectionOwners {
        collection: String,
        iters: u32,
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerInfo {
    // The token id for the NFT
    pub id: String,
    // The owner of the token, as a string
    pub owner: String,
}
