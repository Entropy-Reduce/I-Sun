
use std::{thread,time,collections::HashMap};
use candid::types::number::Nat;
use ic_agent::{ic_types::Principal,Agent, agent::http_transport::ReqwestHttpReplicaV2Transport};

use super::types::DFINANCE::dswap;
use super::types::DFINANCE::token;
use super::types::DFINANCE::dfinance;

use crate::QueryFunctions::DataStructure::_DFINANCE::_REGISTRY as registry_data;
// use crate::QueryFunctions::DataStructure::_DFINANCE::_DTOKEN as token_data;
use crate::QueryFunctions::DataStructure::_DFINANCE::_DSWAP as dswap_data;
use crate::QueryFunctions::DFINANCE::REGISTRY as registry_query;
use crate::QueryFunctions::DFINANCE::DSWAP as dswap_query;
use crate::QueryFunctions::DFINANCE::DTOKEN as token_query;
use crate::PostProcess::utils;


const TRANSACTION_PROCESS_INTERVAL:u64 = 100;// amount of txs
// const TOKEM_INFO_UPDATE_INTERVAL:u64 = 1000;// amout of txs
const TRANSACTION_PROCESS_GAP:time::Duration = time::Duration::from_millis(3_000);
// const TOKEM_INFO_UPDATE_GAP:time::Duration = time::Duration::from_millis(3_000);
// #[derive(Debug)]
pub struct DFianane {
    // tx_process_interval:u64,
    // tx_process_gap:time::Duration,
    token_canister_id: String, 
    dswap_canister_id: String,
    dfinance_info:dfinance::DfinanceInfo,
    db:MookDatabase,
    agent:Agent,
}

pub async fn new_dfinance(token_canister_id: String, dswap_canister_id: String)->DFianane{
    let agent = Agent::builder()
    .with_transport(ReqwestHttpReplicaV2Transport::create("https://ic0.app/").unwrap())
    .build()
    .unwrap();

    let dswap_info = dswap_query::get_dswap_info(&dswap_canister_id, &agent).await;
    let token_list = registry_query::query_token_list(&token_canister_id, &agent).await;
    let dfinance_info = dfinance::new_dfinance_info(&dswap_info, &token_list);
    let db = MookDatabase {
        token_user_table : HashMap::new(),
        dswap_user_table : HashMap::new(),

        tokens : HashMap::new(),
        swap_tokens : HashMap::new(),
        pairs:HashMap::new(),
        token_txs:Vec::new(),
        dswap_txs:Vec::new(),
    };

    DFianane{
        token_canister_id: token_canister_id, 
        dswap_canister_id: dswap_canister_id,
        agent:agent,
        db:db,
        dfinance_info:dfinance_info, 
    }
}

