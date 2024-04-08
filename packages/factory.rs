use crate::pair::TokenInfo;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct PoolInfo {
    pub pair_addr: String,
    // pub fee_tier: Uint256,
    pub assets: [TokenInfo; 2],
}

#[cw_serde]
pub struct InstantiateMsg {
    pub pair_code_id: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// CreatePair instantiates pair contract
    CreateNewPair {
        /// Asset infos
        asset_infos: [TokenInfo; 2],
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PoolInfo)]
    Pair { asset_infos: [TokenInfo; 2] },
    #[returns(Addr)]
    GetOwner {},
}

// We define a custom struct for each query response
// #[cw_serde]
// pub struct PairsResponse {
//     pub pairs: Vec<PairInfo>,
// }

#[cw_serde]
pub struct MigrateMsg {}