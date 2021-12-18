use crate::PostProcess::types::DFINANCE::dswap;
use crate::PostProcess::types::DFINANCE::token;
use std::collections::HashMap;
use crate::PostProcess::utils;
use serde_json;
use mysql::*;
use mysql::prelude::*;


use crate::QueryFunctions::DataStructure::_DFINANCE::_REGISTRY as registry_data;
// use crate::QueryFunctions::DataStructure::_DFINANCE::_DTOKEN as token_data;
use crate::QueryFunctions::DataStructure::_DFINANCE::_DSWAP as dswap_data;

pub fn insert_tokens_info(conn: &mut PooledConn, tokens_info:&Vec<registry_data::TokenInfo>){
    for token_info in tokens_info{
        insert_token_info(conn,token_info);
    }
}

pub fn insert_token_info(conn: &mut PooledConn,token_info:&registry_data::TokenInfo){
    let res = conn.query_first(format!("select canister_id from tokens where canister_id = '{}'", token_info.canisterId.to_string()))
        .map(
            |row| {
                row.map(|canister_id|->String {canister_id})
            }
        );
    
    match res.unwrap() {
        // Some(_id) => println!{"[insert_token_info]:the token info {} has existed!",token_info.canisterId.to_string()},
        Some(_id) => (),
        None => {
            // println!{"[insert_token_info]:the token info doesn't exist!"}
            conn.exec_drop(
                "insert into tokens (canister_id,decimals,fee,index_,logo,name_,owner_,symbol,timestamp_,supply) values (:canister_id,:decimals,:fee,:index_,:logo,:name_,:owner_,:symbol,:timestamp_,:supply)", 
                params! {
                    "canister_id" => token_info.canisterId.to_string(),
                    "decimals" =>  token_info.decimals,
                    "fee" => utils::Nat2u64(&token_info.fee),
                    "index_" => utils::Nat2u64(&token_info.index),
                    "logo" => token_info.logo.clone(),
                    "name_" => token_info.name.clone(),
                    "owner_" => token_info.owner.to_string(),
                    "symbol" => token_info.symbol.clone(),
                    "timestamp_" => utils::Bigint2u64(&token_info.timestamp),
                    "supply" => utils::Nat2u64(&token_info.totalSupply),
                }
            ).unwrap();
        },
    };
    
    
}
//后续在做NONE的处理吧
pub fn get_token_info(conn: &mut PooledConn,canister_id:&str) -> Option<token::TokenInfo>{
    //decimals(0),fee(1),index_(2),logo(3),name_(4),owner_(5),symbol(6),timestamp_(7),supply(8)
    let res:Result<Option<(u8,u64,u64,String,String,String,String,u64,u64)>> = conn.query_first(format!(
        "select decimals,fee,index_,logo,name_,owner_,symbol,timestamp_,supply from tokens where canister_id = '{}'", 
        String::from(canister_id)));

    match res.unwrap(){
        Some(result) => {
            let token_info = token::TokenInfo{
                canister_id: String::from(canister_id),
                decimals: result.0,
                fee: result.1,
                index: result.2,
                logo: result.3.clone(),
                name: result.4.clone(),
                owner: result.5.clone(),
                symbol: result.6.clone(),
                timestamp: result.7,
                supply: result.8,
            };
            return Some(token_info);
        },
        _ => return None,
    }
    
}

pub fn insert_pairs_info(conn: &mut PooledConn,pairs_info:&Vec<dswap_data::PairInfoExt>){
    for pair_info in pairs_info{
        insert_pair_info(conn,&pair_info);
    }
}