impl DFianane {
    //todo：我们目前没有存dswap_user的info
    pub async fn start(&mut self){
        //初始化数据库  
        let mut token_txs_pool : Vec<token::Transaction> = Vec::new();
        let mut dswap_txs_pool : Vec<dswap::Transaction> = Vec::new();
        let mut token_users_pool : HashMap<String,token::User> = HashMap::new();
        let mut dswap_users_pool : HashMap<String,dswap::User> = HashMap::new();
        // let mut token_users_pool : Vec<token::User> = Vec::new();
        // let mut swap_users_pool : Vec<dswap::User> = Vec::new();
        let mut token_tx_index:u64 = 0;
        let mut dswap_tx_index:u64 = 0;
    
        //start : 
        let dswap_init_info = dswap_query::get_dswap_info(&self.dswap_canister_id, &self.agent).await;
        let token_list = registry_query::query_token_list(&self.token_canister_id, &self.agent).await;
        self.dfinance_info.update(&dswap_init_info, &token_list);

        for token_info in token_list {
            //查询token_info是否存在，不存在写入数据库；
            let token_canister_id = utils::Principal2Identifier(&token_info.canisterId);
            let history_size = token_query::history_size(&token_canister_id, &self.agent).await;
            let _history_size = utils::Nat2u64(&history_size);
    
            while token_tx_index < _history_size {
                let end: Nat;
                if token_tx_index+TRANSACTION_PROCESS_INTERVAL > _history_size {
                    end = history_size.clone();
                }else{
                    end = utils::u642Nat(token_tx_index+TRANSACTION_PROCESS_INTERVAL);
                }
                let txs = token_query::get_transactions(&token_canister_id, &self.agent, &utils::u642Nat(token_tx_index), &end).await;
                token_tx_index = utils::Nat2u64(&end);
                for tx in &txs{
                    let caller_id = match tx.caller{
                        Some(_caller_id) => utils::Principal2Identifier(&_caller_id),
                        None => String::from("empty caller"),
                    };
                    let from_id = utils::Principal2Identifier(&tx.from);
                    let to_id = utils::Principal2Identifier(&tx.to);
    
                    let _tx = token::Transaction{
                        amount: utils::Nat2u64(&tx.amount),
                        fee: utils::Nat2u64(&tx.fee),
                        from: from_id.clone(), // principal
                        index: utils::Nat2u64(&tx.index),
                        op: utils::token_op_to_op(tx.op.clone()),
                        timestamp: utils::Bigint2u64(&tx.timestamp),
                        to: to_id.clone(),
                        caller: caller_id.clone(),
                    };
                    
                    //过于丑陋，后面研究一下
                    let mut caller = token::User{
                        principal:caller_id.clone(),
                        balances:HashMap::new(),
                        transactions:HashMap::new(),
                    };
                    match token_users_pool.get_mut(&caller_id){
                        Some(caller_info) => {
                            caller.balances = caller_info.balances.clone();
                            caller.transactions = caller_info.transactions.clone();
                        },
                        None => (),
                    };
    
                    let mut from = token::User{
                        principal:from_id.clone(),
                        balances:HashMap::new(),
                        transactions:HashMap::new(),
                    };
                    match token_users_pool.get_mut(&from_id){
                        Some(from_info) => {
                            from.balances = from_info.balances.clone();
                            from.transactions = from_info.transactions.clone();
                        },
                        None => (),
                    };
                
                    let mut to = token::User{
                        principal:to_id.clone(),
                        balances:HashMap::new(),
                        transactions:HashMap::new(),
                    };
                    match token_users_pool.get_mut(&to_id){
                        Some(to_info) => {
                            to.balances = to_info.balances.clone();
                            to.transactions = to_info.transactions.clone();
                        },
                        None => (),
                    };
    
                    _tx.process_transaction(&mut caller,&mut from,&mut to,&token_canister_id);
                    token_txs_pool.push(_tx);
                    token_users_pool.insert(caller_id.clone(), caller);
                    token_users_pool.insert(from_id.clone(), from);
                    token_users_pool.insert(to_id.clone(), to);
                }
                //todo:落数据库
                for (_, val) in token_users_pool.iter() {
                    let user_info = token::User{
                        principal:val.principal.clone(),
                        balances:val.balances.clone(),
                        transactions:val.transactions.clone(),
                    };
                    self.db.put_token_user_info(user_info);
                }
                self.db.insert_token_transactions(&mut token_txs_pool);
                token_txs_pool = vec![];
                token_users_pool = HashMap::new();
            }
        }
    
        
        let dswap_storage_canisster_id = utils::Principal2Identifier(&dswap_init_info.storageCanisterId);
        let dswap_history_size = dswap_query::history_size(&dswap_storage_canisster_id, &self.agent).await;
        let _dswap_history_size = utils::Nat2u64(&dswap_history_size);
        
        while dswap_tx_index < _dswap_history_size{
            let end: Nat;
            if dswap_tx_index+TRANSACTION_PROCESS_INTERVAL > _dswap_history_size {
                end = dswap_history_size.clone();
            }else{
                end = utils::u642Nat(token_tx_index+TRANSACTION_PROCESS_INTERVAL);
            }
            let txs = dswap_query::get_transactions(&self.dswap_canister_id, &self.agent, &utils::u642Nat(dswap_tx_index), &end).await;
            dswap_tx_index = utils::Nat2u64(&end);
            for tx in &txs{
                let caller_id = utils::Principal2Identifier(&tx.caller);
                let from_id = utils::Principal2Identifier(&tx.from);
                let to_id = utils::Principal2Identifier(&tx.to);
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
    
                let mut caller = dswap::User{
                    principal:caller_id.clone(),
                    balances:HashMap::new(),
                    lp_balances:HashMap::new(),
                    transactions:HashMap::new(),
                };
                match dswap_users_pool.get(&caller_id){
                    Some(caller_info) =>  {
                        caller.balances = caller_info.balances.clone();
                        caller.lp_balances = caller_info.lp_balances.clone();
                        caller.transactions = caller_info.transactions.clone();
                    },
                    None => (),
                };
    
                let mut from = dswap::User{
                    principal:from_id.clone(),
                    balances:HashMap::new(),
                    lp_balances:HashMap::new(),
                    transactions:HashMap::new(),
                };
                match dswap_users_pool.get(&from_id){
                    Some(from_info) => {
                        from.balances = from_info.balances.clone();
                        from.lp_balances = from_info.lp_balances.clone();
                        from.transactions = from_info.transactions.clone();
                    },
                    None => (),
                };
            
                let mut to = dswap::User{
                    principal:from_id.clone(),
                    balances:HashMap::new(),
                    lp_balances:HashMap::new(),
                    transactions:HashMap::new(),
                };
                match dswap_users_pool.get(&to_id){
                    Some(to_info) => {
                        to.balances = to_info.balances.clone();
                        to.lp_balances = to_info.lp_balances.clone();
                        to.transactions = to_info.transactions.clone();
                    },
                    None => (),
                };
    
                _tx.process_transaction(&mut caller, &mut from, &mut to);
                dswap_users_pool.insert(caller_id.clone(),caller);
                dswap_users_pool.insert(from_id.clone(),from);
                dswap_users_pool.insert(to_id.clone(),to);
                dswap_txs_pool.push(_tx);
            }
            
             //todo:落数据库
            for (_, val) in dswap_users_pool.iter(){
                let user_info = dswap::User{
                    principal:val.principal.clone(),
                    balances:val.balances.clone(),
                    lp_balances:val.lp_balances.clone(),
                    transactions:val.transactions.clone(),
                };
                self.db.put_dswap_user_info(user_info); 
            }
            self.db.insert_dswap_transactions(&mut dswap_txs_pool);
            dswap_txs_pool = vec![];
        }
    
        loop{
            //来个定时器：
            //上面的操作来一遍
            thread::sleep(TRANSACTION_PROCESS_GAP);
            //更新基础信息
            let dswap_info = dswap_query::get_dswap_info(&self.dswap_canister_id, &self.agent).await;
            let token_list = registry_query::query_token_list(&self.token_canister_id, &self.agent).await;
            self.dfinance_info.update(&dswap_info, &token_list);
            //目前是直接全部更新，太丑陋了
            self.db.insert_tokens_info(&token_list);
            self.db.insert_pairs_info(&dswap_info.pairs);
            self.db.insert_swap_tokens_info(&dswap_init_info.tokens);
            //更新每一个token的用户信息，并且搞定transitions
            for token_info in token_list {
                let token_canister_id = utils::Principal2Identifier(&token_info.canisterId);
                let history_size = token_query::history_size(&token_canister_id, &self.agent).await;
                let txs = token_query::get_transactions(&token_canister_id, &self.agent,&utils::u642Nat(token_tx_index),&history_size).await;
                token_tx_index = utils::Nat2u64(&history_size);
                for tx in &txs{
                    let caller_id = match tx.caller{
                        Some(_caller_id) => utils::Principal2Identifier(&_caller_id),
                        None => String::from("empty caller"),
                    };
                    let from_id = utils::Principal2Identifier(&tx.from);
                    let to_id = utils::Principal2Identifier(&tx.to);
    
                    let _tx = token::Transaction{
                        amount: utils::Nat2u64(&tx.amount),
                        fee: utils::Nat2u64(&tx.fee),
                        from: from_id.clone(), // principal
                        index: utils::Nat2u64(&tx.index),
                        op: utils::token_op_to_op(tx.op.clone()),
                        timestamp: utils::Bigint2u64(&tx.timestamp),
                        to: to_id.clone(),
                        caller: caller_id.clone(),
                    };
                        //todo:从数据库中查询user，没有的话就给一个新的
                    let mut caller = self.db.get_token_user_info(&caller_id);
                    let mut from = self.db.get_token_user_info(&from_id);
                    let mut to = self.db.get_token_user_info(&to_id);
        
                    _tx.process_transaction(&mut caller,&mut from,&mut to,&token_canister_id);
                    token_txs_pool.push(_tx);
                    self.db.put_token_user_info(caller);
                    self.db.put_token_user_info(from);
                    self.db.put_token_user_info(to);
                }
                    
                self.db.insert_token_transactions(&mut token_txs_pool);
                token_txs_pool = vec![];
                //token_users_pool = HashMap::new();
        }
            
            let dswap_history_size = dswap_query::history_size(&dswap_storage_canisster_id, &self.agent).await;
            let txs = dswap_query::get_transactions(&self.dswap_canister_id, &self.agent, &utils::u642Nat(dswap_tx_index), &dswap_history_size).await;
            dswap_tx_index = utils::Nat2u64(&dswap_history_size);
            for tx in &txs{
                let caller_id = utils::Principal2Identifier(&tx.caller);
                let from_id = utils::Principal2Identifier(&tx.from);
                let to_id = utils::Principal2Identifier(&tx.to);
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
                let mut caller = self.db.get_dswap_user_info(&caller_id);
                let mut from = self.db.get_dswap_user_info(&from_id);
                let mut to = self.db.get_dswap_user_info(&to_id);
                _tx.process_transaction(&mut caller, &mut from, &mut to);
                dswap_txs_pool.push(_tx);
                self.db.put_dswap_user_info(caller);
                self.db.put_dswap_user_info(from);
                self.db.put_dswap_user_info(to);
            }
            dswap_txs_pool = vec![];
            //dswap_users_pool = HashMap::new();
        }
    }

