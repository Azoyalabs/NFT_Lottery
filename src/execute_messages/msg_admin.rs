use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::structs::{LotteryStatus, PrizePool};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AdminExecuteMsg {
    UpdateLotteryState {
        new_state: LotteryStatus,
    },
    StartNewLottery {},
    SetPrizePool {
        target_id: u32,
        prize_pool: PrizePool,
    },

    SetEntryPrice {
        price: Option<Coin>,
    },

    // new version, only this 
    Withdraw { denom: String, amount: String },
    UpdateFee { new_fee: u32 }, // fee in %. Or ask numerator, denominator? much better
}
