use ic_agent::{Agent, ic_types::Principal, agent::http_transport::ReqwestHttpReplicaV2Transport};
use crate::QueryFunctions::EXT as EXT_query;
use crate::QueryFunctions::DataStructure::_EXT as EXT_data;
use crate::PostProcess::EXT as EXT_process;
use crate::PostProcess::types::NFT as NFT_data;

pub async fn EXT_transactions_thread(info: &mut NFT_data::NFTGeneralInfo, agent: &Agent){
    let Canister_id = &info.canisterId;
    let pre_txs: Vec<EXT_data::transaction> = EXT_query::query_transactions(&Canister_id.to_text(), agent).await;
    //println!("{:#?}", pre_txs);
    let pre_tx_amount = pre_txs.len() as u64;

    if pre_tx_amount == info.total_txs{
        return
    }
    else {
        EXT_process::HandleTrancastions(info, &pre_txs);
    }

}

pub async fn EXT_Listings_thread(info: &mut NFT_data::NFTGeneralInfo, agent: &Agent){
    let Canister_id = &info.canisterId;
    let listings = EXT_query::query_listings(&Canister_id.to_text(), agent).await;
  
    EXT_process::HandleListings(info, &listings);
}




