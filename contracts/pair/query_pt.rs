use crate::state::PAIR_INFO;
use cosmwasm_std::{to_binary, Deps, StdError, StdResult};
use cw20::BalanceResponse as CW20_BalanceResponse;
use packages::pair::PairInfo;
// version info for migration info
const _CONTRACT_NAME: &str = "crates.io:nibiru-hack";
const _CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod query {
    use std::u128;

    use super::*;
    use cosmwasm_std::{
        Addr, AllBalanceResponse, BalanceResponse, BankQuery, Coin, CosmosMsg, Empty, Env,
        MessageInfo, QuerierWrapper, QueryRequest, Uint128, WasmMsg, WasmQuery,
    };
    use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, TokenInfoResponse};
    use cw20_base::state::TOKEN_INFO;
    use packages::pair::{Token, TokenInfo};

    pub fn query_pair_info(deps: Deps) -> StdResult<PairInfo> {
        let pair_info: PairInfo = PAIR_INFO.load(deps.storage).unwrap();
        Ok(pair_info)
    }

    pub fn query_lp_token_amount(
        deps: Deps,
        info: MessageInfo,
        env: Env,
        assets: [Token; 2],
    ) -> StdResult<Uint128> {
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
                            owner: info.sender.into_string(),
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

        return Ok(liquidity_minted);
    }

    pub fn _query_token_info(
        querier: &QuerierWrapper,
        contract_addr: Addr,
    ) -> StdResult<TokenInfoResponse> {
        let token_info: TokenInfoResponse =
            querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: contract_addr.to_string(),
                msg: to_binary(&Cw20QueryMsg::TokenInfo {})?,
            }))?;

        Ok(token_info)
    }

    pub fn query_native_balance(
        querier: &QuerierWrapper,
        account_addr: Addr,
        denom: String,
    ) -> StdResult<Uint128> {
        // load price form the oracle
        let balance: BalanceResponse = querier.query(&QueryRequest::Bank(BankQuery::Balance {
            address: account_addr.to_string(),
            denom,
        }))?;
        Ok(balance.amount.amount)
    }

    pub fn _query_all_balances(
        querier: &QuerierWrapper,
        account_addr: Addr,
    ) -> StdResult<Vec<Coin>> {
        // load price form the oracle
        let all_balances: AllBalanceResponse =
            querier.query(&QueryRequest::Bank(BankQuery::AllBalances {
                address: account_addr.to_string(),
            }))?;
        Ok(all_balances.amount)
    }

    pub fn query_token_balance(
        querier: &QuerierWrapper,
        contract_addr: Addr,
        account_addr: Addr,
    ) -> StdResult<Uint128> {
        let res: CW20_BalanceResponse = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: contract_addr.to_string(),
            msg: to_binary(&Cw20QueryMsg::Balance {
                address: account_addr.to_string(),
            })?,
        }))?;

        // load balance form the token contract
        Ok(res.balance)
    }
}
