
use std::{thread,time,collections::HashMap};
use candid::types::number::Nat;
use ic_agent::{ic_types::Principal,Agent, agent::http_transport::ReqwestHttpReplicaV2Transport};
use std::sync::{Arc,RwLock};
use mysql::Pool;
use async_trait::async_trait;

use super::types::DFINANCE::dswap;
use super::types::DFINANCE::token;
use super::types::DFINANCE::dfinance;

// use crate::QueryFunctions::DataStructure::_DFINANCE::_REGISTRY as registry_data;
// use crate::QueryFunctions::DataStructure::_DFINANCE::_DTOKEN as token_data;
use crate::QueryFunctions::DataStructure::_DFINANCE::_DSWAP as dswap_data;
use crate::QueryFunctions::DFINANCE::REGISTRY as registry_query;
use crate::QueryFunctions::DFINANCE::DSWAP as dswap_query;
use crate::QueryFunctions::DFINANCE::DTOKEN as token_query;
use crate::PostProcess::utils;
use crate::SQLProcess::dfinance as sql;
use crate::PostProcess::types::dapp_func;

const TRANSACTION_PROCESS_INTERVAL:u64 = 100;// amount of txs
// const TOKEM_INFO_UPDATE_INTERVAL:u64 = 1000;// amout of txs
const TRANSACTION_PROCESS_GAP:time::Duration = time::Duration::from_millis(60_000);
// const TOKEM_INFO_UPDATE_GAP:time::Duration = time::Duration::from_millis(3_000);
#[derive(Clone)]
pub struct DFinance {
    dapp_id : u64,
    // tx_process_interval:u64,
    // tx_process_gap:time::Duration,
    token_canister_id: String, 
    dswap_canister_id: String,
    pub dfinance_info:Arc<RwLock<dfinance::DfinanceInfo>>,
    pub pool:Pool,
    pub agent:Agent,
    pub token_tx_index:HashMap<String,u64>,
    pub dswap_tx_index:u64,
}

impl DFinance{
    pub fn new(dapp_id:u64, token_canister_id: &str, dswap_canister_id: &str,pool:Pool)->DFinance{
        let agent = Agent::builder()
        .with_transport(ReqwestHttpReplicaV2Transport::create("https://ic0.app/").unwrap())
        .build()
        .unwrap();
        let dswap_info = dswap_data::DSwapInfo{
            cycles: utils::u642Nat(0),
            owner: Principal::from_text("42vp6-2iaaa-aaaah-qbooa-cai").unwrap(),
            pairs: Vec::new(),
            storageCanisterId: Principal::from_text("42vp6-2iaaa-aaaah-qbooa-cai").unwrap(),
            tokens: Vec::new(),
        };
        let token_list = Vec::new();
        let dfinance_info = dfinance::new_dfinance_info(token_canister_id,dswap_canister_id,&dswap_info, &token_list);
        DFinance{
            dapp_id : dapp_id,
            token_canister_id: String::from(token_canister_id), 
            dswap_canister_id: String::from(dswap_canister_id), 
            agent:agent,
            pool:pool,
            dfinance_info:Arc::new(RwLock::new(dfinance_info)), 
            token_tx_index:HashMap::new(),
            dswap_tx_index:0,
        }   
    }
}

