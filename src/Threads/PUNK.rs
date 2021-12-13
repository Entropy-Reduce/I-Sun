use ic_agent::{Agent, ic_types::Principal, agent::http_transport::ReqwestHttpReplicaV2Transport};
use crate::QueryFunctions::PUNK as PUNK_query;
use crate::QueryFunctions::DataStructure::_PUNK as PUNK_data;
use crate::PostProcess::PUNK as PUNK_process;
use crate::PostProcess::types::NFT as NFT_data;
use crate::utils as utils;
pub async fn PUNK_transactions_thread(info: &mut NFT_data::NFTGeneralInfo, agent: &Agent){
    let Canister_id = &info.canisterId;
    let pre_tx_amout: u64 = utils::Nat2u64(&PUNK_query::query_txAmount(&Canister_id.to_text(), agent).await);
    //let pre_tx_amout = 100 as u64;
    if pre_tx_amout == info.total_txs{
        return
    }
    else {
        let mut txs: Vec<PUNK_data::transaction> = Vec::new();
        for i in (pre_tx_amout - 10)..(pre_tx_amout - info.total_txs){
            println!("{:#?}", i);
            let transaction = PUNK_query::query_HistoryByIndex(&Canister_id.to_text(), agent, &utils::u642Nat(info.total_txs + i)).await;
            txs.push(transaction);
        }
        PUNK_process::HandleTrancastions(info, &txs);
    }

}

pub async fn PUNK_Listings_thread(info: &mut NFT_data::NFTGeneralInfo, agent: &Agent){
    let Canister_id = &info.canisterId;
    let len = utils::Nat2u64(&PUNK_query::query_listing_len(&Canister_id.to_text(), agent).await);
    //println!("{:#?}", len);
    let page_len = 10 as u64;
    let mut listings: Vec<PUNK_data::Listing> = Vec::new();
    for i in 0..=((len / page_len) as u64){
        //println!("{:#?}", i);
        let mut pre_listing = PUNK_query::query_page_listings(&Canister_id.to_text()
                        , agent, &utils::u642Nat(i)).await;
        
        listings.append(&mut pre_listing);
    }
    //println!("{:#?}", listings.len());
    PUNK_process::HandleListings(info, &listings);
}


