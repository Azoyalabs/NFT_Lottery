use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::structs::PrizeRegistered;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct EntryPriceResponse {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct WinnersResponse {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PrizePoolResponse {
    pub prizes: Vec<PrizeRegistered>,
}
