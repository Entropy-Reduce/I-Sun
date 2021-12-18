pub mod QueryFunctions;
pub mod PostProcess;
pub mod SQLProcess;
pub mod Threads;

use ic_agent::{Agent, ic_types::Principal,agent::http_transport::ReqwestHttpReplicaV2Transport};
use std::{time::Duration,thread::sleep,collections::{HashSet,VecDeque,HashMap}};
use candid::types::number::{Int, Nat};
use mysql::{Opts, Pool};
use tokio::runtime::Runtime;
use tokio::time::*;
use async_std::task;
use num_bigint::{BigUint,ToBigUint};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::{Json, Value};
use rocket::{State, get, post, routes, put, delete, catchers, catch};
use serde_json;


use QueryFunctions::EXT as EXT_query;
use QueryFunctions::DataStructure::_EXT as EXT_data;
use QueryFunctions::DataStructure::_DFINANCE::_DTOKEN as token;
use QueryFunctions::DFINANCE::DTOKEN as token_query;

use PostProcess::utils;

use PostProcess::EXT as EXT_process;
use PostProcess::PUNK as PUNK_process;
use PostProcess::DFINANCE as Dfinance_process;

use PostProcess::types::dapp_func;
use PostProcess::types::NFT as NFT_data;
use PostProcess::types::DFINANCE::dfinance as dfinance_data;
use PostProcess::types::DFINANCE::dswap as dswap_data;
use PostProcess::types::DFINANCE::token as token_data;


use SQLProcess::NFT;

use Threads::PUNK as PUNK_threads;
use Threads::EXT as EXT_threads;
use Threads::init_process::DAPP as DAPP;
use Threads::init_process::DAPP_info as DAPP_info;

//todo: 换成enum
//type Message<'r> = &'r State<Vec<DAPP_info>>;
type static_data<'r> = &'r State<(Vec<DAPP_info>, Pool)>;

#[derive(Serialize, Deserialize)]
struct general_info{
    canisterId: String,
    total_txs: u64,
    name: String,
    symbol: String,
    supply: u64,
    average_price: f64,
    lowest_price: f64,
    highest_price: f64,
    volume: f64,
    tx_amount_in_past_24h: u64,
    volume_in_past_24h: f64,
    //volume_change_in_past_24: f64,
    floor_price: f64,
    listing_number: u64,
}

#[get("/general_info")]
async fn get_dfinance_general_info(message: static_data<'_>) -> String {
    let mut response = String::from("Null");
    for item in message.0.iter(){
        if let DAPP_info::Dfinance_info(data) = &item{
            let mut info: dfinance_data::DfinanceInfo = (*data.read().unwrap()).clone();
            let mut pairs = HashMap::new();
            for (key,pair) in info.pairs.iter(){
                let symbol = Dfinance_process::get_dswap_token_symbol(message.1.clone(), key);
                let pair = dswap_data::PairInfo{
                    id: pair.id.clone(), // principal
                    supply: pair.supply,
                    token0: pair.token0.clone(), //Principal;
                    token1: pair.token1.clone(), //Principal;
                    lp_token: symbol.clone(),
                    creator: pair.creator.clone(),
                    last_update_time: pair.last_update_time,
                    price0_cumulative: pair.price0_cumulative,
                    price1_cumulative: pair.price1_cumulative,
                    k: pair.k,
                };
                pairs.insert(key.to_string(),pair);
            }
            info.pairs = pairs;
            response = serde_json::to_string(&info).unwrap();
        }   
    }
    return response;
}

#[get("/token/balances?<principal>")]
async fn get_token_user_balances(principal:String,message: static_data<'_>) -> String{
    let balances = Dfinance_process::get_token_user_balances(message.1.clone(), &principal);
    let response = serde_json::to_string(&balances).unwrap();
    return response;
}

#[get("/token/transactions?<principal>&<canister_id>&<from>&<to>")]
async fn get_token_user_transactions(principal:String,canister_id:String,from:usize,to:usize,message: static_data<'_>) -> String{
    let transactions = Dfinance_process::get_token_transactions_by_user(
                                        message.1.clone(), 
                                        &canister_id,
                                        &principal, 
                                        from, 
                                        to
                                    );
    let response = serde_json::to_string(&transactions).unwrap();
    return response;
}

