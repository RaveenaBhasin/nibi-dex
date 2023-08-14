use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

use packages::pair::TokenInfo;

#[cw_serde]
pub struct PairInfo {
    pub assets: [TokenInfo; 2],
    pub lp_token_decimal: u8,
}

pub const PAIR_INFO: Item<PairInfo> = Item::new("pair info");