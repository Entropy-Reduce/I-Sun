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

pub fn Principal2Identifier(principal: &Principal)->String{
    //TODO: update principal2identifier

    let principal_id = PrincipalId::from(*principal);
    let identifier = AccountIdentifier::new(principal_id, None);
    identifier.to_hex()
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