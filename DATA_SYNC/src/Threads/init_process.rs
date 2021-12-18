use crate::PostProcess::EXT::EXT_DAPP as EXT_DAPP;
use crate::PostProcess::PUNK::PUNK_DAPP as PUNK_DAPP;
use crate::PostProcess::DFINANCE::DFinance as Dfinance_DAPP;
extern crate serde_json;
use std::fs::File;
use crate::PostProcess::types::NFT::NFTGeneralInfo as NFT_info;
use crate::PostProcess::types::DFINANCE::dfinance::DfinanceInfo as Dfinance_info;
use mysql::{Opts, Pool};
use ic_agent::{Agent, ic_types::Principal,agent::http_transport::ReqwestHttpReplicaV2Transport};
use std::collections::{HashSet,VecDeque,HashMap};
use std::sync::RwLock;
use std::sync::Arc;

pub enum DAPP_info{
    EXT_info(Arc<RwLock<NFT_info>>),
    PUNK_info(Arc<RwLock<NFT_info>>),
    Dfinance_info(Arc<RwLock<Dfinance_info>>),

}

pub enum DAPP{
    EXT_DAPP(EXT_DAPP),
    PUNK_DAPP(PUNK_DAPP),
    Dfinance(Dfinance_DAPP),
}

pub fn init_DAPPs(path: String, pool: Pool)->VecDeque<DAPP>{
    let f = File::open(path).unwrap();
    let v: serde_json::Value = serde_json::from_reader(f).unwrap();
    let v = v.as_array().unwrap();
    let mut all_dapp: VecDeque<DAPP> = VecDeque::new();
    for item in v.iter(){
        //println!("{}", item);
        let dapp_type = &item["dapp_type"].as_str().unwrap();
        let dapp_id = &item["dapp_id"].as_u64().unwrap();
        if *dapp_type == "EXT_dapp"{
            let agent = Agent::builder()
            	.with_transport(ReqwestHttpReplicaV2Transport::create("https://ic0.app/").unwrap())
            	.build()
            	.unwrap();
            let Canister_id = item["dapp_canister"].as_str().unwrap();
            let Canister_id = Principal::from_text(Canister_id).unwrap();
            let name = item["name"].as_str().unwrap();
            let symbol = item["symbol"].as_str().unwrap();
            let supply = item["supply"].as_u64().unwrap();
            let decimal = item["decimal"].as_u64().unwrap() as u8;
            let dapp = EXT_DAPP::new(Canister_id, String::from(name), String::from(symbol), supply, decimal, *dapp_id, agent, pool.clone());
            let dapp = DAPP::EXT_DAPP(dapp);
            all_dapp.push_back(dapp);
        }
        else if *dapp_type == "PUNK_dapp"{
            let agent = Agent::builder()
            	.with_transport(ReqwestHttpReplicaV2Transport::create("https://ic0.app/").unwrap())
            	.build()
            	.unwrap();
            let Canister_id = item["dapp_canister"].as_str().unwrap();
            let Canister_id = Principal::from_text(Canister_id).unwrap();
            let name = item["name"].as_str().unwrap();
            let symbol = item["symbol"].as_str().unwrap();
            let supply = item["supply"].as_u64().unwrap();
            let decimal = item["decimal"].as_u64().unwrap() as u8;
            let dapp = PUNK_DAPP::new(Canister_id, String::from(name), String::from(symbol), supply, decimal, *dapp_id, agent, pool.clone());
            let dapp = DAPP::PUNK_DAPP(dapp);
            all_dapp.push_back(dapp);
        }
        else if *dapp_type == "Dfinance"{
            let agent = Agent::builder()
            	.with_transport(ReqwestHttpReplicaV2Transport::create("https://ic0.app/").unwrap())
            	.build()
            	.unwrap();
            let token_canister_id = item["token_canister"].as_str().unwrap();
            let dswap_canister_id = item["dswap_canister"].as_str().unwrap();
            let dapp_id = item["dapp_id"].as_u64().unwrap();
            let dapp = Dfinance_DAPP::new(dapp_id, token_canister_id, dswap_canister_id, pool.clone());
            let dapp = DAPP::Dfinance(dapp);
            all_dapp.push_back(dapp);
        }
    }
    all_dapp
}