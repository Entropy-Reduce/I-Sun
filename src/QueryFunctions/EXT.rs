use ic_agent::{Agent, ic_types::Principal, agent::http_transport::ReqwestHttpReplicaV2Transport};
use candid::{CandidType,Encode,Decode, types::number::Int, types::number::Nat};
use serde::Deserialize;

use crate::QueryFunctions::DataStructure::_EXT as EXT_data;


pub async fn query_listings(Canister_id: &String, agent: &Agent) -> EXT_data::Listings{
    println!("Start! Canister_id");

    let canister_str = String::from(Canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("listings");
    let empty = EXT_data::Empty{};
    // let mut transactions : Vec <transaction> = vec![]

    let response = agent.query(&canister_id, function_str)
    .with_arg(Encode!(&empty).unwrap())
    //.expire_after(Duration::from_secs(5))
    .call()
    .await
    .unwrap();
    

    let result = Decode!(response.as_slice(),EXT_data::Listings);
    //result
    
    //println!("{:#?}",result);
    result.expect("Listing error")
}

pub async fn query_transactions(Canister_id: &String, agent: &Agent) -> EXT_data::transactions{
    println!("Start! Canister_id");

    let canister_str = String::from(Canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("transactions");
    let empty = EXT_data::Empty{};
    // let mut transactions : Vec <transaction> = vec![]

    let response = agent.query(&canister_id, function_str)
    .with_arg(Encode!(&empty).unwrap())
    //.expire_after(Duration::from_secs(5))
    .call()
    .await
    .unwrap();

    let result = Decode!(response.as_slice(),EXT_data::transactions);
    result.expect("transactions error")
    // transactions
    //println!("{:#?}",transactions);
}

pub async fn query_getRegistry(Canister_id: &String, agent: &Agent)->EXT_data::Registrys{
    println!("Start! Canister_id");

    let canister_str = String::from(Canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getRegistry");
    let empty = EXT_data::Empty{};

    let response = agent.query(&canister_id, function_str)
    .with_arg(Encode!(&empty).unwrap())
    //.expire_after(Duration::from_secs(5))
    .call()
    .await
    .unwrap();

    let result = Decode!(response.as_slice(), EXT_data::Registrys);
    //getregistrys
    //println!("{:#?}",transactions);
    result.expect("registry error!")
}

pub async fn query_supply(Canister_id: &String, agent: &Agent)->Nat{
    println!("Start! Canister_id");

    let canister_str = String::from(Canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("supply");
    let empty = String::from("aaaaaaaaaaaaaaa");

    let response = agent.query(&canister_id, function_str)
    .with_arg(Encode!(&empty).unwrap())
    //.expire_after(Duration::from_secs(5))
    .call()
    .await
    .unwrap();

    let result = Decode!(response.as_slice(), EXT_data::Result_2);
    let result = result.expect("supply error!");
    match result{
        EXT_data::Result_2::ok(n)=>n,
        _=>Nat::from(0),
    }
    //getregistrys
    //println!("{:#?}",transactions);
    //result.expect("registry error!")
}
