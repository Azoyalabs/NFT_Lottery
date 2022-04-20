use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

use crate::structs::{LotteryState, PrizePool, PrizeRegistered};

// authorizations
pub const ADMIN: Item<Addr> = Item::new("admin");
pub const LOTTERY_STATE: Item<LotteryState> = Item::new("lottery_state");

pub const ID_CURRENT_LOTTERY: Item<u32> = Item::new("id_current_lottery");

// id lottery, caller address, key to check ('registration', 'winner')
pub const USER_STATE: Map<(&str, &str, &str), bool> = Map::new("user_state");

// id lottery caller address
pub const REGISTRATION_STATUS: Map<(&str, &str), bool> = Map::new("registration_status");

// id lottery, id prize
pub const CLAIM_STATUS: Map<(&str, &str), bool> = Map::new("claim_status");

// id lottery, id prize => prize registered contains claim and winner status 
pub const PRIZES: Map<(&str, &str), PrizeRegistered> = Map::new("prizes");
pub const PRIZE_POOL_SIZES: Map<&str, u32> = Map::new("prize_pool_sizes");

// id lottery, id item
pub const PRIZE_POOLS: Item<Vec<PrizePool>> = Item::new("prize_pools");

pub const ENTRY_PRICE: Item<Option<Coin>> = Item::new("entry_price");

