use cosmwasm_std::{Deps, StdResult};

use crate::state::state_reads;

use super::query_response::PrizePoolResponse;

pub fn get_prize_pool(deps: Deps, id_lottery: u32) -> StdResult<PrizePoolResponse> {
    let prize_pool = state_reads::get_prize_pool(deps, id_lottery).unwrap();

    return Ok(prize_pool);
}
