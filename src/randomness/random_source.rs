use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Binary, Deps};

use crate::randomness::state_reads;

// use rand::RngCore; // neededto perform actual draw from pcg
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // https://github.com/LoTerra/terrand-contract-step1/blob/master/src/msg.rs
    /// Get the config state
    Config {},
    /// Get the last randomness
    LatestDrand {},
    /// Get a specific randomness
    GetRandomness { round: u64 },
    /// Not used to be call directly
    Verify {
        signature: Binary,
        msg_g2: Binary,
        worker: String,
        round: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LatestRandomResponse {
    pub round: u64,
    pub randomness: Binary,
    pub worker: String,
}

#[cfg(test)]
fn get_seed(_deps: Deps) -> String {
    return "TEST_SEED".into();
}

#[cfg(not(test))]
fn get_seed(deps: Deps) -> String {
    // terrand msgs here:
    // https://github.com/LoTerra/terrand-contract-step1/blob/master/src/msg.rs
    //use cosmwasm_std::WasmQuery;

    let beacon_address = state_reads::get_beacon_address(deps).unwrap();

    let msg = QueryMsg::LatestDrand {};
    let res: LatestRandomResponse = deps.querier.query_wasm_smart(beacon_address, &msg).unwrap();

    return res.randomness.to_string();
}

pub fn get_source_rng(deps: Deps) -> Pcg64 {
    // from https://rust-random.github.io/book/guide-seeding.html
    // call beacon to get random, add nonce and create pcg
    // for initial, use current random beacon value, + round beacon? then later can just use current nonce or something
    let seed = get_seed(deps);
    let nonce = state_reads::get_random_nonce(deps).unwrap();
    let rng: Pcg64 = Seeder::from(seed + &nonce.to_string()).make_rng();

    return rng;
}
