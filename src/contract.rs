use crate::error::ContractError;
use crate::msg::{
     ExecuteMsg, InstantiateMsg, OwnerInfo, QueryMsg,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, WasmQuery, QueryRequest, StdError
};
use cw721::OwnerOfResponse;
use cw721_base::helpers::Cw721Contract;
use sg721_base::msg::QueryMsg as Sg721QueryMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", _info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    match _msg {
        QueryMsg::CollectionOwnersRange {
            collection,
            start,
            end,
        } => to_binary(&collection_owners_range(deps, collection, start, end)?),

        QueryMsg::AllCollectionOwners {
            collection,
            iters,
            start_after,
            limit,
        } => to_binary(&all_collection_owners(
            deps,
            collection,
            iters,
            start_after,
            limit,
        )?),
    }
}

/// Returns a list of token id --> owner wallet mappings
///
/// # Arguments
/// * `collection` - the address of the collection to map for
/// * `iters` - the maximum number of messages to send
/// * `start_after` - the starting token id for the iteration
/// * `limit` - the number of tokens to query in each message
///
fn all_collection_owners(
    deps: Deps,
    collection: String,
    iters: Option<u32>,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Vec<OwnerInfo>> {
    let mut owners: Vec<OwnerInfo> = vec![];

    // Ensure the collection is a valid address and create CW721 contract from it
    let coll_addr = deps.api.addr_validate(&collection)?;
    let contract = Cw721Contract(coll_addr);

    // Fetch the owner of each token
    let mut i: u32 = 0;
    let mut last_token = start_after;
    while i < iters.unwrap_or(2) {
        // Fetch all token IDs from the source contract
        let query_res = match contract.all_tokens(&deps.querier, last_token.clone(), limit) {
            Ok(tokens) => tokens,
            Err(err) => return Err(err),
        };

        // For each token ID, fetch the owner
        for token_id in query_res.tokens.clone() {
            let owner = match contract.owner_of(&deps.querier, token_id.clone(), false) {
                Ok(owner) => owner,
                Err(err) => return Err(err),
            };
            owners.push(OwnerInfo {
                id: token_id,
                owner: owner.owner,
            });
        }
        // Update last_token to use in pagination
        last_token = match query_res.tokens.last() {
            Some(token) => Some(token.to_string()),
            _ => last_token,
        };
        i += 1;
    }
    Ok(owners)
}


/// Returns a list of token id --> owner wallet mappings
///
/// # Arguments
/// * `collection` - the address of the collection to map for
/// * `start` - starting token id
/// * `end` - ending token id (inclusive)
///
fn collection_owners_range(
    deps: Deps,
    collection: String,
    start: i32,
    end: i32,
) -> StdResult<Vec<OwnerInfo>> {
    let coll_addr = deps.api.addr_validate(&collection)?;
    let mut owners: Vec<OwnerInfo> = vec![];

    if start > end {
        return Err(StdError::generic_err(
            "Invalid Range. Start must be less than End",
        ));
    }

    if start < 0 {
        return Err(StdError::generic_err(
            "Negative Range. Start must be greater than 0",
        ));
    }

    if end - start > 100 {
        return Err(StdError::generic_err(
            "Invalid Range Size. You can only query 100 owners at a time.",
        ));
    }

    for i in start..end {
        let owner_query: OwnerOfResponse =
            match deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: coll_addr.to_string(),
                msg: to_binary(&Sg721QueryMsg::OwnerOf {
                    token_id: i.to_string(),
                    include_expired: Some(false),
                })?,
            })) {
                Ok(owner) => owner,
                Err(_) => continue,
            };
        let owner_info = OwnerInfo {
            id: i.to_string(),
            owner: owner_query.owner,
        };
        owners.push(owner_info);
    }
    Ok(owners)
}