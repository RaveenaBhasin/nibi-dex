use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw20::Cw20ExecuteMsg;

use crate::pair::{Token, TokenInfo};

#[cw_serde]
pub struct InstantiateMsg {
    pub factory_addr: Addr,
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
    GetFactoryAddr {},
}

pub enum QueryResponse {
    FactoryAddr { factory_addr: String },
}