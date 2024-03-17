use cw_storage_plus::Item;
use packages::pair::PairInfo;

pub const PAIR_INFO: Item<PairInfo> = Item::new("pair info");

