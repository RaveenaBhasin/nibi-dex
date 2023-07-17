use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use pair::msg::TokenInfo;

// use cosmwasm_std::Addr;
use cw_storage_plus::{Map, Item};

#[cw_serde]
pub struct PoolInfo {
    pub addr: Addr,
    pub pool_id: u64,
    assets: [TokenInfo; 2]
}


pub const POOLS_COUNT: Item<u64> = Item::new("pool count");
pub const PAIRS: Map<&[u8], PoolInfo> = Map::new("pairs");

pub const ASSET_TO_POOL_ID: Map<(TokenInfo, TokenInfo), u64> = Map::new("assetToPoolId");




