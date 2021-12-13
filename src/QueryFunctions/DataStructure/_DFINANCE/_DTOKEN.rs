use ic_agent::ic_types::Principal;
use candid::{CandidType, types::number::Int, types::number::Nat};
use serde::Deserialize;

#[derive(Debug, Deserialize, CandidType)]
pub struct Empty{}

#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct Transaction {
    pub amount: Nat,
    pub caller: Option<Principal>,
    pub fee: Nat,
    pub from: Principal,
    pub index: Nat,
    pub op: Operation,
    pub timestamp: Int,
    pub to: Principal,
}

pub type Transactions = Vec<Transaction>;

#[derive(Debug, Deserialize, CandidType, Clone)]
pub enum Operation{
    approve,
    burn(u64),
    mint,
    transfer,
    transferFrom,
} 
