pub mod NFT;
pub mod DFINANCE;

use async_trait::async_trait;

#[async_trait]
pub trait dapp_func{
    async fn loop_query_update(&mut self);
    async fn cold_boot(&mut self);
    async fn warm_boot(&mut self);
   
}