#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::execute::admin::dispatch_admin;
use crate::execute::default::dispatch_default;

use crate::execute_messages::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg};

use crate::query::query_message::QueryMsg;

use crate::error::ContractError;
use crate::state::state_entries::{ADMIN, ID_CURRENT_LOTTERY};

//use cw2::{set_contract_version, get_contract_version, ContractVersion};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "terrarity";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    ADMIN.save(deps.storage, &info.sender)?;
    //LOTTERY_STATE.save(deps.storage, &LotteryStatus::Inactive)?;

    ID_CURRENT_LOTTERY.save(deps.storage, &0)?;

    return Ok(Response::default());
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    // No state migrations performed, just returned a Response
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // Admin
        ExecuteMsg::Admin(admin_msg) => dispatch_admin(deps, env, info, admin_msg),
        // Default
        _ => dispatch_default(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        _ => return to_binary(&42),
    }
}
