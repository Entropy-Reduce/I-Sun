use candid::types::number::Int;
use crate::QueryFunctions::DataStructure::_PUNK as PUNK_data;
use crate::PostProcess::types::NFT as NFT_types;
use crate::PostProcess::utils as utils;
use crate::PostProcess::types::dapp_func;
use crate::Threads::PUNK as PUNK_thread;
use ic_agent::{Agent, ic_types::Principal,agent::http_transport::ReqwestHttpReplicaV2Transport};
use mysql::Pool;
use std::sync::RwLock;
use std::sync::Arc;
use async_trait::async_trait;
use std::mem::drop;
use std::time;
use std::thread;
use mysql::PooledConn;

use crate::SQLProcess::NFT as NFT_sql;

const FIVE_MINUTE:u64 = 5 * 60 * 1000000000; 


#[derive(Clone)]
pub struct PUNK_DAPP{
    pub info: Arc<RwLock<NFT_types::NFTGeneralInfo>>,
    pub agent: Agent,
    pub sql_pool: Pool,
    pub dapp_id: u64,
}

impl PUNK_DAPP{
    pub fn new(canisterId:Principal,name:String,symbol:String,supply:u64,decimals:u8, dapp_id: u64, agent: Agent, sql_pool: Pool)->PUNK_DAPP{
        let NFT_info = NFT_types::NewNFTGeneralInfo(canisterId, name, symbol, supply, decimals);
        PUNK_DAPP{
            info: Arc::new(RwLock::new(NFT_info)),
            agent: agent,
            sql_pool: sql_pool,
            dapp_id: dapp_id,
        }
    }
}

#[async_trait]
impl dapp_func for PUNK_DAPP{
    async fn loop_query_update(&mut self){
        let three_minates = time::Duration::from_millis(10000 * 6 * 3);
        loop{
            let mut conn = self.sql_pool.get_conn().unwrap();
            let mut pre_info: NFT_types::NFTGeneralInfo = (*self.info.read().unwrap()).clone();
            PUNK_thread::PUNK_transactions_thread(&mut pre_info, &mut conn, self.dapp_id.clone(), &self.agent, 0).await;
            PUNK_thread::PUNK_Listings_thread(&mut pre_info, &self.agent).await;
            let mut info = self.info.write().unwrap();
            (*info) = pre_info;
            drop(info);
            thread::sleep(three_minates);
        }
    }
    async fn cold_boot(&mut self){
        let mut conn = self.sql_pool.get_conn().unwrap();
        let mut pre_info: NFT_types::NFTGeneralInfo = (*self.info.read().unwrap()).clone();
        PUNK_thread::PUNK_transactions_thread(&mut pre_info, &mut conn, self.dapp_id.clone(), &self.agent, 200).await;
        // //let mut pre_info = (*self.info.read().unwrap()).clone();
        // //PUNK_thread::PUNK_transactions_thread(&mut pre_info, &self.agent).await;
        let mut info = self.info.write().unwrap();
        (*info) = pre_info;
    }
    async fn warm_boot(&mut self){
        let mut pre_info = (*self.info.read().unwrap()).clone();
        let mut conn = self.sql_pool.get_conn().unwrap();
        PUNK_thread::PUNK_warm_boot_thread(&mut pre_info, &mut conn, self.dapp_id, &self.agent).await;
        let mut info = self.info.write().unwrap();
        (*info) = pre_info;
    }
   
}


pub fn HandleTrancastions(NFTInfo:&mut NFT_types::NFTGeneralInfo,transactions: &PUNK_data::transactions, dapp_id: u64, conn: &mut PooledConn){
    
    for transaction in transactions{
        println!("{:#?}", transaction);
        if transaction.from == None{
            continue;
        }
        let mut price = 0;
        let mut to = String::from("None");
        if transaction.price != None{
            price = transaction.price.unwrap();
        }
        if transaction.to == None{
            to = utils::Principal2Identifier(&transaction.from.unwrap(), conn).clone(); 
        }
        else{
            to = utils::Principal2Identifier(&transaction.to.unwrap(), conn).clone(); 
        }
       
        let process_transaction = NFT_types::Transaction{
            tokenIndex: utils::Nat2u64(&transaction.tokenId),
            from: utils::Principal2Identifier(&transaction.from.unwrap(), conn).clone(),
            to: to,
            time: utils::Bigint2u64(&transaction.timestamp),
            price: price,   
        };

        //println!("{:#?}", process_transaction);
        HandleTrancastion(NFTInfo, &process_transaction, dapp_id, conn);
    }

}

pub fn HandleTrancastion(NFTInfo:&mut NFT_types::NFTGeneralInfo, transaction:&NFT_types::Transaction, dapp_id: u64, conn: &mut PooledConn){
    NFTInfo.tx_update(transaction);

    //TODO:update seller
    if transaction.to != transaction.from{
        NFT_sql::delete_user_index(conn, transaction, dapp_id);
    }
    
    //TODO:update buyer
    NFT_sql::insert_user_index(conn, transaction, dapp_id);

    //TODO:update transactions
    NFT_sql::insert_tx(conn, transaction, dapp_id);
}

pub fn HandleListings(NFTInfo:&mut NFT_types::NFTGeneralInfo, listings: &PUNK_data::Listings){
    let mut floor_price = 99999999999999 as u64;
    let mut listing_number = 0 as u64;
    for l in listings{
        if l.price < floor_price{
            floor_price = l.price;
        }
        listing_number += 1;
    }
    NFTInfo.listing_update(listing_number, floor_price);

}



