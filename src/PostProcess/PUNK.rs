use crate::QueryFunctions::DataStructure::_PUNK as PUNK_data;
use crate::PostProcess::types::NFT as NFT_types;
use crate::PostProcess::utils as utils;

const FIVE_MINUTE:u64 = 5 * 60 * 1000000000; 


pub fn HandleTrancastions(NFTInfo:&mut NFT_types::NFTGeneralInfo,transactions: &PUNK_data::transactions){
    
    for transaction in transactions{
        println!("{:#?}", transaction);
        if transaction.from == None{
            continue;
        }
        let mut price = 0;
        let mut to = String::from("None");
        if transaction.price != None{
            price = transaction.price.unwrap();
        }
        if transaction.to == None{
            to = utils::Principal2Identifier(&transaction.from.unwrap()).clone(); 
        }
        else{
            to = utils::Principal2Identifier(&transaction.to.unwrap()).clone(); 
        }
       
        let process_transaction = NFT_types::Transaction{
            tokenIndex: utils::Nat2u64(&transaction.tokenId),
            from: utils::Principal2Identifier(&transaction.from.unwrap()).clone(),
            to: to,
            time: utils::Bigint2u64(&transaction.timestamp),
            price: price,   
        };

        //println!("{:#?}", process_transaction);
        HandleTrancastion(NFTInfo,&process_transaction);
    }

}

pub fn HandleTrancastion(NFTInfo:&mut NFT_types::NFTGeneralInfo, transaction:&NFT_types::Transaction){
    NFTInfo.tx_update(transaction);

    //TODO:update seller

    //TODO:update buyer

    //TODO:update transactions
}

pub fn HandleListings(NFTInfo:&mut NFT_types::NFTGeneralInfo, listings: &PUNK_data::Listings){
    let mut floor_price = 99999999999999 as u64;
    let mut listing_number = 0 as u64;
    for l in listings{
        if l.price < floor_price{
            floor_price = l.price;
        }
        listing_number += 1;
    }
    NFTInfo.listing_update(listing_number, floor_price);

}



