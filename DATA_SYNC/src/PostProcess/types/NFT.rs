use ic_types::time;
use std::collections::{HashSet,VecDeque};
use crate::QueryFunctions::DataStructure::_EXT as EXT_data;
use ic_agent::{Agent, ic_types::Principal,agent::http_transport::ReqwestHttpReplicaV2Transport};
use rocket::serde::{Serialize, Deserialize};


#[derive(Debug,Clone)]
pub struct NFTGeneralInfo{
    //basic info
    pub canisterId:Principal,
    pub name:String,
    pub symbol:String,
    pub supply:u64,
    pub decimals:u8,
    //price relavant
    
    
    pub average_price:u64,
    pub lowest_price:u64,
    pub highest_price:u64,
    pub volume :u64,
    pub total_txs:u64,
    //fluctuate; 
    pub tx_amount_in_past_24h:u64,
    pub volume_in_past_24h:u64,
    pub volumet_change_in_past_24:u64,
    //holder relavant
    // pub holder_num:u64,
    // pub top_10_holder:Vec<Principal>,

    //TODO
    //pub listings: Listings,
    pub floor_price: u64,
    pub listing_number:u64,
   
    //不需要展示的数据
    pub prices : Vec<u64>,
    pub volumes_in_past_24:VecDeque<u64>,
    pub tx_amounts_in_past_24: VecDeque<u64>,
    
    pub cur_period_volume:u64,
    pub cur_period_tx_amount:u64,
    pub last_tx_time:u64,
    // pub cur_moment_tx_amount:u64;

    pub gap:u64,
}

impl NFTGeneralInfo{
    pub fn tx_update(&mut self, tx : &Transaction){
        
        self.total_txs += 1;

        self.volume = self.volume - self.prices[tx.tokenIndex as usize] + tx.price;
        self.average_price = self.volume / self.supply;
        //println!("a_p {:#?}", self.average_price);
        if tx.price < self.lowest_price{
            self.lowest_price = tx.price;
        }
        if tx.price > self.highest_price{
            self.highest_price = tx.price;
        }
        self.prices[tx.tokenIndex as usize] = tx.price;
        

        if tx.time < self.last_tx_time{
        }

        else if tx.time > self.last_tx_time + self.gap{
            self.tx_amounts_in_past_24.push_back(self.cur_period_tx_amount);
            self.tx_amount_in_past_24h = self.tx_amount_in_past_24h + self.cur_period_tx_amount;

            self.volumes_in_past_24.push_back(self.cur_period_volume);
            self.volume_in_past_24h = self.volume_in_past_24h + self.cur_period_volume;

            self.last_tx_time = self.last_tx_time + self.gap;

            let count: u64 = ( tx.time - self.last_tx_time ) / self.gap;

            for _i in 0..count{
                self.tx_amounts_in_past_24.push_back(0 as u64);
                self.volumes_in_past_24.push_back(0 as u64);

                self.last_tx_time = self.last_tx_time + self.gap;
                
                //self.tx_amounts_in_past_24h.push_back(0 as u64);
            }

            while self.tx_amounts_in_past_24.len() as u64 > (24*60*60*1000000000 as u64 / self.gap) {
                self.tx_amount_in_past_24h = self.tx_amount_in_past_24h - self.tx_amounts_in_past_24.pop_front().unwrap();
            }

            while self.volumes_in_past_24.len() as u64 > (24*60*60*1000000000 as u64 / self.gap) {
                self.volume_in_past_24h = self.volume_in_past_24h - self.volumes_in_past_24.pop_front().unwrap();
            }


            self.cur_period_tx_amount = 1;
            self.cur_period_volume = tx.price;
        }
        else{
            self.cur_period_tx_amount += 1;
            self.cur_period_volume += tx.price;
        }
    }

    pub fn query_update(&mut self, t :u64){
        if t > self.last_tx_time + self.gap{
            //println!("yes yes yes yes yes");
            self.tx_amounts_in_past_24.push_back(self.cur_period_tx_amount);
            self.tx_amount_in_past_24h = self.tx_amount_in_past_24h + self.cur_period_tx_amount;

            self.volumes_in_past_24.push_back(self.cur_period_volume);
            self.volume_in_past_24h = self.volume_in_past_24h + self.cur_period_volume;

            self.last_tx_time = self.last_tx_time + self.gap;

            let count: u64 = ( t - self.last_tx_time ) / self.gap;
            println!("{:#?}", t);
            println!("{:#?}", self.last_tx_time);
            println!("{:#?}", self.gap);
            println!("{:#?}", count);
            for _i in 0..count{
                self.tx_amounts_in_past_24.push_back(0 as u64);
                self.volumes_in_past_24.push_back(0 as u64);

                self.last_tx_time = self.last_tx_time + self.gap;
                
                //self.tx_amounts_in_past_24h.push_back(0 as u64);
            }

            while self.tx_amounts_in_past_24.len() as u64 > (24*60*60*1000000000 as u64 / self.gap) {
                self.tx_amount_in_past_24h = self.tx_amount_in_past_24h - self.tx_amounts_in_past_24.pop_front().unwrap();
            }
            self.cur_period_tx_amount = 0;
            self.cur_period_volume = 0;

        }


    }


    pub fn listing_update(&mut self, listing_number: u64, floor_price: u64){
        self.listing_number = listing_number;
        self.floor_price = floor_price;
    }

}



pub fn NewNFTGeneralInfo(canisterId:Principal,name:String,symbol:String,supply:u64,decimals:u8) -> NFTGeneralInfo{
    NFTGeneralInfo{
        canisterId:canisterId,
        name:name,
        symbol:symbol,
        supply:supply,
        decimals:decimals,
        floor_price : 0,
        average_price : 0,
        lowest_price : 999999999999999,
        highest_price: 0,
        volume : 0,
        prices : vec![0;supply as usize],
        tx_amount_in_past_24h : 0,
        volume_in_past_24h : 0,
        volumet_change_in_past_24 : 0,
        // holder_num:0,
        // top_10_holder:Vec::new(),
        //listings:Vec::new(),
        listing_number:0,
        

        total_txs:0,
        gap: 5 * 60 * 100000000,

        //todo：用个queue 我瞎写的
        volumes_in_past_24:VecDeque::new(),
        tx_amounts_in_past_24:VecDeque::new(),
        last_tx_time: time::current_time().as_nanos_since_unix_epoch() - 24*60*60*1000000000 as u64,
        cur_period_tx_amount: 0,
        cur_period_volume:0,

    }

}


#[derive(Debug,Clone)]
pub struct NFT{
    pub id:u64,
    pub price:u64,
    pub holder:Principal,
    pub listed : bool,
    pub listing_price:Option<u64>,
    pub historyPrice:Vec<u64>,
}
#[derive(Debug,Clone)]
pub struct Holder{
    pub principal: Principal,
    pub nfts: HashSet<EXT_data::TokenIndex>,
}
#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Transaction{
    pub tokenIndex:u64,
    pub from:AccountIdentifier,
    pub to:AccountIdentifier,
    pub time:u64,
    pub price:u64,
}


pub type Transactions = Vec<Transaction>;
pub type AccountIdentifier = String;
pub type Listings = Vec<(u64, u64)>;

fn init_empty_vector(size:u64)-> Vec<u64>{
    let mut empty_vec : Vec<u64>  = Vec::new();
    for i in 0..size{
        empty_vec.push(0);
    }
    empty_vec
}
