use cosmwasm_std::{Deps, WasmMsg, to_binary, Addr};

use cw721::{Cw721QueryMsg, Cw721ExecuteMsg, OwnerOfResponse};


/* 
#[cfg(test)]
pub fn is_owner_nft(_deps: Deps, _nft_address: String, _token_id: String) -> bool {
    return true;

}
*/

//#[cfg(not(test))]
pub fn is_owner_nft(deps: Deps, recipient: Addr, nft_address: String, token_id: String) -> bool {
    let msg = Cw721QueryMsg::OwnerOf { token_id: token_id, include_expired: Some(false) };
    let res: OwnerOfResponse = deps.querier.query_wasm_smart(nft_address, &msg).unwrap();

    let nft_owner = deps.api.addr_validate(res.owner.as_str()).unwrap();

    return nft_owner == recipient;
}


pub fn prep_msg_transfer_ownership_nft(recipient: String, nft_address: String, token_id: String) -> WasmMsg {
    let msg = Cw721ExecuteMsg::TransferNft { recipient: recipient, token_id: token_id };
    let wrapped = WasmMsg::Execute { contract_addr: nft_address, msg: to_binary(&msg).unwrap(), funds: vec![]};   

    return wrapped;
}