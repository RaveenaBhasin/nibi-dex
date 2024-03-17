use cw_storage_plus::Item;
use packages::router::Config;

pub const CONFIG: Item<Config> = Item::new("config");
