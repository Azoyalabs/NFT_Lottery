use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const RANDOM_BEACON_ADDRESS: Item<Addr> = Item::new("random_beacon_address");
pub const RANDOM_NONCE: Item<u64> = Item::new("random_nonce");
