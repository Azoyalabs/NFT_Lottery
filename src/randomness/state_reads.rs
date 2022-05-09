use cosmwasm_std::{Addr, Deps};

use crate::ContractError;

use crate::randomness::state_entries::{RANDOM_BEACON_ADDRESS, RANDOM_NONCE};

pub fn get_beacon_address(deps: Deps) -> Result<Addr, ContractError> {
    let beacon_address = RANDOM_BEACON_ADDRESS.load(deps.storage)?;

    return Ok(beacon_address);
}

pub fn get_random_nonce(deps: Deps) -> Result<u64, ContractError> {
    let nonce = RANDOM_NONCE.load(deps.storage)?;

    return Ok(nonce);
}
