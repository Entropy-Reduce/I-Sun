use ic_agent::{Agent, ic_types::Principal};
use candid::{Encode,Decode};
use crate::QueryFunctions::DataStructure::_DFINANCE::_REGISTRY as registry;

pub async fn query_token_list(canister_id: &String, agent: &Agent) -> registry::TokenList{
    // println!("[Query Registry:query_token_list]:Start! Canister_id {:#?}", canister_id);

    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getTokenList");
    let empty = registry::Empty{};
    // // let mut transactions : Vec <transaction> = vec![]

    let response = loop{
        let response = agent.query(&canister_id, function_str.clone())
        .with_arg(Encode!(&empty).unwrap())
        .call()
        .await;
        if let Ok(message) = response{
            break message;
        }
    };

    let result = Decode!(response.as_slice(),registry::TokenList);
    //result
    //println!("{:#?}",result);
    result.expect("Listing error")
}

pub async fn get_user_token_list(canister_id: &String, agent: &Agent,principal:&Principal) -> registry::TokenList {
    // println!("[Query Registry:query_token_list]:Start! Canister_id {:#?}", canister_id);

    let canister_str = String::from(canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getUserTokenList");
    // // let mut transactions : Vec <transaction> = vec![]

    let response = loop{
        let response = agent.query(&canister_id, function_str.clone())
        .with_arg(Encode!(principal).unwrap())
        .call()
        .await;
        if let Ok(message) = response{
            break message;
        }
    };
    

    let result = Decode!(response.as_slice(),registry::TokenList);
    //result
    //println!("{:#?}",result);
    result.expect("Listing error")
}