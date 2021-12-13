use ic_agent::ic_types::Principal;
use candid::{CandidType, types::number::Int, types::number::Nat};
use serde::Deserialize;

#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct Empty{}



#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct Profile{
    address: String,
    name: String,
    description: String,
}

pub type Profiles = Vec<Profile>;





