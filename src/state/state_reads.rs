use cosmwasm_std::{Addr, Deps};
//use cw721::Approval;

use crate::error::ContractError;

use crate::state::state_entries::ADMIN;
use crate::structs::LotteryState;

use super::state_entries::{LOTTERY_STATE, ID_CURRENT_LOTTERY, USER_STATE};

pub fn is_admin(deps: Deps, caller: Addr) -> Result<bool, ContractError> {
    let admin = ADMIN.load(deps.storage)?;
    return Ok(admin == caller);
}

pub fn is_contract_active(deps: Deps) -> Result<bool, ContractError> {
    let lottery_state = LOTTERY_STATE.load(deps.storage)?;

    match lottery_state {
        LotteryState::Inactive => return Ok(false),
        _ => return Ok(true),
    }

}


pub fn get_id_current_lottery(deps: Deps) -> Result<u32, ContractError> {
    let current_id = ID_CURRENT_LOTTERY.load(deps.storage)?;

    return Ok(current_id);
}


pub fn can_register(deps: Deps) -> Result<bool, ContractError> {
    let lottery_state = LOTTERY_STATE.load(deps.storage)?;

    return Ok(lottery_state == LotteryState::Registration);

}

pub fn is_registered(deps: Deps, caller: Addr) -> Result<bool, ContractError> {
    let id_current_lottery = get_id_current_lottery(deps)?;

    let registration = USER_STATE.may_load(deps.storage, (&caller.to_string(), "registration", &id_current_lottery.to_string()))?;

    match registration {
        None => return Ok(false),
        Some(value) => return Ok(value),
    }

}
