use crate::state::PAIR_INFO;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{to_binary, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use cw20::Cw20ExecuteMsg;
use packages::pair::PairInfo;

// version info for migration info
const _CONTRACT_NAME: &str = "crates.io:nibiru-hack";
const _CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
use crate::query_pt::query;

pub mod execute {

    use cosmwasm_std::{BankMsg, Coin, CosmosMsg, Decimal256, Empty, Uint128, WasmMsg};
    use cw20_base::{
        state::{BALANCES, TOKEN_INFO},
        ContractError,
    };

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

        if (from_token != pair_info.assets[0] && from_token != pair_info.assets[1])
            || (to_token != pair_info.assets[0] && to_token != pair_info.assets[1])
        {
            return Err(StdError::generic_err("Pair does not exist"));
        }

        // Funds sent should not just be equal, but greater than equal
        // funds array should just be checked in case of native tokens only
        // Funds should not be asserted as equality but include_str!("")

        // if info.funds != vec![Coin {
        //     amount: Uint128::from(tok)
        // }])

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
        println!("Amount out {:?}", amount_out);

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
                })?,
                funds: vec![],
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
        let amount_out = token_balances[1].u128()
            - (token_balances[1] * token_balances[0]).u128()
                / (token_balances[0].u128() + amount_in);

        println!(
            "Logging inside contract swap function{:?} {:?} {:?} ",
            assets, amount_out, token_balances
        );
        Ok(amount_out)
    }

    pub fn execute_mint(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        recipient: String,
        amount: Uint128,
    ) -> Result<Response, ContractError> {
        let mut config = TOKEN_INFO.load(deps.storage)?;
        // update supply and enforce cap
        config.total_supply += amount;
        if let Some(limit) = config.get_cap() {
            if config.total_supply > limit {
                return Err(ContractError::CannotExceedCap {});
            }
        }
        TOKEN_INFO.save(deps.storage, &config)?;

        // add amount to recipient balance
        let rcpt_addr = deps.api.addr_validate(&recipient)?;
        BALANCES.update(
            deps.storage,
            &rcpt_addr,
            |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + amount) },
        )?;

        let res = Response::new()
            .add_attribute("action", "mint")
            .add_attribute("to", recipient)
            .add_attribute("amount", amount);
        Ok(res)
    }

    pub fn add_liquidity(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        assets: [Token; 2],
        min_liquidity: Uint128,
    ) -> StdResult<Response> {
        // check if the pair exists
        let pair_info: PairInfo = PAIR_INFO.load(deps.storage).unwrap();
        if !((assets[0].info == pair_info.assets[0] && assets[1].info == pair_info.assets[1])
            || (assets[0].info == pair_info.assets[1] && assets[1].info == pair_info.assets[0]))
        {
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
        for (_, asset) in assets.iter().enumerate() {
            let token_bal = match &asset.info {
                TokenInfo::CW20Token { contract_addr } => query::query_token_balance(
                    &deps.querier,
                    contract_addr.clone(),
                    info.sender.clone(),
                )?,
                TokenInfo::NativeToken { denom } => query::query_native_balance(
                    &deps.querier,
                    info.sender.clone(),
                    denom.to_string(),
                )?,
            };
            if token_bal == Uint128::from(0u128) {
                return Err(StdError::generic_err(format!(
                    "Balance found zero {:?}",
                    asset.info
                )));
            }
            token_balances.push(token_bal);
        }

        let asset0_value = assets[0].amount;
        let asset1_value = assets[1].amount;

        let token_info = TOKEN_INFO.load(deps.storage)?;
        let res = cw20::TokenInfoResponse {
            name: token_info.name,
            symbol: token_info.symbol,
            decimals: token_info.decimals,
            total_supply: token_info.total_supply,
        };

        let liquidity_minted: Uint128;
        if res.total_supply == Uint128::from(0u128) {
            liquidity_minted = std::cmp::min(asset0_value, asset1_value);
        } else {
            liquidity_minted = std::cmp::min(
                asset0_value.multiply_ratio(res.total_supply, token_balances[0]),
                asset1_value.multiply_ratio(res.total_supply, token_balances[1]),
            );
        }

        if liquidity_minted < min_liquidity {
            return Err(StdError::generic_err(format!(
                "Insufficient liquidity minted {:?}",
                liquidity_minted
            )));
        }

        // mint the lp token, to the sender
        execute_mint(
            deps,
            env,
            info.clone(),
            info.sender.to_string().clone(),
            liquidity_minted,
        )
        .unwrap();

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
        let this_address = env.contract.address.clone();

        let pair_info: PairInfo = PAIR_INFO.load(deps.storage).unwrap();
        let token_info = TOKEN_INFO.load(deps.storage)?;
        let res = cw20::TokenInfoResponse {
            name: token_info.name,
            symbol: token_info.symbol,
            decimals: token_info.decimals,
            total_supply: token_info.total_supply,
        };

        let _ratio = Decimal256::from_ratio(lp_token.amount, res.total_supply);

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
            lp_token
                .amount
                .multiply_ratio(token_balances[0], res.total_supply),
            lp_token
                .amount
                .multiply_ratio(token_balances[1], res.total_supply),
        ];

        cw20_base::contract::execute_burn(deps, env, info.clone(), lp_token.amount).unwrap();

        let mut messages = vec![];
        for (i, asset) in pair_info.assets.iter().enumerate() {
            match &asset {
                TokenInfo::CW20Token { contract_addr } => {
                    let cw20_asset_transfer: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
                        contract_addr: contract_addr.to_string(),
                        msg: to_binary(&Cw20ExecuteMsg::Transfer {
                            recipient: info.sender.clone().to_string(),
                            amount: assets_returned[i],
                        })?,
                        funds: vec![],
                    });
                    messages.push(cw20_asset_transfer);
                }
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

        Ok(Response::new().add_messages(messages).add_attributes(vec![
            ("action", "withdraw liquidity"),
            (
                "Assets returned",
                &format!("{}, {}", assets_returned[0], assets_returned[1]),
            ),
        ]))
    }
}