#[async_trait]
impl dapp_func for DFinance {
    //todo：我们目前没有存dswap_user的info
    async fn loop_query_update(&mut self){
        let mut token_txs_pool : Vec<token::Transaction> = Vec::new();
        let mut dswap_txs_pool : Vec<dswap::Transaction> = Vec::new();       

        // start : 
        loop{
            //更新基础信息
            println!("[Dfinance]:loop");
            
            let dswap_info = dswap_query::get_dswap_info(&self.dswap_canister_id, &self.agent).await;
            let token_list = registry_query::query_token_list(&self.token_canister_id, &self.agent).await;
            let dswap_storage_canister_id = dswap_info.storageCanisterId.to_string();

            let mut dfinance_info = (*self.dfinance_info.read().unwrap()).clone();
            dfinance_info.update(&dswap_info, &token_list);
            //let mut self_dfinance_info = self.dfinance_info.write().unwrap();
            (*self.dfinance_info.write().unwrap()) = dfinance_info;
            //drop(self_dfinance_info);
            let mut conn = self.pool.get_conn().unwrap();
            sql::insert_tokens_info(&mut conn,&token_list);
            sql::insert_pairs_info(&mut conn,&dswap_info.pairs);
            sql::insert_swap_tokens_info(&mut conn,&dswap_info.tokens);
            // println!("[Dfinance]:loop:The current tokens index is {:#?}",self.token_tx_index);

            for token_info in token_list {
                let token_canister_id = token_info.canisterId.to_string();
                let token_tx_index = match self.token_tx_index.get_mut(&token_canister_id){
                    Some(_token_tx_index) => *_token_tx_index,
                    None => 0,
                };
                // println!("[Dfinance]:loop:The current token index is {}:{}",&token_canister_id,&token_tx_index);

                let history_size = token_query::history_size(&token_canister_id, &self.agent).await;
                let txs = token_query::get_transactions(&token_canister_id, &self.agent,&utils::u642Nat(token_tx_index),&history_size).await;
                self.token_tx_index.insert(token_canister_id.clone(),utils::Nat2u64(&history_size));
                // token_tx_index = utils::Nat2u64(&history_size);
                for tx in &txs{
                    let caller_id = match tx.caller{
                        Some(_caller_id) => _caller_id.to_string(),
                        None => String::from("None"),
                    };
                    let from_id = tx.from.to_string();
                    let to_id = tx.to.to_string();
    
                    let mut _tx = token::Transaction{
                        amount: utils::Nat2u64(&tx.amount),
                        fee: utils::Nat2u64(&tx.fee),
                        from: from_id.clone(), // principal
                        index: utils::Nat2u64(&tx.index),
                        op: utils::token_op_to_op(tx.op.clone()),
                        timestamp: utils::Bigint2u64(&tx.timestamp),
                        to: to_id.clone(),
                        caller: caller_id.clone(),
                        successful:true,
                    };
                        //todo:从数据库中查询user，没有的话就给一个新的
                    let mut from = sql::get_token_user_info(&mut conn,&from_id);
                    let mut to = sql::get_token_user_info(&mut conn,&to_id);
        
                    _tx.process_transaction(&mut from,&mut to,&token_canister_id);
                    token_txs_pool.push(_tx);
                    sql::put_token_user_info(&mut conn,from);
                    sql::put_token_user_info(&mut conn,to);
                }
                    
                sql::insert_token_transactions(&mut conn,&mut token_txs_pool,&token_canister_id);
                token_txs_pool = vec![];
        }
            
            let dswap_history_size = dswap_query::history_size(&dswap_storage_canister_id, &self.agent).await;
            let txs = dswap_query::get_transactions(&dswap_storage_canister_id, &self.agent, &utils::u642Nat(self.dswap_tx_index), &dswap_history_size).await;
            self.dswap_tx_index = utils::Nat2u64(&dswap_history_size);
            for tx in &txs{
                let caller_id = tx.caller.to_string();
                let from_id = tx.from.to_string();
                let to_id = tx.to.to_string();
                let _tx = dswap::Transaction{
                    amount : utils::Nat2u64(&tx.amount),
                    amount0: utils::Nat2u64(&tx.amount0),
                    amount1: utils::Nat2u64(&tx.amount1),
                    caller: caller_id.clone(),
                    fee: utils::Nat2u64(&tx.fee),
                    from: from_id.clone(),
                    index: utils::Nat2u64(&tx.index),
                    op: utils::swap_op_to_op(tx.op.clone()),
                    timestamp: utils::Bigint2u64(&tx.timestamp),
                    to: to_id.clone(),
                    token_id: tx.tokenId.clone(),
                };
                let mut caller = sql::get_dswap_user_info(&mut conn,&caller_id);
                let mut from = sql::get_dswap_user_info(&mut conn,&from_id);
                let mut to = sql::get_dswap_user_info(&mut conn,&to_id);
                _tx.process_transaction(&mut caller, &mut from, &mut to);
                dswap_txs_pool.push(_tx);
                sql::put_dswap_user_info(&mut conn, caller);
                sql::put_dswap_user_info(&mut conn,from);
                sql::put_dswap_user_info(&mut conn,to);
            }
            dswap_txs_pool = vec![];
            thread::sleep(TRANSACTION_PROCESS_GAP);
        }
    }

