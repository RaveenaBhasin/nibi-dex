use cw_storage_plus::Item;
use packages::pair::{PairInfo, Fees};

pub const PAIR_INFO: Item<PairInfo> = Item::new("pair info");
pub const FEES: Item<Fees> = Item::new("fees");