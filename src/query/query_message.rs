use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    PrizePool { id_lottery: u32 },
    Winners { id_lottery: u32 },
    EntryPrice {},
}
