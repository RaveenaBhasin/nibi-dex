#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
// const CONTRACT_NAME: &str = "crates.io:nibiru-hack";
// const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: Deps,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateNewPair { asset_infos } => execute::create_pair(deps, env, info, asset_infos)
    }
}

pub mod execute {
    use pair::msg::TokenInfo;

    use super::*;
    pub fn create_pair(
        _deps: Deps,
        _env: Env,
        _info: MessageInfo,
        _asset_infos: [TokenInfo; 2]
    ) -> StdResult<Response> {
        Ok(Response::new())
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Pair { asset_infos } => to_binary(&query::pool_info(deps, asset_infos)?)
    }
}

pub mod query {
    use pair::msg::TokenInfo;

    use super::*;
    pub fn pool_info(_deps: Deps, _assetinfos: [TokenInfo; 2]) -> StdResult<Response> {
        Ok(Response::new())
    }

}
    
