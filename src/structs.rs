use cosmwasm_std::{Addr, Api};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum LotteryStatus {
    Inactive,
    SettingUp,
    Registration,
    WaitingForDraw,
    PrizeDistribution,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Prize {
    pub nft_address: String,
    pub token_id: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PrizePool {
    pub prizes: Vec<Prize>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PrizeRegistered {
    pub nft_address: Addr,
    pub token_id: String,
    pub winner: Option<Addr>,
    pub is_claimed: bool,
}

impl PrizeRegistered {
    pub fn from_prize(api: &dyn Api, prize: Prize) -> PrizeRegistered {
        return PrizeRegistered {
            nft_address: api.addr_validate(&prize.nft_address).unwrap(),
            token_id: prize.token_id,
            winner: None,
            is_claimed: false,
        };
    }
}


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct LotteryData {
    pub status: LotteryStatus,
    pub admins: Vec<Addr>,
    pub prizes: Vec<PrizeRegistered>,
}

impl LotteryData {
    pub fn get_winners(&self) -> Option<Vec<Addr>> {
        if self.status != LotteryStatus::PrizeDistribution {
            return None;
        }

        let winners: Vec<Addr> = self.prizes.to_owned()
            .into_iter()
            .map(|prize| prize.winner.unwrap())
            .collect();

        return Some(winners);
    }

}