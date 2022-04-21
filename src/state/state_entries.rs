use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

use crate::structs::{LotteryStatus, PrizeRegistered, LotteryData};

// authorizations
pub const ADMIN: Item<Addr> = Item::new("admin");
pub const LOTTERY_STATE: Item<LotteryStatus> = Item::new("lottery_state");

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


pub const ENTRY_PRICE: Item<Option<Coin>> = Item::new("entry_price");


// new version  
pub const LOTTERIES_DATA: Map<&str, LotteryData> = Map::new("lotteries_data");
// map registration status accross lotteries: id lottery, address user
pub const USER_REGISTRATION_STATUS: Map<(&str, &str), bool> = Map::new("user_registration_status");


// prevent double registration of prizes 
// address contract, id token. True if already registered. If in double, will need to check if previous lottery already used that price. 
pub const PRIZES_TRACKER: Map<(&str, &str), bool> = Map::new("prizes_tracker"); 

