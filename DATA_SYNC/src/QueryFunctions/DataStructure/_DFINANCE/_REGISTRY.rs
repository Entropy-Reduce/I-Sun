use ic_agent::{ic_types::Principal};
use candid::{CandidType, types::number::Int, types::number::Nat};
use serde::Deserialize;

#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct TokenInfo {
   pub canisterId: Principal,
   pub decimals: u8,
   pub fee: Nat,
   pub index: Nat,
   pub logo: String,
   pub name: String,
   pub owner: Principal,
   pub symbol: String,
   pub timestamp: Int,
   pub totalSupply: Nat,
}

#[derive(Debug, Deserialize, CandidType)]
pub struct Empty{}

pub type TokenList = Vec<TokenInfo>;