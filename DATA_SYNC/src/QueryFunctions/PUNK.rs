use ic_agent::{Agent, ic_types::Principal};
use candid::{Encode,Decode};
use candid::Nat;
use crate::QueryFunctions::DataStructure::_PUNK as PUNK_data;
use crate::PostProcess::utils as utils;

pub async fn query_listing_len(Canister_id: &String, agent: &Agent) -> Nat{
    println!("Start! Canister_id {:#?}", Canister_id);
    let canister_str = String::from(Canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("get_listed_count");
    let input = PUNK_data::Empty{};

    let response = loop{
        let response = agent.query(&canister_id, function_str.clone())
        .with_arg(Encode!(&input).unwrap())
        .call()
        .await;
        if let Ok(message) = response{
            break message;
        }
    };

    let result = Decode!(response.as_slice(), Nat);
    //result
    
    //println!("{:#?}",result);
    result.expect("Listing error")

}

pub async fn query_page_listings(Canister_id: &String, agent: &Agent, pagecount: &PUNK_data::LenofListings) -> PUNK_data::Listings{
    println!("Start! Canister_id {:#?}", Canister_id);
    let canister_str = String::from(Canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("get_listed");
    let input = pagecount;

    let response = loop{
        let response = agent.query(&canister_id, function_str.clone())
        .with_arg(Encode!(&input).unwrap())
        .call()
        .await;
        if let Ok(message) = response{
            break message;
        }
    };
    
    let result = Decode!(response.as_slice(),PUNK_data::Listings);
    //result
    
    //println!("{:#?}",result);
    result.expect("Listing error")

}



pub async fn query_storage_canister(Canister_id: &String, agent: &Agent) -> PUNK_data::StorageCanister{
    println!("Start! Canister_id {:#?}", Canister_id);
    let canister_str = String::from(Canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("get_storage_canister");
    let input = PUNK_data::Empty{};
    let response = loop{
        let response = agent.query(&canister_id, function_str.clone())
        .with_arg(Encode!(&input).unwrap())
        .call()
        .await;
        if let Ok(message) = response{
            break message;
        }
    };
    
    let result = Decode!(response.as_slice(), PUNK_data::StorageCanister);
    //result
    
    //println!("{:#?}",result);
    result.expect("Listing error")

}

pub async fn query_allhistory(Canister_id: &String, agent: &Agent) -> PUNK_data::transactions{
    println!("Start! Canister_id {:#?}", Canister_id);
    let canister_str = String::from(Canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("get_storage_canister");
    let input = PUNK_data::Empty{};

    let response = agent.query(&canister_id, function_str)
    .with_arg(Encode!(&input).unwrap())
    .call()
    .await
    .unwrap();
    let result = Decode!(response.as_slice(), PUNK_data::StorageCanister);
    //result
    let result = result.expect("Listing error").unwrap().to_text();

    let canister_str = String::from(result);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("allHistory");
    let input = PUNK_data::Empty{};

    let response = agent.query(&canister_id, function_str)
    .with_arg(Encode!(&input).unwrap())
    .call()
    .await
    .unwrap();
    let result = Decode!(response.as_slice(), PUNK_data::transactions);

    //println!("{:#?}",result);
    result.expect("Listing error")

}

pub async fn query_txAmount(Canister_id: &String, agent: &Agent) -> PUNK_data::txAmount{
    println!("Start! Canister_id {:#?}", Canister_id);
    let canister_str = String::from(Canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("get_storage_canister");
    let input = PUNK_data::Empty{};

    let response = loop{
        let response = agent.query(&canister_id, function_str.clone())
        .with_arg(Encode!(&input).unwrap())
        .call()
        .await;
        if let Ok(message) = response{
            break message;
        }
    };

    let result = Decode!(response.as_slice(), PUNK_data::StorageCanister);
    //result
    let result = result.expect("Listing error").unwrap().to_text();

    let canister_str = String::from(result);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("txAmount");
    let input = PUNK_data::Empty{};

    let response = loop{
        let response = agent.query(&canister_id, function_str.clone())
        .with_arg(Encode!(&input).unwrap())
        .call()
        .await;
        if let Ok(message) = response{
            break message;
        }
    };

    let result = Decode!(response.as_slice(), PUNK_data::txAmount);

    //println!("{:#?}",result);
    result.expect("Listing error")

}


pub async fn query_HistoryByIndex(Canister_id: &String, agent: &Agent, index: &PUNK_data::txAmount) -> PUNK_data::transaction{
    println!("Start! Canister_id {:#?}", Canister_id);
    let canister_str = String::from(Canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("get_storage_canister");
    let input = PUNK_data::Empty{};

    let response = loop{
        let response = agent.query(&canister_id, function_str.clone())
        .with_arg(Encode!(&input).unwrap())
        .call()
        .await;
        if let Ok(message) = response{
            break message;
        }
    };
    let result = Decode!(response.as_slice(), PUNK_data::StorageCanister);
    //result
    let result = result.expect("Listing error").unwrap().to_text();

    let canister_str = String::from(result);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("getHistoryByIndex");
    let input = index;

    let response = loop{
        let response = agent.query(&canister_id, function_str.clone())
        .with_arg(Encode!(&input).unwrap())
        .call()
        .await;
        if let Ok(message) = response{
            break message;
        }
    };
    let result = Decode!(response.as_slice(), PUNK_data::transaction);

    //println!("{:#?}",result);
    result.expect("Listing error")

}


pub async fn query_supply(Canister_id: &String, agent: &Agent) -> Nat{
    println!("Start! Canister_id {:#?}", Canister_id);

    let canister_str = String::from(Canister_id);
    let canister_id = Principal::from_text(canister_str).unwrap();
    let function_str = String::from("total_supply");
    let input = PUNK_data::Empty{};
    // let mut transactions : Vec <transaction> = vec![]

   let response = loop{
        let response = agent.query(&canister_id, function_str.clone())
        .with_arg(Encode!(&input).unwrap())
        .call()
        .await;
        if let Ok(message) = response{
            break message;
        }
    };
    

    let result = Decode!(response.as_slice(),Nat);
    //result
    
    //println!("{:#?}",result);
    result.expect("Listing error")
}
