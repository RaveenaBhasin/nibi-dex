use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{PairInfo, PAIR_INFO};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError};
use cw20::Cw20ExecuteMsg;
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
    cw20_base::contract::instantiate(
        deps,
        _env,
        _info,
        msg.cw20_instantiate,
    ).unwrap();
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

        ExecuteMsg::RemoveLiquidity { lp_token } => execute::withdraw_liquidity(deps, env, info, lp_token),

        ExecuteMsg::TokenExecute (token_execute_msg) => {
            match cw20_base::contract::execute(deps, env, info, token_execute_msg) {
                Ok(res) => Ok(res),
                Err(err) => {
                    Err(StdError::generic_err(format!("cw20_base::contract::execute error: {}", err)))
                }
            }
        }
    }
}

pub mod execute {

    use cosmwasm_std::{CosmosMsg, WasmMsg, Empty};

    use super::*;
    use crate::msg::{Token, TokenInfo};

    pub fn swap(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _from_token: TokenInfo,
        _to_token: TokenInfo,
        _amount_in: u128,
        _min_amount_out: u128,
    ) -> StdResult<Response> {
        Ok(Response::new())
    }

    pub fn add_liquidity(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        assets: [Token; 2],
        _min_liquidity: u128,
    ) -> StdResult<Response> {
        // check if the pair exists
        let pair_info: PairInfo = PAIR_INFO.load(deps.storage).unwrap();
        if (assets[0].info == pair_info.assets[0] && assets[1].info == pair_info.assets[1]) || (assets[0].info == pair_info.assets[1] && assets[1].info == pair_info.assets[0]){
            return Err(StdError::generic_err("Pair does not exist"));
        }

        let mut messages = vec![];
        for (_i, asset) in assets.iter().enumerate() {
            let _asset_transfer = match &asset.info {
                TokenInfo::CW20Token { contract_addr } => {
                    let asset_transfer: CosmosMsg<Empty>   = CosmosMsg::Wasm(WasmMsg::Execute {
                        contract_addr: contract_addr.to_string(),
                        msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                            owner: info.sender.to_string(),
                            recipient: env.contract.address.to_string(),
                            amount: asset.amount,
                        })?,
                        funds: vec![],
                    });
                    messages.push(asset_transfer);
                }, 
                TokenInfo::NativeToken { denom: _denom } => {
            
                }
            };
        }


      
        // transfer from both the asset amounts
        // calculate the amount of lp token to mint
        // mint the lp token, to the sender
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
