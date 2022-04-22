use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_binary, Binary, Coin, CosmosMsg, StdResult, WasmMsg};

use crate::{
    execute_messages::msg_admin::AdminExecuteMsg,
    structs::{LotteryStatus, Prize},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Admin(AdminExecuteMsg),

    // users
    Register {
        id_lottery: u32,
    },
    ClaimPrize {
        id_lottery: u32,
        id_prize: u32,
    },

    // lottery creators
    CreateLottery {
        entry_price: Option<Coin>,
        admins: Vec<String>,
        prizes: Option<Vec<Prize>>,
    },
    UpdatePrizes {
        id_lottery: u32,
        prizes: Vec<Prize>,
    },
    UpdateEntryPrice {
        id_lottery: u32,
        entry_price: Option<Coin>,
    },

    UpdateLotteryStatus {
        id_lottery: u32,
        new_status: LotteryStatus,
    },

    ValidatePrizes {
        id_lottery: u32,
    }, // transfer prizes ownership to lottery contract

    Withdraw {
        id_lottery: u32,
        denom: String,
        amount: String,
    },

    DrawLottery {
        id_lottery: u32
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Cw721ReceiveMsg {
    pub sender: String,
    pub token_id: String,
    pub msg: Binary,
}

impl Cw721ReceiveMsg {
    /// serializes the message
    pub fn into_binary(self) -> StdResult<Binary> {
        let msg = ReceiverExecuteMsg::ReceiveNft(self);
        to_binary(&msg)
    }

    /// creates a cosmos_msg sending this struct to the named contract
    pub fn into_cosmos_msg<T: Into<String>, C>(self, contract_addr: T) -> StdResult<CosmosMsg<C>>
    where
        C: Clone + std::fmt::Debug + PartialEq + JsonSchema,
    {
        let msg = self.into_binary()?;
        let execute = WasmMsg::Execute {
            contract_addr: contract_addr.into(),
            msg,
            funds: vec![],
        };
        Ok(execute.into())
    }
}

/// This is just a helper to properly serialize the above message.
/// The actual receiver should include this variant in the larger ExecuteMsg enum
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
enum ReceiverExecuteMsg {
    ReceiveNft(Cw721ReceiveMsg),
}