    async fn warm_boot(&mut self){
        let mut token_txs_pool : Vec<token::Transaction> = Vec::new();
        let mut dswap_txs_pool : Vec<dswap::Transaction> = Vec::new(); 
        let mut conn = self.pool.get_conn().unwrap();
        
        let dswap_info = dswap_query::get_dswap_info(&self.dswap_canister_id, &self.agent).await;
        let token_list = registry_query::query_token_list(&self.token_canister_id, &self.agent).await;
        let dswap_storage_canister_id = dswap_info.storageCanisterId.to_string();
        
        let mut dfinance_info = (*self.dfinance_info.read().unwrap()).clone();
        dfinance_info.update(&dswap_info, &token_list);

        sql::insert_tokens_info(&mut conn,&token_list);
        sql::insert_pairs_info(&mut conn,&dswap_info.pairs);
        sql::insert_swap_tokens_info(&mut conn,&dswap_info.tokens);

        //let mut self_dfinance_info = self.dfinance_info.write().unwrap();
        (*self.dfinance_info.write().unwrap()) = dfinance_info;
        for token_info in token_list {
            //查询token_info是否存在，不存在写入数据库；
            let token_canister_id = token_info.canisterId.to_string();
            println!("[Dfinance]:The current token canister id is {}",&token_canister_id);
            let mut token_tx_index = match self.token_tx_index.get_mut(&token_canister_id){
                Some(_token_tx_index) => *_token_tx_index,
                None => 0,
            };
            let history_size = token_query::history_size(&token_canister_id, &self.agent).await;
            let _history_size = utils::Nat2u64(&history_size);
            // println!("[Dfinance]:[start]:[token]:history size {}",_history_size);
            while token_tx_index < _history_size {
                // println!("[Dfinance]:[start]:[token]:current index is {}:{}",&token_canister_id,token_tx_index);
                let end: Nat;
                if token_tx_index+TRANSACTION_PROCESS_INTERVAL > _history_size {
                    end = history_size.clone();
                }else{
                    end = utils::u642Nat(token_tx_index+TRANSACTION_PROCESS_INTERVAL);
                }
                // println!("[dfinance]:current end is {}",utils::Nat2u64(&end));
                let txs = token_query::get_transactions(&token_canister_id, &self.agent, &utils::u642Nat(token_tx_index), &end).await;
                token_tx_index = utils::Nat2u64(&end);
                self.token_tx_index.insert(token_canister_id.clone(), utils::Nat2u64(&end));
                for tx in &txs{
                    // println!("[Dfinance]:[start]:[token]:user pool {:#?}",&self.db.token_user_table);
                    let caller_id = match tx.caller{
                        Some(_caller_id) => _caller_id.to_string(),
                        None => String::from("None"),
                    };
                    let from_id = tx.from.to_string();
                    let to_id = tx.to.to_string();
    
                    let mut _tx = token::Transaction{
                        amount: utils::Nat2u64(&tx.amount),
                        fee: utils::Nat2u64(&tx.fee),
                        from: from_id.clone(), // principal
                        index: utils::Nat2u64(&tx.index),
                        op: utils::token_op_to_op(tx.op.clone()),
                        timestamp: utils::Bigint2u64(&tx.timestamp),
                        to: to_id.clone(),
                        caller: caller_id.clone(),
                        successful:true,
                    };
                    let mut from = sql::get_token_user_info(&mut conn,&from_id);
                    let mut to = sql::get_token_user_info(&mut conn,&to_id);
                    
                    _tx.process_transaction(&mut from,&mut to,&token_canister_id);
                    // println!("[dfinance]-after:current from is {:#?}",&from);
                    // println!("[dfinance]-after:current to is {:#?}",&to);
                    token_txs_pool.push(_tx);
                    sql::put_token_user_info(&mut conn,from);
                    sql::put_token_user_info(&mut conn,to);
                }

                sql::insert_token_transactions(&mut conn,&mut token_txs_pool,&token_canister_id);
                token_txs_pool = vec![];
                // token_users_pool = HashMap::new();
            }
        }
        
        // println!("[Dfinance]:current token tx index is {:#?}",self.token_tx_index);
        // log::info!("dswap storage canisster id is {}",&dswap_storage_canisster_id);
        let dswap_history_size = dswap_query::history_size(&dswap_storage_canister_id, &self.agent).await;
        let _dswap_history_size = utils::Nat2u64(&dswap_history_size);
        // println!("[Dfinance]:dswap transaction amount is {}",&_dswap_history_size);
        while self.dswap_tx_index < _dswap_history_size{
            let end: Nat;
            if self.dswap_tx_index+TRANSACTION_PROCESS_INTERVAL > _dswap_history_size {
                end = dswap_history_size.clone();
            }else{
                end = utils::u642Nat(self.dswap_tx_index+TRANSACTION_PROCESS_INTERVAL);
            }
            let txs = dswap_query::get_transactions(&dswap_storage_canister_id, &self.agent, &utils::u642Nat(self.dswap_tx_index), &end).await;
            self.dswap_tx_index = utils::Nat2u64(&end);
            for tx in &txs{
                let caller_id = tx.caller.to_string();
                let from_id = tx.from.to_string();
                let to_id = tx.to.to_string();
                let _tx = dswap::Transaction{
                    amount : utils::Nat2u64(&tx.amount),
                    amount0: utils::Nat2u64(&tx.amount0),
                    amount1: utils::Nat2u64(&tx.amount1),
                    caller: caller_id.clone(),
                    fee: utils::Nat2u64(&tx.fee),
                    from: from_id.clone(),
                    index: utils::Nat2u64(&tx.index),
                    op: utils::swap_op_to_op(tx.op.clone()),
                    timestamp: utils::Bigint2u64(&tx.timestamp),
                    to: to_id.clone(),
                    token_id: tx.tokenId.clone(),
                };
                let mut caller = sql::get_dswap_user_info(&mut conn,&caller_id);
                let mut from = sql::get_dswap_user_info(&mut conn,&from_id);
                let mut to = sql::get_dswap_user_info(&mut conn,&to_id);
    
                _tx.process_transaction(&mut caller, &mut from, &mut to);
                dswap_txs_pool.push(_tx);
                sql::put_dswap_user_info(&mut conn,caller);
                sql::put_dswap_user_info(&mut conn,from);
                sql::put_dswap_user_info(&mut conn,to);
            }
            sql::insert_dswap_transactions(&mut conn,&mut dswap_txs_pool);
            dswap_txs_pool = vec![];
        }
        println!("Finish warm boot!");
    }

