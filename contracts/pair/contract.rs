use crate::state::PAIR_INFO;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
};
use packages::pair::{ExecuteMsg, InstantiateMsg, PairInfo, QueryMsg};
// version info for migration info
const _CONTRACT_NAME: &str = "crates.io:nibiru-hack";
const _CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
use crate::execute_pt::execute;
use crate::query_pt::query;
use packages::pair::Token;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let pair_info: PairInfo = PairInfo {
        assets: msg.token_info,
        lp_token_decimal: msg.lp_token_decimal,
    };
    PAIR_INFO.save(deps.storage, &pair_info)?;
    cw20_base::contract::instantiate(deps, env, _info, msg.cw20_instantiate).unwrap();
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

        ExecuteMsg::TokenExecute(token_execute_msg) => {
            match cw20_base::contract::execute(deps, env, info, token_execute_msg) {
                Ok(res) => Ok(res),
                Err(err) => Err(StdError::generic_err(format!(
                    "cw20_base::contract::execute error: {}",
                    err
                ))),
            }
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::PoolInfo {} => to_binary(&query::query_pair_info(deps)?),
        QueryMsg::TokenQuery(token_query_msg) => {
            match cw20_base::contract::query(deps, env, token_query_msg) {
                Ok(res) => Ok(res),
                Err(err) => Err(StdError::generic_err(format!(
                    "cw20_base::contract::query error: {}",
                    err
                ))),
            }
        }
        QueryMsg::GetLpTokenAmount { assets } => {
            to_binary(&query::query_lp_token_amount(deps, assets)?)
        }
        QueryMsg::GetAmountOut {
            from_token,
            to_token,
            amount_in,
        } => {
            todo!()
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        0u64 => reply::instantiate_reply(deps, env, msg),
        _ => Ok(Response::default()),
    }
}

pub mod reply {
    use super::*;

    pub fn instantiate_reply(_deps: DepsMut, _env: Env, _msg: Reply) -> StdResult<Response> {
        Ok(Response::new())
    }
}
