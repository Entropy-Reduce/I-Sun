use std::collections::HashMap;

//Data stucture in swap
#[derive(Debug,Clone)]
pub struct TokenInfo{
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub fee: u64, // fee for internal transfer/approve
    pub supply: u64,
}

#[derive(Debug,Clone)]
pub struct PairInfo{
    pub id: String, // principal
    pub supply: u64,
    pub token0: String, //Principal;
    pub token1: String, //Principal;
    pub lp_token: String,
    pub creator: String,
    pub last_update_time: u64,
    pub price0_cumulative: u64,
    pub price1_cumulative: u64,
    pub k: u64,
}

//Data stucture in staking 
// pub struct PoolInfo {
//     pub id: u64, // pool id
//     pub is_dswap_lp: bool, // is the staking token DSwap-LP
//     pub acc_token_per_share: u64,
//     pub start_time: u64, // nano seconds // pool start time
//     pub end_time: u64,  // pool end time
//     pub last_reward_time: u64,// nano second
//     pub reward_rate: u64, // per nano second
//     pub reward_token: String, // reward token
//     pub staking_token: String, // staked token
//     pub total_reward: u64, // 
//     pub total_supply: u64,// total tokens staked
// }


// pub struct UserInfo{
//     stake_info:StakingUserInfo,
//     token_inof:TokenUserInfo,
//     transactions:Vec<u64>,
// }

// pub struct StakingUserInfo {
//     pub amount: u64, // staked amount
//     pub last_update_time: u64,  // last user info update time
//     pub reward_debt: u64, 
//     pub unclaimed_reward: u64, // unclaimed reward
// }
#[derive(Debug)]
pub struct User{
    pub principal: String,
    pub balances: HashMap<String,u64>,
    pub lp_balances:HashMap<String,u64>,
    pub transactions:HashMap<String,Vec<u64>>,
}

impl User {
    fn insert_transaction(&mut self,token_id:String, transaction:&Transaction){
        match self.transactions.get_mut(&token_id) {
            Some(txs) => {
                txs.push(transaction.index);
            },
            _ => {
                let mut txs = Vec::new();
                txs.push(transaction.index);
                self.transactions.insert(transaction.token_id.clone(), txs);
            }
        }
    }
}

#[derive(Debug)]
pub struct Transaction{
    pub amount:u64,
    pub amount0: u64,
    pub amount1: u64,
    pub caller: String,
    pub fee: u64,
    pub from: String,
    pub index: u64,
    pub op: Operation,
    pub timestamp: u64,
    pub to: String,
    pub token_id: String,
}

#[derive(Debug)]
pub enum Operation {
    AddLiquidity,
    CreatePair,
    Deposit,
    LpApprove,
    LpTransfer,
    LpTransferFrom,
    RemoveLiquidity,
    Swap,
    TokenApprove,
    TokenTransfer,
    TokenTransferFrom,
    Withdraw,
}

impl Transaction {
    pub fn process_transaction(&self,caller:&mut User,from:&mut User,to:&mut User){
        match self.op {
            Operation::AddLiquidity=>{
                let tokens = split_pairs(&self.token_id);
                caller.insert_transaction(tokens.0,self);
                caller.insert_transaction(tokens.1,self);
                caller.insert_transaction(self.token_id.clone(), self);

                // let new_amount0 = caller.balances.get(tokens.0) - self.amount0;
                // caller.balances.insert(tokens.0,new_amount0);
                // let new_amount1 = caller.balances.get(tokens.1) - self.amount1;
                // caller.balances.insert(tokens.1,new_amount1);
                // let new_amount = caller.balances.get(self.token_id) - self.amount;
                // caller.balances.insert(self.token0,new_amount);
            },
            Operation::RemoveLiquidity=>{
                let tokens = split_pairs(&self.token_id);
                caller.insert_transaction(tokens.0,self);
                caller.insert_transaction(tokens.1,self);
                caller.insert_transaction(self.token_id.clone(), self);
            },
            Operation::Deposit => {
                // let new_amount = caller.balances.get(self.token_id).unswap() + self.amount;
                // caller.balances.insert(self.token0,new_amount);
                caller.insert_transaction(self.token_id.clone(), self);
                if caller.principal.ne(&to.principal) {
                    to.insert_transaction(self.token_id.clone(), self);
                }
            },
            Operation::Withdraw=>{
                caller.insert_transaction(self.token_id.clone(), self);
                if caller.principal.ne(&to.principal) {
                    to.insert_transaction(self.token_id.clone(), self);
                }
            },
            Operation::CreatePair=>{
                let tokens = split_pairs(&self.token_id);
                caller.insert_transaction(tokens.0,self);
                caller.insert_transaction(tokens.1,self);
            },
            Operation::Swap=>{
                let tokens = split_pairs(&self.token_id);
                caller.insert_transaction(tokens.0,self);
                caller.insert_transaction(tokens.1,self);
            },
            _ => {
                from.insert_transaction(self.token_id.clone(), self);
                to.insert_transaction(self.token_id.clone(), self);
            }
        }
    }
}

fn split_pairs(pair_id:&String) -> (String,String) {
    let v: Vec<&str> = pair_id.split(":").collect();
    let token0 = String::from(v[0]);
    let token1 = String::from(v[1]);
    return (token0,token1);
}