#[get("/dswap/balances?<principal>")]
async fn get_dswap_user_balances(principal:String,message: static_data<'_>) -> String{
    let mut response = String::from("Null");
    for item in message.0.iter(){
        if let DAPP_info::Dfinance_info(data) = &item{
            let info: dfinance_data::DfinanceInfo = (*data.read().unwrap()).clone();
            let principal = Principal::from_text(&principal).unwrap();
            let agent = Agent::builder()
	            .with_transport(ReqwestHttpReplicaV2Transport::create("https://ic0.app/").unwrap())
	            .build()
	            .unwrap();
            let balances = Dfinance_process::get_dswap_user_balances(
                                                                    message.1.clone(),
                                                                    &principal, 
                                                                    &agent, 
                                                                    &info.dswap_canister_id).await;
            response = serde_json::to_string(&balances).unwrap();
        }  
    }
    return response;
}

#[get("/dswap/transactions?<principal>&<canister_id>&<from>&<to>")]
async fn get_dswap_user_transactions(principal:String,canister_id:String,from:usize,to:usize,message: static_data<'_>) -> String{
    let transactions = Dfinance_process::get_dswap_transactions_by_user(
                                        message.1.clone(), 
                                        &principal,
                                        &canister_id, 
                                        from, 
                                        to
                                    );
    println!("Transactions:{:#?}",transactions);
    let response = serde_json::to_string(&transactions).unwrap();
    return response;
}

#[get("/NFT/general_info")]
async fn get_general_info(message: static_data<'_>) -> String{
    let mut response: Vec<general_info> = Vec::new();
    for item in message.0.iter(){
        if let DAPP_info::EXT_info(data) = &item{
            let info: PostProcess::types::NFT::NFTGeneralInfo = (*data.read().unwrap()).clone();
            let send_info: general_info = general_info{
                canisterId: info.canisterId.to_text(),
                total_txs: info.total_txs,
                name: info.name,
                symbol: info.symbol,
                supply: info.supply,
                average_price: (info.average_price as f64) / (10_i32.pow(info.decimals as u32) as f64),
                lowest_price: (info.lowest_price as f64) / (10_i32.pow(info.decimals as u32) as f64),
                highest_price: (info.highest_price as f64) / (10_i32.pow(info.decimals as u32) as f64),
                volume: (info.volume as f64) / (10_i32.pow(info.decimals as u32) as f64),
                tx_amount_in_past_24h: info.tx_amount_in_past_24h,
                volume_in_past_24h: (info.volume_in_past_24h as f64) / (10_i32.pow(info.decimals as u32) as f64),
                //volume_change_in_past_24: info.volumet_change_in_past_24,
                floor_price: (info.floor_price as f64) / (10_i32.pow(info.decimals as u32) as f64),
                listing_number: info.listing_number,     
            };
            response.push(send_info);
        }
    }
    //response
    serde_json::to_string(&response).unwrap()
}

#[get("/NFT/user_info?<id>")]
async fn get_user_info(id: String, data: static_data<'_>) -> String{
    let mut conn = data.1.get_conn().unwrap();
    let mut mydapps: Vec<(String, u64, String, u64)> = Vec::new();
    let principal = Principal::from_text(&id).unwrap();
    let mut identifier = PostProcess::utils::Principal2Identifier(&principal, &mut conn);
    let mut all_dapps: HashMap<u64, Vec<u64>> = SQLProcess::NFT::sql_userinfo(&mut conn, &mut identifier);
    for (key, value) in all_dapps.iter(){
        let item = &data.0[(*key) as usize];
        if let DAPP_info::EXT_info(dapp_data) = item{
            let dapp_info = (*dapp_data.read().unwrap()).clone();
            let canister_id = dapp_info.canisterId.clone();
            for index in value.iter(){
                let token_url = PostProcess::utils::get_token_url(*index, &canister_id);
                mydapps.push((dapp_info.name.clone(), *index, token_url, *key));
            }
        }
        
    }

    //response
    serde_json::to_string(&mydapps).unwrap()
}

