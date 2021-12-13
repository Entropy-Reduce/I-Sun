use ic_agent::{Agent, ic_types::Principal};
use candid::{Encode,Decode};
use candid::types::number::Nat;
use crate::QueryFunctions::DataStructure::_DFINANCE::_STAKING as staking;

pub async fn get_all_open_pools(canister_id: &String, agent: &Agent) -> staking::PoolList{
    println!("Start! Canister_id");

    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getAllOpenPools");
    let empty = staking::Empty{};

    let response = agent.query(&canister_id, function_str)
    .with_arg(Encode!(&empty).unwrap())
    //.expire_after(Duration::from_secs(5))
    .call()
    .await
    .unwrap();

    let result = Decode!(response.as_slice(),staking::PoolList);
    result.expect("Listing error")
}

pub async fn get_all_pools(canister_id: &String, agent: &Agent) -> staking::PoolList{
    println!("Start! Canister_id");

    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getAllPools");
    let empty = staking::Empty{};

    let response = agent.query(&canister_id, function_str)
    .with_arg(Encode!(&empty).unwrap())
    //.expire_after(Duration::from_secs(5))
    .call()
    .await
    .unwrap();

    let result = Decode!(response.as_slice(),staking::PoolList);
    result.expect("Listing error")
}

pub async fn get_user_info(canister_id: &String, agent: &Agent, pool_id : &Nat, principal:&Principal) -> staking::UserInfoExt{
    println!("Start! Canister_id");

    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getUserInfo");

    let response = agent.query(&canister_id, function_str)
    .with_arg(Encode!(pool_id,principal).unwrap())
    .call()
    .await
    .unwrap();

    let result = Decode!(response.as_slice(),staking::UserInfoExt);
    result.expect("Listing error")
}

pub async fn get_user_pending_reward(canister_id: &String, agent: &Agent,pool_id:&Nat, principal:&Principal) -> Nat{
    println!("Start! Canister_id");

    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getUserPendingReward");

    let response = agent.query(&canister_id, function_str)
    .with_arg(Encode!(pool_id,principal).unwrap())
    .call()
    .await
    .unwrap();

    let result = Decode!(response.as_slice(),Nat);
    result.expect("Listing error")
}

#[cfg(test)]
mod tests {
    use ic_agent::{Agent,ic_types::Principal, agent::http_transport::ReqwestHttpReplicaV2Transport};
    // use crate::QueryFunctions::DataStructure::_DFINANCE::_STAKING as staking;
    use candid::types::number::Nat;
    use num_bigint::ToBigUint;
    use super::*;

    #[actix_rt::test]
    async fn test_query_function(){
        let principal = Principal::from_text("4qehi-lqyo6-afz4c-hwqwo-lubfi-4evgk-5vrn5-rldx2-lheha-xs7a4-gae").unwrap();
        let canister_id = String::from("4ityh-wyaaa-aaaah-qbona-cai");
        let agent = Agent::builder()
	    .with_transport(ReqwestHttpReplicaV2Transport::create("https://ic0.app/").unwrap())
	    .build()
	    .unwrap();
        let pool_id = Nat(ToBigUint::to_biguint(&0).unwrap());
        get_all_open_pools(&canister_id, &agent).await;
        get_all_pools(&canister_id, &agent).await;

        get_user_info(&canister_id, &agent,&pool_id,&principal).await;
        get_user_pending_reward(&canister_id, &agent,&pool_id, &principal).await;
    }

}