pub fn insert_pair_info(conn: &mut PooledConn,pair_info:&dswap_data::PairInfoExt){
    let res = conn.query_first(format!("select id from pairs where id = '{}'",pair_info.id.clone()))
        .map(
            |row| {
                row.map(|id|->String {id})
            }
        );
    
    match res.unwrap() {
        Some(_id) => {
            // println!{"[insert_pair_info]:the pair info {} has existed!",pair_info.id.clone()};
            let stmt = conn.prep("delete from pairs where id=:id").unwrap();
            conn.exec_drop(&stmt, 
                params!{
                    "id" => pair_info.id.clone(),
                }
            ).unwrap();

            conn.exec_drop(
                "insert into pairs (id,supply,token0,token1,lp_token,creator,last_update_time,price0_cumulative,price1_cumulative,k) values (:id,:supply,:token0,:token1,:lp_token,:creator,:last_update_time,:price0_cumulative,:price1_cumulative,:k)", 
                params! {
                    "id" => pair_info.id.clone(), // principal
                    "supply" => utils::Nat2u64(&pair_info.totalSupply),
                    "token0" => pair_info.token0.clone(), //Principal;
                    "token1" => pair_info.token1.clone(), 
                    "lp_token" => pair_info.lptoken.clone(), 
                    "creator" => pair_info.creator.to_string(),
                    "last_update_time" => utils::Bigint2u64(&pair_info.blockTimestampLast),
                    "price0_cumulative" => utils::Nat2u64(&pair_info.price0CumulativeLast),
                    "price1_cumulative" => utils::Nat2u64(&pair_info.price1CumulativeLast),
                    "k" =>  utils::Nat2u64(&pair_info.kLast),
                }
            ).unwrap();
        },
        None => {
            // println!{"[insert_pair_info]:the pair info {} doesn't exist!",pair_info.id.clone()};
            conn.exec_drop(
                "insert into pairs (id,supply,token0,token1,lp_token,creator,last_update_time,price0_cumulative,price1_cumulative,k) values (:id,:supply,:token0,:token1,:lp_token,:creator,:last_update_time,:price0_cumulative,:price1_cumulative,:k)", 
                params! {
                    "id" => pair_info.id.clone(), // principal
                    "supply" => utils::Nat2u64(&pair_info.totalSupply),
                    "token0" => pair_info.token0.clone(), //Principal;
                    "token1" => pair_info.token1.clone(), 
                    "lp_token" => pair_info.lptoken.clone(), 
                    "creator" => pair_info.creator.to_string(),
                    "last_update_time" => utils::Bigint2u64(&pair_info.blockTimestampLast),
                    "price0_cumulative" => utils::Nat2u64(&pair_info.price0CumulativeLast),
                    "price1_cumulative" => utils::Nat2u64(&pair_info.price1CumulativeLast),
                    "k" =>  utils::Nat2u64(&pair_info.kLast),
                }
            ).unwrap();
        },
    }
}

pub fn get_pair_info(conn: &mut PooledConn,canister_id:&str) -> Option<dswap::PairInfo>{
    let res:Result<Option<(u64,String,String,String,String,u64,u64,u64,u64)>> = conn.query_first(format!(
        "select supply,token0,token1,lp_token,creator,last_update_time,price0_cumulative,price1_cumulative,k from pairs where id = '{}'", 
        canister_id));

    match res.unwrap(){
        Some(result) => {
            let token_info = dswap::PairInfo{
                id: String::from(canister_id),
                supply: result.0,
                token0: result.1.clone(),
                token1: result.2.clone(),
                lp_token: result.3.clone(),
                creator: result.4.clone(),
                last_update_time: result.5,
                price0_cumulative: result.6,
                price1_cumulative: result.7,
                k: result.8,
            };
            return Some(token_info);
        },
        _ => return None,
    }
}

pub fn insert_swap_tokens_info(conn: &mut PooledConn,swap_tokens_info:&Vec<dswap_data::TokenInfoExt>){
    for swap_token_info in swap_tokens_info{
        insert_swap_token_info(conn, swap_token_info)
    }
}

pub fn insert_swap_token_info(conn: &mut PooledConn, swap_token_info:&dswap_data::TokenInfoExt){
    let res = conn.query_first(format!("select id from swap_tokens where id = '{}'",&swap_token_info.id))
        .map(
            |row|{
                row.map(|id|->String{id})
            }
        );

    match res.unwrap() {
        Some(_result)=>{
            let stmt = conn.prep("delete from swap_tokens where id=:id").unwrap();
            conn.exec_drop(&stmt, 
                params!{
                    "id" => swap_token_info.id.clone(),
                }
            ).unwrap();
            
            conn.exec_drop(
                "insert into swap_tokens (id,name_,symbol,decimals,fee,supply) values (:id,:name_,:symbol,:decimals,:fee,:supply)", 
                params!{
                    "id" => swap_token_info.id.clone(),
                    "name_" => swap_token_info.name.clone(),
                    "symbol" =>swap_token_info.symbol.clone(), 
                    "decimals" => swap_token_info.decimals.clone(),
                    "fee" => utils::Nat2u64(&swap_token_info.fee), 
                    "supply" =>utils::Nat2u64(&swap_token_info.totalSupply), 
                }
            ).unwrap();
        },
        _ =>{
            conn.exec_drop(
                "insert into swap_tokens (id,name_,symbol,decimals,fee,supply) values (:id,:name_,:symbol,:decimals,:fee,:supply)", 
                params!{
                    "id" => swap_token_info.id.clone(),
                    "name_" => swap_token_info.name.clone(),
                    "symbol" => swap_token_info.symbol.clone(),
                    "decimals" => swap_token_info.decimals, 
                    "fee" => utils::Nat2u64(&swap_token_info.fee), 
                    "supply" => utils::Nat2u64(&swap_token_info.totalSupply), 
                }
            ).unwrap();
        }
    } 
}

