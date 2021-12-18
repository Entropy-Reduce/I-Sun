use ic_agent::ic_types::Principal;
use candid::{CandidType, types::number::{Int, Nat}};
use serde::Deserialize;
use crate::PostProcess::types::NFT;

#[derive(Debug, Deserialize, CandidType)]
pub struct Empty{}

//transaction
#[derive(Debug, Deserialize, CandidType)]
pub struct transaction{
	pub buyer : NFT::AccountIdentifier,
	pub price :u64,
	pub seller : Principal,
	pub time: Int,
	pub token: TokenIdentifier,
}

pub type transactions = Vec<transaction>;


//Listing
#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct Fungible{
	decimals:u8,
	metadata:Option<Vec<u8>>,
	name:String,
	symbol:String,
}
#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct NonFungible{
	metadata:Option<Vec<u8>>,
}
#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum Metadata{
	fungible(Fungible),
	nonfungible(NonFungible),

}
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Listing{
	locked: Option<Int>,
	pub price: u64,
	pub seller: Principal,
}

pub type Listings = Vec<(TokenIndex, Listing, Metadata)>;

//Registry
pub type Registrys = Vec<(TokenIndex, String)>;

pub type TokenIdentifier = String;
pub type TokenIndex = u32;


#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum CommonError{
	InvalidToken(TokenIdentifier),
	Other(String),
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum Result_1{
	err(CommonError),
	ok(Vec<TokenIndex>),
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum Result_2{
	err(CommonError),
	ok(Nat),
}