use crate::PostProcess::types::NFT as NFT_data;
use std::collections::{HashSet,VecDeque,HashMap};
use ic_agent::ic_types::Principal;
use mysql::*;
use mysql::prelude::*;


fn get_connection()->PooledConn{
    // Create connection
    let url = "mysql://root:SJTUZY@localhost:3306/test";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    pool.get_conn().unwrap()
}


//insert transaction
pub fn insert_tx(conn: &mut PooledConn, tx:&NFT_data::Transaction, dapp_id: u64){
    let token_id = tx.tokenIndex;
    // let dapp: String = Canister_id.to_text();
    //time price from to 字段在tx中

    // insert
    conn.exec_drop(
        "insert into tx (from_, to_, time_, price, dapp_id, index_) values (:from_, :to_, :time_, :price, :dapp_id, :index_ )",
        params! {
            "from_" => tx.from.clone(),
            "to_" => tx.to.clone(),
            "time_" => tx.time,
            "price" => tx.price,
            "dapp_id" => dapp_id,
            "index_" => token_id
        }
    ).unwrap();

}


//insert user index
pub fn insert_user_index(conn: &mut PooledConn, tx:&NFT_data::Transaction, dapp_id: u64){
    let user: String = tx.to.clone();
    let index: u64 = tx.tokenIndex;
    // let dapp: String = Canister_id.to_text();
    //insert index 到该用户的dapp下的set中

    // insert user index
    conn.exec_drop(
        "insert into user_dapp_index (identifier, dapp_id, index_) values(:identifier, :dapp_id, :index_)",
        params! {
            "identifier" => user,
            "dapp_id" => dapp_id,
            "index_" => index,
        }
    ).unwrap();
}


pub fn delete_user_index(conn: &mut PooledConn, tx:&NFT_data::Transaction, dapp_id:u64){
    let user: String = tx.from.clone();
    let index: u64 = tx.tokenIndex;
    // let dapp: String = Canister_id;
    //delete 该用户在该dapp下的set中

    // get user_id
    // let res = conn.query_first(format!("select id from user where identifier = '{}'", user))
    //     .map(
    //         |row| {
    //             row.map(|id|->u32 {id})
    //         }
    //     );
    
    // let user_id = match res.unwrap() {
    //     Some(id) => id,
    //     None => {println!("Can not find this user. "); 0},
    // };

    // delete
    let stmt = conn.prep("delete from user_dapp_index where identifier=:identifier and dapp_id=:dapp_id and index_=:index").unwrap();
    conn.exec_drop(&stmt, params!{
        "identifier" => user,
        "dapp_id" => dapp_id,
        "index" => index,
    }).unwrap();

}

//update principal
pub fn insert_principal_identifier_pair(conn: &mut PooledConn, Principal:&Principal, Identifier:&NFT_data::AccountIdentifier){
    let principal = Principal.to_text();
    let identifier = Identifier;

    let res = conn.query_first(format!("select count(*) from user where identifier = '{}'", identifier))
        .map(
            |row| {
                row.map(|id|->u32 {id})
            }
        );
    
    let is_exist = match res.unwrap() {
        Some(exist) => exist,
        None => {0},
    };
    
    if is_exist == 0 {
        conn.exec_drop(
            "insert into user (identifier, principal) values (:identifier, :principal)",
            params! {
                "identifier" => identifier,
                "principal" => principal,
            }
        ).unwrap();
    }
}

// pub fn sql_principal(conn: &mut PooledConn, Identifier:&NFT_data::AccountIdentifier)->{
//     let res = conn.query_first(format!("select count(*) from user where identifier = '{}'", identifier));
//     if let Ok(data) = res{
//         if let 
//     }
//     let is_exist = match res.unwrap() {
//         Some(exist) => exist,
//         None => {0},
//     };
//     is_exist
// }

//query tx
pub fn sql_transactions(conn: &mut PooledConn, dapp_id: u64, index:&Vec<u64>)->HashMap<u64, Vec<NFT_data::Transaction>>{
    // pub fn sql_transactions(conn: &mut PooledConn,dapp_id: u64, index:&Vec<u64>){
    
    //找出所有的在该canister下面的index的交易记录
    let mut transactions = HashMap::new();
    
    // let res: Vec<(AccountIdentifier, AccountIdentifier, u64, u64)> = conn.query(format!("select (from_, to_ , time, price) from user_dapp_index where dapp_id={} and index_={}", dapp_id, idx)).unwrap();
    for idx in index {
        let res: Vec<(NFT_data::AccountIdentifier, NFT_data::AccountIdentifier, u64, u64)> = conn.query(format!("select from_, to_ , time_, price from tx where dapp_id={} and index_={}", dapp_id, idx)).unwrap();
        let mut current_transactions: Vec<NFT_data::Transaction> = Vec::new();
        for r in res {
            current_transactions.push(NFT_data::Transaction{
                tokenIndex: *idx,
                from: r.0,
                to: r.1,
                time: r.2,
                price: r.3,
            });
        }
        transactions.insert(*idx, current_transactions);
    }
    transactions
    //查询后封装，按照返回类型封装
}


//query user info
pub fn sql_userinfo(conn: &mut PooledConn, user: &NFT_data::AccountIdentifier)->HashMap<u64, Vec<u64>>
{
    //这里存疑，每个用户的dapp不一样，是否可以直接拿出来，并能显示dapp的索引
    //Principal.from_text(String).unwrap() 

    // get user_id
    // let res = conn.query_first(format!("select id from user where identifier = '{}'", user))
    //     .map(
    //         |row| {
    //             row.map(|id|->u32 {id})
    //         }
    //     );
    
    // let user_id = match res.unwrap() {
    //     Some(id) => id,
    //     None => {println!("Can not find this user. "); 0},
    // };

    // query user info
    let mut user_info: HashMap<u64, Vec<u64>> = HashMap::new();
    let res: Vec<(u64, u64)> = conn.query(format!("select dapp_id, index_ from user_dapp_index where identifier='{}'", user)).unwrap();
    // let mut tmp: Vec<u64> = Vec::new();
    for r in res {
        let mut tmp = match user_info.get(&r.0) {
            None => {Vec::new()}, 
            Some(_) => {
                user_info[&r.0].clone()
                }
        };
        tmp.push(r.1);
        user_info.insert(r.0, tmp);
    }
    user_info
}


// 返回某个dapp在一个时间戳以后的所有交易记录
pub fn tx_after_timestamp(conn:&mut PooledConn, dapp_id: u64, timestamp: u64)->Vec<NFT_data::Transaction> {
    let mut transactions: Vec<NFT_data::Transaction> = Vec::new();
    let res: Vec<(NFT_data::AccountIdentifier, NFT_data::AccountIdentifier, u64, u64, u64)> = conn.query(format!("select from_, to_ , time_, price, index_ from tx where dapp_id={} and time_ >= {} order by time_", dapp_id, timestamp)).unwrap();
    for r in res {
        transactions.push(NFT_data::Transaction{
            tokenIndex: r.4,
            from: r.0,
            to: r.1,
            time: r.2,
            price: r.3,
        });
    }
    transactions
}


// 返回该dapp下一共多少条交易
pub fn get_length(conn: &mut PooledConn, dapp_id: u64)->u64 {
    let res = conn.exec_first("select count(*) from tx where dapp_id=:dapp_id", params! {"dapp_id" => dapp_id})
    .map(
        |row| {
            row.map(|length| length)
        }
    );
    res.unwrap().unwrap()
}