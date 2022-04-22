use cosmwasm_std::{Addr, Coin, DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::execute_messages::msg::ExecuteMsg;
use crate::state::state_entries::{ID_CURRENT_LOTTERY, LOTTERIES_DATA};
use crate::state::{state_reads, state_writes};
use crate::structs::{LotteryData, LotteryStatus, Prize, PrizeRegistered};

pub fn dispatch_default(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register { id_lottery } => try_register(deps, info, id_lottery),
        ExecuteMsg::CreateLottery {
            entry_price,
            admins,
            prizes,
        } => try_create_lottery(deps, entry_price, admins, prizes),
        ExecuteMsg::UpdatePrizes { id_lottery, prizes } => {
            try_update_prizes(deps, info, id_lottery, prizes)
        }
        ExecuteMsg::UpdateLotteryStatus {
            id_lottery,
            new_status,
        } => try_update_lottery_status(deps, info, id_lottery, new_status),
        ExecuteMsg::UpdateEntryPrice {
            id_lottery,
            entry_price,
        } => try_update_entry_price(deps, info, id_lottery, entry_price),
        ExecuteMsg::Withdraw { id_lottery, denom, amount } => try_withdraw(deps, info, id_lottery, denom, amount),
        ExecuteMsg::ClaimPrize { id_lottery, id_prize } => try_claim_prize(deps, info, id_lottery, id_prize),
        _ => Err(ContractError::Never {}),
    }
}

fn try_claim_prize(deps: DepsMut, info: MessageInfo, id_lottery: u32, id_prize: u32) -> Result<Response, ContractError> {
    
    return Ok(Response::new());
}

fn try_withdraw(deps: DepsMut, info: MessageInfo, id_lottery: u32, denom: String, amount: String) -> Result<Response, ContractError> {
    if !state_reads::is_valid_id_lottery(deps.as_ref(), id_lottery)? {
        return Err(ContractError::InvalidIdLottery {});
    }

    if !state_reads::is_lottery_admin(deps.as_ref(), id_lottery, info.sender.clone())? {
        return Err(ContractError::Unauthorized {});
    }    


    return Ok(Response::new());
}

fn try_update_entry_price(
    deps: DepsMut,
    info: MessageInfo,
    id_lottery: u32,
    entry_price: Option<Coin>,
) -> Result<Response, ContractError> {
    if !state_reads::is_valid_id_lottery(deps.as_ref(), id_lottery)? {
        return Err(ContractError::InvalidIdLottery {});
    }

    if !state_reads::is_lottery_admin(deps.as_ref(), id_lottery, info.sender.clone())? {
        return Err(ContractError::Unauthorized {});
    }

    state_writes::update_pricing(deps, id_lottery, entry_price)?;

    return Ok(Response::new());
}

fn try_update_lottery_status(
    deps: DepsMut,
    info: MessageInfo,
    id_lottery: u32,
    new_status: LotteryStatus,
) -> Result<Response, ContractError> {
    if !state_reads::is_valid_id_lottery(deps.as_ref(), id_lottery)? {
        return Err(ContractError::InvalidIdLottery {});
    }

    if !state_reads::is_lottery_admin(deps.as_ref(), id_lottery, info.sender.clone())? {
        return Err(ContractError::Unauthorized {});
    }

    state_writes::update_lottery_status(deps, id_lottery, new_status)?;

    return Ok(Response::new());
}

fn try_update_prizes(
    deps: DepsMut,
    info: MessageInfo,
    id_lottery: u32,
    prizes: Vec<Prize>,
) -> Result<Response, ContractError> {
    if !state_reads::is_valid_id_lottery(deps.as_ref(), id_lottery)? {
        return Err(ContractError::InvalidIdLottery {});
    }

    if !state_reads::is_lottery_admin(deps.as_ref(), id_lottery, info.sender.clone())? {
        return Err(ContractError::Unauthorized {});
    }

    let prizes_registered: Vec<PrizeRegistered> = prizes
        .into_iter()
        .map(|prize| PrizeRegistered::from_prize(deps.api, prize))
        .collect();

    state_writes::set_lottery_prizes(deps, id_lottery, prizes_registered)?;

    return Ok(Response::new());
}

fn try_create_lottery(
    deps: DepsMut,
    entry_price: Option<Coin>,
    admins: Vec<String>,
    prizes: Option<Vec<Prize>>,
) -> Result<Response, ContractError> {
    if admins.len() == 0 {
        return Err(ContractError::LotteryRequiresAdmin {});
    }

    let registered_prizes: Vec<PrizeRegistered> = match prizes {
        None => vec![],
        Some(val) => val
            .into_iter()
            .map(|curr_prize| PrizeRegistered::from_prize(deps.api, curr_prize))
            .collect(),
    };

    let admins: Vec<Addr> = admins
        .into_iter()
        .map(|elem| deps.api.addr_validate(&elem).unwrap())
        .collect();

    let lottery_data = LotteryData {
        entry_price: entry_price,
        status: LotteryStatus::SettingUp,
        admins: admins,
        prizes: registered_prizes,
    };

    let current_id = ID_CURRENT_LOTTERY.load(deps.storage)?;
    LOTTERIES_DATA.save(deps.storage, &current_id.to_string(), &lottery_data)?;
    ID_CURRENT_LOTTERY.save(deps.storage, &(current_id + 1))?;

    return Ok(Response::new());
}

fn try_register(
    deps: DepsMut,
    info: MessageInfo,
    id_lottery: u32,
) -> Result<Response, ContractError> {
    if !state_reads::can_register(deps.as_ref(), id_lottery)? {
        return Err(ContractError::RegistrationsClosed {});
    }

    if state_reads::is_registered(deps.as_ref(), id_lottery, info.sender.clone())? {
        return Err(ContractError::AlreadyRegistered {});
    }

    state_writes::register(deps, id_lottery, info.sender.clone())?;

    return Ok(Response::new());
}