    async fn cold_boot(&mut self){
        return;
    }
}

pub fn get_token_user_balance(pool:Pool,token_principal:&str, user_principal:&str) -> u64{
    let mut conn = pool.get_conn().unwrap();
    let user = sql::get_token_user_info(&mut conn,user_principal);
    match user.balances.get(token_principal){
        Some(balance) => *balance,
        None => 0,
    }
}

pub fn get_token_symbol(pool:Pool,token_principal:&str) ->String{
    let mut conn = pool.get_conn().unwrap();
    match sql::get_token_info(&mut conn,token_principal){
        Some(token_info) => token_info.symbol,
        None => String::from("Null"),
    }
}

pub fn get_tokens_symbol(pool:Pool,tokens_principal:&Vec<String>) ->Vec<String>{
    let mut symbols = Vec::new();
    let mut conn = pool.get_conn().unwrap();
    for canister_id in tokens_principal{
        let symbol = match sql::get_token_info(&mut conn, canister_id){
            Some(token_info) => token_info.symbol,
            None => String::from("Null"),
        };
        symbols.push(symbol)
    }
    return symbols;
}

pub fn get_dswap_token_symbol(pool:Pool,token_principal:&str) -> String {
    // match sql::get_swap_token_info(&mut conn,token_principal){
    //     Some(token_info) => token_info.symbol,
    //     None => String::from("Null"),
    // }
    if token_principal.contains(":"){
        let canisters : Vec<&str> = token_principal.split(':').collect();
        let mut symbol1 = get_token_symbol(pool.clone(), canisters[0]);
        let symbol2 = get_token_symbol(pool.clone(), canisters[1]);
        symbol1.push(':');
        symbol1.push_str(&symbol2);
        return symbol1;
    } else{
        let symbol = get_token_symbol(pool.clone(), token_principal);
        return symbol;
    }
}

