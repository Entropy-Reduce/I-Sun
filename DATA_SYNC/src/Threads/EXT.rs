use ic_agent::{Agent, ic_types::Principal, agent::http_transport::ReqwestHttpReplicaV2Transport};
use crate::QueryFunctions::EXT as EXT_query;
use crate::QueryFunctions::DataStructure::_EXT as EXT_data;
use crate::PostProcess::EXT as EXT_process;
use crate::PostProcess::types::NFT as NFT_data;
use crate::PostProcess::utils;
use ic_types::time::current_time;
use mysql::PooledConn;
use crate::SQLProcess::NFT::{tx_after_timestamp, get_length};

pub async fn EXT_warm_boot_thread(info: &mut NFT_data::NFTGeneralInfo, conn: &mut PooledConn, dapp_id: u64, agent: &Agent){
    //let t: u64 = current_time().as_nanos_since_unix_epoch();
    let recent_txs = tx_after_timestamp(conn, dapp_id, 0);
    let total_txs = get_length(conn, dapp_id);
    info.total_txs = total_txs;
    for transaction in &recent_txs{
        info.tx_update(transaction);
    }
    EXT_Listings_thread(info, agent).await;

}


pub async fn EXT_transactions_thread(info: &mut NFT_data::NFTGeneralInfo, conn: &mut PooledConn, dapp_id: u64, agent: &Agent){
    let Canister_id = &info.canisterId;
    let pre_txs: Vec<EXT_data::transaction> = EXT_query::query_transactions(&Canister_id.to_text(), agent).await;
    //println!("{:#?}", pre_txs);
    let pre_tx_amount = pre_txs.len() as u64;

    if pre_tx_amount == info.total_txs{
        return
    }
    else {
        EXT_process::HandleTrancastions(info, &pre_txs, dapp_id, conn);
    }

}

pub async fn EXT_Listings_thread(info: &mut NFT_data::NFTGeneralInfo, agent: &Agent){
    let Canister_id = &info.canisterId;
    let listings = EXT_query::query_listings(&Canister_id.to_text(), agent).await;
    // println!("{:#?}", listings);
    // println!("{:#?}", listings.len());
    EXT_process::HandleListings(info, &listings);
}




