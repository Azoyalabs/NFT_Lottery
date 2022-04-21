use crate::structs::PrizeRegistered;
use crate::{error::ContractError, structs::LotteryStatus};
use cosmwasm_std::{Addr, Coin, DepsMut, Storage};

use crate::state::state_entries::ADMIN;

use crate::state::state_reads;

use crate::state::state_entries::{LOTTERY_STATE, USER_STATE};

use super::state_entries::{ENTRY_PRICE, ID_CURRENT_LOTTERY, PRIZES, PRIZE_POOL_SIZES};

pub fn update_admin(storage: &mut dyn Storage, new_admin: Addr) -> Result<(), ContractError> {
    ADMIN.save(storage, &new_admin).unwrap();

    return Ok(());
}

pub fn update_lottery_state(deps: DepsMut, new_state: LotteryStatus) -> Result<(), ContractError> {
    LOTTERY_STATE.save(deps.storage, &new_state)?;

    return Ok(());
}

pub fn register(deps: DepsMut, caller: Addr) -> Result<(), ContractError> {
    let id_current_lottery = state_reads::get_id_current_lottery(deps.as_ref())?;

    USER_STATE.save(
        deps.storage,
        (
            &id_current_lottery.to_string(),
            &caller.to_string(),
            "registration",
        ),
        &true,
    )?;

    return Ok(());
}

pub fn start_new_lottery(deps: DepsMut, new_id: u32) -> Result<(), ContractError> {
    ID_CURRENT_LOTTERY.save(deps.storage, &new_id)?;
    LOTTERY_STATE.save(deps.storage, &LotteryStatus::Registration)?;

    return Ok(());
}

pub fn update_entry_price(deps: DepsMut, entry_price: Option<Coin>) -> Result<(), ContractError> {
    ENTRY_PRICE.save(deps.storage, &entry_price)?;

    return Ok(());
}

pub fn set_prize_pool(
    deps: DepsMut,
    target_id: u32,
    prizes: Vec<PrizeRegistered>,
) -> Result<(), ContractError> {
    let nb_prizes = prizes.len() as u32;

    PRIZE_POOL_SIZES.save(deps.storage, &target_id.to_string(), &nb_prizes)?;

    let lottery_id = &target_id.to_string();
    for id_prize in 0..nb_prizes as usize {
        PRIZES.save(
            deps.storage,
            (lottery_id, &id_prize.to_string()),
            &prizes[id_prize],
        )?;
    }

    return Ok(());
}