pub fn get_dswap_tokens_symbol(pool:Pool,tokens_principal:&Vec<String>) ->Vec<String>{
    let mut symbols = Vec::new();
    for token_principal in tokens_principal{
        let symbol = get_dswap_token_symbol(pool.clone(), token_principal);
        symbols.push(symbol);
    }
    // let mut conn = pool.get_conn().unwrap();
    // for canister_id in tokens_principal{
    //     let symbol = match sql::get_swap_token_info(&mut conn, canister_id){
    //         Some(token_info) => token_info.symbol,
    //         None => String::from("Null"),
    //     };
    //     symbols.push(symbol)
    // }
    
    return symbols;
}

pub fn get_token_user_balances(pool:Pool,user_principal:&String) -> HashMap<String,(String,f64)>{
    let mut conn = pool.get_conn().unwrap();
    let mut new_balances = HashMap::new();
    let balances = sql::get_token_user_info(&mut conn,user_principal).balances;
    for (canister_id,balance) in balances.iter(){
        let mut decimals = 1;
        let mut symbol = String::from("Null");
        match sql::get_token_info(&mut conn, canister_id){
            Some(token_info) => {
                decimals = token_info.decimals;
                symbol = token_info.symbol;
            },
            None => continue,
        };
        let balance:f64 = *balance as f64 / 10_i32.pow(decimals as u32) as f64;
        new_balances.insert((*canister_id).clone(), (symbol,balance));
    }
    return new_balances;    
}

pub async fn get_token_user_history_size(token_principal:&String,user_principal:&Principal,agent:&Agent) -> u64{
    let size = token_query::get_user_transactiom_amount(token_principal,&agent,user_principal).await;
    return utils::Nat2u64(&size);
}

pub fn get_token_transactions_by_user(pool:Pool,token_principal:&String, user_principal:&String,from:usize,to:usize,) -> Vec<token::Transaction>{
    let mut conn = pool.get_conn().unwrap();
    let user = sql::get_token_user_info(&mut conn,user_principal);
    let txs_index = match user.transactions.get(token_principal){
        Some(_txs_index) => {
            let mut transactions = Vec::new();
            if to < _txs_index.len(){
                for i in from..to{
                    transactions.push(_txs_index[i]);
                }
            } else{
                for i in from.._txs_index.len(){
                    transactions.push(_txs_index[i]);
                }
            }
            transactions
        },
        None => {
            return vec![];
        },
    };
    let transactions = sql::get_token_transactions(&mut conn,token_principal,txs_index);
    return transactions;
        //根据token的principal和index取token
}

pub async fn get_dswap_user_balances(pool:Pool,user_principal:&Principal,agent:&Agent,dswap_canister_id:&String) -> HashMap<String,(String,f64)>{
    let mut conn = pool.get_conn().unwrap();
    let mut new_balances = HashMap::new();
    let balances = dswap_query::get_user_info_above(&dswap_canister_id,
                                                    &agent,
                                                    user_principal,
                                                    &utils::u642Nat(0),
                                                    &utils::u642Nat(0)
                                                    ).await;

    for balance in &balances{
        if balance.0.contains(":"){
            let canisters : Vec<&str> = balance.0.split(':').collect();
            let decimals = match sql::get_token_info(&mut conn, &canisters[0]){
                Some(token_info) => token_info.decimals,
                None => continue,
            };
            let symbol = get_dswap_token_symbol(pool.clone(), &balance.0);
            let _balance:f64 = utils::Nat2u64(&balance.1) as f64 /  10_i32.pow(decimals as u32) as f64;
            new_balances.insert(balance.0.clone(), (symbol,_balance));
        } else{
            let decimals = match sql::get_token_info(&mut conn, &balance.0){
                Some(token_info) => token_info.decimals,
                None => continue,
            };
            let symbol = get_dswap_token_symbol(pool.clone(), &balance.0);
            let _balance:f64 = utils::Nat2u64(&balance.1) as f64 / 10_i32.pow(decimals as u32) as f64 ;
            new_balances.insert(balance.0.clone(), (symbol,_balance));
        }    
    }
    return new_balances;
}

