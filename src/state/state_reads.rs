use cosmwasm_std::{Addr, Deps};
//use cw721::Approval;

use crate::query::query_response::{LotteryPrizesResponse};
use crate::{error::ContractError};

use crate::state::state_entries::ADMIN;
use crate::structs::LotteryStatus;

use super::state_entries::{
    ID_CURRENT_LOTTERY, LOTTERIES_DATA, 
    USER_REGISTRATION_STATUS,
};

pub fn is_valid_id_lottery(deps: Deps, id_lottery: u32) -> Result<bool, ContractError> {
    let current_id = ID_CURRENT_LOTTERY.load(deps.storage)?;

    return Ok(id_lottery < current_id);
}

pub fn is_contract_admin(deps: Deps, caller: Addr) -> Result<bool, ContractError> {
    let admin = ADMIN.load(deps.storage)?;
    return Ok(admin == caller);
}

pub fn is_lottery_admin(deps: Deps, id_lottery: u32, target: Addr) -> Result<bool, ContractError> {
    let lottery_data = LOTTERIES_DATA.load(deps.storage, &id_lottery.to_string())?;

    return Ok(lottery_data.is_admin(target));
}


pub fn get_id_current_lottery(deps: Deps) -> Result<u32, ContractError> {
    let current_id = ID_CURRENT_LOTTERY.load(deps.storage)?;

    return Ok(current_id);
}

pub fn can_register(deps: Deps, id_lottery: u32) -> Result<bool, ContractError> {
    let lottery_data = LOTTERIES_DATA.load(deps.storage, &id_lottery.to_string())?;

    return Ok(lottery_data.status == LotteryStatus::Registration);
}

pub fn is_registered(deps: Deps, id_lottery: u32, caller: Addr) -> Result<bool, ContractError> {
    let registration_status = USER_REGISTRATION_STATUS
        .may_load(deps.storage, (&id_lottery.to_string(), &caller.to_string()))?;

    match registration_status {
        None => return Ok(false),
        Some(status) => return Ok(status),
    }
}

pub fn get_lottery_prizes(deps: Deps, id_lottery: u32) -> Result<LotteryPrizesResponse, ContractError> {
    let lottery_data = LOTTERIES_DATA.load(deps.storage, &id_lottery.to_string())?;

    return Ok(
        LotteryPrizesResponse{
            prizes: lottery_data.prizes
        }
    );
}

//pub fn is_winner(deps: Deps, )
