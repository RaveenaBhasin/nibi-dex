#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError, CosmosMsg, WasmMsg, SubMsg, Reply, ReplyOn};
use packages::factory::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{FACTORY_CONFIG, FactoryConfig, PoolInfo, TEMP_POOL_INFO, TmpPoolInfo};
use packages::pair::{InstantiateMsg as InstantiatePairMsg, ExecuteMsg as ExecutePairMsg};
use cw0::*;
use cw20::{ MinterResponse, Cw20ExecuteMsg };
// version info for migration info
// const CONTRACT_NAME: &str = "crates.io:nibiru-hack";
// const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let factory_config = FactoryConfig {
        pair_code_id: msg.pair_code_id,
    };
    FACTORY_CONFIG.save(deps.storage, &factory_config).unwrap();
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateNewPair { asset_infos } => execute::create_pair(deps, env, info, asset_infos)
    }
}

pub mod execute {
    use packages::pair::TokenInfo;

    use crate::state::{TmpPoolInfo, POOL_ID_TO_POOL_INFO};

    // use crate::console;
    use super::*;
    pub fn create_pair(
        deps: DepsMut,
        env: Env,
        _info: MessageInfo,
        asset_infos: [TokenInfo; 2]
    ) -> StdResult<Response> {
        let factory_config: FactoryConfig = FACTORY_CONFIG.load(deps.storage)?;
        
        let mut asset_in_bytes = asset_infos.iter().map(|info| info.get_as_bytes()).collect::<Vec<&[u8]>>();
        asset_in_bytes.sort();

        // console.log("hello id", asset_in_bytes);
        
        let pair_id =  asset_in_bytes.concat();
        if let Ok(Some(_)) = POOL_ID_TO_POOL_INFO.may_load(deps.storage, &pair_id) {
            return Err(StdError::generic_err("Pair already exists"));
        }
        
        TEMP_POOL_INFO.save(deps.storage, &TmpPoolInfo {
            pool_id: pair_id.clone(), 
            assets: asset_infos.clone(),
        })?;

        let minter_response = MinterResponse {
            minter: env.contract.address.to_string().clone(),
            cap: None
        };
        
        let instantiate_pair = CosmosMsg::Wasm(WasmMsg::Instantiate {
            code_id: factory_config.pair_code_id,
            funds: vec![],
            admin: Some(env.contract.address.to_string()),
            label: "pair contract".to_string(),
            msg: to_binary(&InstantiatePairMsg {
                token_info: asset_infos,
                lp_token_decimal:  18u8,
                cw20_instantiate: cw20_base::msg::InstantiateMsg {
                    name: "pair token".to_string(),
                    symbol: "pair".to_string(),
                    decimals: 18u8,
                    initial_balances: vec![], 
                    mint: Some(minter_response),
                    marketing: None,
                },
            })?,
        });

        Ok(Response::new()
            .add_submessage(SubMsg {id: 1u64, msg: instantiate_pair, gas_limit: None, reply_on: ReplyOn::Success})
        )
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Pair { asset_infos } => to_binary(&query::pool_info(deps, asset_infos)?)
    }
}

pub mod query {
    use packages::pair::TokenInfo;

    use crate::state::POOL_ID_TO_POOL_INFO;

    use super::*;
    pub fn pool_info(_deps: Deps, assetinfos: [TokenInfo; 2]) -> StdResult<PoolInfo> {
        let mut asset_in_bytes = assetinfos.iter().map(|info| info.get_as_bytes()).collect::<Vec<&[u8]>>();
        asset_in_bytes.sort();
        let pair_id =  asset_in_bytes.concat();
        Ok(POOL_ID_TO_POOL_INFO.load(_deps.storage,  &pair_id)?)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut, 
    env: Env, 
    msg: Reply
) -> StdResult<Response> {
    match msg.id {
        1u64 => reply::instantiate_reply(deps, env, msg),
        _ => Ok(Response::default()),
    }
}

pub mod reply {
    use cosmwasm_std::Empty;

    use crate::state::POOL_ID_TO_POOL_INFO;

    use super::*;

    pub fn instantiate_reply(
        deps: DepsMut,
        _env: Env, 
        msg: Reply
    ) ->  StdResult<Response> {
        let temp_pool_info: TmpPoolInfo = TEMP_POOL_INFO.load(deps.storage)?;

        let res = parse_reply_instantiate_data(msg)
        .map_err(|e| StdError::generic_err(format!("parse reply instantiate data error: {}", e)))?;
        
        POOL_ID_TO_POOL_INFO.save(
            deps.storage,
            &temp_pool_info.pool_id,
            &PoolInfo {
                pair_addr: res.contract_address.clone(),
                assets: temp_pool_info.assets,
            },
        )?;

        let update_minter: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute { 
            contract_addr: res.contract_address.clone(), 
            msg: to_binary(&ExecutePairMsg::TokenExecute(
                Cw20ExecuteMsg::UpdateMinter { 
                    new_minter: Some(res.contract_address.clone()) 
                }
            ))?, 
            funds: vec![],
        });

        Ok(Response::new().add_submessage(SubMsg { id: 2u64, msg: update_minter, gas_limit: None, reply_on: ReplyOn::Success }))
    }
}
    