pub fn get_dswap_transactions_by_user(pool:Pool,user_principal:&String,token_principal:&String,from:usize,to:usize) -> Vec<dswap::Transaction>{
    let mut conn = pool.get_conn().unwrap();
    let user = sql::get_dswap_user_info(&mut conn,user_principal);
    // println!("[get_dswap_transactions_by_user]:User_info:{:#?},token_principal:{:#?}",&user,token_principal);
    // println!("[get_dswap_transactions_by_user]:Before transactions index is {:#?}",&user.transactions);
    let txs_index = match user.transactions.get(token_principal){
        Some(_txs_index) => {
            // println!("[get_dswap_transactions_by_user]:tx_index:{:#?}",_txs_index);
            let mut transactions = Vec::new();
            if to < _txs_index.len(){
                for i in from..to{
                    transactions.push(_txs_index[i]);
                }
            } else{
                    for i in from.._txs_index.len(){
                        transactions.push(_txs_index[i]);
                    }
                }
                transactions
            },
            None => {
                // println!("[get_dswap_transactions_by_user]:Can not get tx {}",token_principal);
                return vec![];
            },
        };
        println!("[get_dswap_transactions_by_user]:After transactions indes is {:#?}",&txs_index);
        let transactions = sql::get_dswap_transactions(&mut conn,txs_index);
        return transactions;    
}


// #[derive(Debug,Clone)]
// pub struct MookDatabase {
//     token_user_table : HashMap<String,token::User>,
//     dswap_user_table : HashMap<String,dswap::User>,
//     tokens : HashMap<String,token::TokenInfo>,
//     swap_tokens : HashMap<String,dswap::TokenInfo>,
//     pairs:HashMap<String,dswap::PairInfo>,
//     token_txs:Vec<token::Transaction>,
//     dswap_txs:Vec<dswap::Transaction>,
// }

// impl MookDatabase{
//     pub fn insert_tokens_info(&mut self, tokens_info:&Vec<registry_data::TokenInfo>){
//         for token_info in tokens_info{
//             self.insert_token_info(token_info);
//         }
//     }

//     pub fn insert_token_info(&mut self,token_info:&registry_data::TokenInfo){
//         match self.tokens.get(&token_info.canisterId.to_string()){
//             Some(_) => (),
//             None => {
//                 let new_token_info = token::TokenInfo{
//                     canister_id: token_info.canisterId.to_string(),
//                     decimals: token_info.decimals,
//                     fee: utils::Nat2u64(&token_info.fee),
//                     index: utils::Nat2u64(&token_info.index),
//                     logo: token_info.logo.clone(),
//                     name: token_info.name.clone(),
//                     owner: token_info.owner.to_string(),
//                     symbol: token_info.symbol.clone(),
//                     timestamp: utils::Bigint2u64(&token_info.timestamp),
//                     supply: utils::Nat2u64(&token_info.totalSupply),
//                 };
//                 self.tokens.insert(token_info.canisterId.to_string(),new_token_info);
//             },
//         }
//     }
//     //后续在做NONE的处理吧
//     pub fn get_token_info(&self,canister_id:&String) -> token::TokenInfo{
//         let token_info = self.tokens.get(canister_id).unwrap();
//         let token_info = token::TokenInfo{
//             canister_id: token_info.canister_id.clone(),
//             decimals: token_info.decimals,
//             fee: token_info.fee,
//             index: token_info.index,
//             logo: token_info.logo.clone(),
//             name: token_info.name.clone(),
//             owner: token_info.owner.clone(),
//             symbol: token_info.symbol.clone(),
//             timestamp: token_info.timestamp,
//             supply: token_info.supply,
//         };
//         return token_info;
//     }

//     pub fn insert_pairs_info(&mut self,pairs_info:&Vec<dswap_data::PairInfoExt>){
//         for pair_info in pairs_info{
//             self.insert_pair_info(&pair_info);
//         }
//     }

//     pub fn insert_pair_info(&mut self,pair_info:&dswap_data::PairInfoExt){
//         match self.pairs.get(&pair_info.id){
//             Some(_) => (),
//             None => {
//                 let new_pair_info = dswap::PairInfo{
//                     id: pair_info.id.clone(), // principal
//                     supply: utils::Nat2u64(&pair_info.totalSupply),
//                     token0: pair_info.token0.clone(), //Principal;
//                     token1: pair_info.token1.clone(), 
//                     lp_token: pair_info.lptoken.clone(), 
//                     creator: pair_info.creator.to_string(),
//                     last_update_time: utils::Bigint2u64(&pair_info.blockTimestampLast),
//                     price0_cumulative: utils::Nat2u64(&pair_info.price0CumulativeLast),
//                     price1_cumulative: utils::Nat2u64(&pair_info.price1CumulativeLast),
//                     k:  utils::Nat2u64(&pair_info.kLast),
//                 };
//                 self.pairs.insert(pair_info.id.clone(), new_pair_info);
//             }
//         }
//     }

