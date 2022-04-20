use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum LotteryState {
    Inactive,
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
    pub prizes: Vec<Prize>
}


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PrizeRegistered {
    pub nft_address: String,
    pub token_id: String,
    pub winner: Option<Addr>,
    pub is_claimed: bool
}

impl PrizeRegistered {
    pub fn FromPrize(prize: Prize) -> PrizeRegistered {
        return PrizeRegistered { nft_address: prize.nft_address, token_id: prize.token_id, winner: None, is_claimed: false };
    }
}
