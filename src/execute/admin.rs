use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;

use crate::execute_messages::msg_admin::AdminExecuteMsg;
use crate::state::{state_reads, state_writes};
use crate::structs::{LotteryStatus, PrizePool, PrizeRegistered};

pub fn dispatch_admin(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    admin_msg: AdminExecuteMsg,
) -> Result<Response, ContractError> {
    if !state_reads::is_admin(deps.as_ref(), info.sender.clone())? {
        return Err(ContractError::Unauthorized {});
    }

    match admin_msg {
        AdminExecuteMsg::UpdateLotteryState { new_state } => {
            try_update_lottery_state(deps, new_state)
        }
        AdminExecuteMsg::StartNewLottery {} => try_start_new_lottery(deps),
        AdminExecuteMsg::SetPrizePool {
            target_id,
            prize_pool,
        } => try_set_prize_pool(deps, target_id, prize_pool),
        AdminExecuteMsg::SetEntryPrice { price } => try_set_entry_price(deps, price),
        
        
        
        _ => return Ok(Response::new()),
    }
}

fn try_set_entry_price(deps: DepsMut, price: Option<Coin>) -> Result<Response, ContractError> {
    state_writes::update_entry_price(deps, price)?;

    return Ok(Response::new());
}

fn try_set_prize_pool(
    deps: DepsMut,
    target_id: u32,
    prize_pool: PrizePool,
) -> Result<Response, ContractError> {
    // transform prize_pool into vec of RegisteredPrize
    let registered_prizes: Vec<PrizeRegistered> = prize_pool
        .prizes
        .into_iter()
        .map(|elem| PrizeRegistered::from_prize(deps.api, elem))
        .collect();

    state_writes::set_prize_pool(deps, target_id, registered_prizes)?;

    return Ok(Response::new());
}

fn try_start_new_lottery(deps: DepsMut) -> Result<Response, ContractError> {
    let id_new_lottery = state_reads::get_id_current_lottery(deps.as_ref())? + 1;

    state_writes::start_new_lottery(deps, id_new_lottery.clone())?;
    
    return Ok(Response::new());
}

fn try_update_lottery_state(
    deps: DepsMut,
    new_state: LotteryStatus,
) -> Result<Response, ContractError> {
    state_writes::update_lottery_state(deps, new_state)?;

    return Ok(Response::new());
}