    pub fn get_dfinance_info(&self)->dfinance::DfinanceInfo{
        return self.dfinance_info.clone();
    }

    pub fn get_token_user_balance(&self,token_principal:&String, user_principal:&String) -> u64{
        let user = self.db.get_token_user_info(user_principal);
        match user.balances.get(token_principal){
            Some(balance) => *balance,
            None => 0,
        }
    }

    pub fn get_token_symbol(&self,token_principal:&String) ->String{
        self.db.get_token_info(token_principal).symbol
    }

    pub fn get_tokens_symbol(&self,tokens_principal:&Vec<String>) ->Vec<String>{
        let mut symbols = Vec::new();
        for canister_id in tokens_principal{
            let symbol = self.db.get_token_info(&canister_id).symbol;
            symbols.push(symbol);
        }
        return symbols;
    }

    pub fn get_dswap_token_symbol(&self,token_principal:&String) -> String{
        return self.db.get_swap_token_info(token_principal).symbol;
    }

    pub fn get_dswap_tokens_symbol(&self,tokens_principal:&Vec<String>) ->Vec<String>{
        let mut symbols = Vec::new();
        for canister_id in tokens_principal{
            let symbol = self.db.get_swap_token_info(canister_id).symbol;
            symbols.push(symbol);
        }
        return symbols;
    }


