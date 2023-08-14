use cosmwasm_schema::cw_serde;
use packages::pair::TokenInfo;
use cw_storage_plus::{Map, Item};

#[cw_serde]
pub struct PoolInfo {
    pub pair_addr: String,
    pub assets: [TokenInfo; 2]
}

#[cw_serde]
pub struct TmpPoolInfo {
    pub pool_id: Vec<u8>,
    pub assets: [TokenInfo; 2]
}

#[cw_serde]
pub struct FactoryConfig {
    pub pair_code_id: u64,
}

pub const FACTORY_CONFIG: Item<FactoryConfig> = Item::new("factory_config");
pub const TEMP_POOL_INFO: Item<TmpPoolInfo> = Item::new("temp_pool_info");
pub const POOL_ID_TO_POOL_INFO: Map<&[u8], PoolInfo> = Map::new("pairs");



