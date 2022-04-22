use cosmwasm_std::{Deps, StdResult};

use crate::state::state_reads;

use crate::query::query_response::LotteryPrizesResponse;

pub fn get_lottery_prizes(deps: Deps, id_lottery: u32) -> StdResult<LotteryPrizesResponse> {
    let prize_pool = state_reads::get_lottery_prizes(deps, id_lottery).unwrap();

    return Ok(prize_pool);
}
