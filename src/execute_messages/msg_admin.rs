use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::structs::{LotteryState, PrizePool};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AdminExecuteMsg {
    UpdateLotteryState { new_state: LotteryState },
    StartNewLottery {},
    SetPrizePool{target_id: u32, prize_pool: PrizePool},

    SetEntryPrice { price: Option<Coin> },
}

