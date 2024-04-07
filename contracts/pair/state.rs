use cw_storage_plus::Item;
use packages::pair::PairInfo;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};

pub const PAIR_INFO: Item<PairInfo> = Item::new("pair info");

pub enum FeeTier {
    Tier1,
    Tier2,
    Tier3
}

#[cw_serde]
pub struct Fees {
    pub protocol_fee_recipient: Addr,
    pub protocol_fee_percent: Decimal,
    pub lp_fee_percent: Decimal,
    //pub tier_fees: [(FeeTier, Decimal); 3],
}


// (FeeTier::Tier1, Decimal::percent(0.03))
pub const FEES: Item<Fees> = Item::new("fees");