    //用这个
    pub fn get_token_user_balances(&self,user_principal:&String) -> HashMap<String,u64>{
        let balanes = self.db.get_token_user_info(user_principal).balances;
        return balanes;
    }

   

    pub async fn get_token_user_history_size(&self,token_principal:&String,user_principal:&Principal) -> u64{
        let size = token_query::get_user_transactiom_amount(token_principal,&self.agent,user_principal).await;
        return utils::Nat2u64(&size);
    }

    pub fn get_token_transactions_by_user(&self,token_principal:&String, user_principal:&String,from:usize,to:usize) -> Vec<token::Transaction>{
        let user = self.db.get_token_user_info(user_principal);
        let txs_index = match user.transactions.get(token_principal){
            Some(_txs_index) => {
                let mut transactions = Vec::new();
                if to < _txs_index.len(){
                    for i in (from..to){
                        transactions.push(_txs_index[i]);
                    }
                } else{
                    for i in (from.._txs_index.len()){
                        transactions.push(_txs_index[i]);
                    }
                }
                transactions
            },
            None => {
                return vec![];
            },
        };
        let transactions = self.db.get_token_transactions(user_principal,token_principal,txs_index);
        return transactions;
        //根据token的principal和index取token
    }

    // pub fn get_all_token_transactions_by_user(&self,user_principal:&String,from:u64,to:u64) -> HashMap<String,Vec<token::Transaction>>{
    //     let user = self.db.get_token_user_info(user_principal);
        
    // }

    pub async fn get_dswap_user_balances(&self, user_principal:&Principal) -> dswap::User{
        let balances = dswap_query::get_user_info_above(&self.dswap_canister_id,
                                                         &self.agent,
                                                         user_principal,
                                                         &utils::u642Nat(0),
                                                         &utils::u642Nat(0)
                                                        ).await;
        let mut user = dswap::User{
            principal: utils::Principal2Identifier(user_principal),
            balances: HashMap::new(),
            lp_balances:HashMap::new(),
            transactions:HashMap::new(),
        };

        for balance in &balances{
            if balance.0.contains(":"){
                user.lp_balances.insert(balance.0.clone(),utils::Nat2u64(&balance.1));
            } else{
                user.balances.insert(balance.0.clone(),utils::Nat2u64(&balance.1));
            }    
        }
        return user
    }

