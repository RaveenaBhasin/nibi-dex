use crate::execute_pt::execute;
use crate::query_pt::query;
use crate::state::{FEES, PAIR_INFO};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw2::set_contract_version;
use packages::pair::{ExecuteMsg, Fees, InstantiateMsg, MigrateMsg, PairInfo, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:nibiru-hack";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

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
    let fees = Fees {
        lp_fee_percent: Uint128::from(30u32),
        protocol_fee_percent: Uint128::from(10u32),
        protocol_fee_recipient: msg.treasury,
    };
    FEES.save(deps.storage, &fees)?;
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

        ExecuteMsg::RemoveLiquidity { lp_token_amount } => {
            execute::withdraw_liquidity(deps, env, info, lp_token_amount)
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
        QueryMsg::GetEstimatedLpAmount { assets } => {
            to_binary(&query::query_lp_token_amount(deps, env, assets)?)
        }
        QueryMsg::GetEstimatedTokenAmounts { lp_token_amount } => {
            to_binary(&query::query_estimated_token_amounts(deps, env, lp_token_amount)?)
        }
        QueryMsg::GetAmountOut {
            from_token,
            to_token,
            amount_in,
        } => to_binary(&query::query_amount_out(
            deps, env, from_token, to_token, amount_in,
        )?),
        QueryMsg::GetReserves0 {} => to_binary(&query::query_reserves_0(deps, env)?),
        QueryMsg::GetReserves1 {} => to_binary(&query::query_reserves_1(deps, env)?),
        QueryMsg::GetFees {} => to_binary(&query::query_fees(deps, env)?)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    let info_str: String = format!(
        "migrating contract: {}, new_contract_version: {}, contract_name: {}",
        env.contract.address,
        CONTRACT_VERSION.to_string(),
        CONTRACT_NAME.to_string()
    );
    deps.api.debug(&info_str);
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> StdResult<Response> {
//     match msg.id {
//         0u64 => reply::instantiate_reply(deps, env, msg),
//         _ => Ok(Response::default()),
//     }
// }

// pub mod reply {
//     use super::*;

//     pub fn instantiate_reply(_deps: DepsMut, _env: Env, _msg: Reply) -> StdResult<Response> {
//         Ok(Response::new())
//     }
// }
