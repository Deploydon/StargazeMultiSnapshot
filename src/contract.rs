#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult
};
use cw721_base::helpers::Cw721Contract;
use crate::error::ContractError;
use crate::msg::{InstantiateMsg, OwnerInfo, QueryMsg};


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
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    let api = _deps.api;

    match _msg {
        QueryMsg::CollectionOwners {
            collection,
            iters,
            start_after,
            limit,
        } => to_binary(&collection_owners(
            _deps,
            api.addr_validate(&collection.to_string())?,
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
fn collection_owners(
    deps: Deps,
    collection: Addr,
    iters: u32,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Vec<OwnerInfo>> {
    let mut owners: Vec<OwnerInfo> = vec![];
    let contract = Cw721Contract(collection.clone());
    
    let mut i: u32 = 0;
    let mut last_token = start_after.clone();
    while i < iters {
        let query_res = match contract.all_tokens(&deps.querier, last_token.clone(), limit) {
            Ok(tokens) => tokens,
            Err(err) => return Err(err),
        };
        for token_id in query_res.tokens.clone() {
            let owner = match contract.owner_of(&deps.querier, token_id.clone(), false) {
                Ok(owner) => owner,
                Err(err) => return Err(err),
            };

            owners.push(OwnerInfo{
                id: token_id,
                owner: owner.owner,
            });
        }
        last_token = match query_res.tokens.last() {
            Some(token) => Some(token.to_string()),
            _ => last_token
        };
        i += 1;
    }
    Ok(owners)
}

#[cfg(test)]
mod tests {}
