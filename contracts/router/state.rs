use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use crate::msg::TokenInfo;

#[cw_serde]
pub struct PairInfo {
    pub assets: [TokenInfo; 2],
    pub lp_token_decimal: u8,
    pub lp_token_addr: Addr
}

pub const PAIR_INFO: Item<PairInfo> = Item::new("pair info");
