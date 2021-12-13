use ic_agent::{Agent, ic_types::Principal, agent::http_transport::ReqwestHttpReplicaV2Transport};
pub mod QueryFunctions;
pub mod PostProcess;
pub mod SQLProcess;
pub mod Threads;
use std::collections::{HashSet,VecDeque,HashMap};
use candid::types::number::{Int, Nat};
use QueryFunctions::EXT as EXT_query;
use QueryFunctions::DataStructure::_EXT as EXT_data;
use PostProcess::EXT as EXT_process;

// use QueryFunctions::PUNK as PUNK_query;
// use QueryFunctions::DataStructure::_PUNK as PUNK_data;


// use QueryFunctions::DFINANCE::DSWAP as DSWAP_query;
// use QueryFunctions::DataStructure::_DFINANCE::_DSWAP as DSWAP_data;
// use QueryFunctions::DFINANCE::REGISTRY as Dfinance_registry;
// use QueryFunctions::DataStructure::_DFINANCE::_REGISTRY as REGISTRY_data;

// use ic_agent::{Agent, ic_types::Principal,agent::http_transport::ReqwestHttpReplicaV2Transport};
// use crate::QueryFunctions::DataStructure::_DFINANCE::_DTOKEN as token;
// use candid::types::number::Nat;
// use num_bigint::{BigUint,ToBigUint};
// use crate::QueryFunctions::DFINANCE::DTOKEN as token_query;

use QueryFunctions::WALL as WALL_query;
use QueryFunctions::DataStructure::_WALL as WALL_data;

use PostProcess::utils;
use PostProcess::types::NFT as NFT_data;
use SQLProcess::NFT;

use Threads::PUNK as PUNK_threads;
use Threads::EXT as EXT_threads;



pub async fn test_punk_threads() {
    let Canister_id = String::from("qcg3w-tyaaa-aaaah-qakea-cai"); //punk
    let agent = Agent::builder()
	.with_transport(ReqwestHttpReplicaV2Transport::create("https://ic0.app/").unwrap())
	.build()
	.unwrap();

    let mut nft_info = NFT_data::NewNFTGeneralInfo(Principal::from_text("qcg3w-tyaaa-aaaah-qakea-cai").unwrap(), String::from("ICPUNK"), String::from("ICPUNK"), 10000 as u64, 0 as u8);
    PUNK_threads::PUNK_Listings_thread(&mut nft_info, &agent).await;
    println!("{:#?}", nft_info.floor_price);
    println!("{:#?}", nft_info.listing_number);

    PUNK_threads::PUNK_transactions_thread(&mut nft_info, &agent).await;
    println!("{:#?}", nft_info.average_price);
    println!("{:#?}", nft_info.total_txs);
    println!("{:#?}", nft_info.tx_amount_in_past_24h);
    println!("{:#?}", nft_info.volume_in_past_24h);
    println!("{:#?}", nft_info.last_tx_time);
    println!("{:#?}", nft_info.cur_period_volume);
    for v in nft_info.volumes_in_past_24{
        if v!=0{
            println!{"{:#?}", v};
        }
    }
}

pub async fn test_EXT_threads() {
    let Canister_id = String::from("njgly-uaaaa-aaaah-qb6pa-cai"); //EXT
    let agent = Agent::builder()
	.with_transport(ReqwestHttpReplicaV2Transport::create("https://ic0.app/").unwrap())
	.build()
	.unwrap();

    let mut nft_info = NFT_data::NewNFTGeneralInfo(Principal::from_text("njgly-uaaaa-aaaah-qb6pa-cai").unwrap(), String::from("ICPUNK"), String::from("ICPUNK"), 10000 as u64, 0 as u8);
    EXT_threads::EXT_Listings_thread(&mut nft_info, &agent).await;
    println!("{:#?}", nft_info.floor_price);
    println!("{:#?}", nft_info.listing_number);

    EXT_threads::EXT_transactions_thread(&mut nft_info, &agent).await;
    nft_info.query_update(1638333657462808860 as u64 + nft_info.gap * 2);
    println!("{:#?}", nft_info.average_price);
    println!("{:#?}", nft_info.total_txs);
    println!("{:#?}", nft_info.tx_amount_in_past_24h);
    println!("{:#?}", nft_info.volume_in_past_24h);
    println!("{:#?}", nft_info.last_tx_time);
    println!("{:#?}", nft_info.cur_period_volume);
    for v in nft_info.volumes_in_past_24{
        if v!=0{
            println!{"{:#?}", v};
        }
    }
    //println!("{:#?}", nft_info.volumes_in_past_24);
}




