use ic_agent::{Agent, ic_types::Principal};
use candid::{Encode,Decode, types::number::Nat};
use crate::QueryFunctions::DataStructure::_DFINANCE::_DTOKEN as token;

pub async fn get_transactions(canister_id: &String, agent: &Agent,from: &Nat, to:&Nat) -> token::Transactions{
    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getTransactions");

    let response = agent.query(&canister_id, function_str)
    .with_arg(Encode!(from,to).unwrap())
    .call()
    .await
    .unwrap();
    let result = Decode!(response.as_slice(), token::Transactions);
    result.unwrap()
}

pub async fn history_size(canister_id: &String, agent: &Agent) -> Nat {
    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("historySize");
    let empty = token::Empty{};

    let response = agent.query(&canister_id, function_str)
    .with_arg(Encode!(&empty).unwrap())
    .call()
    .await
    .unwrap();

    let result = Decode!(response.as_slice(), Nat);
    return result.unwrap();
}

pub async fn get_user_transactions(canister_id: &String, agent: &Agent,principal:&Principal, from: &Nat, to:&Nat) -> token::Transactions{
    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getUserTransactions");

    let response = agent.query(&canister_id, function_str)
    .with_arg(Encode!(principal,from,to).unwrap())
    .call()
    .await
    .unwrap();
    let result = Decode!(response.as_slice(), token::Transactions);
    result.expect("Listing error")
}

pub async fn get_user_transactiom_amount(canister_id: &String, agent: &Agent, principal:&Principal) -> Nat{
    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getUserTransactionAmount");

    let response = agent.query(&canister_id, function_str)
    .with_arg(Encode!(principal).unwrap())
    .call()
    .await
    .unwrap();
    let result = Decode!(response.as_slice(), Nat);
   result.expect("Listing error")
}

#[cfg(test)]
mod tests {
    use ic_agent::{Agent,ic_types::Principal, agent::http_transport::ReqwestHttpReplicaV2Transport};
    use candid::types::number::Nat;
    use num_bigint::ToBigUint;
    use super::*;

    #[actix_rt::test]
    async fn test_query_function(){
        let principal = Principal::from_text("4qehi-lqyo6-afz4c-hwqwo-lubfi-4evgk-5vrn5-rldx2-lheha-xs7a4-gae").unwrap();
        let canister_id = String::from("5l7rb-caaaa-aaaah-qbolq-cai");
        let agent = Agent::builder()
	    .with_transport(ReqwestHttpReplicaV2Transport::create("https://ic0.app/").unwrap())
	    .build()
	    .unwrap();
        let from = Nat(ToBigUint::to_biguint(&0).unwrap());
        let to = Nat(ToBigUint::to_biguint(&10).unwrap());
        get_transactions(&canister_id, &agent,&from,&to).await;
        history_size(&canister_id, &agent).await;
        get_user_transactions(&canister_id, &agent,&principal,&from,&to).await;
        get_user_transactiom_amount(&canister_id, &agent,&principal).await;
    }

}

