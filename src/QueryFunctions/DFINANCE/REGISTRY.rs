use ic_agent::{Agent, ic_types::Principal};
use candid::{Encode,Decode};
use crate::QueryFunctions::DataStructure::_DFINANCE::_REGISTRY as registry;

pub async fn query_token_list(canister_id: &String, agent: &Agent) -> registry::TokenList{
    println!("Start! Canister_id");

    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getTokenList");
    let empty = registry::Empty{};
    // // let mut transactions : Vec <transaction> = vec![]

    let response = agent.query(&canister_id, function_str)
    .with_arg(Encode!(&empty).unwrap())
    //.expire_after(Duration::from_secs(5))
    .call()
    .await
    .unwrap();
    

    let result = Decode!(response.as_slice(),registry::TokenList);
    //result
    //println!("{:#?}",result);
    result.expect("Listing error")
}

pub async fn get_user_token_list(canister_id: &String, agent: &Agent,principal:&Principal) -> registry::TokenList {
    println!("Start! Canister_id");

    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getUserTokenList");
    // // let mut transactions : Vec <transaction> = vec![]

    let response = agent.query(&canister_id, function_str)
    .with_arg(Encode!(principal).unwrap())
    //.expire_after(Duration::from_secs(5))
    .call()
    .await
    .unwrap();
    

    let result = Decode!(response.as_slice(),registry::TokenList);
    //result
    //println!("{:#?}",result);
    result.expect("Listing error")
}