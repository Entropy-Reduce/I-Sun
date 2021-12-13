use ic_agent::ic_types::Principal;
use crate::QueryFunctions::DataStructure::_EXT as EXT_data;
use crate::PostProcess::types::NFT as NFT_types;
use crate::PostProcess::utils;
use std::convert::TryInto;
use base32;
const FIVE_MINUTE:u64 = 5 * 60 * 1000000000; 

fn byte2nat32(S:Vec<u8>)->u32{
    let mut index: u32 = 0;
    let mut value: u32 = 0;
    let size = S.len();
    
    //println!("{:#?}",S);
    for i in 0..size{
        index = index + 1;
        value = value + ((S[size-i-1] as u32)<<((index-1)*8));
    }
    value
}



pub fn TokenIdentifier2TokenId(identifier: &EXT_data::TokenIdentifier)->u64{
    //println!("{:#?}", identifier);
    let mut s  = identifier.to_string();
    s.make_ascii_lowercase();
    s.retain(|c| c != '-');
    let principal_blob_vec = base32::decode(base32::Alphabet::RFC4648 { padding: false }, &s).unwrap();
    let mut principal_blob: &[u8] = &principal_blob_vec.as_slice()[4..];
    //println!("{:#?}", principal_blob);
    //let principal = Principal::from_text(identifier).unwrap();
    //let mut principal_blob: &[u8] = principal.as_slice();
    //println!("{:#?}", principal_blob);

    //let mut principal_blob: &[u8] = identifier.as_bytes();
    
    
    let mut index: u8 = 0;
    let mut _canister: Vec<u8> = Vec::new();
    let mut _token_index: Vec<u8> = Vec::new();
    let mut _tdscheck: Vec<u8> = Vec::new();
    let mut length: u8 = 0;

    for b in principal_blob{
        length = length + 1;
        if length <= 4 {
            _tdscheck.push(*b);
        }
    }
    //println!("{:#?}",_tdscheck);

    if _tdscheck[0] != 10 || _tdscheck[1] != 116 || _tdscheck[2] != 105 || _tdscheck[3] != 100{
        let result: u64 = 0;
        result
    }else{ for b in principal_blob{
        index = index + 1;
        if index >= 5{
            if index <= (length - 4){
                _canister.push(*b);
            }
            else {
                _token_index.push(*b);
            }
        }
    }
    byte2nat32(_token_index) as u64
    }
}

pub fn TokenId2TokenIdentifier(id: EXT_data::TokenIndex, canister_id: &Principal)->String{
    let canister_blob: &[u8] = canister_id.as_slice();
    let mut data: [u8;18] = [10, 116, 105, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut count = 4;
    for b in canister_blob{
        data[count] = *b;
        count += 1;
    }
    let id_blob = id.to_be_bytes();
    for b in &id_blob{
        data[count] = *b;
        count += 1;
    }


    Principal::from_slice(&data).to_text()



}



pub fn HandleTrancastions(NFTInfo:&mut NFT_types::NFTGeneralInfo,transactions: &EXT_data::transactions){
    
    for transaction_id in NFTInfo.total_txs..(transactions.len() as u64){
        //println!("{:#?}", transaction_id);
        let transaction = &transactions[transaction_id as usize];
        // if transaction_id > 6350{
        //     println!("{:#?}", transaction_id);
        //     println!("{:#?}", transaction);
        // }
        
        let process_transaction = NFT_types::Transaction{
            tokenIndex: TokenIdentifier2TokenId(&transaction.token),
            from: utils::Principal2Identifier(&transaction.seller).clone(),
            to: transaction.buyer.clone(),
            time: utils::Bigint2u64(&transaction.time),
            price: transaction.price,   
        };

        HandleTrancastion(NFTInfo,&process_transaction);
    }

}

pub fn HandleTrancastion(NFTInfo:&mut NFT_types::NFTGeneralInfo, transaction:&NFT_types::Transaction){
    NFTInfo.tx_update(transaction);

    //TODO:update seller

    //TODO:update buyer

    //TODO:update transactions
}

pub fn HandleListings(NFTInfo:&mut NFT_types::NFTGeneralInfo, listings: &EXT_data::Listings){
    let mut floor_price = 99999999999999 as u64;
    let mut listing_number = 0 as u64;
    for l in listings{
        if l.1.price < floor_price{
            floor_price = l.1.price;
        }
        listing_number += 1;
    }
    NFTInfo.listing_update(listing_number, floor_price);

}




#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use ic_agent::ic_types::Principal;
    #[test]
    fn test_fn() {  
    //    let mut map : HashMap<Principal,u8> = HashMap::new();
    //    map.insert(Principal::from_text("4qehi-lqyo6-afz4c-hwqwo-lubfi-4evgk-5vrn5-rldx2-lheha-xs7a4-gae").unwrap()
    //    ,0u8);
    //    println!("{:?}",map);

    //     let a = String::from("42vp6-2iaaa-aaaah-qbooa-cai");
    //     let b = String::from("5l7rb-caaaa-aaaah-qbolq-cai");
    //     println!("{}",a<b);
    let a  = (1,2);
    let b = a.0;
    let c = a.1;
        
    }
}