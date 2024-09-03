use chrono::Utc;
use diesel::{Insertable, Queryable};
use crate::models::address_stake::AddressStake;
use crate::schema::stake_reward_detail;
use crate::schema::stake_reward_summary;

#[derive(Queryable, Insertable, Debug, Clone, Default)]
#[diesel(table_name = stake_reward_detail)]
pub struct StakeRewardDetail {
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    pub staked_object_id: String,
    pub staker_address: String,
    pub pool_id: String,
    pub validator_address: String,
    pub stake_coin: String,
    pub principal_epoch: i64,
    pub principal_amount: i64,
    pub principal_amount_bfc: i64,
    pub principal_timestamp_ms: i64,
    pub estimated_reward: i64,
    pub estimated_at_epoch: i64,
    pub stake_activation_epoch: i64,
    pub timestamp_ms: i64,
}

impl StakeRewardDetail {
    pub fn new(
        staked_object_id: String,
        staker_address: String,
        pool_id: String,
        validator_address: String,
        stake_coin: String,
        principal_epoch: i64,
        principal_amount: i64,
        principal_amount_bfc: i64,
        principal_timestamp_ms: i64,
        estimated_reward: i64,
        estimated_at_epoch: i64,
        stake_activation_epoch: i64,
        timestamp_ms: i64,
    ) -> Self {
        Self {
            id: None,
            staked_object_id,
            staker_address,
            pool_id,
            validator_address,
            stake_coin,
            principal_epoch,
            principal_amount,
            principal_amount_bfc,
            principal_timestamp_ms,
            estimated_reward,
            estimated_at_epoch,
            stake_activation_epoch,
            timestamp_ms,
        }
    }

    pub fn build(stake: AddressStake, reward_last_epoch: i64, stake_bfc: i64) -> Self {
        Self {
            id: None,
            staked_object_id: stake.staked_object_id.clone(),
            staker_address: stake.staker_address,
            pool_id: stake.pool_id,
            validator_address: stake.validator_address,
            stake_coin: stake.stake_coin,
            principal_epoch: stake.principal_epoch,
            principal_amount: stake.principal_amount,
            principal_amount_bfc: stake_bfc,
            principal_timestamp_ms: stake.principal_timestamp_ms,
            estimated_reward: reward_last_epoch,
            estimated_at_epoch: stake.estimated_at_epoch - 1,
            stake_activation_epoch: stake.stake_activation_epoch,
            timestamp_ms: Utc::now().timestamp(),
        }
    }
}

#[derive(Queryable, Insertable, Debug, Clone, Default)]
#[diesel(table_name = stake_reward_summary)]
pub struct StakeRewardSummary {
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    pub staker_address: String,
    pub stake_amount: i64,
    pub stake_reward: i64,
    pub estimated_at_epoch: i64,
    pub principal_timestamp_ms: i64,
    pub timestamp_ms: i64,
}

impl StakeRewardSummary {
    pub fn new(
        staker_address: String,
        stake_amount: i64,
        stake_reward: i64,
        estimated_at_epoch: i64,
        principal_timestamp_ms: i64,
        timestamp_ms: i64,
    ) -> Self {
        Self {
            id: None,
            staker_address,
            stake_amount,
            stake_reward,
            estimated_at_epoch,
            principal_timestamp_ms,
            timestamp_ms,
        }
    }

    pub fn build(staker_address: String,
                 stake_amount: i64,
                 stake_reward: i64,
                 estimated_at_epoch: i64,
                 epoch_ms: Option<u64>,
    ) -> Self {
        let now = if epoch_ms.is_none() {Utc::now().timestamp()} else {epoch_ms.unwrap() as i64};
        Self {
            id: None,
            staker_address,
            stake_amount,
            stake_reward,
            estimated_at_epoch,
            principal_timestamp_ms: now,
            timestamp_ms: now,
        }
    }
}