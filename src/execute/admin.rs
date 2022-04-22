use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;

use crate::execute_messages::msg_admin::AdminExecuteMsg;
use crate::state::{state_reads, state_writes};
use crate::structs::{PrizePool, PrizeRegistered};

pub fn dispatch_admin(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    admin_msg: AdminExecuteMsg,
) -> Result<Response, ContractError> {
    if !state_reads::is_contract_admin(deps.as_ref(), info.sender.clone())? {
        return Err(ContractError::Unauthorized {});
    }

    match admin_msg {
        AdminExecuteMsg::Withdraw { denom, amount } => try_withdraw(deps, info, denom, amount),
        _ => return Ok(Response::new()),
    }
}

fn try_withdraw(
    _deps: DepsMut,
    _info: MessageInfo,
    _denom: String,
    _amount: String,
) -> Result<Response, ContractError> {
    return Ok(Response::new());
}

