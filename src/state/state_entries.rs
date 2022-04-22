use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

use crate::structs::{LotteryData, LotteryStatus, PrizeRegistered};

// authorizations
pub const ADMIN: Item<Addr> = Item::new("admin");


pub const ID_CURRENT_LOTTERY: Item<u32> = Item::new("id_current_lottery");

// id lottery caller address
pub const REGISTRATION_STATUS: Map<(&str, &str), bool> = Map::new("registration_status");

// id lottery, id prize
pub const CLAIM_STATUS: Map<(&str, &str), bool> = Map::new("claim_status");

// id lottery, id prize => prize registered contains claim and winner status


pub const ENTRY_PRICE: Item<Option<Coin>> = Item::new("entry_price");

// new version
pub const LOTTERIES_DATA: Map<&str, LotteryData> = Map::new("lotteries_data");
// map registration status accross lotteries: id lottery, address user
pub const USER_REGISTRATION_STATUS: Map<(&str, &str), bool> = Map::new("user_registration_status");

// prevent double registration of prizes
// address contract, id token. True if already registered. If in double, will need to check if previous lottery already used that price.
pub const PRIZES_TRACKER: Map<(&str, &str), bool> = Map::new("prizes_tracker");


// track what currency has been deposited for a lottery 
pub const LOTTERY_CURRENCY_DEPOSITED: Map<&str, &str> = Map::new("lottery_currency_deposited");
// track what has been deposited in a contract. id_lottery, denom => amount 
pub const LOTTERY_DEPOSIT_AMOUNTS: Map<(&str, &str), u128> = Map::new("lottery_deposits_amounts");


// track what currencies is available for contract admin  
pub const CONTRACT_ADMIN_CURRENCIES: Item<Vec<String>> = Item::new("contract_admin_currencies");
// track amount that can be withdrawn by contract admin. denom => amount  
pub const CONTRACT_ADMIN_AMOUNTS: Map<&str, u128> = Map::new("contract_admin_amounts");

// can get it all in a single map actually?
