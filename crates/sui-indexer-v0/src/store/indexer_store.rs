// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use async_trait::async_trait;
use prometheus::{Histogram, IntCounter};

use move_core_types::identifier::Identifier;
use sui_json_rpc_types::{
    Checkpoint as RpcCheckpoint, CheckpointId, ClassicPage, DaoProposalFilter, EpochInfo,
    EventFilter, EventPage, IndexedStake, MoveCallMetrics, NetworkMetrics, NetworkOverview,
    StakeMetrics, SuiDaoProposal, SuiMiningNFT, SuiObjectData, SuiObjectDataFilter,
    SuiOwnedMiningNFTFilter, SuiOwnedMiningNFTOverview, SuiOwnedMiningNFTProfit,
    SuiTransactionBlockResponse, SuiTransactionBlockResponseOptions,
};
use sui_types::base_types::{EpochId, ObjectID, SequenceNumber, SuiAddress, VersionNumber};
use sui_types::digests::CheckpointDigest;
use sui_types::event::EventID;
use sui_types::messages_checkpoint::CheckpointSequenceNumber;
use sui_types::object::ObjectRead;
use sui_types::storage::error::Error;
use sui_types::storage::ObjectStore;
use sui_types::TypeTag;

use crate::errors::IndexerError;
use crate::metrics::IndexerMetrics;
use crate::models::address_stake::{AddressStake, ExtractedAddressStake};
use crate::models::addresses::{ActiveAddress, Address, AddressStats};
use crate::models::checkpoint_metrics::CheckpointMetrics;
use crate::models::checkpoints::Checkpoint;
use crate::models::dao_proposals::Proposal;
use crate::models::epoch::DBEpochInfo;
use crate::models::epoch_stake;
use crate::models::events::Event;
use crate::models::mining_nft::{MiningNFT, MiningNFTHistoryProfit, MiningNFTLiquiditiy};
use crate::models::objects::{DeletedObject, Object, ObjectStatus};
use crate::models::packages::Package;
use crate::models::prices::PriceHistory;
use crate::models::system_state::{DBSystemStateSummary, DBValidatorSummary};
use crate::models::transaction_index::{ChangedObject, InputObject, MoveCall, Recipient};
use crate::models::transactions::Transaction;
use crate::types::CheckpointTransactionBlockResponse;
use crate::utils::stable_pool::StablePoolSummary;
use crate::utils::validator_stake::ValidatorStake;

#[async_trait]
pub trait IndexerStore {
    type ModuleCache;

    async fn get_latest_tx_checkpoint_sequence_number(&self) -> Result<i64, IndexerError>;
    async fn get_latest_object_checkpoint_sequence_number(&self) -> Result<i64, IndexerError>;
    async fn get_checkpoint(&self, id: CheckpointId) -> Result<RpcCheckpoint, IndexerError>;
    async fn get_checkpoints(
        &self,
        cursor: Option<CheckpointId>,
        limit: usize,
    ) -> Result<Vec<RpcCheckpoint>, IndexerError>;
    async fn get_indexer_checkpoint(&self) -> Result<Checkpoint, IndexerError>;
    async fn get_indexer_checkpoints(
        &self,
        cursor: i64,
        limit: usize,
    ) -> Result<Vec<Checkpoint>, IndexerError>;
    async fn get_checkpoint_sequence_number(
        &self,
        digest: CheckpointDigest,
    ) -> Result<CheckpointSequenceNumber, IndexerError>;

    async fn get_event(&self, id: EventID) -> Result<Event, IndexerError>;
    async fn get_events(
        &self,
        query: EventFilter,
        cursor: Option<EventID>,
        limit: Option<usize>,
        descending_order: bool,
    ) -> Result<EventPage, IndexerError>;

    async fn get_object(
        &self,
        object_id: ObjectID,
        version: Option<SequenceNumber>,
    ) -> Result<ObjectRead, IndexerError>;

    async fn query_objects_history(
        &self,
        filter: SuiObjectDataFilter,
        at_checkpoint: CheckpointSequenceNumber,
        cursor: Option<ObjectID>,
        limit: usize,
    ) -> Result<Vec<ObjectRead>, IndexerError>;

    async fn query_latest_objects(
        &self,
        filter: SuiObjectDataFilter,
        cursor: Option<ObjectID>,
        limit: usize,
    ) -> Result<Vec<ObjectRead>, IndexerError>;

    async fn get_total_transaction_number_from_checkpoints(&self) -> Result<i64, IndexerError>;

    // TODO: combine all get_transaction* methods
    async fn get_transaction_by_digest(&self, tx_digest: &str)
        -> Result<Transaction, IndexerError>;
    async fn multi_get_transactions_by_digests(
        &self,
        tx_digests: &[String],
    ) -> Result<Vec<Transaction>, IndexerError>;

