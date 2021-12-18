use ic_base_types::PrincipalId;
use ledger_canister::AccountIdentifier;
use ic_agent::ic_types::Principal;
//use candid::{Int, Nat};
use candid::types::number::{Int, Nat};
use num_bigint::ToBigUint;
use crate::QueryFunctions::DataStructure::_DFINANCE::_DTOKEN;
use crate::QueryFunctions::DataStructure::_DFINANCE::_DSWAP;
use crate::PostProcess::types::DFINANCE::token;
use crate::PostProcess::types::DFINANCE::dswap;
use crate::SQLProcess::NFT as NFT_sql;
use mysql::PooledConn;

pub fn TokenId2TokenIdentifier(id: u64, canister_id: &Principal)->String{
    let id = id as u32;
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

pub fn Principal2Identifier(principal: &Principal, conn: &mut PooledConn)->String{
    //TODO: update principal2identifier

    let principal_id = PrincipalId::from(*principal);
    let identifier = AccountIdentifier::new(principal_id, None);
    let identifier = identifier.to_hex();
    NFT_sql::insert_principal_identifier_pair(conn, principal, &identifier);
    identifier
}

pub fn Bigint2u64(v: &Int)->u64{
    let value = v.0.iter_u64_digits().collect::<Vec<u64>>();
    if value.len() == 0{
        0 as u64
    }
    else{
        value[0]
    }
}
pub fn Nat2u64(v: &Nat)->u64{
    let value = v.0.iter_u64_digits().collect::<Vec<u64>>();
    if value.len() == 0{
        0 as u64
    }
    else{
        value[0]
    }
}

pub fn u642Nat(v: u64)->Nat{
    let value = Nat(ToBigUint::to_biguint(&v).unwrap());
    value
}

pub fn token_op_to_op(op:_DTOKEN::Operation) -> token::Operation{
    match op {
        _DTOKEN::Operation::approve => token::Operation::Approve,
        _DTOKEN::Operation::burn(amount) => token::Operation::Burn(amount),
        _DTOKEN::Operation::mint => token::Operation::Mint,
        _DTOKEN::Operation::transfer => token::Operation::Transfer,
        _DTOKEN::Operation::transferFrom => token::Operation::TransferFrom,
    }
}

pub fn swap_op_to_op(op:_DSWAP::Operation) -> dswap::Operation{
    match op {
        _DSWAP::Operation::addLiquidity => dswap::Operation::AddLiquidity,
        _DSWAP::Operation::createPair => dswap::Operation::CreatePair,
        _DSWAP::Operation::deposit => dswap::Operation::Deposit,
        _DSWAP::Operation::lpApprove => dswap::Operation::LpApprove,
        _DSWAP::Operation::lpTransfer => dswap::Operation::LpTransfer,
        _DSWAP::Operation::lpTransferFrom => dswap::Operation::LpTransferFrom,
        _DSWAP::Operation::removeLiquidity => dswap::Operation::RemoveLiquidity,
        _DSWAP::Operation::swap => dswap::Operation::Swap,
        _DSWAP::Operation::tokenApprove => dswap::Operation::TokenApprove,
        _DSWAP::Operation::tokenTransfer => dswap::Operation::TokenTransfer,
        _DSWAP::Operation::tokenTransferFrom => dswap::Operation::TokenTransferFrom,
        _DSWAP::Operation::withdraw => dswap::Operation::Withdraw,
    }
}

pub fn get_token_url(index: u64, canister_id: &Principal)->String{
    let identifier = TokenId2TokenIdentifier(index, canister_id);
    let canister_ = canister_id.to_text();
    let mut url = String::from("https://");
    url.push_str(&canister_);
    url.push_str(".raw.ic0.app/?cc=0&type=thumbnail&tokenid=");
    url.push_str(&identifier);
    url    
}