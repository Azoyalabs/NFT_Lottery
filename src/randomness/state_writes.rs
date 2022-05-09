use cosmwasm_std::DepsMut;

use crate::randomness::state_entries::RANDOM_NONCE;

use crate::ContractError;

pub fn advance_randomness(deps: DepsMut) -> Result<(), ContractError> {
    RANDOM_NONCE.update(deps.storage, |nonce| -> Result<_, ContractError> {
        return Ok(nonce + 1);
    })?;

    return Ok(());
}