#[actix_web::main]
async fn main() {
    let principal = Principal::from_text("wvfv2-tk6ba-62pub-fqu5l-ppnb6-owke3-57sjp-egw3b-m6rhu-f2rfm-zqe").unwrap();
    let Canister_id = String::from("rivyl-6aaaa-aaaaf-qaapq-cai"); //ext
    let agent = Agent::builder()
	.with_transport(ReqwestHttpReplicaV2Transport::create("https://ic0.app/").unwrap())
	.build()
	.unwrap();
    let data = WALL_query::query_ProfilebyPrincipal(&Canister_id, &agent, &principal).await;
    //let num = utils::Nat2u64(&num);
    println!("{:#?}", data);
}
    //test_punk_threads().await;
    //let a  = PostProcess::EXT::TokenIdentifier2TokenId(&String::from("3mujy-cakor-uwiaa-aaaaa-b4arg-qaqca-aaa2r-q"));
    //println!("{:#?}", a);
    //test_EXT_threads().await;

    // let Canister_id = String::from("6gqxb-wiaaa-aaaab-abbgq-cai");

    // //let tokenindex = EXT_process::TokenIdentifier2TokenId(&String::from("hancg-5ykor-uwiaa-aaaaa-b4aaq-maqca-aabuk-a")) as u32;


    // //let Canister_id = Principal::from_text("e3izy-jiaaa-aaaah-qacbq-cai").unwrap(); //ext
    

    // //let s = EXT_process::TokenId2TokenIdentifier(tokenindex, &Canister_id);
    
    // // let principal = Principal::from_text("hr6jv-v65mn-lgvm3-yhiqe-ox7eo-ilbpj-2zz7e-zhvwq-vi6v6-uymok-4ae").unwrap();
    // // let identifier = utils::Principal2Identifier(&principal);
    
    // // println!("{:#?}", identifier);

    // //let Canister_id = String::from("qcg3w-tyaaa-aaaah-qakea-cai"); //punk
    // let agent = Agent::builder()
	// .with_transport(ReqwestHttpReplicaV2Transport::create("https://ic0.app/").unwrap())
	// .build()
	// .unwrap();

    // //let data: DSWAP_data::PairInfoExt = DSWAP_query::query_getPair(&Canister_id, &agent, &Principal::from_text("6gqxb-wiaaa-aaaab-abbgq-cai").unwrap(), &Principal::from_text("6gqxb-wiaaa-aaaab-abbgq-cai").unwrap()).await;
    
    // //let data: PUNK_data::transaction = PUNK_query::query_HistoryByIndex(&Canister_id, &agent, &PUNK_data::txAmount::from(44684)).await;
    // //let data: PUNK_data::txAmount = PUNK_query::query_txAmount(&Canister_id, &agent).await;


//     // //let data: PUNK_data::transactions = PUNK_query::query_allhistory(&Canister_id, &agent).await;
//     // // let data: PUNK_data::StorageCanister = PUNK_query::query_storage_canister(&Canister_id, &agent).await;
//     // // let data_string = data.unwrap().to_text();
//     // //let data: PUNK_data::Listings = PUNK_query::query_listings(&Canister_id, &agent, &PUNK_data::LenofListings::from(0)).await;


    // //let data: PUNK_data::transactions = PUNK_query::query_allhistory(&Canister_id, &agent).await;
    // // let data: PUNK_data::StorageCanister = PUNK_query::query_storage_canister(&Canister_id, &agent).await;
    // // let data_string = data.unwrap().to_text();
    // //let data: PUNK_data::Listings = PUNK_query::query_listings(&Canister_id, &agent, &PUNK_data::LenofListings::from(0)).await;

    // let data: Nat = PUNK_query::query_listing_len(&Canister_id, &agent).await;
    // println!("{:#?}", data);

    // //let data: EXT_data::Listings = EXT_query::query_listings(&Canister_id, &agent).await;
    // // println!("{:#?}",data);
    // // let data: EXT_data::Registrys = EXT_query::query_getRegistry(&Canister_id, &agent).await;
    // // println!("{:#?}",data);
    // // let data: EXT_data::transactions = EXT_query::query_transactions(&Canister_id, &agent).await;
    // // let last_data: &EXT_data::transaction = &data[1000];
    // // let token_id: EXT_data::TokenIndex = EXT_process::TokenIdentifier2TokenId(&last_data.tokenIdentifier);
    // // println!("{:#?}",token_id);

