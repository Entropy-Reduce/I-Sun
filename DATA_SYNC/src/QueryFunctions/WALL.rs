use ic_agent::{Agent, ic_types::Principal, agent::http_transport::ReqwestHttpReplicaV2Transport};
use candid::{CandidType,Encode,Decode, types::number::Int};
use serde::Deserialize;
// use std::time::Duration;
use std::string::String;

use crate::QueryFunctions::DataStructure::_WALL as WALL_data;

pub async fn query_ProfilebyPrincipal(Canister_id: &String, agent: &Agent, principal: &Principal) -> Option<WALL_data::Profile>{
    println!("Start! Canister_id {:#?}", Canister_id);
    let canister_str = String::from(Canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getProfileByPrincipal");
    let input = principal;

    let response = agent.query(&canister_id, function_str)
    .with_arg(Encode!(input).unwrap())
    .call()
    .await
    .unwrap();
    let result = Decode!(response.as_slice(),  Option<WALL_data::Profile>);
    //result
    
    //println!("{:#?}",result);
    result.expect("List error")

}


pub async fn query_list(Canister_id: &String, agent: &Agent) -> WALL_data::Profiles{
    println!("Start! Canister_id {:#?}", Canister_id);
    let canister_str = String::from(Canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("list");
    let input = WALL_data::Empty{};

    let response = agent.query(&canister_id, function_str)
    .with_arg(Encode!(&input).unwrap())
    .call()
    .await
    .unwrap();
    let result = Decode!(response.as_slice(), WALL_data::Profiles);
    //result
    
    //println!("{:#?}",result);
    result.expect("List error")

}