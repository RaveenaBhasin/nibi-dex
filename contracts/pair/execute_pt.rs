use crate::state::{PairInfo, PAIR_INFO};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_binary, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use cw20::Cw20ExecuteMsg;
// version info for migration info
const _CONTRACT_NAME: &str = "crates.io:nibiru-hack";
const _CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
use crate::query_pt::query;

pub mod execute {

    use cosmwasm_std::{CosmosMsg, Decimal256, Empty, WasmMsg, BankMsg, Coin, Uint128};

    use super::*;
    use packages::pair::{Token, TokenInfo};

    pub fn swap(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        from_token: TokenInfo,
        to_token: TokenInfo,
        amount_in: u128,
        min_amount_out: u128,
    ) -> StdResult<Response> {
        let pair_info: PairInfo = PAIR_INFO.load(deps.storage).unwrap();
        
        if from_token == to_token {
            return Err(StdError::generic_err("Cannot swap same token"));
        }
        
        if(from_token != pair_info.assets[0] && from_token != pair_info.assets[1]) || (to_token != pair_info.assets[0] && to_token != pair_info.assets[1]) {
            return Err(StdError::generic_err("Pair does not exist"));
        }

        let mut res = Response::new();
        match &from_token {
            TokenInfo::CW20Token { contract_addr } => {
                let asset_transfer = CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: contract_addr.to_string(),
                    msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                        owner: info.sender.to_string(),
                        recipient: env.contract.address.to_string().clone(),
                        amount: amount_in.into(),
                    })?,
                    funds: vec![],
                });
                res = res.add_message(asset_transfer);
            }
            TokenInfo::NativeToken { denom: _denom } => {}
        };
        
        let amount_out = calculate_swap_amount(deps, env, from_token, to_token.clone(), amount_in)?;

        if amount_out < min_amount_out {
            return Err(StdError::generic_err("Insufficient amount out"));
        }

       // let res = Response::new().add_message(token_transfer);
        let token_transfer: CosmosMsg<Empty> = match &to_token {
            TokenInfo::CW20Token { contract_addr } => CosmosMsg::Wasm(WasmMsg::Execute { 
                contract_addr: contract_addr.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer { 
                    recipient: info.sender.to_string().clone(), 
                    amount: amount_out.into(), 
                })?, funds: vec![], 
                
            }),
            TokenInfo::NativeToken { denom } => CosmosMsg::Bank(BankMsg::Send { 
                to_address: info.sender.to_string().clone(), 
                amount: vec![Coin { 
                    denom: denom.to_string().clone(), 
                    amount: amount_out.into(), 
                }],
            }),
        };
        
        Ok(res.add_message(token_transfer))
    }

    fn calculate_swap_amount(
        deps: DepsMut,
        env: Env,
        from_token: TokenInfo,
        to_token: TokenInfo,
        amount_in: u128,
    ) -> StdResult<u128> {  
        let mut token_balances = vec![];
        let this_address = env.contract.address.clone();
        let assets = [from_token.clone(), to_token.clone()];
        for (_, asset) in assets.iter().enumerate() {
            let token_bal = match &asset {
                TokenInfo::CW20Token { contract_addr } => query::query_token_balance(
                    &deps.querier,
                    contract_addr.clone(),
                    this_address.clone(),
                )?,
                TokenInfo::NativeToken { denom } => query::query_native_balance(
                    &deps.querier,
                    this_address.clone(),
                    denom.to_string(),
                )?,
            };
            token_balances.push(token_bal);
        }

        // x * y = k = (x+a) * (y-b)
        // (x * y) / (x+a) = (y-b)
        // (x * y) / (x+a) + b = y
        // b = y - ( (x * y) / (x+a) )
        let amount_out = token_balances[1].u128() - (token_balances[1] * token_balances[0]).u128() / (token_balances[0].u128() + amount_in);

        Ok(amount_out)
    }


    pub fn add_liquidity(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        assets: [Token; 2],
        min_liquidity: Uint128
    ) -> StdResult<Response> {
        // check if the pair exists
        let pair_info: PairInfo = PAIR_INFO.load(deps.storage).unwrap();
        if !(
                (assets[0].info == pair_info.assets[0] && assets[1].info == pair_info.assets[1])
                || (assets[0].info == pair_info.assets[1] && assets[1].info == pair_info.assets[0])
            ){
            return Err(StdError::generic_err("Pair does not exist"));
        }

        // transfer from both the asset amounts
        let mut messages = vec![];
        for (_i, asset) in assets.iter().enumerate() {
            let _asset_transfer = match &asset.info {
                TokenInfo::CW20Token { contract_addr } => {
                    let asset_transfer: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
                        contract_addr: contract_addr.to_string(),
                        msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                            owner: info.sender.to_string(),
                            recipient: env.contract.address.to_string().clone(),
                            amount: asset.amount,
                        })?,
                        funds: vec![],
                    });
                    messages.push(asset_transfer);
                }
                TokenInfo::NativeToken { denom: _denom } => {}
            };
        }

        let mut token_balances = vec![];
        let this_address = env.contract.address.clone();
        for (_, asset) in assets.iter().enumerate() {
            let token_bal = match &asset.info {
                TokenInfo::CW20Token { contract_addr } => query::query_token_balance(
                    &deps.querier,
                    contract_addr.clone(),
                    this_address.clone(),
                )?,
                TokenInfo::NativeToken { denom } => query::query_native_balance(
                    &deps.querier,
                    this_address.clone(),
                    denom.to_string(),
                )?,
            };
            token_balances.push(token_bal);
        }

        let asset0_value = assets[0].amount;
        let asset1_value = assets[1].amount;
        let total_supply = query::query_token_info(&deps.querier, env.contract.address)
            .unwrap()
            .total_supply;
        let liquidity_minted = std::cmp::min(
            asset0_value.multiply_ratio(total_supply, token_balances[0]),
            asset1_value.multiply_ratio(total_supply, token_balances[1]),
        );

        if liquidity_minted < min_liquidity {
            return Err(StdError::generic_err("Insufficient liquidity minted"));
        }
        
        // mint the lp token, to the sender
        let mint_liquidity: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: this_address.to_string().clone(),
            msg: to_binary(&Cw20ExecuteMsg::Mint {
                recipient: info.sender.to_string(),
                amount: liquidity_minted,
            })?,
            funds: vec![],
        });
        messages.push(mint_liquidity.clone());
        // store the lp token balance in the state
        return Ok(Response::new().add_messages(messages).add_attributes(vec![
            ("action", "add_liquidity"),
            ("sender", info.sender.as_str()),
            ("amount", liquidity_minted.to_string().as_str()),
        ]));
    }

    pub fn withdraw_liquidity(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        lp_token: Token,
    ) -> StdResult<Response> {
        let this_address = env.contract.address;

        let pair_info: PairInfo = PAIR_INFO.load(deps.storage).unwrap();
        let total_supply = query::query_token_info(&deps.querier, this_address.clone())
            .unwrap()
            .total_supply;

        let _ratio = Decimal256::from_ratio(lp_token.amount, total_supply);

        let mut token_balances = vec![];
        for (_, asset) in pair_info.assets.iter().enumerate() {
            let token_bal = match &asset {
                TokenInfo::CW20Token { contract_addr } => query::query_token_balance(
                    &deps.querier,
                    contract_addr.clone(),
                    this_address.clone(),
                )?,
                TokenInfo::NativeToken { denom } => query::query_native_balance(
                    &deps.querier,
                    this_address.clone(),
                    denom.to_string(),
                )?,
            };
            token_balances.push(token_bal);
        }

        let assets_returned = [ 
            lp_token.amount.multiply_ratio(token_balances[0], total_supply),
            lp_token.amount.multiply_ratio(token_balances[1], total_supply),
        ];

        let burn_liquidity: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute { 
            contract_addr: this_address.clone().to_string(), 
            msg: to_binary(&Cw20ExecuteMsg::Burn { 
                amount: lp_token.amount,
            })?, 
            funds: vec![],
        });


        let mut messages = vec![];
        messages.push(burn_liquidity);
        for (i, asset) in pair_info.assets.iter().enumerate() {
            match &asset {
                TokenInfo::CW20Token { contract_addr } => {
                    let cw20_asset_transfer: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
                        contract_addr: contract_addr.to_string(),
                        msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                            owner: info.sender.to_string(),
                            recipient: this_address.to_string().clone(),
                            amount: assets_returned[i],
                        })?,
                        funds: vec![],
                    });
                    messages.push(cw20_asset_transfer);
                },
                TokenInfo::NativeToken { denom } => {
                    let native_asset_transfer: CosmosMsg<Empty> = CosmosMsg::Bank(BankMsg::Send {
                        to_address: info.sender.to_string().clone(),
                        amount: vec![Coin {
                            denom: denom.to_string().clone(),
                            amount: assets_returned[i],
                        }],
                    });
                    messages.push(native_asset_transfer);
                }
            };
        }

        Ok(Response::new()
            .add_messages(messages)
            .add_attributes(vec![
                ("action", "withdraw liquidity"),
                ("Assets returned", &format!("{}, {}", assets_returned[0], assets_returned[1])),
            ]))  
    }
}