    pub fn get_dswap_transactions_by_user(&self,user_principal:&String,token_principal:&String,from:usize,to:usize) -> Vec<dswap::Transaction>{
        let user = self.db.get_dswap_user_info(user_principal);
        let txs_index = match user.transactions.get(token_principal){
            Some(_txs_index) => {
                let mut transactions = Vec::new();
                if to < _txs_index.len(){
                    for i in (from..to){
                        transactions.push(_txs_index[i]);
                    }
                } else{
                    for i in (from.._txs_index.len()){
                        transactions.push(_txs_index[i]);
                    }
                }
                transactions
            },
            None => {
                return vec![];
            },
        };
        let transactions = self.db.get_dswap_transactions(user_principal,token_principal,txs_index);
        return transactions;
        
    }
}

#[derive(Debug)]
pub struct MookDatabase {
    token_user_table : HashMap<String,token::User>,
    dswap_user_table : HashMap<String,dswap::User>,
    tokens : HashMap<String,token::TokenInfo>,
    swap_tokens : HashMap<String,dswap::TokenInfo>,
    pairs:HashMap<String,dswap::PairInfo>,
    token_txs:Vec<token::Transaction>,
    dswap_txs:Vec<dswap::Transaction>,
}

impl MookDatabase{
    pub fn insert_tokens_info(&mut self, tokens_info:&Vec<registry_data::TokenInfo>){
        for token_info in tokens_info{
            self.insert_token_info(token_info);
        }
    }

    pub fn insert_token_info(&mut self,token_info:&registry_data::TokenInfo){
        match self.tokens.get(&utils::Principal2Identifier(&token_info.canisterId)){
            Some(_) => (),
            None => {
                let new_token_info = token::TokenInfo{
                    canister_id: utils::Principal2Identifier(&token_info.canisterId),
                    decimals: token_info.decimals,
                    fee: utils::Nat2u64(&token_info.fee),
                    index: utils::Nat2u64(&token_info.index),
                    logo: token_info.logo.clone(),
                    name: token_info.name.clone(),
                    owner: utils::Principal2Identifier(&token_info.owner),
                    symbol: token_info.symbol.clone(),
                    timestamp: utils::Bigint2u64(&token_info.timestamp),
                    supply: utils::Nat2u64(&token_info.totalSupply),
                };
                self.tokens.insert(utils::Principal2Identifier(&token_info.canisterId),new_token_info);
            },
        }
    }
    //后续在做NONE的处理吧
    pub fn get_token_info(&self,canister_id:&String) -> token::TokenInfo{
        let token_info = self.tokens.get(canister_id).unwrap();
        let token_info = token::TokenInfo{
            canister_id: token_info.canister_id.clone(),
            decimals: token_info.decimals,
            fee: token_info.fee,
            index: token_info.index,
            logo: token_info.logo.clone(),
            name: token_info.name.clone(),
            owner: token_info.owner.clone(),
            symbol: token_info.symbol.clone(),
            timestamp: token_info.timestamp,
            supply: token_info.supply,
        };
        return token_info;
    }

    pub fn insert_pairs_info(&mut self,pairs_info:&Vec<dswap_data::PairInfoExt>){
        for pair_info in pairs_info{
            self.insert_pair_info(&pair_info);
        }
    }

    pub fn insert_pair_info(&mut self,pair_info:&dswap_data::PairInfoExt){
        match self.pairs.get(&pair_info.id){
            Some(_) => (),
            None => {
                let new_pair_info = dswap::PairInfo{
                    id: pair_info.id.clone(), // principal
                    supply: utils::Nat2u64(&pair_info.totalSupply),
                    token0: pair_info.token0.clone(), //Principal;
                    token1: pair_info.token1.clone(), 
                    lp_token: pair_info.lptoken.clone(), 
                    creator: utils::Principal2Identifier(&pair_info.creator),
                    last_update_time: utils::Bigint2u64(&pair_info.blockTimestampLast),
                    price0_cumulative: utils::Nat2u64(&pair_info.price0CumulativeLast),
                    price1_cumulative: utils::Nat2u64(&pair_info.price1CumulativeLast),
                    k:  utils::Nat2u64(&pair_info.kLast),
                };
                self.pairs.insert(pair_info.id.clone(), new_pair_info);
            }
        }
    }

