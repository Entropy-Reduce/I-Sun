use ic_agent::ic_types::Principal;
use candid::{CandidType, types::number::Int, types::number::Nat};
use serde::Deserialize;

#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct Empty{}

#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct Listing{
    pub owner : Principal,
    pub price : u64,
    pub time: Int,
    pub token_id: Nat,
}


pub type Listings = Vec<Listing>;


pub type LenofListings = Nat;

pub type StorageCanister = Option<Principal>;

#[derive(Debug, Deserialize, CandidType, Clone)]
pub enum Operation{
    delist,
    init,
    list,
    mint,
    purchase,
    transfer,
}

#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct transaction{
    pub caller: Principal,
    pub to: Option<Principal>,
    pub from: Option<Principal>,
    pub index: Nat,
    pub price: Option<u64>,
    pub timestamp: Int,
    pub tokenId: Nat,
    pub op: Operation,
}

pub type transactions = Vec<transaction>;
pub type txAmount = Nat;