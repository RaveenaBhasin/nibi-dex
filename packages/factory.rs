use cosmwasm_schema::cw_serde;
use crate::pair::TokenInfo;

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
pub enum QueryMsg {
    Pair {
        asset_infos: [TokenInfo; 2],
    },
}

// We define a custom struct for each query response
// #[cw_serde]
// pub struct PairsResponse {
//     pub pairs: Vec<PairInfo>,
// }
