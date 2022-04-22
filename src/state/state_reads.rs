use cosmwasm_std::{Addr, Deps, Coin};
//use cw721::Approval;

use crate::error::ContractError;
use crate::query::query_response::LotteryPrizesResponse;

use crate::state::state_entries::ADMIN;
use crate::structs::LotteryStatus;

use super::state_entries::{ID_CURRENT_LOTTERY, LOTTERIES_DATA, USER_REGISTRATION_STATUS};

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

pub fn get_lottery_prizes(
    deps: Deps,
    id_lottery: u32,
) -> Result<LotteryPrizesResponse, ContractError> {
    let lottery_data = LOTTERIES_DATA.load(deps.storage, &id_lottery.to_string())?;

    return Ok(LotteryPrizesResponse {
        prizes: lottery_data.prizes,
    });
}

pub fn check_lottery_prizes_claimable(deps: Deps, id_lottery: u32) -> Result<bool, ContractError> {
    let lottery_data = LOTTERIES_DATA.load(deps.storage, &id_lottery.to_string())?;

    return Ok(lottery_data.status == LotteryStatus::PrizeDistribution);
}

pub fn check_valid_funds_for_lottery(deps: Deps, id_lottery: u32, funds: Vec<Coin>) -> Result<bool, ContractError> {
    let lottery_data = LOTTERIES_DATA.load(deps.storage, &id_lottery.to_string())?;
    let pricing = lottery_data.entry_price;

    match pricing {
        None => return _check_valid_funds_for_lottery_non_payable(funds),
        Some(pricing) => return _check_valid_funds_for_lottery_payable(pricing, funds),
    }

}

fn _check_valid_funds_for_lottery_payable(pricing: Coin, funds: Vec<Coin>) -> Result<bool, ContractError> {
    if funds.len() == 0 {
        return Err(ContractError::PayableLottery{});
    } else if funds.len() > 1 {
        return Err(ContractError::SingleCurrencyLottery{});
    } else if funds[0] != pricing {
        return Err(ContractError::InvalidAmountEntryPrice {  });
    }

    return Ok(true);
}

fn _check_valid_funds_for_lottery_non_payable(funds: Vec<Coin>) -> Result<bool, ContractError> {
    if funds.len() > 0 {
        return Err(ContractError::NotPayableLottery{});
    }

    return Ok(true);
}

pub fn check_is_valid_prize_id(deps: Deps, id_lottery: u32, id_prize: u32) -> Result<bool, ContractError> {
    let lottery_data = LOTTERIES_DATA.load(deps.storage, &id_lottery.to_string())?;

    return Ok(id_prize < lottery_data.prizes.len() as u32);
}

pub fn check_is_prize_winner(deps: Deps, id_lottery: u32, id_prize: u32, caller: Addr) -> Result<bool, ContractError> {
    let lottery_data = LOTTERIES_DATA.load(deps.storage, &id_lottery.to_string())?;
    let prize_data = lottery_data.prizes[id_prize as usize].to_owned();

    match prize_data.winner {
        None => return Ok(false),
        Some(address_winner) => return Ok(address_winner == caller)
    };

}

//pub fn is_winner(deps: Deps, )
