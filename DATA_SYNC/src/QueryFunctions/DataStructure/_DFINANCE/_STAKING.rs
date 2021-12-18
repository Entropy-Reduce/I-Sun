use ic_agent::{ic_types::Principal};
use candid::{CandidType, types::number::Int, types::number::Nat};
use serde::Deserialize;

#[derive(Debug, Deserialize, CandidType)]
pub struct Empty{}

#[derive(Debug, Deserialize, CandidType, Clone)]
pub struct PoolInfoExt{
    pub accTokenPerShare: Nat,
    pub endTime: Int,  // pool end time
    pub id: Nat, // pool id
    pub isDSwapLP: bool, // is the staking token DSwap-LP
    pub lastRewardTime: Int,// nano second
    pub rewardRate: Nat, // per nano second
    pub rewardToken: Principal, // reward token
    pub stakingToken: String, // staked token
    pub startTime: Int, // nano seconds // pool start time
    pub totalReward: Nat, // pool end time
    pub totalSupply: Nat,// total tokens staked
 }

 pub type PoolList = Vec<PoolInfoExt>;

#[derive(Debug, Deserialize, CandidType, Clone)]
 pub struct UserInfoExt {
    pub amount: Nat, // staked amount
    pub lastUpdateTime: Int,  // last user info update time
    pub rewardDebt: Nat, 
    pub unclaimedReward: Nat, // unclaimed reward
 }
