use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::execute_messages::msg::ExecuteMsg;
use crate::state::{state_reads, state_writes};

pub fn dispatch_default(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register {} => try_register(deps, info),
        _ => Err(ContractError::Never {}),
    }
}

fn try_register(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    if !state_reads::can_register(deps.as_ref())? {
        return Err(ContractError::RegistrationsClosed {  });
    }

    if state_reads::is_registered(deps.as_ref(), info.sender.clone())? {
        return Err(ContractError::AlreadyRegistered {  });
    }

    state_writes::register(deps, info.sender.clone())?;


    return Ok(Response::new());
}
