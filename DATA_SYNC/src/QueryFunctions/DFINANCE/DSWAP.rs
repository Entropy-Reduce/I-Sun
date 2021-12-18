use ic_agent::{Agent, ic_types::Principal};
use candid::types::number::Nat;
use candid::{Encode,Decode};

use crate::QueryFunctions::DataStructure::_DFINANCE::_DSWAP as DSWAP_DATA;


pub async fn get_pair(canister_id: &String, agent: &Agent, token0: &Principal, token1: &Principal) -> DSWAP_DATA::PairInfoExt{
    // println!("[Query DSWP:get_pair]:Start! Canister_id {:#?}", canister_id);
    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getPair");

    let response = loop{
        let response = agent.query(&canister_id, function_str.clone())
        .with_arg(Encode!(token0,token1).unwrap())
        .call()
        .await;
        if let Ok(message) = response{
            break message;
        }
    };

    let result = Decode!(response.as_slice(), Option<DSWAP_DATA::PairInfoExt>);
    result.unwrap().expect("Listing error")
}

pub async fn get_dswap_info(canister_id: &String, agent: &Agent) -> DSWAP_DATA::DSwapInfo{
    // println!("[Query DSWP:get_dswap_info]:Start! Canister_id {:#?}", canister_id);
    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getDSwapInfo");
    let empty = DSWAP_DATA::Empty{};


    let response = loop{
        let response = agent.query(&canister_id, function_str.clone())
        .with_arg(Encode!(&empty).unwrap())
        .call()
        .await;
        if let Ok(message) = response{
            break message;
        }
    };
    
    let result = Decode!(response.as_slice(), DSWAP_DATA::DSwapInfo);
    result.unwrap()
}

pub async fn get_user_info(canister_id: &String, agent: &Agent, user_principal: &Principal) -> DSWAP_DATA::UserInfo {
    // println!("[Query DSWP:get_user_info]:Start! Canister_id {:#?}", canister_id);
    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getUserInfo");

    let response = loop{
        let response = agent.query(&canister_id, function_str.clone())
        .with_arg(Encode!(user_principal).unwrap())
        .call()
        .await;
        if let Ok(message) = response{
            break message;
        }
    };

    let result = Decode!(response.as_slice(), DSWAP_DATA::UserInfo);
    result.unwrap()
}

pub async fn get_user_info_above(canister_id: &String, agent: &Agent, user_principal: &Principal, token_balance_low_bound:&Nat, lp_token_balance_low_bound:&Nat) -> Vec<(String,Nat)> {
    // println!("[Query DSWP:get_user_info_above]:Start! Canister_id {:#?}", canister_id);

    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getUserBalances");

    let response = loop{
        let response = agent.query(&canister_id, function_str.clone())
        .with_arg(Encode!(user_principal,token_balance_low_bound,lp_token_balance_low_bound).unwrap())
        .call()
        .await;
        if let Ok(message) = response{
            break message;
        }
    };


    let result = Decode!(response.as_slice(), Vec<(String,Nat)>);
    result.unwrap()
}

pub async fn get_user_balances(canister_id: &String, agent: &Agent, user_principal: &Principal) ->Vec<(String,Nat)> {
    // println!("[Query DSWP:get_user_balances]:Start! Canister_id {:#?}", canister_id);

    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getUserBalances");

    let response = loop{
        let response = agent.query(&canister_id, function_str.clone())
        .with_arg(Encode!(user_principal).unwrap())
        .call()
        .await;
        if let Ok(message) = response{
            break message;
        }
    };

    let result = Decode!(response.as_slice(), Vec<(String,Nat)>);
    result.unwrap()
}
//transactions 
pub async fn history_size(canister_id: &String, agent: &Agent) -> Nat {
    // println!("[Query DSWP:history_size]:Start! Canister_id {:#?}", canister_id);

    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("historySize");
    let empty = DSWAP_DATA::Empty{};

    let waiter = garcon::Delay::builder()
    .throttle(std::time::Duration::from_millis(500))
    .timeout(std::time::Duration::from_secs(60 * 5))
    .build();

    let response = loop{
        let response = agent.update(&canister_id, function_str.clone())
        .with_arg(Encode!(&empty).unwrap())
        .call_and_wait(waiter.clone())
        .await;
        if let Ok(message) = response{
            break message;
        }
    };


    let result = Decode!(response.as_slice(), Nat);
    return result.unwrap();
}

pub async fn get_transactions(canister_id: &String, agent: &Agent,from: &Nat, to:&Nat) -> DSWAP_DATA::TxRecords{
    // println!("[Query DSWP:get_transactions]:Start! Canister_id {:#?}", canister_id);

    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getTransactions");
    
    let waiter = garcon::Delay::builder()
        .throttle(std::time::Duration::from_millis(500))
        .timeout(std::time::Duration::from_secs(60 * 5))
        .build();

    let response = loop{
        let response = agent.update(&canister_id, function_str.clone())
        .with_arg(Encode!(from,to).unwrap())
        .call_and_wait(waiter.clone())
        .await;
        if let Ok(message) = response{
            break message;
        }
    };

    let result = Decode!(response.as_slice(), DSWAP_DATA::TxRecords);
    result.unwrap()
}
#[cfg(test)]
mod tests {
    use ic_agent::{Agent,ic_types::Principal, agent::http_transport::ReqwestHttpReplicaV2Transport};
    use num_bigint::ToBigUint;
    use super::*;

    #[actix_rt::test]
    async fn test_query_function(){
        let agent = Agent::builder()
	    .with_transport(ReqwestHttpReplicaV2Transport::create("https://ic0.app/").unwrap())
	    .build()
	    .unwrap();
        
        // let user_principal = Principal::from_text("4qehi-lqyo6-afz4c-hwqwo-lubfi-4evgk-5vrn5-rldx2-lheha-xs7a4-gae").unwrap();
        let swap_canister_id = String::from("4grvp-niaaa-aaaah-qboma-cai");
        let storage_canister_id = String::from("4bqt3-aqaaa-aaaah-qbomq-cai");
        let user_principal = Principal::from_text("4qehi-lqyo6-afz4c-hwqwo-lubfi-4evgk-5vrn5-rldx2-lheha-xs7a4-gae").unwrap();
        let token0 = Principal::from_text("5l7rb-caaaa-aaaah-qbolq-cai").unwrap();
        let token1 = Principal::from_text("42vp6-2iaaa-aaaah-qbooa-cai").unwrap();
        let zero = Nat(ToBigUint::to_biguint(&0).unwrap());
        let to = Nat(ToBigUint::to_biguint(&10).unwrap());
        get_transactions(&storage_canister_id, &agent,&zero,&to).await;
        // println!("Transactions is {:#?}",transactions);
        get_dswap_info(&swap_canister_id, &agent).await;
        get_pair(&swap_canister_id, &agent,&token0,&token1).await;
        get_user_balances(&swap_canister_id, &agent,&user_principal).await;
        get_user_info(&swap_canister_id, &agent,&user_principal).await;
        get_user_info_above(&swap_canister_id, &agent,&user_principal,&zero,&zero).await;
    }
}