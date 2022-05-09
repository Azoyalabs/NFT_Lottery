use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AdminExecuteMsg {
    Withdraw { denom: String, amount: String },
    UpdateFee { new_fee: u32 }, // fee in %. Or ask numerator, denominator? much better
}
