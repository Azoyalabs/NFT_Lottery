#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier};
    use cosmwasm_std::{coins, from_binary, Coin, MemoryStorage, OwnedDeps, Uint128};

    use crate::execute_messages::msg::{ExecuteMsg, InstantiateMsg};

    use crate::contract::{execute, instantiate, query};

    use crate::query::query_message::QueryMsg;
    use crate::query::query_response::LotteryPrizesResponse;
    use crate::structs::{LotteryStatus, Prize};

    const TEST_DENOM: &str = "uusd";
    const TEST_CREATOR: &str = "creator";
    const TEST_USER: &str = "user";
    const TEST_USER2: &str = "user2";

    const TESTNET_BEACON: &str = "terra1a62jxn3hh54fa5slan4dkd7u6v4nzgz3pjhygm";

    const _TEST_PRICE: u64 = 10000000;

    const _TEST_INVALID_DENOM: &str = "notuusd";

    fn instantiate_contract() -> OwnedDeps<MemoryStorage, MockApi, MockQuerier> {
        let mut deps = mock_dependencies(&[Coin {
            denom: TEST_DENOM.to_string(),
            amount: Uint128::from(100u64),
        }]);

        let msg = InstantiateMsg {
            beacon_address: TESTNET_BEACON.to_string(),
        };
        let info = mock_info(TEST_CREATOR, &coins(1000, TEST_DENOM));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        return deps;
    }

    #[test]
    fn instantiate_success() {
        let _deps = instantiate_contract();
    }

    #[test]
    fn new_full_cycle() {
        let mut deps = instantiate_contract();

        // create new lottery
        let msg = ExecuteMsg::CreateLottery {
            entry_price: None,
            admins: vec![TEST_CREATOR.to_string()],
            prizes: None,
        };
        let info = mock_info(TEST_CREATOR, &vec![]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // set prizes
        let nft_address = "TEST_NFT".to_string();
        let prizes: Vec<Prize> = (0..10)
            .into_iter()
            .map(|elem| Prize {
                nft_address: nft_address.clone(),
                token_id: elem.to_string(),
            })
            .collect();

        let msg = ExecuteMsg::UpdatePrizes {
            id_lottery: 0,
            prizes: prizes,
        };
        let info = mock_info(TEST_CREATOR, &vec![]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // pricing? later
        let pricing = Coin {
            denom: TEST_DENOM.to_string(),
            amount: Uint128::from(10000u32),
        };
        let msg = ExecuteMsg::UpdateEntryPrice {
            id_lottery: 0,
            entry_price: Some(pricing.clone()),
        };
        let info = mock_info(TEST_CREATOR, &vec![]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // open registration
        let msg = ExecuteMsg::UpdateLotteryStatus {
            id_lottery: 0,
            new_status: LotteryStatus::Registration,
        };
        let info = mock_info(TEST_CREATOR, &vec![]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // register
        let msg = ExecuteMsg::Register { id_lottery: 0 };
        let info = mock_info(TEST_USER, &vec![pricing.clone()]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let msg = ExecuteMsg::Register { id_lottery: 0 };
        let info = mock_info(TEST_USER2, &vec![pricing.clone()]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // close registrations
        let msg = ExecuteMsg::UpdateLotteryStatus {
            id_lottery: 0,
            new_status: LotteryStatus::WaitingForDraw,
        };
        let info = mock_info(TEST_CREATOR, &vec![]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // draw
        // not implemented
        let msg = ExecuteMsg::DrawLottery { id_lottery: 0 };
        let info = mock_info(TEST_CREATOR, &vec![]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // allow claims
        let msg = ExecuteMsg::UpdateLotteryStatus {
            id_lottery: 0,
            new_status: LotteryStatus::PrizeDistribution,
        };
        let info = mock_info(TEST_CREATOR, &vec![]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // claim prizes
        // need draw to work

        let msg = QueryMsg::PrizePool { id_lottery: 0 };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();

        //let rslt: WinnersResponse = from_binary(&res).unwrap();
        let rslt: LotteryPrizesResponse = from_binary(&res).unwrap();

        println!("{:?}", rslt);
    }
}