pub fn get_swap_token_info(conn: &mut PooledConn,id:&str) -> Option<dswap::TokenInfo>{
    let res:Result<Option<(String,String,u8,u64,u64)>> = conn.query_first(format!(
        "select name_,symbol,decimals,fee,supply from swap_tokens where id = '{}'", 
        id));

    match res.unwrap(){
        Some(result) => {
            let token_info = dswap::TokenInfo{
                id: String::from(id),
                name: result.0.clone(),
                symbol:result.1.clone(),
                decimals:result.2,
                fee:result.3,
                supply:result.4,
            };
            return Some(token_info);
        },
        _ => return None,
    }
}

pub fn put_token_user_info(conn: &mut PooledConn, user_info:token::User){
    if user_info.transactions.len() == 0{
        return
    }

    let res = conn.query_first(format!("select principal from token_user where principal = '{}'",&user_info.principal))
        .map(
            |row|{
                row.map(|principal|->String{principal})
            }
        );
    
    match res.unwrap() {
        Some(_result)=>{
            // println!("[sql]-[put_token_user_info]:the user info {} has exist",&user_info.principal);
            let stmt = conn.prep("delete from token_user where principal=:principal").unwrap();
            conn.exec_drop(&stmt, 
                params!{
                    "principal" => user_info.principal.clone(),
                }
            ).unwrap();
            conn.exec_drop(
                "insert into token_user (principal,balances,transactions) values (:principal,:balances,:transactions)", 
                params!{
                    "principal" => user_info.principal.clone(),
                    "balances" => serde_json::to_string(&user_info.balances).unwrap(),
                    "transactions" =>serde_json::to_string(&user_info.transactions).unwrap(), 
                }
            ).unwrap();
        },
        _ =>{
            // println!("[sql]-[put_token_user_info]:the user info {} doesn't exist",&user_info.principal);
            conn.exec_drop(
                "insert into token_user (principal,balances,transactions) values (:principal,:balances,:transactions)", 
                params!{
                    "principal" => user_info.principal.clone(),
                    "balances" => serde_json::to_string(&user_info.balances).unwrap(),
                    "transactions" =>serde_json::to_string(&user_info.transactions).unwrap(), 
                }
            ).unwrap();
        }
    } 
}

pub fn get_token_user_info(conn: &mut PooledConn, user_id:&str) -> token::User{
    let res:Result<Option<(String,String)>> = conn.query_first(format!(
        "select balances, transactions from token_user where principal = '{}'",user_id));
        
    match res.unwrap() {
        Some(result) => {
            let balances:HashMap<String,u64> = serde_json::from_str(&result.0).unwrap();
            let transactions: HashMap<String,Vec<u64>> = serde_json::from_str(&result.1).unwrap();
            let user_info = token::User{
                principal:String::from(user_id),
                balances:balances.clone(),
                transactions:transactions,
                };
            return user_info
            },
        _ => {
            let user = token::User{
                        principal:String::from(user_id),
                        balances:HashMap::new(),
                        transactions:HashMap::new(),
                    };
            return user
            },
        }
}

pub fn put_dswap_user_info(conn: &mut PooledConn, user_info:dswap::User){
    if user_info.transactions.len() == 0{
        return
    }
    let res = conn.query_first(format!("select principal from swap_user where principal = '{}'",&user_info.principal))
        .map(
            |row|{
                row.map(|principal|->String{principal})
            }
        );

    match res.unwrap() {
        Some(_result)=>{
            let stmt = conn.prep("delete from swap_user where principal=:principal").unwrap();
            conn.exec_drop(&stmt, 
                params!{
                    "principal" => user_info.principal.clone(),
                }
            ).unwrap();
            
            conn.exec_drop(
                "insert into swap_user (principal,balances,lp_balances,transactions) values (:principal,:balances,:lp_balances,:transactions)", 
                params!{
                    "principal" => user_info.principal.clone(),
                    "balances" => serde_json::to_string(&user_info.balances).unwrap(),
                    "lp_balances" =>serde_json::to_string(&user_info.lp_balances).unwrap(),
                    "transactions" =>serde_json::to_string(&user_info.transactions).unwrap(), 
                }
            ).unwrap();
        },
        _ =>{
            conn.exec_drop(
                "insert into swap_user (principal,balances,lp_balances,transactions) values (:principal,:balances,:lp_balances,:transactions)", 
                params!{
                    "principal" => user_info.principal.clone(),
                    "balances" => serde_json::to_string(&user_info.balances).unwrap(),
                    "lp_balances" =>serde_json::to_string(&user_info.lp_balances).unwrap(),
                    "transactions" =>serde_json::to_string(&user_info.transactions).unwrap(), 
                }
            ).unwrap();
        }
    } 
}

