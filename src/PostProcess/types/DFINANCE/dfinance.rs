use std::collections::HashMap;

use super::dswap;
use super::token;

use crate::QueryFunctions::DataStructure::_DFINANCE::_REGISTRY as registry_data;
// use crate::QueryFunctions::DataStructure::_DFINANCE::_DTOKEN as token_data;
use crate::QueryFunctions::DataStructure::_DFINANCE::_DSWAP as dswap_data;

use crate::PostProcess::utils;

// pub fn init_token_info(token_list:&registry_data::TokenList) -> token::TokenLists{
//     let mut token_list = Vec::new();
//     for token_info in token_list{
//         // let _token = token::TokenInfo{
//         //     canister_id:token_info.canisterId,
//         // }
//     }
// }
#[derive(Debug,Clone)]
pub struct DfinanceInfo{
    pub pairs:HashMap<String,dswap::PairInfo>,
    pub lp_tokens:HashMap<String,dswap::TokenInfo>,
    pub tokens:HashMap<String,token::TokenInfo>,
}

pub fn new_dfinance_info(dswap_info:&dswap_data::DSwapInfo,tokens_info:&registry_data::TokenList) -> DfinanceInfo {
    let mut pairs = HashMap::new();
    let mut lp_tokens = HashMap::new();
    let mut tokens = HashMap::new();

    for token_info in tokens_info{
        let _token_info = token::TokenInfo{
            canister_id : utils::Principal2Identifier(&token_info.canisterId),
            decimals : token_info.decimals,
            fee : utils::Nat2u64(&token_info.fee),
            index :  utils::Nat2u64(&token_info.index),
            logo:token_info.logo.clone(),
            name:token_info.name.clone(),
            owner:utils::Principal2Identifier(&token_info.owner),
            symbol:token_info.symbol.clone(),
            timestamp: utils::Bigint2u64(&token_info.timestamp),
            supply: utils::Nat2u64(&token_info.totalSupply),
        };
        tokens.insert(_token_info.canister_id.clone(),_token_info);
    }

    for pair_info in &dswap_info.pairs {
        let _pair_info = dswap::PairInfo{
            id: pair_info.id.clone(), // principal
            supply: utils::Nat2u64(&pair_info.totalSupply),
            token0: pair_info.token0.clone(), //Principal;
            token1: pair_info.token1.clone(), //Principal;
            lp_token:pair_info.lptoken.clone(),
            creator: utils::Principal2Identifier(&pair_info.creator),
            last_update_time: utils::Bigint2u64(&pair_info.blockTimestampLast),
            price0_cumulative: utils::Nat2u64(&pair_info.price0CumulativeLast),
            price1_cumulative: utils::Nat2u64(&pair_info.price1CumulativeLast),
            k: utils::Nat2u64(&pair_info.kLast),
        };
        pairs.insert(_pair_info.id.clone(), _pair_info);
    }

    for lp_token_info in &dswap_info.tokens{
        let _lp_token_info = dswap::TokenInfo{
            id: lp_token_info.id.clone(),
            name: lp_token_info.name.clone(),
            symbol: lp_token_info.symbol.clone(),
            decimals: lp_token_info.decimals,
            fee: utils::Nat2u64(&lp_token_info.fee), // fee for internal transfer/approve
            supply: utils::Nat2u64(&lp_token_info.totalSupply),
        };
        lp_tokens.insert(lp_token_info.id.clone(), _lp_token_info);
    }

    //todo: 落数据库，时间做一个key
    return DfinanceInfo{
        pairs:pairs,
        lp_tokens:lp_tokens,
        tokens : tokens,
    }
}

impl DfinanceInfo{
    pub fn update(&mut self,dswap_info:&dswap_data::DSwapInfo,tokens_info:&registry_data::TokenList){
        for token_info in tokens_info{
            let id = utils::Principal2Identifier(&token_info.canisterId);
            match self.tokens.get_mut(&id){
                Some(token) => {
                    token.supply = utils::Nat2u64(&token_info.totalSupply);
                },
                _ => {
                    let _token_info = token::TokenInfo{
                        canister_id : utils::Principal2Identifier(&token_info.canisterId),
                        decimals : token_info.decimals,
                        fee : utils::Nat2u64(&token_info.fee),
                        index :  utils::Nat2u64(&token_info.index),
                        logo:token_info.logo.clone(),
                        name:token_info.name.clone(),
                        owner:utils::Principal2Identifier(&token_info.owner),
                        symbol:token_info.symbol.clone(),
                        timestamp: utils::Bigint2u64(&token_info.timestamp),
                        supply: utils::Nat2u64(&token_info.totalSupply),
                    };
                    self.tokens.insert(id,_token_info);
                },
            }
        }
    
        for pair_info in &dswap_info.pairs {
            let id = pair_info.id.clone();
            match self.pairs.get_mut(&id){
                Some(pair) => {
                    pair.supply = utils::Nat2u64(&pair_info.totalSupply);
                    pair.last_update_time =  utils::Bigint2u64(&pair_info.blockTimestampLast);
                    pair.price0_cumulative = utils::Nat2u64(&pair_info.price0CumulativeLast);
                    pair.price1_cumulative = utils::Nat2u64(&pair_info.price1CumulativeLast);
                    pair.k = utils::Nat2u64(&pair_info.kLast);
                }
                _ => {
                    let _pair_info = dswap::PairInfo{
                        id: pair_info.id.clone(), // principal
                        supply: utils::Nat2u64(&pair_info.totalSupply),
                        token0: pair_info.token0.clone(), //Principal;
                        token1: pair_info.token1.clone(), //Principal;
                        lp_token:pair_info.lptoken.clone(),
                        creator: utils::Principal2Identifier(&pair_info.creator),
                        last_update_time: utils::Bigint2u64(&pair_info.blockTimestampLast),
                        price0_cumulative: utils::Nat2u64(&pair_info.price0CumulativeLast),
                        price1_cumulative: utils::Nat2u64(&pair_info.price1CumulativeLast),
                        k: utils::Nat2u64(&pair_info.kLast),
                    };
                    self.pairs.insert(id, _pair_info);
                },
            }
        }
    
        for lp_token_info in &dswap_info.tokens{
            let id = lp_token_info.id.clone();
            match self.lp_tokens.get_mut(&id){
                Some(lp_token) => {
                    lp_token.supply = utils::Nat2u64(&lp_token_info.totalSupply);
                }
                _ =>{
                    let _lp_token_info = dswap::TokenInfo{
                        id: lp_token_info.id.clone(),
                        name: lp_token_info.name.clone(),
                        symbol: lp_token_info.symbol.clone(),
                        decimals: lp_token_info.decimals,
                        fee: utils::Nat2u64(&lp_token_info.fee), // fee for internal transfer/approve
                        supply: utils::Nat2u64(&lp_token_info.totalSupply),
                    };
                    self.lp_tokens.insert(id, _lp_token_info);
                }
            }
        }
    }
}