use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{PairInfo, PAIR_INFO};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

// version info for migration info
const _CONTRACT_NAME: &str = "crates.io:nibiru-hack";
const _CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let pair_info: PairInfo = PairInfo {
        assets: msg.token_info,
        lp_token_decimal: msg.lp_token_decimal,
    };
    PAIR_INFO.save(deps.storage, &pair_info)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SwapAsset {
            from_token,
            to_token,
            amount_in,
            min_amount_out,
        } => execute::swap(
            deps,
            env,
            info,
            from_token,
            to_token,
            amount_in,
            min_amount_out,
        ),
        ExecuteMsg::AddLiquidity {
            assets,
            min_liquidity_amt,
        } => execute::add_liquidity(deps, env, info, assets, min_liquidity_amt),
        ExecuteMsg::RemoveLiquidity { lp_token } => {
            execute::withdraw_liquidity(deps, env, info, lp_token)
        }
    }
}

pub mod execute {
    use super::*;
    use crate::msg::{Token, TokenInfo};

    pub fn swap(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _from_token: TokenInfo,
        _to_token: TokenInfo,
        _amount_in: u64,
        _min_amount_out: u64,
    ) -> StdResult<Response> {
        Ok(Response::new())
    }

    pub fn add_liquidity(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _assets: [Token; 2],
        _min_liquidity: u64,
    ) -> StdResult<Response> {
        Ok(Response::new())
    }

    pub fn withdraw_liquidity(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _lp_token: Token,
    ) -> StdResult<Response> {
        Ok(Response::new())
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::PoolInfo {} => to_binary(&query::pool_info(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn pool_info(_deps: Deps) -> StdResult<Response> {
        Ok(Response::new())
    }
}