    async fn compose_sui_transaction_block_response(
        &self,
        tx: Transaction,
        options: Option<&SuiTransactionBlockResponseOptions>,
    ) -> Result<SuiTransactionBlockResponse, IndexerError>;

    async fn get_all_transaction_page(
        &self,
        start_sequence: Option<i64>,
        limit: usize,
        is_descending: bool,
    ) -> Result<Vec<Transaction>, IndexerError>;

    async fn get_transaction_page_by_checkpoint(
        &self,
        checkpoint_sequence_number: i64,
        start_sequence: Option<i64>,
        limit: usize,
        is_descending: bool,
    ) -> Result<Vec<Transaction>, IndexerError>;

    async fn get_transaction_page_by_transaction_kinds(
        &self,
        kind_names: Vec<String>,
        start_sequence: Option<i64>,
        limit: usize,
        is_descending: bool,
    ) -> Result<Vec<Transaction>, IndexerError>;

    async fn get_transaction_page_by_sender_address(
        &self,
        sender_address: String,
        start_sequence: Option<i64>,
        limit: usize,
        is_descending: bool,
    ) -> Result<Vec<Transaction>, IndexerError>;

    async fn get_transaction_page_by_recipient_address(
        &self,
        sender_address: Option<SuiAddress>,
        recipient_address: SuiAddress,
        start_sequence: Option<i64>,
        limit: usize,
        is_descending: bool,
    ) -> Result<Vec<Transaction>, IndexerError>;

    // `address` can be either sender or recipient address of the transaction
    async fn get_transaction_page_by_address(
        &self,
        address: SuiAddress,
        start_sequence: Option<i64>,
        limit: usize,
        is_descending: bool,
    ) -> Result<Vec<Transaction>, IndexerError>;

    async fn get_transaction_page_by_input_object(
        &self,
        object_id: ObjectID,
        version: Option<i64>,
        start_sequence: Option<i64>,
        limit: usize,
        is_descending: bool,
    ) -> Result<Vec<Transaction>, IndexerError>;

    async fn get_transaction_page_by_changed_object(
        &self,
        object_id: ObjectID,
        version: Option<i64>,
        start_sequence: Option<i64>,
        limit: usize,
        is_descending: bool,
    ) -> Result<Vec<Transaction>, IndexerError>;

    async fn get_transaction_page_by_move_call(
        &self,
        package: ObjectID,
        module: Option<Identifier>,
        function: Option<Identifier>,
        start_sequence: Option<i64>,
        limit: usize,
        is_descending: bool,
    ) -> Result<Vec<Transaction>, IndexerError>;

    async fn get_transaction_sequence_by_digest(
        &self,
        tx_digest: Option<String>,
        is_descending: bool,
    ) -> Result<Option<i64>, IndexerError>;

    async fn get_move_call_sequence_by_digest(
        &self,
        tx_digest: Option<String>,
        is_descending: bool,
    ) -> Result<Option<i64>, IndexerError>;

    async fn get_input_object_sequence_by_digest(
        &self,
        tx_digest: Option<String>,
        is_descending: bool,
    ) -> Result<Option<i64>, IndexerError>;

    async fn get_changed_object_sequence_by_digest(
        &self,
        tx_digest: Option<String>,
        is_descending: bool,
    ) -> Result<Option<i64>, IndexerError>;

    async fn get_recipient_sequence_by_digest(
        &self,
        tx_digest: Option<String>,
        is_descending: bool,
    ) -> Result<Option<i64>, IndexerError>;

    async fn get_network_metrics(&self) -> Result<NetworkMetrics, IndexerError>;
    async fn get_network_overview(&self) -> Result<NetworkOverview, IndexerError>;
    async fn get_move_call_metrics(&self) -> Result<MoveCallMetrics, IndexerError>;
    async fn get_dao_proposals(
        &self,
        filter: Option<DaoProposalFilter>,
    ) -> Result<Vec<SuiDaoProposal>, IndexerError>;
    async fn get_stake_metrics(
        &self,
        epoch: Option<SequenceNumber>,
    ) -> Result<StakeMetrics, IndexerError>;

    async fn get_historic_price(
        &self,
        timestamp_ms: i64,
        coin: String,
        exact_match: bool,
    ) -> Result<PriceHistory, IndexerError>;

    async fn get_past_prices(
        &self,
        timestamp_ms: i64,
        coin: String,
    ) -> Result<Vec<PriceHistory>, IndexerError>;

    async fn persist_price(&self, price: PriceHistory) -> Result<(), IndexerError>;

