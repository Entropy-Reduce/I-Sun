use std::collections::HashMap;

#[derive(Debug)]
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

#[derive(Debug,Clone)]
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

#[derive(Debug)]
pub struct Transaction{
    pub amount: u64,
    pub fee: u64,
    pub from: String, // principal
    pub index: u64,
    pub op: Operation,
    pub timestamp: u64,
    pub to: String,
    pub caller:String,
}

//todo: return : Result<(),Err>
impl Transaction {
    pub fn process_transaction(&self,caller:&mut User,from:&mut User,to:&mut User,token_principal:&String){
        match self.op {
            Operation::Approve=>{
                let pre_balance:u64 = match caller.balances.get(token_principal){
                    Some(balance) => *balance,
                    None => 0,
                };
                caller.balances.insert(token_principal.to_string(), pre_balance-self.fee);
                caller.insert_transaction(token_principal, self);
            },
            Operation::Burn(amount)=>{
                let pre_balance:u64 = match caller.balances.get(token_principal){
                    Some(balance) => *balance,
                    None => 0,
                };
                caller.balances.insert(token_principal.to_string(), pre_balance - self.fee - amount);
                caller.insert_transaction(token_principal, self);
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
                if from.principal.eq(&to.principal){
                    let pre_balance:u64 = match to.balances.get(token_principal){
                        Some(balance) => *balance,
                        None => 0,
                    };
                    to.balances.insert(token_principal.to_string(), pre_balance - self.fee);
                    to.insert_transaction(token_principal, self);
                }else{
                    let pre_balance:u64 = match to.balances.get(token_principal){
                        Some(balance) => *balance,
                        None => 0,
                    };
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

#[derive(Debug)]
pub enum Operation{
    Approve,
    Burn(u64),
    Mint,
    Transfer,
    TransferFrom,
} 
