use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::{Addr, Uint128};
use cw20_base::msg::ExecuteMsg as Cw20ExecuteMsg;
use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;
use cw20_base::msg::QueryMsg as CW20QueryMsg;

#[cw_serde]
pub enum TokenInfo {
    CW20Token { contract_addr: Addr },
    NativeToken { denom: String },
}

impl TokenInfo {
    pub fn get_as_bytes(&self) -> &[u8] {
        match self {
            TokenInfo::CW20Token { contract_addr } => contract_addr.as_bytes(),
            TokenInfo::NativeToken { denom } => denom.as_bytes(),
        }
    }
}

#[cw_serde]
pub struct InstantiateMsg {
    pub token_info: [TokenInfo; 2],
    pub lp_token_decimal: u8,
    pub cw20_instantiate: Cw20InstantiateMsg,
    pub treasury: Addr,
}

#[cw_serde]
pub struct Fees {
    pub protocol_fee_recipient: Addr,
    pub protocol_fee_percent: Uint128,
    pub lp_fee_percent: Uint128,
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
    AddLiquidity {
        assets: [Token; 2],
        min_liquidity_amt: Uint128,
    },
    RemoveLiquidity {
        lp_token_amount: Uint128,
    },
    TokenExecute(Cw20ExecuteMsg),
}

#[cw_serde]
pub struct PairInfo {
    pub assets: [TokenInfo; 2],
    pub lp_token_decimal: u8,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PairInfo)]
    PoolInfo {},

    #[returns(CW20QueryMsg)]
    TokenQuery(CW20QueryMsg),

    #[returns(u128)]
    GetEstimatedLpAmount { assets: [Token; 2] },

    #[returns(u128)]
    GetAmountOut {
        from_token: TokenInfo,
        to_token: TokenInfo,
        amount_in: u128,
    },

    #[returns([Token; 2])]
    GetEstimatedTokenAmounts { lp_token_amount: Uint128 },

    #[returns(Uint128)]
    GetReserves0 {},

    #[returns(Uint128)]
    GetReserves1 {},

    #[returns(Fees)]
    GetFees {}
}

#[cw_serde]
pub struct MigrateMsg {}