    async fn persist_fast_path(
        &self,
        tx: Transaction,
        tx_object_changes: TransactionObjectChanges,
    ) -> Result<usize, IndexerError>;
    async fn persist_checkpoint_transactions(
        &self,
        checkpoint: &Checkpoint,
        transactions: &[Transaction],
        total_transaction_chunk_committed_counter: IntCounter,
    ) -> Result<usize, IndexerError>;
    async fn persist_object_changes(
        &self,
        tx_object_changes: &[TransactionObjectChanges],
        total_object_change_chunk_committed_counter: IntCounter,
        object_mutation_latency: Histogram,
        object_deletion_latency: Histogram,
    ) -> Result<(), IndexerError>;
    async fn persist_events(&self, events: &[Event]) -> Result<(), IndexerError>;
    async fn persist_addresses(
        &self,
        addresses: &[Address],
        active_addresses: &[ActiveAddress],
    ) -> Result<(), IndexerError>;
    async fn persist_packages(&self, packages: &[Package]) -> Result<(), IndexerError>;
    // NOTE: these tables are for tx query performance optimization
    async fn persist_transaction_index_tables(
        &self,
        input_objects: &[InputObject],
        changed_objects: &[ChangedObject],
        move_calls: &[MoveCall],
        recipients: &[Recipient],
    ) -> Result<(), IndexerError>;

    async fn persist_epoch(&self, data: &TemporaryEpochStore) -> Result<(), IndexerError>;
    async fn persist_proposals(&self, proposals: &[Proposal]) -> Result<(), IndexerError>;
    async fn persist_address_stake(
        &self,
        checkpoint: Checkpoint,
        stake: ExtractedAddressStake,
        deleted_objects: Vec<DeletedObject>,
    ) -> Result<(), IndexerError>;
    async fn get_ongoing_address_stakes(&self) -> Result<Vec<AddressStake>, IndexerError>;
    async fn get_address_stakes(
        &self,
        owner: SuiAddress,
    ) -> Result<Vec<IndexedStake>, IndexerError>;
    async fn update_address_stake_reward(&self, stake: &AddressStake) -> Result<(), IndexerError>;
    async fn get_network_total_transactions_previous_epoch(
        &self,
        epoch: i64,
    ) -> Result<i64, IndexerError>;

    async fn get_epochs(
        &self,
        cursor: Option<EpochId>,
        limit: usize,
        descending_order: Option<bool>,
    ) -> Result<Vec<EpochInfo>, IndexerError>;

    async fn get_current_epoch(&self) -> Result<EpochInfo, IndexerError>;

    fn module_cache(&self) -> &Self::ModuleCache;

    fn indexer_metrics(&self) -> &IndexerMetrics;

    /// methods for address stats
    async fn get_last_address_processed_checkpoint(&self) -> Result<i64, IndexerError>;
    async fn calculate_address_stats(&self, checkpoint: i64) -> Result<AddressStats, IndexerError>;
    async fn persist_address_stats(&self, addr_stats: &AddressStats) -> Result<(), IndexerError>;
    async fn get_latest_address_stats(&self) -> Result<AddressStats, IndexerError>;
    async fn get_checkpoint_address_stats(
        &self,
        checkpoint: i64,
    ) -> Result<AddressStats, IndexerError>;
    async fn get_all_epoch_address_stats(
        &self,
        descending_order: Option<bool>,
    ) -> Result<Vec<AddressStats>, IndexerError>;

    /// methods for checkpoint metrics
    async fn calculate_checkpoint_metrics(
        &self,
        current_checkpoint: i64,
        last_checkpoint_metrics: &CheckpointMetrics,
        checkpoints: &[Checkpoint],
    ) -> Result<CheckpointMetrics, IndexerError>;
    async fn persist_checkpoint_metrics(
        &self,
        checkpoint_metrics: &CheckpointMetrics,
    ) -> Result<(), IndexerError>;
    async fn get_latest_checkpoint_metrics(&self) -> Result<CheckpointMetrics, IndexerError>;

    /// TPS related methods
    async fn calculate_real_time_tps(&self, current_checkpoint: i64) -> Result<f64, IndexerError>;
    async fn calculate_peak_tps_30d(
        &self,
        current_checkpoint: i64,
        current_timestamp_ms: i64,
    ) -> Result<f64, IndexerError>;
    async fn persist_mining_nft(&self, operation: MiningNFTOperation, sequence_number: i64) -> Result<(), IndexerError>;

    async fn refresh_mining_nft(&self) -> Result<(), IndexerError>;

    async fn get_mining_nfts(
        &self,
        address: SuiAddress,
        page: usize,
        limit: usize,
        filter: Option<SuiOwnedMiningNFTFilter>,
    ) -> Result<ClassicPage<SuiMiningNFT>, IndexerError>;