//     pub fn get_pair_info(&self,canister_id:&String) -> dswap::PairInfo{
//         let pair_info = self.pairs.get(canister_id).unwrap();
//         let pair_info = dswap::PairInfo{
//             id: pair_info.id.clone(), // principal
//             supply: pair_info.supply,
//             token0: pair_info.token0.clone(), //Principal;
//             token1: pair_info.token1.clone(), 
//             lp_token: pair_info.lp_token.clone(), 
//             creator: pair_info.creator.clone(),
//             last_update_time:pair_info.last_update_time,
//             price0_cumulative: pair_info.price0_cumulative,
//             price1_cumulative: pair_info.price1_cumulative,
//             k:  pair_info.k,
//         };
//         return pair_info;
//     }

//     pub fn insert_swap_tokens_info(&mut self,swap_tokens_info:&Vec<dswap_data::TokenInfoExt>){
//         for swap_token_info in swap_tokens_info{
//             self.insert_swap_token_info(swap_token_info);
//         }
//     }

//     pub fn insert_swap_token_info(&mut self, swap_token_info:&dswap_data::TokenInfoExt){
//         match self.swap_tokens.get(&swap_token_info.id){
//             Some(_) => (),
//             None =>
//             {
//                 let new_swap_toke_info = dswap::TokenInfo{
//                     id: swap_token_info.id.clone(),
//                     name: swap_token_info.name.clone(),
//                     symbol: swap_token_info.symbol.clone(),
//                     decimals: swap_token_info.decimals,
//                     fee: utils::Nat2u64(&swap_token_info.fee), // fee for internal transfer/approve
//                     supply: utils::Nat2u64(&swap_token_info.totalSupply),
//                 };
//                 self.swap_tokens.insert(swap_token_info.id.clone(),new_swap_toke_info);
//             }
//         }
//     }

//     pub fn get_swap_token_info(&self,id:&String) -> dswap::TokenInfo{
//         let swap_token_info = self.swap_tokens.get(id);
//         let info = match swap_token_info{
//             Some(swap_token_info)=>{
//                 let _info = dswap::TokenInfo{
//                     id: swap_token_info.id.clone(),
//                     name: swap_token_info.name.clone(),
//                     symbol: swap_token_info.symbol.clone(),
//                     decimals: swap_token_info.decimals,
//                     fee: swap_token_info.fee, // fee for internal transfer/approve
//                     supply: swap_token_info.supply,
//                 };
//                 _info
//             },
//             None => {
//                 let _info = dswap::TokenInfo{
//                     id: String::from(""),
//                     name: String::from(""),
//                     symbol: String::from(""),
//                     decimals: 0,
//                     fee: 0,
//                     supply: 0,
//                 };
//                 _info
//             },
//         };
//         return info;
//     }

//     pub fn put_token_user_info(&mut self, user_info:token::User){
//         self.token_user_table.insert(user_info.principal.clone(),user_info);
//     }

//     pub fn get_token_user_info(&self, user_id:&String) -> token::User{
//         let mut user = token::User{
//             principal:user_id.clone(),
//             balances:HashMap::new(),
//             transactions:HashMap::new(),
//         };
//         match self.token_user_table.get(user_id){
//             Some(user_info) => {
//                 user.balances = user_info.balances.clone();
//                 user.transactions = user_info.transactions.clone();
//             },
//             None => (),
//         }
//         return user;
//     }

//     pub fn put_dswap_user_info(&mut self, user_info:dswap::User){
//         self.dswap_user_table.insert(user_info.principal.clone(),user_info);
//     }

