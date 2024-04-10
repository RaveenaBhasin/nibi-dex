use cw_storage_plus::Item;
use packages::pair::{PairInfo, Fees};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};

pub const PAIR_INFO: Item<PairInfo> = Item::new("pair info");
pub const FEES: Item<Fees> = Item::new("fees");