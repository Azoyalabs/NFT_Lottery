use crate::structs::PrizeRegistered;
use crate::{error::ContractError, structs::LotteryStatus};
use cosmwasm_std::{Addr, Coin, DepsMut, Storage};
use rand::RngCore;
use rand_pcg::Pcg64;

use crate::state::state_entries::ADMIN;

use crate::state::state_entries::{ID_CURRENT_LOTTERY, USER_REGISTRATION_STATUS};

use super::state_entries::LOTTERIES_DATA;

pub fn update_admin(storage: &mut dyn Storage, new_admin: Addr) -> Result<(), ContractError> {
    ADMIN.save(storage, &new_admin).unwrap();

    return Ok(());
}

pub fn update_lottery_status(
    deps: DepsMut,
    id_lottery: u32,
    new_status: LotteryStatus,
) -> Result<(), ContractError> {
    LOTTERIES_DATA.update(
        deps.storage,
        &id_lottery.to_string(),
        |lottery_data| -> Result<_, ContractError> {
            let mut data = lottery_data.unwrap();

            data.status = new_status;

            return Ok(data);
        },
    )?;

    return Ok(());
}

pub fn register(deps: DepsMut, id_lottery: u32, caller: Addr) -> Result<(), ContractError> {
    USER_REGISTRATION_STATUS.save(
        deps.storage,
        (&id_lottery.to_string(), &caller.to_string()),
        &true,
    )?;

    LOTTERIES_DATA.update(
        deps.storage,
        &id_lottery.to_string(),
        |lottery_data| -> Result<_, ContractError> {
            let mut lottery_data = lottery_data.unwrap();
            lottery_data.participants.push(caller);

            return Ok(lottery_data);
        },
    )?;

    return Ok(());
}

pub fn start_new_lottery(deps: DepsMut, new_id: u32) -> Result<(), ContractError> {
    ID_CURRENT_LOTTERY.save(deps.storage, &new_id)?;

    return Ok(());
}

pub fn update_pricing(
    deps: DepsMut,
    id_lottery: u32,
    entry_price: Option<Coin>,
) -> Result<(), ContractError> {
    LOTTERIES_DATA.update(
        deps.storage,
        &id_lottery.to_string(),
        |lottery_data| -> Result<_, ContractError> {
            let mut data = lottery_data.unwrap();

            data.entry_price = entry_price;

            return Ok(data);
        },
    )?;

    return Ok(());
}

pub fn increment_id_lottery(deps: DepsMut) -> Result<(), ContractError> {
    ID_CURRENT_LOTTERY.update(deps.storage, |id_lottery| -> Result<_, ContractError> {
        return Ok(id_lottery + 1);
    })?;

    return Ok(());
}

pub fn set_lottery_prizes(
    deps: DepsMut,
    id_lottery: u32,
    prizes: Vec<PrizeRegistered>,
) -> Result<(), ContractError> {
    LOTTERIES_DATA.update(
        deps.storage,
        &id_lottery.to_string(),
        |lottery_data| -> Result<_, ContractError> {
            let mut lottery_data = lottery_data.unwrap();
            lottery_data.prizes = prizes;

            return Ok(lottery_data);
        },
    )?;

    return Ok(());
}

pub fn draw_winners(deps: DepsMut, id_lottery: u32, mut rng: Pcg64) -> Result<(), ContractError> {
    LOTTERIES_DATA.update(
        deps.storage,
        &id_lottery.to_string(),
        |lottery_data| -> Result<_, ContractError> {
            let mut lottery_data = lottery_data.unwrap();
            let nb_participants = lottery_data.participants.len() as u32;

            for id_prize in 0..lottery_data.prizes.len() {
                let id_winner = (rng.next_u32() % nb_participants) as usize;
                lottery_data.prizes[id_prize].winner =
                    Some(lottery_data.participants[id_winner].to_owned());
            }

            return Ok(lottery_data);
        },
    )?;

    return Ok(());
}