//     pub fn get_dswap_user_info(&self, user_id:&String) -> dswap::User{
//         let mut user = dswap::User{
//             principal:user_id.clone(),
//             balances:HashMap::new(),
//             lp_balances:HashMap::new(),
//             transactions:HashMap::new(),
//         };
//         match self.dswap_user_table.get(user_id){
//             Some(user_info) => {
//                 user.balances = user_info.balances.clone();
//                 user.lp_balances = user_info.lp_balances.clone();
//                 user.transactions = user_info.transactions.clone();
//             },
//             None => (),
//         }
//         return user;
//     }

//     pub fn insert_token_transactions(&mut self,txs:&mut Vec<token::Transaction>){
//         self.token_txs.append(txs);
//     }

//     pub fn get_token_transactions(&self,principal:&String,token_id:&String,txs:Vec<u64>) -> Vec<token::Transaction>{
//         return vec![];
//     }

//     pub fn insert_dswap_transactions(&mut self,txs:&mut Vec<dswap::Transaction>){
//         self.dswap_txs.append(txs);
//     }

//     pub fn get_dswap_transactions(&self,user_principal:&String,principal:&String,txs:Vec<u64>) -> Vec<dswap::Transaction>{
//         return vec![];
//     }

// }


#[cfg(test)]
mod tests {
    use tokio::runtime::Runtime;
    use ic_agent::{Agent,ic_types::Principal, agent::http_transport::ReqwestHttpReplicaV2Transport};
    // use tokio::time::*;
    use crate::PostProcess::DFINANCE as dfinance_process;
    use mysql::*;
    // use mysql::prelude::*;
    use super::*;

    #[actix_rt::test]
    async fn test_dfinance(){
    let token_register_cansiter_id = String::from("4ps6t-3aaaa-aaaah-qbonq-cai");
    let wicp_canister_id = String::from("42vp6-2iaaa-aaaah-qbooa-cai");
    let dft_canister_id = String::from("5l7rb-caaaa-aaaah-qbolq-cai");
    let dswap_canister_id = String::from("4grvp-niaaa-aaaah-qboma-cai");
    let user_principal = Principal::from_text("4qehi-lqyo6-afz4c-hwqwo-lubfi-4evgk-5vrn5-rldx2-lheha-xs7a4-gae").unwrap();
    let user_principal_string = String::from("4qehi-lqyo6-afz4c-hwqwo-lubfi-4evgk-5vrn5-rldx2-lheha-xs7a4-gae");

    let url = "mysql://root:xyz12345@localhost:3306/xyz";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let agent = Agent::builder()
	    .with_transport(ReqwestHttpReplicaV2Transport::create("https://ic0.app/").unwrap())
	    .build()
	    .unwrap();
    
    let mut dfinance = dfinance_process::DFinance::new(12,&token_register_cansiter_id, &dswap_canister_id.clone(),pool.clone());

    println!("Start run dfinance");
    dfinance.warm_boot().await;
    
    let dfinance_info = dfinance.dfinance_info.clone();


    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        tokio::spawn(
            async move {dfinance.loop_query_update().await}
        );
    });
    
    let dfinance_info = (dfinance_info.read().unwrap()).clone();
    println!("Dfinance info is {:#?}",&dfinance_info);
    
    let symbols = get_tokens_symbol(pool.clone(),&vec![wicp_canister_id.clone(),dft_canister_id.clone()]);
    println!("Token symbol is {:#?}",symbols);

    let symbols = get_dswap_token_symbol(pool.clone(),&String::from("42vp6-2iaaa-aaaah-qbooa-cai:5l7rb-caaaa-aaaah-qbolq-cai"));
    println!("lp token symbol is {:#?}",symbols);

    let user_balance = get_token_user_balances(pool.clone(), &user_principal_string);
    println!("User tokens balances : {:#?}",&user_balance);

    let history_size = get_token_user_history_size(&wicp_canister_id,&user_principal,&agent).await;
    println!("history size is : {}",history_size);

    let txs = get_token_transactions_by_user(pool.clone(),&wicp_canister_id,&user_principal_string,0,10);
    println!("Token transations is {:#?}",&txs);

    let user = get_dswap_user_balances(pool.clone(),&user_principal,&agent,&dswap_canister_id).await;
    println!("User swap balances : {:#?}",&user);

    let txs = get_dswap_transactions_by_user(pool.clone(),&String::from("4grvp-niaaa-aaaah-qboma-cai"),&wicp_canister_id,0,100);
    println!("swap transations is {:#?}",&txs);
    }
}