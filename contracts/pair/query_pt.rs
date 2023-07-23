use crate::state::{PairInfo, PAIR_INFO};
use cosmwasm_std::{to_binary, Deps, Response, StdResult};
// version info for migration info
const _CONTRACT_NAME: &str = "crates.io:nibiru-hack";
const _CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod query {
    use super::*;
    use cosmwasm_std::{
        Addr, AllBalanceResponse, BalanceResponse, BankQuery, Coin, QuerierWrapper, QueryRequest,
        Uint128, WasmQuery,
    };
    use cw20::{Cw20QueryMsg, TokenInfoResponse};

    pub fn pool_info(_deps: Deps) -> StdResult<Response> {
        Ok(Response::new())
    }

    #[allow(dead_code)]
    pub fn query_pair_info(deps: Deps) -> StdResult<PairInfo> {
        let pair_info: PairInfo = PAIR_INFO.load(deps.storage).unwrap();
        Ok(pair_info)
    }


    pub fn query_token_info(
        querier: &QuerierWrapper,
        contract_addr: Addr,
    ) -> StdResult<TokenInfoResponse> {
        let token_info: TokenInfoResponse = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
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

    pub fn query_all_balances(
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
        let res: BalanceResponse = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: contract_addr.to_string(),
            msg: to_binary(&Cw20QueryMsg::Balance {
                address: account_addr.to_string(),
            })?,
        }))?;

        // load balance form the token contract
        Ok(res.amount.amount)
    }
}
