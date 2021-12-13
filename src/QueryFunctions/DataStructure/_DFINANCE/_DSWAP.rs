use ic_agent::{ic_types::Principal};
use candid::{CandidType, types::number::{Int,Nat}};
use serde::Deserialize;

#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct Empty{}

#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct TokenInfoExt{
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub fee: Nat, // fee for internal transfer/approve
    pub totalSupply: Nat,
}

#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct PairInfoExt{
    pub id: String,
    pub token0: String, //Principal;
    pub token1: String, //Principal;
    pub creator: Principal,
    pub reserve0: Nat,
    pub reserve1: Nat,
    pub price0CumulativeLast: Nat,
    pub price1CumulativeLast: Nat,
    pub kLast: Nat,
    pub blockTimestampLast: Int,
    pub totalSupply: Nat,
    pub lptoken: String,
}

#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct DSwapInfo {
    pub cycles: Nat,
    pub owner: Principal,
    pub pairs: Vec<PairInfoExt>,
    pub storageCanisterId: Principal,
    pub tokens: Vec<TokenInfoExt>,
}

#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct UserInfo {
    pub balances: Vec<(String,Nat)>,
    pub lpBalances: Vec<(String,Nat)>,
}

// struct Balances 

 //storage data structure
#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct TxRecord {
    pub amount:Nat,
    pub amount0: Nat,
    pub amount1: Nat,
    pub caller: Principal,
    pub fee: Nat,
    pub from: Principal,
    pub index: Nat,
    pub op: Operation,
    pub timestamp: Int,
    pub to: Principal,
    pub tokenId: String,
 }

 pub type TxRecords = Vec<TxRecord>;

#[derive(Debug, Deserialize, CandidType, Clone)]
 pub enum Operation {
    addLiquidity,
    createPair,
    deposit,
    lpApprove,
    lpTransfer,
    lpTransferFrom,
    removeLiquidity,
    swap,
    tokenApprove,
    tokenTransfer,
    tokenTransferFrom,
    withdraw,
  }
  
 