    async fn get_mining_nft_overview(
        &self,
        address: SuiAddress,
    ) -> Result<(SuiOwnedMiningNFTOverview, Vec<String>, f64), IndexerError>;

    async fn get_unsettle_mining_nfts(
        &self,
        dt_timestamp_ms: i64,
    ) -> Result<Vec<MiningNFT>, IndexerError>;

    async fn get_mining_nft_profit(
        &self,
        nft: &MiningNFT,
        dt_timestamp_ms: i64,
    ) -> Result<MiningNFTHistoryProfit, IndexerError>;

    async fn persist_mining_nft_profits(
        &self,
        profits: Vec<MiningNFTHistoryProfit>,
    ) -> Result<usize, IndexerError>;

    async fn calculate_mining_nft_overall(
        &self,
        dt_timestamp_ms: i64,
        total_pending_reward: u64,
    ) -> Result<(), IndexerError>;

    async fn get_owned_mining_nft_profits(
        &self,
        address: SuiAddress,
        limit: Option<usize>,
    ) -> Result<Vec<SuiOwnedMiningNFTProfit>, IndexerError>;

    async fn get_last_epoch_stake(&self) -> Result<Option<epoch_stake::EpochStake>, IndexerError>;
    async fn persist_epoch_stake(&self, data: &TemporaryEpochStore) -> Result<(), IndexerError>;
    async fn get_last_epoch_stake_coin(
        &self,
        coin: TypeTag,
    ) -> Result<Option<epoch_stake::EpochStakeCoin>, IndexerError>;
    async fn persist_mining_nft_liquidities(
        &self,
        mls: Vec<MiningNFTLiquiditiy>,
    ) -> Result<usize, IndexerError>;

    async fn get_mining_nft_liquidities(
        &self,
        base_coin: String,
        limit: usize,
    ) -> Result<Vec<MiningNFTLiquiditiy>, IndexerError>;

    async fn get_mining_nft_total_addressess(&self) -> Result<u64, IndexerError>;
}

#[derive(Clone, Debug)]
pub struct CheckpointData {
    pub checkpoint: RpcCheckpoint,
    pub transactions: Vec<CheckpointTransactionBlockResponse>,
    pub changed_objects: Vec<(ObjectStatus, SuiObjectData)>,
}

impl ObjectStore for CheckpointData {
    fn get_object(
        &self,
        object_id: &ObjectID,
    ) -> Result<Option<sui_types::object::Object>, Error> {
        Ok(self
            .changed_objects
            .iter()
            .find_map(|(status, o)| match status {
                ObjectStatus::Created | ObjectStatus::Mutated if &o.object_id == object_id => {
                    o.clone().try_into().ok()
                }
                _ => None,
            }))
    }

    fn get_object_by_key(
        &self,
        object_id: &ObjectID,
        version: VersionNumber,
    ) -> Result<Option<sui_types::object::Object>, Error> {
        Ok(self
            .changed_objects
            .iter()
            .find_map(|(status, o)| match status {
                ObjectStatus::Created | ObjectStatus::Mutated
                    if &o.object_id == object_id && o.version == version =>
                {
                    o.clone().try_into().ok()
                }
                _ => None,
            }))
    }
}

// Per checkpoint indexing
#[derive(Clone, Debug)]
pub struct TemporaryCheckpointStore {
    pub checkpoint: Checkpoint,
    pub transactions: Vec<Transaction>,
    pub events: Vec<Event>,
    pub object_changes: Vec<TransactionObjectChanges>,
    pub packages: Vec<Package>,
    pub input_objects: Vec<InputObject>,
    pub changed_objects: Vec<ChangedObject>,
    pub move_calls: Vec<MoveCall>,
    pub recipients: Vec<Recipient>,
}

#[derive(Clone, Debug)]
pub struct TransactionObjectChanges {
    pub changed_objects: Vec<Object>,
    pub deleted_objects: Vec<DeletedObject>,
}

// Per epoch indexing
#[derive(Clone, Debug)]
pub struct TemporaryEpochStore {
    pub last_epoch: Option<DBEpochInfo>,
    pub new_epoch: DBEpochInfo,
    pub system_state: DBSystemStateSummary,
    pub validators: Vec<DBValidatorSummary>,
    pub stable_pools: Vec<StablePoolSummary>,
    pub validator_stakes: Vec<ValidatorStake>,
    pub last_epoch_stable_rate: HashMap<TypeTag, u64>,
}

#[derive(Clone, Debug)]
pub enum MiningNFTOperation {
    Creation(MiningNFT),
    Operation(crate::models::mining_nft::MiningNFTOperation),
}
