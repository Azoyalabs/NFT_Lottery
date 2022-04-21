use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Addr};

use crate::error::ContractError;
use crate::execute_messages::msg::ExecuteMsg;
use crate::state::state_entries::{LOTTERIES_DATA, ID_CURRENT_LOTTERY};
use crate::state::{state_reads, state_writes};
use crate::structs::{LotteryData, Prize, PrizeRegistered};


pub fn dispatch_default(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register { id_lottery } => try_register(deps, info, id_lottery),
        ExecuteMsg::CreateLottery { admins, prizes } => try_create_lottery(deps, admins, prizes),
        _ => Err(ContractError::Never {}),
    }
}


fn try_create_lottery(deps: DepsMut, admins: Vec<String>, prizes: Option<Vec<Prize>>) -> Result<Response, ContractError> {
    if admins.len() == 0 {
        return Err(ContractError::LotteryRequiresAdmin {  });
    }

    let registered_prizes: Vec<PrizeRegistered> = match prizes {
        None => vec![],
        Some(val) =>             
            val.into_iter().map(|curr_prize| PrizeRegistered::from_prize(deps.api, curr_prize)).collect()
    };

    let admins: Vec<Addr> = admins
        .into_iter()
        .map(|elem| deps.api.addr_validate(&elem).unwrap())
        .collect();

    let lottery_data = LotteryData {
        status: crate::structs::LotteryStatus::SettingUp,
        admins: admins,
        prizes: registered_prizes,
    };

    let current_id = ID_CURRENT_LOTTERY.load(deps.storage)?;
    LOTTERIES_DATA.save(deps.storage, &current_id.to_string(), &lottery_data)?;
    ID_CURRENT_LOTTERY.save(deps.storage, &(current_id + 1))?;

    return Ok(
        Response::new()
    );
}


fn try_register(deps: DepsMut, info: MessageInfo, id_lottery: u32) -> Result<Response, ContractError> {
    if !state_reads::can_register(deps.as_ref())? {
        return Err(ContractError::RegistrationsClosed {});
    }

    if state_reads::is_registered(deps.as_ref(), info.sender.clone())? {
        return Err(ContractError::AlreadyRegistered {});
    }

    state_writes::register(deps, info.sender.clone())?;

    return Ok(Response::new());
}