#[get("/NFT/nft_info?<dapp_id>&<index>")]
async fn get_nft_info(dapp_id: u64, index: u64, data: static_data<'_>) -> String{
    let mut conn = data.1.get_conn().unwrap();
    let mut mydapps: Vec<(String, String, f64, u64)> = Vec::new();
    let mut decimal = 0 as u8;
    let info = &data.0[dapp_id as usize];
    if let DAPP_info::EXT_info(dapp_data) = info{
        decimal = (*dapp_data.read().unwrap()).decimals.clone();
    }
    else if let DAPP_info::PUNK_info(dapp_data) = info{
        decimal = (*dapp_data.read().unwrap()).decimals.clone();
    }
    let mut nft_info: HashMap<u64, Vec<NFT_data::Transaction>> = SQLProcess::NFT::sql_transactions(&mut conn, dapp_id, &vec![index]);
    let nft_txs_ = nft_info.remove(&index);
    //println!("{:#?}", nft_txs_);
    if let Some(nft_txs) = nft_txs_{
        for tx in nft_txs.iter(){
            mydapps.push((tx.from.clone(), tx.to.clone(), (tx.price as f64) / (10_i32.pow(decimal as u32) as f64), tx.time));
            //println!("{:#?}", mydapps);
        }
    }
    //response
    serde_json::to_string(&mydapps).unwrap()
}

fn main() {
    // let url: &str = "mysql://root:apple@localhost:3306/db1";
    let url: &str = "mysql://root:qian7633210@localhost:3306/test_db";

    let opts:Opts = Opts::from_url(url).unwrap();
    let pool: Pool = Pool::new(opts).unwrap();

    let mut dapps = Threads::init_process::init_DAPPs(String::from("/Users/qianhao/sjtu/ic/DATA_SYNC/src/Canister.json"), pool.clone());
    // let mut dapps = Threads::init_process::init_DAPPs(String::from("/Users/panyeda/work/dfinity/DATA_SYNC/src/Canister.json"), pool.clone());

    //let mut dapps: Vec<DAPP> = Vec::new();
    let rt = Runtime::new().unwrap();
    let mut infos: Vec<DAPP_info> = Vec::new();
    //let mut query_infos = infos.clone();
    for _i in 0..dapps.len(){
        if let DAPP::EXT_DAPP(ext_dapp) = &dapps[_i]{
            let info = ext_dapp.info.clone();
            infos.push(DAPP_info::EXT_info(info));
        } 
        if let DAPP::Dfinance(dfinance) = &dapps[_i]{
            let info = dfinance.dfinance_info.clone();
            infos.push(DAPP_info::Dfinance_info(info));
        }
    }
    rt.block_on(async move {
        //let mut all_dapp = dapps;
        for _i in 0..dapps.len(){
            let dapp = dapps.pop_front();
            tokio::spawn(
                async move {
                    if let Some(DAPP::EXT_DAPP(mut ext_dapp)) = dapp{
                        ext_dapp.warm_boot().await;
                        // ext_dapp.loop_query_update().await;
                    }
                    else if let Some(DAPP::Dfinance(mut dfinance)) = dapp{
                        dfinance.warm_boot().await;
                    }
                });
            }
        tokio::spawn(
            async move {
                rocket::build().manage((infos, pool))
                                .mount("/info", routes![get_general_info, get_nft_info, get_user_info])
                                .mount("/dfinance", routes![get_dfinance_general_info,
                                                            get_token_user_balances,
                                                            get_token_user_transactions,
                                                            get_dswap_user_balances,
                                                            get_dswap_user_transactions])
                                .launch().await;
            }
        )
    });

    loop{
        // for info in query_infos.iter(){
        //     //println!("1 {}", (*info.read().unwrap()).total_txs);
        //     if let DAPP_info::EXT_info(info) = info{
        //         println!("1 {}", (*info.read().unwrap()).total_txs);
        //         println!("2 {}", (*info.read().unwrap()).average_price);
        //         println!("3 {}", (*info.read().unwrap()).name);
        //     }
        //     if let DAPP_info::Dfinance_info(info) = info{
        //         println!("1 {:#?}",(*info.read().unwrap()).token_canister_id);
        //         // println!("2 {:#?}",(*info.read().unwrap()).tokens);
        //     }
        //     sleep(Duration::from_millis(3000));
        // }
    }
}