pub fn get_dswap_user_info(conn: &mut PooledConn, user_id:&str) -> dswap::User{
    let res : Result<Option<(String,String,String)>> = conn.query_first(format!(
        "select balances,lp_balances,transactions from swap_user where principal = '{}'",
        user_id));
    
        match res.unwrap() {
            Some(result) => {
                // println!("Get dswap user {}",user_id);
                let balances:HashMap<String,u64> = serde_json::from_str(&result.0).unwrap();
                let lp_balances:HashMap<String,u64> = serde_json::from_str(&result.1).unwrap();
                let transactions: HashMap<String,Vec<u64>> = serde_json::from_str(&result.2).unwrap();
                let user_info = dswap::User{
                    principal:String::from(user_id),
                    balances:balances.clone(),
                    lp_balances:lp_balances.clone(),
                    transactions:transactions,
                };
                return user_info
            },
            _ => {
                // println!("Can not get dswap user {}",user_id);
                let user = dswap::User{
                            principal:String::from(user_id),
                            balances:HashMap::new(),
                            lp_balances:HashMap::new(),
                            transactions:HashMap::new(),
                        };
                return user;
            },
        }
}

pub fn insert_token_transactions(conn: &mut PooledConn,txs:&mut Vec<token::Transaction>,canister_id:&str){
    for tx in txs{
        insert_token_transaction(conn,tx,canister_id);
    }
}

pub fn insert_token_transaction(conn: &mut PooledConn,tx:&token::Transaction,canister_id:&str){ 
    let res = conn.query_first(format!("select canister_id from token_tx where canister_id='{}' and index_='{}' ",canister_id,tx.index))
        .map(
            |row|{
                row.map(|canister_id|->String{canister_id})
            }
        );
    match res.unwrap(){
        Some(_id) => return,
        None =>{

            conn.exec_drop("insert into token_tx (canister_id,index_,content) values (:canister_id,:index_,:content)", 
            params!{
                "canister_id" => String::from(canister_id),
                "index_" => tx.index,
                "content" => serde_json::to_string(tx).unwrap(), 
            }).unwrap();
        },
    }
}

pub fn get_token_transactions(conn: &mut PooledConn,token_id:&str,tx_indexs:Vec<u64>) -> Vec<token::Transaction>{
    let mut transactions = Vec::new();
    for index in &tx_indexs{
        let res:Option<String> = conn.query_first(format!(
            "select content from token_tx where canister_id = '{}' and index_ = {}",
            token_id,index)).unwrap();
        match res{
            Some(tx) => {
                let tx : token::Transaction = serde_json::from_str(&tx).unwrap();
                transactions.push(tx);
            },
            None => (),
        }     
    }

    return transactions;
}

pub fn insert_dswap_transaction(conn: &mut PooledConn,tx:&dswap::Transaction){
    let res = conn.query_first(format!("select index_ from swap_tx where index_='{}' ",tx.index))
        .map(
            |row|{
                row.map(|index_|->u64{index_})
            }
        );
    
    match res.unwrap(){
        Some(_index) => return,
        None => {
            conn.exec_drop("insert into swap_tx (index_,content) values (:index_,:content)", 
            params!{
                "index_" => tx.index,
                "content" => serde_json::to_string(tx).unwrap(), 
            }).unwrap();
        },
    }   
}

pub fn insert_dswap_transactions(conn: &mut PooledConn,txs:&mut Vec<dswap::Transaction>){
    for tx in txs{
        insert_dswap_transaction(conn,tx);
    }
}

pub fn get_dswap_transactions(conn: &mut PooledConn,tx_indexs:Vec<u64>) -> Vec<dswap::Transaction>{
    // println!("Get dswap transactions");
    let mut transactions = Vec::new();
    for index in &tx_indexs{
        let res:Option<String> = conn.query_first(format!(
            "select content from swap_tx where index_ = {}",
            index)).unwrap();
        match res {
            Some(tx) => {
                // println!("Get dswap transactions {}",tx);
                let tx : dswap::Transaction = serde_json::from_str(&tx).unwrap();
                transactions.push(tx);
            },
            None =>(),
        } 
    }
    return transactions;
}