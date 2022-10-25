#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError};
// use cw2::set_contract_version;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, OwnerInfo, QueryMsg};
use cw721_base::helpers::Cw721Contract;
/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:stargaze-multisnap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

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
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    let api = _deps.api;

    match _msg {
        QueryMsg::CollectionOwners {
            collection,
            start,
            end,
        } => to_binary(&collection_owners(
            _deps,
            api.addr_validate(&collection.to_string())?,
            start,
            end,
        )?),
    }
}

fn collection_owners(
    deps: Deps,
    collection: Addr,
    start: i32,
    end: i32,
) -> StdResult<Vec<OwnerInfo>> {
    let mut owners: Vec<OwnerInfo> = vec![];
    let contract = Cw721Contract(collection.clone());

    //TODO: Query the SG721 contract to get the minter address
    //Query minter contract to get num_tokens. (Can't use the sg721 num_tokens, as that returns the remaining amount after burn, rather than init total)
    
    /*
    let minter_addr = contract.minter(deps)?;
    let mint_contract = Cw721Contract(minter_addr);
    let num_tokens = mint_contract.config(deps)?.num_tokens;
    */

    if start > end {
        return Err(StdError::generic_err("Invalid Range. Start must be less than End"));
    }

    if start < 0 {
        return Err(StdError::generic_err("Negative Range. Start must be greater than 0"));
    }

    if end - start > 100 {
        return Err(StdError::generic_err("Invalid Range Size. You can only query 100 owners at a time."));
    }

    for i in start..end {
        let owner_query = match contract.owner_of(&deps.querier, i.to_string(), false) {
            Ok(owner) => owner,
            Err(_) => continue,
        };
        let owner_info = OwnerInfo {
            id: i,
            owner: Addr::unchecked(owner_query.owner.to_string()), //shouldnt have to check the address, since the CW721 contract already does
        };
        owners.push(owner_info);
    }
    Ok(owners)
}

#[cfg(test)]
mod tests {}
