// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::identifier::Identifier;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;
use serde_with::DisplayFromStr;

use sui_types::base_types::SuiAddress;
use sui_types::base_types::{EpochId, ObjectID};
use sui_types::digests::TransactionDigest;
use sui_types::messages_checkpoint::CheckpointSequenceNumber;
use sui_types::sui_serde::{BigInt, SuiTypeTag};
use sui_types::sui_system_state::sui_system_state_summary::SuiValidatorSummary;
use sui_types::TypeTag;

use crate::Page;

pub type EpochPage = Page<EpochInfo, BigInt<u64>>;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EpochInfo {
    /// epoch number
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub epoch: EpochId,
    /// list of validators included in epoch
    pub validators: Vec<SuiValidatorSummary>,
    /// count of tx in epoch
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub epoch_total_transactions: u64,
    /// first, last checkpoint sequence numbers
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub first_checkpoint_id: CheckpointSequenceNumber,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub epoch_start_timestamp: u64,
    pub end_of_epoch_info: Option<EndOfEpochInfo>,
    pub reference_gas_price: Option<u64>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EndOfEpochInfo {
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub last_checkpoint_id: CheckpointSequenceNumber,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub epoch_end_timestamp: u64,
    /// existing fields from `SystemEpochInfo`
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub protocol_version: u64,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub reference_gas_price: u64,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub total_stake: u64,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub storage_fund_reinvestment: u64,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub storage_charge: u64,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub storage_rebate: u64,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub storage_fund_balance: u64,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub stake_subsidy_amount: u64,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub total_gas_fees: u64,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub total_stake_rewards_distributed: u64,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub leftover_storage_fund_inflow: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetworkMetrics {
    /// Current TPS - Transaction Blocks per Second.
    pub current_tps: f64,
    /// Peak TPS in the past 30 days
    pub tps_30_days: f64,
    /// Total number of packages published in the network
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub total_packages: u64,
    /// Total number of addresses seen in the network
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub total_addresses: u64,
    /// Total number of live objects in the network
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub total_objects: u64,
    /// Current epoch number
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub current_epoch: u64,
    /// Current checkpoint number
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub current_checkpoint: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NetworkOverview {
    /// Total Volume in last 24 hours
    #[schemars(with = "String")]
    #[serde_as(as = "DisplayFromStr")]
    pub volume_24h: String,

    /// Total active addresses in last 24 hours
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub total_addresses_24h: u64,

    /// Avgerage gas cost in the last checkpoint.
    #[schemars(with = "String")]
    #[serde_as(as = "BigInt<u64>")]
    pub current_gas: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct StakeMetrics {
    pub apy: f64,

    /// Total staked BFC in last epoch.
    #[schemars(with = "String")]
    #[serde_as(as = "BigInt<u64>")]
    pub total_stake: u64,

    /// Accumulated rewarded BFC since the network started.
    #[schemars(with = "String")]
    #[serde_as(as = "BigInt<u64>")]
    pub accumulated_reward: u64,

    /// Staking coins in the last epoch.
    pub staking_coins: Vec<StakeCoin>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StakeCoin {
    #[schemars(with = "String")]
    #[serde_as(as = "SuiTypeTag")]
    pub coin_type: TypeTag,

    #[schemars(with = "String")]
    #[serde_as(as = "BigInt<u64>")]
    pub balance: u64,

    #[schemars(with = "String")]
    #[serde_as(as = "BigInt<u64>")]
    pub bfc_value: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct IndexedStake {
    pub staked_object_id: ObjectID,
    pub validator: SuiAddress,
    pub pool_id: ObjectID,

    #[schemars(with = "String")]
    #[serde_as(as = "SuiTypeTag")]
    pub coin_type: TypeTag,

    #[schemars(with = "String")]
    #[serde_as(as = "BigInt<u64>")]
    pub stake_activation_epoch: u64,

    #[schemars(with = "String")]
    #[serde_as(as = "BigInt<u64>")]
    pub principal_amount: u64,

    #[schemars(with = "String")]
    #[serde_as(as = "BigInt<u64>")]
    pub principal_bfc_value: u64,

    #[schemars(with = "String")]
    #[serde_as(as = "BigInt<u64>")]
    pub staked_at_timestamp_ms: u64,

    #[schemars(with = "String")]
    #[serde_as(as = "BigInt<u64>")]
    pub estimated_reward: u64,

    #[schemars(with = "String")]
    #[serde_as(as = "Option<BigInt<u64>>")]
    pub unstaking_epoch: Option<u64>,

    #[schemars(with = "String")]
    #[serde_as(as = "Option<BigInt<u64>>")]
    pub unstaking_amount: Option<u64>,

    #[schemars(with = "String")]
    #[serde_as(as = "Option<BigInt<u64>>")]
    pub reward_amount: Option<u64>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MoveCallMetrics {
    #[schemars(with = "Vec<(MoveFunctionName, BigInt<usize>)>")]
    #[serde_as(as = "Vec<(_, BigInt<usize>)>")]
    pub rank_3_days: Vec<(MoveFunctionName, usize)>,
    #[schemars(with = "Vec<(MoveFunctionName, BigInt<usize>)>")]
    #[serde_as(as = "Vec<(_, BigInt<usize>)>")]
    pub rank_7_days: Vec<(MoveFunctionName, usize)>,
    #[schemars(with = "Vec<(MoveFunctionName, BigInt<usize>)>")]
    #[serde_as(as = "Vec<(_, BigInt<usize>)>")]
    pub rank_30_days: Vec<(MoveFunctionName, usize)>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MoveFunctionName {
    pub package: ObjectID,
    #[schemars(with = "String")]
    #[serde_as(as = "DisplayFromStr")]
    pub module: Identifier,
    #[schemars(with = "String")]
    #[serde_as(as = "DisplayFromStr")]
    pub function: Identifier,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddressMetrics {
    pub checkpoint: u64,
    pub epoch: u64,
    pub timestamp_ms: u64,
    pub cumulative_addresses: u64,
    pub cumulative_active_addresses: u64,
    pub daily_active_addresses: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct SuiDaoProposal {
    /// Proposal id
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub pid: u64,

    /// The name of the DAO action
    #[schemars(with = "String")]
    #[serde_as(as = "DisplayFromStr")]
    pub action_name: String,

    /// The status of the DAO action
    pub action_status: bool,

    /// Who propose this
    #[schemars(with = "String")]
    #[serde_as(as = "DisplayFromStr")]
    pub proposer: String,

    /// When it will be started
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub start_time: u64,

    /// When it will be end
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub end_time: u64,

    /// The count of agree votes
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub for_votes: u64,

    /// The count of disagree votes
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub against_votes: u64,

    /// Execute time at
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub eta: u64,

    /// Action delay time
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub action_delay: u64,

    /// The number of votes to pass
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub quorum_votes: u64,

    /// The description of the DAO action
    #[schemars(with = "String")]
    #[serde_as(as = "DisplayFromStr")]
    pub description: String,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NFTStakingOverview {
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub total_power: u64,

    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub reward_per_day: u64,

    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub total_rewarded: u64,

    pub bfc_usd_price: f64,
    pub bfc_24h_rate: f64,

    pub nft_future_rewards: Vec<SuiMiningNFTFutureReward>,
    pub nft_future_profit_rates: Vec<SuiMiningNFTProfitRate>,
    pub overall_profit_rates: Vec<SuiMiningNFTProfitRate>,

    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub total_addresses: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SuiMiningNFTFutureReward {
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub reward: u64,

    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub dt_timestamp_ms: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SuiMiningNFTProfitRate {
    pub rate: f64,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub dt_timestamp_ms: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SuiOwnedMiningNFTFilter {
    Status(SuiMiningNFTStatus),
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SuiMiningNFT {
    pub owner: SuiAddress,
    pub miner_id: ObjectID,
    pub token_id: String,

    pub miner_url: String,
    pub miner_name: String,

    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub power: u64,

    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub mining_started_at: u64,

    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub mint_at: u64,

    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub mint_duration: u64,

    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub total_mined_bfc: u64,

    pub ticket_id: Option<ObjectID>,

    pub status: SuiMiningNFTStatus,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub enum SuiMiningNFTStatus {
    Mining,
    Idle,
    Redeem,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SuiOwnedMiningNFTOverview {
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub total_power: u64,

    pub num_of_staking_nfts: usize,
    pub total_nfts: usize,
    pub bfc_usd_price: f64,
    pub profit_rate: f64,

    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub total_reward: u64,

    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub yesterady_reward: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SuiOwnedMiningNFTProfit {
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub mint_bfc: u64,

    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub mint_usd: u64,

    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub cost_bfc: u64,

    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub dt_timestamp_ms: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, JsonSchema, Clone)]
pub struct SuiMiningNFTLiquidity {
    /// The transaction digest
    pub transaction_digest: TransactionDigest,

    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub timestamp_ms: u64,

    #[schemars(with = "String")]
    #[serde_as(as = "SuiTypeTag")]
    pub base_coin: TypeTag,

    #[schemars(with = "String")]
    #[serde_as(as = "SuiTypeTag")]
    pub quote_coin: TypeTag,

    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub base_amount: u64,

    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub quote_amount: u64,

    pub price_upper: f64,
    pub price_lower: f64,
}
