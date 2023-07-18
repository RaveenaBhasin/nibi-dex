use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub factory_addr: Addr,
}

#[cw_serde]
pub struct Token {
    pub info: TokenInfo,
    pub amount: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    
}

#[cw_serde]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    PoolInfo{}
}

