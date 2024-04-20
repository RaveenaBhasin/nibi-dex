use crate::state::PAIR_INFO;
use cosmwasm_std::{to_binary, Deps, StdResult};
use cw20::BalanceResponse as CW20_BalanceResponse;
use packages::pair::PairInfo;
// version info for migration info
const _CONTRACT_NAME: &str = "crates.io:nibiru-hack";
const _CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod query {
    use super::*;
    use crate::{execute_pt::execute::calculate_swap_amount, state::FEES};
    use cosmwasm_std::{
        Addr, AllBalanceResponse, BalanceResponse, BankQuery, Coin, Env, QuerierWrapper,
        QueryRequest, StdError, Uint128, WasmQuery,
    };
    use cw20::{Cw20QueryMsg, TokenInfoResponse};
    use cw20_base::state::TOKEN_INFO;
    use packages::pair::{Fees, Token, TokenInfo};

    pub fn query_pair_info(deps: Deps) -> StdResult<PairInfo> {
        let pair_info: PairInfo = PAIR_INFO.load(deps.storage).unwrap();
        Ok(pair_info)
    }

    pub fn query_lp_token_amount(deps: Deps, env: Env, assets: [Token; 2]) -> StdResult<Uint128> {
        // check if the pair exists
        let pair_info: PairInfo = PAIR_INFO.load(deps.storage).unwrap();
        if !((assets[0].info == pair_info.assets[0] && assets[1].info == pair_info.assets[1])
            || (assets[0].info == pair_info.assets[1] && assets[1].info == pair_info.assets[0]))
        {
            return Err(StdError::generic_err("Pair does not exist"));
        }

        let mut token_balances = vec![];
        for (_, asset) in assets.iter().enumerate() {
            let token_bal = match &asset.info {
                TokenInfo::CW20Token { contract_addr } => query::query_token_balance(
                    &deps.querier,
                    contract_addr.clone(),
                    env.contract.address.clone(),
                )?,
                TokenInfo::NativeToken { denom } => query::query_native_balance(
                    &deps.querier,
                    denom.to_string(),
                    env.contract.address.clone(),
                )?,
            };
            //        if token_bal == Uint128::from(0u128) {
            //            return Err(StdError::generic_err(format!(
            //                "Balance found zero {:?}",
            //                asset.info
            //            )));
            //        }
            token_balances.push(token_bal);
        }

        println!("Token balances {:?}", token_balances);

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
        Ok(liquidity_minted)
    }

    pub fn query_amount_out(
        deps: Deps,
        env: Env,
        from_token: TokenInfo,
        to_token: TokenInfo,
        amount_in: Uint128,
    ) -> StdResult<Uint128> {
        let amount_out = calculate_swap_amount(deps, env, from_token, to_token, amount_in).unwrap();
        Ok(amount_out)
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
        denom: String,
        account_addr: Addr,
    ) -> StdResult<Uint128> {
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

    pub fn query_estimated_token_amounts(
        deps: Deps,
        env: Env,
        lp_token_amount: Uint128,
    ) -> StdResult<[Token; 2]> {
        let this_address = env.contract.address.clone();

        let pair_info: PairInfo = PAIR_INFO.load(deps.storage).unwrap();
        let token_info = TOKEN_INFO.load(deps.storage)?;
        let res = cw20::TokenInfoResponse {
            name: token_info.name,
            symbol: token_info.symbol,
            decimals: token_info.decimals,
            total_supply: token_info.total_supply,
        };

        let mut estimated_tokens = vec![];

        for (_, asset) in pair_info.assets.iter().enumerate() {
            let reserve = match &asset {
                TokenInfo::CW20Token { contract_addr } => query::query_token_balance(
                    &deps.querier,
                    contract_addr.clone(),
                    this_address.clone(),
                )?,
                TokenInfo::NativeToken { denom } => query::query_native_balance(
                    &deps.querier,
                    denom.to_string(),
                    this_address.clone(),
                )?,
            };

            estimated_tokens.push(match &asset {
                TokenInfo::CW20Token { contract_addr } => Token {
                    info: TokenInfo::CW20Token {
                        contract_addr: contract_addr.clone(),
                    },
                    amount: lp_token_amount.multiply_ratio(reserve, res.total_supply),
                },
                TokenInfo::NativeToken { denom } => Token {
                    info: TokenInfo::NativeToken {
                        denom: denom.clone(),
                    },
                    amount: lp_token_amount.multiply_ratio(reserve, res.total_supply),
                },
            });
        }

        let estimated_tokens_array: [Token; 2] =
            [estimated_tokens[0].clone(), estimated_tokens[1].clone()];
        Ok(estimated_tokens_array)
    }

    pub fn query_reserves_0(deps: Deps, env: Env) -> StdResult<Uint128> {
        let pair_info: PairInfo = PAIR_INFO.load(deps.storage).unwrap();

        Ok(match &pair_info.assets[0] {
            TokenInfo::CW20Token { contract_addr } => query::query_token_balance(
                &deps.querier,
                contract_addr.clone(),
                env.contract.address.clone(),
            )?,
            TokenInfo::NativeToken { denom } => query::query_native_balance(
                &deps.querier,
                denom.to_string(),
                env.contract.address.clone(),
            )?,
        })
    }

    pub fn query_reserves_1(deps: Deps, env: Env) -> StdResult<Uint128> {
        let pair_info: PairInfo = PAIR_INFO.load(deps.storage).unwrap();

        Ok(match &pair_info.assets[1] {
            TokenInfo::CW20Token { contract_addr } => query::query_token_balance(
                &deps.querier,
                contract_addr.clone(),
                env.contract.address.clone(),
            )?,
            TokenInfo::NativeToken { denom } => query::query_native_balance(
                &deps.querier,
                denom.to_string(),
                env.contract.address.clone(),
            )?,
        })
    }

    pub fn query_fees(deps: Deps, _env: Env) -> StdResult<Fees> {
        let fees_info = FEES.load(deps.storage).unwrap();
        Ok(fees_info)
    }
}