    pub fn get_pair_info(&self,canister_id:&String) -> dswap::PairInfo{
        let pair_info = self.pairs.get(canister_id).unwrap();
        let pair_info = dswap::PairInfo{
            id: pair_info.id.clone(), // principal
            supply: pair_info.supply,
            token0: pair_info.token0.clone(), //Principal;
            token1: pair_info.token1.clone(), 
            lp_token: pair_info.lp_token.clone(), 
            creator: pair_info.creator.clone(),
            last_update_time:pair_info.last_update_time,
            price0_cumulative: pair_info.price0_cumulative,
            price1_cumulative: pair_info.price1_cumulative,
            k:  pair_info.k,
        };
        return pair_info;
    }

    pub fn insert_swap_tokens_info(&mut self,swap_tokens_info:&Vec<dswap_data::TokenInfoExt>){
        for swap_token_info in swap_tokens_info{
            self.insert_swap_token_info(swap_token_info);
        }
    }

    pub fn insert_swap_token_info(&mut self, swap_token_info:&dswap_data::TokenInfoExt){
        match self.swap_tokens.get(&swap_token_info.id){
            Some(_) => (),
            None =>
            {
                let new_swap_toke_info = dswap::TokenInfo{
                    id: swap_token_info.id.clone(),
                    name: swap_token_info.name.clone(),
                    symbol: swap_token_info.symbol.clone(),
                    decimals: swap_token_info.decimals,
                    fee: utils::Nat2u64(&swap_token_info.fee), // fee for internal transfer/approve
                    supply: utils::Nat2u64(&swap_token_info.totalSupply),
                };
                self.swap_tokens.insert(swap_token_info.id.clone(),new_swap_toke_info);
            }
        }
    }

    pub fn get_swap_token_info(&self,id:&String) -> dswap::TokenInfo{
        let swap_token_info = self.swap_tokens.get(id);
        let info = match swap_token_info{
            Some(swap_token_info)=>{
                let _info = dswap::TokenInfo{
                    id: swap_token_info.id.clone(),
                    name: swap_token_info.name.clone(),
                    symbol: swap_token_info.symbol.clone(),
                    decimals: swap_token_info.decimals,
                    fee: swap_token_info.fee, // fee for internal transfer/approve
                    supply: swap_token_info.supply,
                };
                _info
            },
            None => {
                let _info = dswap::TokenInfo{
                    id: String::from(""),
                    name: String::from(""),
                    symbol: String::from(""),
                    decimals: 0,
                    fee: 0,
                    supply: 0,
                };
                _info
            },
        };
        return info;
    }

    pub fn put_token_user_info(&mut self, user_info:token::User){
        self.token_user_table.insert(user_info.principal.clone(),user_info);
    }

    pub fn get_token_user_info(&self, user_id:&String) -> token::User{
        let mut user = token::User{
            principal:user_id.clone(),
            balances:HashMap::new(),
            transactions:HashMap::new(),
        };
        match self.token_user_table.get(user_id){
            Some(user_info) => {
                user.balances = user_info.balances.clone();
                user.transactions = user_info.transactions.clone();
            },
            None => (),
        }
        return user;
    }

    pub fn put_dswap_user_info(&mut self, user_info:dswap::User){
        self.dswap_user_table.insert(user_info.principal.clone(),user_info);
    }

    pub fn get_dswap_user_info(&self, user_id:&String) -> dswap::User{
        let mut user = dswap::User{
            principal:user_id.clone(),
            balances:HashMap::new(),
            lp_balances:HashMap::new(),
            transactions:HashMap::new(),
        };
        match self.dswap_user_table.get(user_id){
            Some(user_info) => {
                user.balances = user_info.balances.clone();
                user.lp_balances = user_info.lp_balances.clone();
                user.transactions = user_info.transactions.clone();
            },
            None => (),
        }
        return user;
    }

    pub fn insert_token_transactions(&mut self,txs:&mut Vec<token::Transaction>){
        self.token_txs.append(txs);
    }

    pub fn get_token_transactions(&self,principal:&String,token_id:&String,txs:Vec<u64>) -> Vec<token::Transaction>{
        return vec![];
    }

    pub fn insert_dswap_transactions(&mut self,txs:&mut Vec<dswap::Transaction>){
        self.dswap_txs.append(txs);
    }

    pub fn get_dswap_transactions(&self,user_principal:&String,principal:&String,txs:Vec<u64>) -> Vec<dswap::Transaction>{
        return vec![];
    }

}
