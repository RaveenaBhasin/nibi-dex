use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;
use cw20_base::msg::ExecuteMsg as Cw20ExecuteMsg;

#[cw_serde]
pub enum TokenInfo {
    CW20Token {
        contract_addr : Addr,
    },
    NativeToken {
        denom: String,
    }
}

impl TokenInfo {
    pub fn get_as_bytes(&self) -> &[u8]  {
        match self {
            TokenInfo::CW20Token { contract_addr } => contract_addr.as_bytes(),
            TokenInfo::NativeToken { denom } => denom.as_bytes()
        }
    }
}

#[cw_serde]
pub struct InstantiateMsg {
    pub token_info: [TokenInfo; 2],
    pub lp_token_decimal: u8,
    pub cw20_instantiate: Cw20InstantiateMsg,
}

#[cw_serde]
pub struct Token {
    pub info: TokenInfo,
    pub amount: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    SwapAsset {
        from_token: TokenInfo,
        to_token: TokenInfo,
        amount_in: u128,
        min_amount_out: u128,
    },
    AddLiquidity{
        assets: [Token; 2],
        min_liquidity_amt : u128,
    },
    RemoveLiquidity {
        lp_token: Token,
    },
    TokenExecute(Cw20ExecuteMsg),
}



#[cw_serde]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    PoolInfo{}
}