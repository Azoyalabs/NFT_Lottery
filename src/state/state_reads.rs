use cosmwasm_std::{Addr, Deps};
//use cw721::Approval;

use crate::query::query_response::PrizePoolResponse;
use crate::{error::ContractError, structs::PrizeRegistered};

use crate::state::state_entries::ADMIN;
use crate::structs::LotteryStatus;

use super::state_entries::{
    ID_CURRENT_LOTTERY, LOTTERY_STATE, PRIZES, PRIZE_POOL_SIZES, USER_STATE,
};

pub fn is_admin(deps: Deps, caller: Addr) -> Result<bool, ContractError> {
    let admin = ADMIN.load(deps.storage)?;
    return Ok(admin == caller);
}

pub fn is_contract_active(deps: Deps) -> Result<bool, ContractError> {
    let lottery_state = LOTTERY_STATE.load(deps.storage)?;

    match lottery_state {
        LotteryStatus::Inactive => return Ok(false),
        _ => return Ok(true),
    }
}

pub fn get_id_current_lottery(deps: Deps) -> Result<u32, ContractError> {
    let current_id = ID_CURRENT_LOTTERY.load(deps.storage)?;

    return Ok(current_id);
}

pub fn can_register(deps: Deps) -> Result<bool, ContractError> {
    let lottery_state = LOTTERY_STATE.load(deps.storage)?;

    return Ok(lottery_state == LotteryStatus::Registration);
}

pub fn is_registered(deps: Deps, caller: Addr) -> Result<bool, ContractError> {
    let id_current_lottery = get_id_current_lottery(deps)?;

    let registration = USER_STATE.may_load(
        deps.storage,
        (
            &caller.to_string(),
            "registration",
            &id_current_lottery.to_string(),
        ),
    )?;

    match registration {
        None => return Ok(false),
        Some(value) => return Ok(value),
    }
}

pub fn get_prize_pool(deps: Deps, id_lottery: u32) -> Result<PrizePoolResponse, ContractError> {
    let nb_prizes = PRIZE_POOL_SIZES.load(deps.storage, &id_lottery.to_string())?;
    
    let id_key = &id_lottery.to_string();
    let mut prizes: Vec<PrizeRegistered> = vec![];
    for id_prize in 0..nb_prizes as usize {
        prizes.push(PRIZES.load(deps.storage, (id_key, &id_prize.to_string()))?);
    }

    return Ok(PrizePoolResponse { prizes: prizes });
}


//pub fn is_winner(deps: Deps, )
