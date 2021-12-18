use std::collections::HashMap;
// use serde::{Deserialize,Serialize};
use rocket::serde::{Serialize, Deserialize};



#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct User {
    pub principal : String,
    //todo：清空0资产。
    pub balances : HashMap<String,u64>,
    pub transactions : HashMap<String,Vec<u64>>, // map<token_principal,vec<transaction_id>>
}

impl User {
    fn insert_transaction(&mut self,token_principal:&String, transaction:&Transaction){
        match self.transactions.get_mut(token_principal) {
            Some(txs) => {
                txs.push(transaction.index);
            },
            _ => {
                let mut txs = Vec::new();
                txs.push(transaction.index);
                self.transactions.insert(token_principal.clone(),txs);
            }
        }
    }
}

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct TokenInfo {
    pub canister_id: String,
    pub decimals: u8,
    pub fee: u64,
    pub index: u64,
    pub logo: String,
    pub name: String,
    pub owner: String,
    pub symbol: String,
    pub timestamp: u64,
    pub supply: u64,
}

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct Transaction{
    pub amount: u64,
    pub fee: u64,
    pub from: String, // principal
    pub index: u64,
    pub op: Operation,
    pub timestamp: u64,
    pub to: String,
    pub caller:String,
    pub successful:bool,
}

//todo: return : Result<(),Err>
impl Transaction {
    pub fn process_transaction(&mut self,from:&mut User,to:&mut User,token_principal:&String){
        // println!("[token]:current tx is {:#?}",self);
        // println!("[token]:current from is {:#?}",from);
        // println!("[token]:current to is {:#?}",to);

        match self.op {
            Operation::Approve=>{
                let pre_balance:u64 = match from.balances.get(token_principal){
                    Some(balance) => *balance,
                    None => 0,
                };
                if pre_balance < self.fee + self.amount{
                    self.successful = false;
                    from.insert_transaction(token_principal, self);
                }else{
                    from.balances.insert(token_principal.to_string(), pre_balance - self.fee);
                    from.insert_transaction(token_principal, self);
                }
                
            },
            Operation::Burn(amount)=>{
                let pre_balance:u64 = match from.balances.get(token_principal){
                    Some(balance) => *balance,
                    None => 0,
                };
                if pre_balance < self.fee + self.amount{
                    self.successful = false;
                    from.insert_transaction(token_principal, self);
                }else{
                    from.balances.insert(token_principal.to_string(), pre_balance - self.fee - amount);
                    from.insert_transaction(token_principal, self);
                }
            },
            Operation::Mint => {
                let pre_balance:u64 = match to.balances.get(token_principal){
                    Some(balance) => *balance,
                    None => 0,
                };
                to.balances.insert(token_principal.to_string(), pre_balance + self.amount);
                to.insert_transaction(token_principal, self);
            },
            _ =>{
                let pre_balance:u64 = match from.balances.get(token_principal){
                    Some(balance) => *balance,
                    None => 0,
                };
                if pre_balance < self.amount+ self.fee{
                    self.successful = false;
                    from.insert_transaction(token_principal, self);
                    to.insert_transaction(token_principal, self);
                }else{
                    if from.principal.eq(&to.principal){
                        to.balances.insert(token_principal.to_string(), pre_balance - self.fee);
                        to.insert_transaction(token_principal, self);
                    }else{
                        // println!("[token]-[Transfer]:pre balance is {:#?},fee is {}",&pre_balance,&self.fee);
                        from.balances.insert(token_principal.to_string(), pre_balance - self.amount - self.fee);
                        from.insert_transaction(token_principal, self);
                        let pre_balance:u64 = match to.balances.get(token_principal){
                            Some(balance) => *balance,
                            None => 0,
                        };
                        to.balances.insert(token_principal.to_string(), pre_balance + self.amount);
                        to.insert_transaction(token_principal, self);
                    } 
                }
            }
        }
    }
}

#[derive(Debug,Clone,Deserialize,Serialize)]
pub enum Operation{
    Approve,
    Burn(u64),
    Mint,
    Transfer,
    TransferFrom,
} 
