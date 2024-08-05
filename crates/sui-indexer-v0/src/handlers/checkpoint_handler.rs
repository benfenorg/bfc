// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::cmp::max;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;
use std::{collections::BTreeMap, time::Duration};

use anyhow::anyhow;
use chrono::Utc;
use fastcrypto::traits::ToFromBytes;
use futures::future::join_all;
use futures::FutureExt;
use itertools::Itertools;
use jsonrpsee::http_client::HttpClient;
use move_core_types::parser::parse_type_tag;
use move_core_types::{account_address::AccountAddress, ident_str};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};
use tokio::task::JoinHandle;
use tracing::{error, info, warn};

use mysten_metrics::spawn_monitored_task;
use sui_core::subscription_handler::SubscriptionHandler;
use sui_json_rpc_api::ReadApiClient;
use sui_json_rpc_types::{
    OwnedObjectRef, SuiGetPastObjectRequest, SuiObjectData, SuiObjectDataOptions, SuiRawData,
    SuiTransactionBlockDataAPI, SuiTransactionBlockEffects, SuiTransactionBlockEffectsAPI,
};
use sui_sdk::error::Error;
use sui_types::base_types::{ObjectID, SequenceNumber, SuiAddress};
use sui_types::committee::EpochId;
use sui_types::messages_checkpoint::{CheckpointCommitment, CheckpointSequenceNumber};
use sui_types::sui_system_state::sui_system_state_summary::SuiSystemStateSummary;
use sui_types::sui_system_state::{
    get_sui_system_state, PoolTokenExchangeRate, SuiSystemStateTrait,
};
use sui_types::{TypeTag, SUI_SYSTEM_ADDRESS};

use crate::models::address_stake;
use crate::models::address_stake::AddressStake;
use crate::models::checkpoints::Checkpoint;
use crate::models::dao_proposals::Proposal;
use crate::models::epoch::{DBEpochInfo, SystemEpochInfoEvent};
use crate::models::events::Event;
use crate::models::mining_nft::{self, MiningNFT, MiningNFTHistoryProfit};
use crate::models::prices::PriceHistory;
use crate::models::transactions::Transaction;
use crate::store::{
    CheckpointData, IndexerStore, TemporaryCheckpointStore, TemporaryEpochStore,
    TransactionObjectChanges,
};
use crate::types::{CheckpointTransactionBlockResponse, TemporaryTransactionBlockResponseStore};
use crate::utils::multi_get_full_transactions;
use crate::{benfen, IndexerConfig};
use crate::{errors::IndexerError, models::transaction_index::ChangedObject};
use crate::{metrics::IndexerMetrics, utils::stable_pool};
use crate::{
    models::objects::{DeletedObject, Object, ObjectStatus},
    utils::validator_stake::ValidatorSet,
};
use crate::{models::packages::Package, utils::validator_stake::extract_stable_stakes};

const MAX_PARALLEL_DOWNLOADS: usize = 24;
const DOWNLOAD_RETRY_INTERVAL_IN_SECS: u64 = 10;
const DB_COMMIT_RETRY_INTERVAL_IN_MILLIS: u64 = 100;
const MULTI_GET_CHUNK_SIZE: usize = 50;
const CHECKPOINT_QUEUE_LIMIT: usize = 24;
const EPOCH_QUEUE_LIMIT: usize = 2;
const STAKING_QUEUE_LIMIT: usize = 64;

#[derive(Clone)]
pub struct CheckpointHandler<S> {
    state: S,
    http_client: HttpClient,
    // MUSTFIX(gegaowp): remove subscription_handler from checkpoint_handler;
    // b/c subscription_handler should be on indexer reader, while checkpoint_handler is on indexer writer.
    // subscription_handler: Arc<SubscriptionHandler>,
    metrics: IndexerMetrics,
    config: IndexerConfig,
    tx_checkpoint_sender: Arc<Mutex<Sender<TemporaryCheckpointStore>>>,
    tx_checkpoint_receiver: Arc<Mutex<Receiver<TemporaryCheckpointStore>>>,
    object_checkpoint_sender: Arc<Mutex<Sender<TemporaryCheckpointStore>>>,
    object_checkpoint_receiver: Arc<Mutex<Receiver<TemporaryCheckpointStore>>>,
    epoch_sender: Arc<Mutex<Sender<TemporaryEpochStore>>>,
    epoch_receiver: Arc<Mutex<Receiver<TemporaryEpochStore>>>,

    staking_sender: Arc<Mutex<Sender<(u64, Option<TemporaryEpochStore>)>>>,
    staking_receiver: Arc<Mutex<Receiver<(u64, Option<TemporaryEpochStore>)>>>,
}

impl<S> CheckpointHandler<S>
where
    S: IndexerStore + Clone + Sync + Send + 'static,
{
    pub fn new(
        state: S,
        http_client: HttpClient,
        _subscription_handler: Arc<SubscriptionHandler>,
        metrics: IndexerMetrics,
        config: &IndexerConfig,
    ) -> Self {
        let (tx_checkpoint_sender, tx_checkpoint_receiver) = mpsc::channel(CHECKPOINT_QUEUE_LIMIT);
        let (object_checkpoint_sender, object_checkpoint_receiver) =
            mpsc::channel(CHECKPOINT_QUEUE_LIMIT);
        let (epoch_sender, epoch_receiver) = mpsc::channel(EPOCH_QUEUE_LIMIT);
        let (staking_sender, staking_receiver) = mpsc::channel(STAKING_QUEUE_LIMIT);

        Self {
            state,
            http_client,
            // subscription_handler,
            metrics,
            config: config.clone(),
            tx_checkpoint_sender: Arc::new(Mutex::new(tx_checkpoint_sender)),
            tx_checkpoint_receiver: Arc::new(Mutex::new(tx_checkpoint_receiver)),
            object_checkpoint_sender: Arc::new(Mutex::new(object_checkpoint_sender)),
            object_checkpoint_receiver: Arc::new(Mutex::new(object_checkpoint_receiver)),
            epoch_sender: Arc::new(Mutex::new(epoch_sender)),
            epoch_receiver: Arc::new(Mutex::new(epoch_receiver)),

            staking_sender: Arc::new(Mutex::new(staking_sender)),
            staking_receiver: Arc::new(Mutex::new(staking_receiver)),
        }
    }

    pub fn spawn(self) -> JoinHandle<()> {
        info!("Indexer checkpoint handler started...");
        self.spawn_periodical_tasks();
        let tx_download_handler = self.clone();
        spawn_monitored_task!(async move {
            let mut checkpoint_download_index_res = tx_download_handler
                .start_download_and_index_tx_checkpoint()
                .await;
            while let Err(e) = &checkpoint_download_index_res {
                warn!(
                    "Indexer checkpoint download & index failed with error: {:?}, retrying after {:?} secs...",
                    e, DOWNLOAD_RETRY_INTERVAL_IN_SECS
                );
                tokio::time::sleep(Duration::from_secs(DOWNLOAD_RETRY_INTERVAL_IN_SECS,)).await;
                checkpoint_download_index_res = tx_download_handler.start_download_and_index_tx_checkpoint().await;
            }
        });

        let object_download_handler = self.clone();
        spawn_monitored_task!(async move {
            let mut object_download_index_res = object_download_handler
                .start_download_and_index_object_checkpoint()
                .await;
            while let Err(e) = &object_download_index_res {
                warn!(
                    "Indexer object download & index failed with error: {:?}, retrying after {:?} secs...",
                    e, DOWNLOAD_RETRY_INTERVAL_IN_SECS
                );
                tokio::time::sleep(Duration::from_secs(DOWNLOAD_RETRY_INTERVAL_IN_SECS,)).await;
                object_download_index_res = object_download_handler.start_download_and_index_object_checkpoint().await;
            }
        });

        let tx_checkpoint_commit_handler = self.clone();
        spawn_monitored_task!(async move {
            let mut checkpoint_commit_res = tx_checkpoint_commit_handler
                .start_tx_checkpoint_commit()
                .await;
            while let Err(e) = &checkpoint_commit_res {
                warn!(
                    "Indexer checkpoint commit failed with error: {:?}, retrying after {:?} secs...",
                    e, DOWNLOAD_RETRY_INTERVAL_IN_SECS
                );
                tokio::time::sleep(Duration::from_secs(DOWNLOAD_RETRY_INTERVAL_IN_SECS,)).await;
                checkpoint_commit_res = tx_checkpoint_commit_handler.start_tx_checkpoint_commit().await;
            }
        });

        let object_checkpoint_commit_handler = self.clone();
        spawn_monitored_task!(async move {
            let mut object_checkpoint_commit_res = object_checkpoint_commit_handler
                .start_object_checkpoint_commit()
                .await;
            while let Err(e) = &object_checkpoint_commit_res {
                warn!(
                    "Indexer object checkpoint commit failed with error: {:?}, retrying after {:?} secs...",
                    e, DOWNLOAD_RETRY_INTERVAL_IN_SECS
                );
                tokio::time::sleep(std::time::Duration::from_secs(
                    DOWNLOAD_RETRY_INTERVAL_IN_SECS,
                ))
                .await;
                object_checkpoint_commit_res = object_checkpoint_commit_handler
                    .start_object_checkpoint_commit()
                    .await;
            }
        });

        let handler = self.clone();
        spawn_monitored_task!(async move {
            let mut epoch_staking_commit_res = handler.start_epoch_staking_commit().await;
            while let Err(e) = &epoch_staking_commit_res {
                warn!(
                    "Indexer epoch_staking commit failed with error: {:?}, retrying after {:?} secs...",
                    e, DOWNLOAD_RETRY_INTERVAL_IN_SECS
                );
                tokio::time::sleep(std::time::Duration::from_secs(
                    DOWNLOAD_RETRY_INTERVAL_IN_SECS,
                ))
                .await;
                epoch_staking_commit_res = handler.start_epoch_staking_commit().await;
            }
        });

        spawn_monitored_task!(async move {
            let mut epoch_commit_res = self.start_epoch_commit().await;
            while let Err(e) = &epoch_commit_res {
                warn!(
                    "Indexer epoch commit failed with error: {:?}, retrying after {:?} secs...",
                    e, DOWNLOAD_RETRY_INTERVAL_IN_SECS
                );
                tokio::time::sleep(std::time::Duration::from_secs(
                    DOWNLOAD_RETRY_INTERVAL_IN_SECS,
                ))
                .await;
                epoch_commit_res = self.start_epoch_commit().await;
            }
        })
    }

    async fn start_download_and_index_tx_checkpoint(&self) -> Result<(), IndexerError> {
        info!("Indexer checkpoint download & index task started...");
        // NOTE: important not to cast i64 to u64 here,
        // because -1 will be returned when checkpoints table is empty.
        let last_seq_from_db = self
            .state
            .get_latest_tx_checkpoint_sequence_number()
            .await?;
        if last_seq_from_db > 0 {
            info!("Resuming tx handler from checkpoint {last_seq_from_db}");
        }
        let mut next_cursor_sequence_number = last_seq_from_db + 1;
        self.check_epoch_staking(next_cursor_sequence_number).await?;

        // NOTE: we will download checkpoints in parallel, but we will commit them sequentially.
        // We will start with MAX_PARALLEL_DOWNLOADS, and adjust if no more checkpoints are available.
        let mut current_parallel_downloads = MAX_PARALLEL_DOWNLOADS;
        loop {
            // Step 1: download tx checkpoint data for checkpoints in the current batch
            let download_futures = (next_cursor_sequence_number
                ..next_cursor_sequence_number + current_parallel_downloads as i64)
                .map(|seq_num| self.download_checkpoint_data(seq_num as u64));
            info!("download checkpoints from {:?} to {:?}", next_cursor_sequence_number, next_cursor_sequence_number + current_parallel_downloads as i64);
            let download_results = join_all(download_futures).await;
            let mut downloaded_checkpoints = vec![];
            // NOTE: Push sequentially and if one of the downloads failed,
            // we will discard all following checkpoints and retry, to avoid messing up the DB commit order.
            for download_result in download_results {
                if let Ok(checkpoint) = download_result {
                    downloaded_checkpoints.push(checkpoint);
                } else {
                    if let Err(IndexerError::UnexpectedFullnodeResponseError(fn_e)) =
                        download_result
                    {
                        warn!("Unexpected response from fullnode for checkpoints: {}",fn_e);
                    } else if let Err(IndexerError::FullNodeReadingError(fn_e)) = download_result {
                        warn!("Fullnode reading error for checkpoints {}: {}. It can be transient or due to rate limiting.", next_cursor_sequence_number, fn_e);
                    } else {
                        warn!("Error downloading checkpoints: {:?}", download_result);
                    }
                    break;
                }
            }
            next_cursor_sequence_number += downloaded_checkpoints.len() as i64;
            // NOTE: with this line, we can make sure that:
            // - when indexer is way behind and catching up, we download MAX_PARALLEL_DOWNLOADS checkpoints in parallel;
            // - when indexer is up to date, we download at least one checkpoint at a time.
            current_parallel_downloads = std::cmp::min(downloaded_checkpoints.len() + 1, MAX_PARALLEL_DOWNLOADS);
            if downloaded_checkpoints.is_empty() {
                warn!("No checkpoints were downloaded for sequence number {}, retrying...",next_cursor_sequence_number);
                continue;
            }

            // Step 2: Transform tx checkpoint data to indexed_checkpoints
            let index_timer = self.metrics.checkpoint_index_latency.start_timer();
            let indexed_checkpoint_epoch_vec = join_all(downloaded_checkpoints.iter().map(
                |downloaded_checkpoint| async {
                    self.index_checkpoint_and_epoch(downloaded_checkpoint).await
                },
            ))
            .await
            .into_iter()
            .collect::<Result<Vec<_>, IndexerError>>()
            .map_err(|e| {
                error!(
                    "Failed to index checkpoints {:?} with error: {}",
                    downloaded_checkpoints,
                    e.to_string()
                );
                e
            })?;
            let (indexed_checkpoints, indexed_epochs): (Vec<_>, Vec<_>) =
                indexed_checkpoint_epoch_vec.into_iter().unzip();
            index_timer.stop_and_record();

            // Step 3: send indexed_checkpoints to channel to be committed later.
            let tx_checkpoint_sender_guard = self.tx_checkpoint_sender.lock().await;
            // NOTE: when the channel is full, checkpoint_sender_guard will wait until the channel has space.
            // Checkpoints are sent sequentially to stick to the order of checkpoint sequence numbers.
            for indexed_checkpoint in indexed_checkpoints {
                tx_checkpoint_sender_guard
                .send(indexed_checkpoint)
                .await
                .map_err(|e| {
                    error!("Failed to send indexed checkpoint to checkpoint commit handler with error: {}", e.to_string());
                    IndexerError::MpscChannelError(e.to_string())
                })?;
            }
            drop(tx_checkpoint_sender_guard);

            // Step 4: handle indexed_epochs, which depends on downloaded object changes.
            for epoch in indexed_epochs.into_iter().flatten() {
                // commit first epoch immediately, send other epochs to channel to be committed later.
                if epoch.last_epoch.is_none() {
                    let epoch_db_guard = self.metrics.epoch_db_commit_latency.start_timer();
                    info!("Persisting first epoch...");
                    let mut persist_first_epoch_res = self.state.persist_epoch(&epoch).await;
                    while persist_first_epoch_res.is_err() {
                        warn!("Failed to persist first epoch, retrying...");
                        persist_first_epoch_res = self.state.persist_epoch(&epoch).await;
                    }
                    epoch_db_guard.stop_and_record();
                    self.metrics.total_epoch_committed.inc();
                    info!("Persisted first epoch");
                } else {
                    let epoch_sender_guard = self.epoch_sender.lock().await;
                    // NOTE: when the channel is full, epoch_sender_guard will wait until the channel has space.
                    epoch_sender_guard.send(epoch.clone()).await.map_err(|e| {
                        error!("Failed to send indexed epoch to epoch commit handler with error {}",e.to_string());
                        IndexerError::MpscChannelError(e.to_string())
                    })?;
                    drop(epoch_sender_guard);
                }

                let staking_sender_guard = self.staking_sender.lock().await;
                staking_sender_guard
                    .send((epoch.new_epoch.epoch as u64, Some(epoch)))
                    .await
                    .map_err(|e| {
                        error!("Failed to send indexed epoch to epoch staking handler with error {}",e.to_string());
                        IndexerError::MpscChannelError(e.to_string())
                    })?;
                drop(staking_sender_guard);
            }
        }
    }

    async fn start_download_and_index_object_checkpoint(&self) -> Result<(), IndexerError> {
        info!("Indexer object checkpoint download & index task started...");
        let last_seq_from_db = self
            .state
            .get_latest_object_checkpoint_sequence_number()
            .await?;
        if last_seq_from_db > 0 {
            info!("Resuming obj handler from checkpoint {last_seq_from_db}");
        }
        let mut next_cursor_sequence_number = last_seq_from_db + 1;
        // NOTE: we will download checkpoints in parallel, but we will commit them sequentially.
        // We will start with MAX_PARALLEL_DOWNLOADS, and adjust if no more checkpoints are available.
        let mut current_parallel_downloads = MAX_PARALLEL_DOWNLOADS;
        loop {
            // Step 1: download tx checkpoint data for checkpoints in the current batch
            let download_futures = (next_cursor_sequence_number
                ..next_cursor_sequence_number + current_parallel_downloads as i64)
                .map(|seq_num| self.download_checkpoint_data(seq_num as u64));
            info!("download objects from {:?} to {:?}", next_cursor_sequence_number, next_cursor_sequence_number + current_parallel_downloads as i64);
            let download_results = join_all(download_futures).await;
            let mut downloaded_checkpoints = vec![];
            // NOTE: Push sequentially and if one of the downloads failed,
            // we will discard all following checkpoints and retry, to avoid messing up the DB commit order.
            for download_result in download_results {
                if let Ok(checkpoint) = download_result {
                    downloaded_checkpoints.push(checkpoint);
                } else {
                    if let Err(IndexerError::UnexpectedFullnodeResponseError(fn_e)) =
                        download_result
                    {
                        warn!(
                            "Unexpected response from fullnode for checkpoints: {}",
                            fn_e
                        );
                    } else if let Err(IndexerError::FullNodeReadingError(fn_e)) = download_result {
                        warn!("Fullnode reading error for checkpoints {}: {}. It can be transient or due to rate limiting.", next_cursor_sequence_number, fn_e);
                    } else {
                        warn!("Error downloading checkpoints: {:?}", download_result);
                    }
                    break;
                }
            }
            next_cursor_sequence_number += downloaded_checkpoints.len() as i64;
            // NOTE: with this line, we can make sure that:
            // - when indexer is way behind and catching up, we download MAX_PARALLEL_DOWNLOADS checkpoints in parallel;
            // - when indexer is up to date, we download at least one checkpoint at a time.
            current_parallel_downloads =
                std::cmp::min(downloaded_checkpoints.len() + 1, MAX_PARALLEL_DOWNLOADS);
            if downloaded_checkpoints.is_empty() {
                warn!(
                    "No checkpoints were downloaded for sequence number {}, retrying...",
                    next_cursor_sequence_number
                );
                continue;
            }

            // Step 2: Transform tx checkpoint data to indexed_checkpoints and indexed_epochs
            let index_timer = self.metrics.checkpoint_index_latency.start_timer();
            let indexed_checkpoint_epoch_vec = join_all(downloaded_checkpoints.iter().map(
                |downloaded_checkpoint| async {
                    self.index_checkpoint_and_epoch(downloaded_checkpoint).await
                },
            ))
            .await
            .into_iter()
            .collect::<Result<Vec<_>, IndexerError>>()
            .map_err(|e| {
                error!(
                    "Failed to index checkpoints {:?} with error: {}",
                    downloaded_checkpoints,
                    e.to_string()
                );
                e
            })?;
            let (indexed_checkpoints, _indexed_epochs): (Vec<_>, Vec<_>) =
                indexed_checkpoint_epoch_vec.into_iter().unzip();
            index_timer.stop_and_record();

            // Step 3: send indexed_checkpoints to object channel to be committed later.
            let object_checkpoint_sender_guard = self.object_checkpoint_sender.lock().await;
            // NOTE: when the channel is full, checkpoint_sender_guard will wait until the channel has space.
            // Checkpoints are sent sequentially to stick to the order of checkpoint sequence numbers.
            for indexed_checkpoint in indexed_checkpoints {
                object_checkpoint_sender_guard
                .send(indexed_checkpoint)
                .await
                .map_err(|e| {
                    error!("Failed to send indexed checkpoint to checkpoint commit handler with error: {}", e.to_string());
                    IndexerError::MpscChannelError(e.to_string())
                })?;
            }
            drop(object_checkpoint_sender_guard);
        }
    }

    async fn start_tx_checkpoint_commit(&self) -> Result<(), IndexerError> {
        info!("Indexer tx checkpoint commit task started...");
        loop {
            let mut tx_checkpoint_receiver_guard = self.tx_checkpoint_receiver.lock().await;
            let indexed_checkpoint = tx_checkpoint_receiver_guard.recv().await;
            drop(tx_checkpoint_receiver_guard);

            if let Some(indexed_checkpoint) = indexed_checkpoint {
                if self.config.skip_db_commit {
                    info!(
                        "Downloaded and indexed tx checkpoint {} successfully, skipping DB commit...",
                        indexed_checkpoint.checkpoint.sequence_number,
                    );
                    continue;
                }
                info!("download indexed_checkpoint {:?}", indexed_checkpoint);
                let indexed_checkpoint_nft = indexed_checkpoint.clone();
                // Write checkpoint to DB
                let TemporaryCheckpointStore {
                    checkpoint,
                    transactions,
                    events,
                    object_changes,
                    packages,
                    input_objects,
                    changed_objects,
                    move_calls,
                    recipients,
                } = indexed_checkpoint;
                let move_calls_len = move_calls.len();
                let checkpoint_seq = checkpoint.sequence_number;

                // NOTE: retrials are necessary here, otherwise results can be popped and discarded.
                let events_handler = self.clone();
                let events_cloned = events.clone();
                spawn_monitored_task!(async move {
                    let mut event_commit_res =
                        events_handler.state.persist_events(&events_cloned).await;
                    while let Err(e) = event_commit_res {
                        warn!(
                            "Indexer event commit failed with error: {:?}, retrying after {:?} milli-secs...",
                            e, DB_COMMIT_RETRY_INTERVAL_IN_MILLIS
                        );
                        tokio::time::sleep(Duration::from_millis(DB_COMMIT_RETRY_INTERVAL_IN_MILLIS,)).await;
                        event_commit_res = events_handler.state.persist_events(&events_cloned).await;
                    }
                });

                let packages_handler = self.clone();
                spawn_monitored_task!(async move {
                    let mut package_commit_res =
                        packages_handler.state.persist_packages(&packages).await;
                    while let Err(e) = package_commit_res {
                        warn!(
                            "Indexer package commit failed with error: {:?}, retrying after {:?} milli-secs...",
                            e, DB_COMMIT_RETRY_INTERVAL_IN_MILLIS
                        );
                        tokio::time::sleep(Duration::from_millis(DB_COMMIT_RETRY_INTERVAL_IN_MILLIS,)).await;
                        package_commit_res = packages_handler.state.persist_packages(&packages).await;
                    }
                });

                let proposals = object_changes
                    .iter()
                    .flat_map(|x| x.changed_objects.iter())
                    .filter(|x| Proposal::is_proposal(x))
                    .map(|object| object.clone().try_into())
                    .map_ok(|x| x)
                    .collect::<Result<Vec<Proposal>, IndexerError>>()?;

                if proposals.len() > 0 {
                    let proposal_handler = self.clone();
                    spawn_monitored_task!(async move {
                        info!("got proposals {:?}", proposals);
                        let mut proposals_commit_res =
                            proposal_handler.state.persist_proposals(&proposals).await;
                        while let Err(e) = proposals_commit_res {
                            warn!("Indexer proposals commit failed with error: {:?}, retrying fater {:?} milli-secs",
                              e, DB_COMMIT_RETRY_INTERVAL_IN_MILLIS);
                            tokio::time::sleep(Duration::from_millis(DB_COMMIT_RETRY_INTERVAL_IN_MILLIS,)).await;
                            proposals_commit_res = proposal_handler.state.persist_proposals(&proposals).await;
                        }
                    });
                }

                let mining_nft_handler = self.clone();
                spawn_monitored_task!(async move {
                    let mut mining_nft_commit_res = mining_nft_handler.index_mining_nfts(indexed_checkpoint_nft.clone()).await;
                    while let Err(e) = mining_nft_commit_res {
                        warn!("Indexer mining NFTs commit failed with error: {:?}, retrying fater {:?} milli-secs",
                              e, DB_COMMIT_RETRY_INTERVAL_IN_MILLIS);
                        tokio::time::sleep(Duration::from_millis(DB_COMMIT_RETRY_INTERVAL_IN_MILLIS,)).await;
                        mining_nft_commit_res = mining_nft_handler.index_mining_nfts(indexed_checkpoint_nft.clone()).await;
                    }
                });

                let tx_index_table_handler = self.clone();
                spawn_monitored_task!(async move {
                    let mut transaction_index_tables_commit_res = tx_index_table_handler
                        .state
                        .persist_transaction_index_tables(
                            &input_objects,
                            &changed_objects,
                            &move_calls,
                            &recipients,
                        )
                        .await;
                    while let Err(e) = transaction_index_tables_commit_res {
                        warn!(
                            "Indexer transaction index tables commit failed with error: {:?}, retrying after {:?} milli-secs...",
                            e, DB_COMMIT_RETRY_INTERVAL_IN_MILLIS
                        );
                        tokio::time::sleep(Duration::from_millis(DB_COMMIT_RETRY_INTERVAL_IN_MILLIS,)).await;
                        transaction_index_tables_commit_res = tx_index_table_handler
                            .state
                            .persist_transaction_index_tables(
                                &input_objects,
                                &changed_objects,
                                &move_calls,
                                &recipients,
                            )
                            .await;
                    }
                });

                let checkpoint_tx_db_guard =
                    self.metrics.checkpoint_db_commit_latency.start_timer();
                let mut checkpoint_tx_commit_res = self
                    .state
                    .persist_checkpoint_transactions(
                        &checkpoint,
                        &transactions,
                        self.metrics.total_transaction_chunk_committed.clone(),
                    )
                    .await;
                while let Err(e) = checkpoint_tx_commit_res {
                    warn!(
                            "Indexer checkpoint & transaction commit failed with error: {:?}, retrying after {:?} milli-secs...",
                            e, DB_COMMIT_RETRY_INTERVAL_IN_MILLIS
                        );
                    tokio::time::sleep(Duration::from_millis(DB_COMMIT_RETRY_INTERVAL_IN_MILLIS, )).await;
                    checkpoint_tx_commit_res = self
                        .state
                        .persist_checkpoint_transactions(
                            &checkpoint,
                            &transactions,
                            self.metrics.total_transaction_chunk_committed.clone(),
                        )
                        .await;
                }

                // Persist staking requests, excluding triggered by the system.
                if move_calls_len > 0 {
                    self.persist_staking_if_has_any(&checkpoint, &object_changes, &events)
                        .await?;
                }

                checkpoint_tx_db_guard.stop_and_record();
                self.metrics
                    .latest_tx_checkpoint_sequence_number
                    .set(checkpoint_seq);
                self.metrics.total_tx_checkpoint_committed.inc();
                let tx_count = transactions.len();
                self.metrics
                    .total_transaction_committed
                    .inc_by(tx_count as u64);
                info!(
                    "Tx checkpoint {} committed with {} transactions.",
                    checkpoint_seq, tx_count,
                );
                self.metrics
                    .transaction_per_checkpoint
                    .observe(tx_count as f64);
            } else {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }

    async fn start_epoch_staking_commit(&self) -> Result<(), IndexerError> {
        info!("Indexer epoch staking commit task started...");
        loop {
            let mut staking_receiver_guard = self.staking_receiver.lock().await;
            let indexed_epoch = staking_receiver_guard.recv().await;
            drop(staking_receiver_guard);
            if let Some((epoch, indexed_epoch)) = indexed_epoch {
                let indexed_epoch = if let Some(indexed_epoch) = indexed_epoch {
                    indexed_epoch
                } else {
                    let epochs = self
                        .state
                        .get_epochs(
                            Some(epoch + 1), /* epoch less than this */
                            1,               /* limit */
                            Some(true),      /* descending_order */
                        )
                        .await?;
                    if epochs.len() != 1 {
                        return Err(IndexerError::UncategorizedError(anyhow!(
                            "Failed to load epoch {} from database",
                            epoch
                        )));
                    }
                    if epochs[0].end_of_epoch_info.is_none() {
                        return Err(IndexerError::UncategorizedError(anyhow!(
                            "Failed to handle staking for epoch {}, as it's not end",
                            epoch
                        )));
                    }
                    let epoch_info = epochs.into_iter().next().unwrap();
                    let checkpoint_id = epoch_info.end_of_epoch_info.unwrap().last_checkpoint_id;
                    let data = match self.download_checkpoint_data(checkpoint_id).await {
                        Ok(data) => data,
                        Err(err) => {
                            if let IndexerError::SerdeError(ref s) = err {
                                if s.contains("ObjectNotFound") {
                                    continue;
                                }
                            }
                            return Err(err);
                        }
                    };
                    if let Some(indexed_epoch) = self.build_temporary_epoch(&data).await? {
                        indexed_epoch
                    } else {
                        return Err(IndexerError::UncategorizedError(anyhow!(
                            "Failed to handle staking for epoch {}, there is no such information",
                            epoch
                        )));
                    }
                };
                info!(
                    "Indexer starts to update address staking, epoch: {}",
                    indexed_epoch.new_epoch.epoch
                );
                let mut update_stakes_res = self.update_address_stakes(&indexed_epoch).await;
                while let Err(e) = update_stakes_res {
                    warn!("Indexer update stakes failed with error: {:?} retrying after {:?} milli-secs...",
                            e, DB_COMMIT_RETRY_INTERVAL_IN_MILLIS
                        );
                    tokio::time::sleep(Duration::from_millis(DB_COMMIT_RETRY_INTERVAL_IN_MILLIS, )).await;
                    update_stakes_res = self.update_address_stakes(&indexed_epoch).await;
                }
                info!(
                    "Indexer starts to persist epoch stake, epoch: {}",
                    indexed_epoch.new_epoch.epoch
                );
                self.state.persist_epoch_stake(&indexed_epoch).await?;
            }
        }
    }

    async fn persist_staking_if_has_any(
        &self,
        checkpoint: &Checkpoint,
        object_changes: &[TransactionObjectChanges],
        events: &[Event],
    ) -> Result<(), IndexerError> {
        let objects: Vec<Object> = object_changes
            .iter()
            .flat_map(|x| x.changed_objects.iter().cloned())
            .collect();
        let deleted_objects: Vec<DeletedObject> = object_changes
            .iter()
            .flat_map(|x| x.deleted_objects.iter().cloned())
            .collect();
        if let Some(extracted) = address_stake::extract(&objects, events) {
            self.state
                .persist_address_stake(checkpoint.clone(), extracted, deleted_objects)
                .await?;
        }
        Ok(())
    }

    async fn start_object_checkpoint_commit(&self) -> Result<(), IndexerError> {
        info!("Indexer object checkpoint commit task started...");
        loop {
            let mut object_checkpoint_receiver_guard = self.object_checkpoint_receiver.lock().await;
            let indexed_checkpoint = object_checkpoint_receiver_guard.recv().await;
            drop(object_checkpoint_receiver_guard);

            if let Some(indexed_checkpoint) = indexed_checkpoint {
                if self.config.skip_db_commit {
                    info!(
                        "Downloaded and indexed object checkpoint {} successfully, skipping DB commit...",
                        indexed_checkpoint.checkpoint.sequence_number,
                    );
                    continue;
                }
                let TemporaryCheckpointStore {
                    checkpoint,
                    transactions: _,
                    events: _,
                    object_changes,
                    packages: _,
                    input_objects: _,
                    changed_objects: _,
                    move_calls: _,
                    recipients: _,
                } = indexed_checkpoint;
                let checkpoint_seq = checkpoint.sequence_number;

                // NOTE: commit object changes in the current task to stick to the original order,
                // spawned tasks are possible to be executed in a different order.
                let object_commit_timer = self.metrics.object_db_commit_latency.start_timer();
                let mut object_changes_commit_res = self
                    .state
                    .persist_object_changes(
                        &object_changes,
                        self.metrics.total_object_change_chunk_committed.clone(),
                        self.metrics.object_mutation_db_commit_latency.clone(),
                        self.metrics.object_deletion_db_commit_latency.clone(),
                    )
                    .await;
                while let Err(e) = object_changes_commit_res {
                    warn!(
                        "Indexer object changes commit failed with error: {:?}, retrying after {:?} milli-secs...",
                        e, DB_COMMIT_RETRY_INTERVAL_IN_MILLIS
                    );
                    tokio::time::sleep(std::time::Duration::from_millis(
                        DB_COMMIT_RETRY_INTERVAL_IN_MILLIS,
                    ))
                    .await;
                    object_changes_commit_res = self
                        .state
                        .persist_object_changes(
                            &object_changes,
                            self.metrics.total_object_change_chunk_committed.clone(),
                            self.metrics.object_mutation_db_commit_latency.clone(),
                            self.metrics.object_deletion_db_commit_latency.clone(),
                        )
                        .await;
                }
                object_commit_timer.stop_and_record();
                self.metrics.total_object_checkpoint_committed.inc();
                self.metrics
                    .total_object_change_committed
                    .inc_by(object_changes.len() as u64);
                self.metrics
                    .latest_indexer_object_checkpoint_sequence_number
                    .set(checkpoint_seq);
                info!(
                    "Object checkpoint {} committed with {} object changes.",
                    checkpoint_seq,
                    object_changes.len(),
                );
            } else {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        }
    }

    async fn start_epoch_commit(&self) -> Result<(), IndexerError> {
        info!("Indexer epoch commit task started...");
        loop {
            let mut epoch_receiver_guard = self.epoch_receiver.lock().await;
            let indexed_epoch = epoch_receiver_guard.recv().await;
            drop(epoch_receiver_guard);

            // Write epoch to DB if needed
            if let Some(indexed_epoch) = indexed_epoch {
                if indexed_epoch.last_epoch.is_some() {
                    let epoch_db_guard = self.metrics.epoch_db_commit_latency.start_timer();
                    let mut epoch_commit_res = self.state.persist_epoch(&indexed_epoch).await;
                    // NOTE: retrials are necessary here, otherwise indexed_epoch can be popped and discarded.
                    while let Err(e) = epoch_commit_res {
                        warn!(
                            "Indexer epoch commit failed with error: {:?}, retrying after {:?} milli-secs...",
                            e, DB_COMMIT_RETRY_INTERVAL_IN_MILLIS
                        );
                        tokio::time::sleep(std::time::Duration::from_millis(
                            DB_COMMIT_RETRY_INTERVAL_IN_MILLIS,
                        ))
                        .await;
                        epoch_commit_res = self.state.persist_epoch(&indexed_epoch).await;
                    }
                    epoch_db_guard.stop_and_record();
                    self.metrics.total_epoch_committed.inc();
                }
            } else {
                // sleep for 1 sec to avoid occupying the mutex, as this happens once per epoch / day
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    }

    async fn update_address_stakes(
        &self,
        indexed_epoch: &TemporaryEpochStore,
    ) -> Result<(), IndexerError> {
        let stakes = self.state.get_ongoing_address_stakes().await?;
        info!("Ongoing stakes number {}", stakes.len());
        for stake in stakes.iter() {
            info!(
                "Indexer is updating address stakes {:?} when the epoch ends",
                stake
            );
            let stake_coin = parse_type_tag(&stake.stake_coin)?;
            let rate = indexed_epoch
                .last_epoch_stable_rate
                .get(&stake_coin)
                .map(|x| x.to_owned())
                .unwrap_or_default();
            let estimated_reward = self
                .get_staking_estimated_reward(indexed_epoch, stake, stake_coin, rate)
                .await?;
            let mut stake = stake.clone();
            stake.estimated_reward = estimated_reward as i64;
            stake.estimated_at_epoch = indexed_epoch.new_epoch.epoch;
            self.state.update_address_stake_reward(&stake).await?;
        }
        Ok(())
    }

    async fn get_staking_estimated_reward(
        &self,
        indexed_epoch: &TemporaryEpochStore,
        stake: &AddressStake,
        stake_coin: TypeTag,
        last_epoch_stable_rate: u64,
    ) -> Result<u64, IndexerError> {
        let stable = stake_coin != address_stake::native_coin().into();
        let validator_address: SuiAddress =
            AccountAddress::from_hex_literal(&stake.validator_address)
                .map_err(|err| {
                    IndexerError::UncategorizedError(anyhow!(
                        "Failed parse address {} with error {:?}",
                        &stake.validator_address,
                        err,
                    ))
                })?
                .into();
        let validator = indexed_epoch
            .validator_stakes
            .iter()
            .find(|x| x.address == validator_address);

        if let Some(validator) = validator {
            let (exchange_rates_id, current_exchange_rates) = if stable {
                let coin_type = parse_type_tag(&stake.stake_coin)?;
                if let Some(stable_pool) = &validator.stable_pool {
                    if let Some(pool) = stable_pool.coins.get(&coin_type) {
                        (pool.exchange_rates.id, &pool.current_exchange_rates)
                    } else {
                        // TODO: figure out why it's none.
                        warn!("Stable stake lacks stable pool: {:?} of coin_type: {:?}, stable_pool: {:?}", stake, coin_type, stable_pool);
                        return Ok(0u64);
                    }
                } else {
                    // TODO: figure out why it's none.
                    warn!("Stable stake lacks stable pool: {:?}", stake);
                    return Ok(0u64);
                }
            } else {
                (
                    validator.staking_pool.exchange_rates.id,
                    &validator.current_exchange_rates,
                )
            };
            info!(
                "Indexer is getting estimated reward, exchange_rates_id: {:?}, staked_object_id: {}, current_exchange_rates: {:?}, last_epoch_stable_rate: {}, stake_coin: {:?}",
                exchange_rates_id, stake.staked_object_id, current_exchange_rates, last_epoch_stable_rate, stake_coin,
            );
            let mut reward_withdraw_amount = self
                .get_estimated_reward(stake, exchange_rates_id, current_exchange_rates)
                .await?;
            if stable {
                let rate = last_epoch_stable_rate;
                // See fun withdraw_rewards<STABLE> in stable_pool.move
                reward_withdraw_amount = ((reward_withdraw_amount as u128) * (rate as u128)
                    / (1000000000 as u128)) as u64;
            }
            return Ok(reward_withdraw_amount);
        }

        warn!("No validator matched of stake: {:?}", stake);
        // TODO: we don't know how to update when the validator is missed.
        Ok(0u64)
    }

    async fn get_estimated_reward(
        &self,
        stake: &AddressStake,
        exchange_rates_id: ObjectID,
        current_rate: &PoolTokenExchangeRate,
    ) -> Result<u64, IndexerError> {
        let stake_rate = match stable_pool::get_pool_exchange_rate(
            self.http_client.clone(),
            exchange_rates_id,
            stake.stake_activation_epoch as u64,
        )
        .await
        {
            Err(err) => {
                warn!(
                    "Failed to read exchange rates with error {:?}, exchange_rates_id: {:?}, stake: {:?}",
                    err, exchange_rates_id, stake,
                );
                PoolTokenExchangeRate::default()
            }
            Ok(rate) => rate,
        };
        info!(
            "Indexer got rates from exchange_rates: {:?}, current_rate: {:?}, stake_rate({}): {:?}",
            exchange_rates_id, current_rate, stake.stake_activation_epoch, stake_rate,
        );
        // copied from crates/sui-json-rpc/src/governance_api.rs
        let estimated_reward =
            ((stake_rate.rate() / current_rate.rate()) - 1.0) * stake.principal_amount as f64;
        return Ok(max(0, estimated_reward.round() as u64));
    }

    /// Download all the data we need for one checkpoint.
    async fn download_checkpoint_data(
        &self,
        seq: CheckpointSequenceNumber,
    ) -> Result<CheckpointData, IndexerError> {
        let latest_fn_checkpoint_seq = self
            .http_client
            .get_latest_checkpoint_sequence_number()
            .await
            .map_err(|e| {
                IndexerError::FullNodeReadingError(format!(
                    "Failed to get latest checkpoint sequence number and error {:?}",
                    e
                ))
            })?;
        self.metrics
            .latest_fullnode_checkpoint_sequence_number
            .set((*latest_fn_checkpoint_seq) as i64);

        let mut checkpoint = self
            .http_client
            .get_checkpoint(seq.into())
            .await
            .map_err(|e| {
                IndexerError::FullNodeReadingError(format!(
                    "Failed to get checkpoint with sequence number {} and error {:?}",
                    seq, e
                ))
            });
        let fn_checkpoint_guard = self
            .metrics
            .fullnode_checkpoint_wait_and_download_latency
            .start_timer();
        while checkpoint.is_err() {
            // sleep for 0.1 second and retry if latest checkpoint is not available yet
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            // TODO(gegaowp): figure how to only measure successful checkpoint download time
            checkpoint = self
                .http_client
                .get_checkpoint(seq.into())
                .await
                .map_err(|e| {
                    IndexerError::FullNodeReadingError(format!(
                        "Failed to get checkpoint with sequence number {} and error {:?}",
                        seq, e
                    ))
                })
        }
        fn_checkpoint_guard.stop_and_record();
        // unwrap here is safe because we checked for error above
        let checkpoint = checkpoint.unwrap();

        let fn_transaction_guard = self
            .metrics
            .fullnode_transaction_download_latency
            .start_timer();
        let transactions = join_all(checkpoint.transactions.chunks(MULTI_GET_CHUNK_SIZE).map(
            |digests| multi_get_full_transactions(self.http_client.clone(), digests.to_vec()),
        ))
        .await
        .into_iter()
        .try_fold(vec![], |mut acc, chunk| {
            acc.extend(chunk?);
            Ok::<_, IndexerError>(acc)
        })?;
        fn_transaction_guard.stop_and_record();

        let fn_object_guard = self.metrics.fullnode_object_download_latency.start_timer();
        let object_changes = transactions
            .iter()
            .flat_map(|tx| get_object_changes(&tx.effects))
            .collect::<Vec<_>>();
        let changed_objects =
            fetch_changed_objects(self.http_client.clone(), object_changes).await?;
        fn_object_guard.stop_and_record();

        Ok(CheckpointData {
            checkpoint,
            transactions,
            changed_objects,
        })
    }

    async fn index_checkpoint_and_epoch(
        &self,
        data: &CheckpointData,
    ) -> Result<(TemporaryCheckpointStore, Option<TemporaryEpochStore>), IndexerError> {
        let CheckpointData {
            checkpoint,
            transactions,
            changed_objects,
        } = data;

        // Index transaction
        let temp_tx_store_iter = transactions
            .iter()
            .map(|tx| TemporaryTransactionBlockResponseStore::from(tx.clone()));
        let db_transactions: Vec<Transaction> = temp_tx_store_iter
            .map(|tx| tx.try_into())
            .collect::<Result<Vec<Transaction>, _>>()?;

        // Index events
        let events = transactions
            .iter()
            .flat_map(|tx| tx.events.data.iter().map(move |event| event.clone().into()))
            .collect::<Vec<_>>();

        // Index objects
        let tx_objects = changed_objects
            .iter()
            // Unwrap safe here as we requested previous tx data in the request.
            .fold(BTreeMap::<_, Vec<_>>::new(), |mut acc, (status, o)| {
                if let Some(digest) = &o.previous_transaction {
                    acc.entry(*digest).or_default().push((status, o));
                }
                acc
            });

        let objects_changes = transactions
            .iter()
            .map(|tx| {
                let changed_db_objects = tx_objects
                    .get(&tx.digest)
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|(status, o)| {
                        Object::from(
                            checkpoint.epoch,
                            Some(checkpoint.sequence_number),
                            status,
                            o,
                        )
                    })
                    .collect::<Vec<_>>();
                let deleted_objects = get_deleted_db_objects(
                    &tx.effects,
                    checkpoint.epoch,
                    Some(checkpoint.sequence_number),
                );

                TransactionObjectChanges {
                    changed_objects: changed_db_objects,
                    deleted_objects,
                }
            })
            .collect();

        // Index packages
        let packages = Self::index_packages(transactions, changed_objects)?;

        // Store input objects, move calls and recipients separately for transaction query indexing.
        let input_objects = transactions
            .iter()
            .map(|tx| tx.get_input_objects(checkpoint.epoch))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        let changed_objects: Vec<ChangedObject> = transactions
            .iter()
            .flat_map(|tx| tx.get_changed_objects(checkpoint.epoch))
            .collect();
        let move_calls = transactions
            .iter()
            .flat_map(|tx| tx.get_move_calls(checkpoint.epoch))
            .collect();
        let recipients = transactions
            .iter()
            .flat_map(|tx| tx.get_recipients(checkpoint.epoch))
            .collect();

        // NOTE: Index epoch when object checkpoint index has reached the same checkpoint,
        // because epoch info is based on the latest system state object by the current checkpoint.
        let epoch_index = self.build_temporary_epoch(data).await?;
        let total_transactions = db_transactions.iter().map(|t| t.transaction_count).sum();
        let total_successful_transaction_blocks = db_transactions
            .iter()
            .filter(|t| t.execution_success)
            .count();
        let total_successful_transactions = db_transactions
            .iter()
            .filter(|t| t.execution_success)
            .map(|t| t.transaction_count)
            .sum();
        let total_transact_bfc = db_transactions
            .iter()
            .filter(|t| t.execution_success)
            .map(|t| t.transact_bfc)
            .sum();

        let system_tick = db_transactions
            .iter()
            .filter(|t| t.sender == format!("0x{}", AccountAddress::ZERO.to_hex()))
            .count()
            == db_transactions.len();

        Ok((
            TemporaryCheckpointStore {
                checkpoint: Checkpoint::from(
                    checkpoint,
                    total_transactions,
                    total_successful_transactions,
                    total_successful_transaction_blocks as i64,
                    total_transact_bfc,
                    system_tick,
                )?,
                transactions: db_transactions,
                events,
                object_changes: objects_changes,
                packages,
                input_objects,
                changed_objects,
                move_calls,
                recipients,
            },
            epoch_index,
        ))
    }

    async fn build_temporary_epoch(
        &self,
        data: &CheckpointData,
    ) -> Result<Option<TemporaryEpochStore>, IndexerError> {
        let CheckpointData {
            checkpoint,
            transactions,
            changed_objects: _,
        } = data;
        if checkpoint.epoch == 0 && checkpoint.sequence_number == 0 {
            // very first epoch
            let system_state = get_sui_system_state(data)?;
            let validator_set = ValidatorSet::from_system_state(&system_state);
            let validator_stakes = validator_set.parse_stake(self.http_client.clone()).await?;
            let stable_pools = extract_stable_stakes(&validator_stakes);
            let system_state: SuiSystemStateSummary = system_state.into_sui_system_state_summary();
            let validators = system_state
                .active_validators
                .iter()
                .map(|v| (system_state.epoch, v.clone()).into())
                .collect();

            Ok(Some(TemporaryEpochStore {
                last_epoch: None,
                new_epoch: DBEpochInfo {
                    epoch: 0,
                    first_checkpoint_id: 0,
                    epoch_start_timestamp: system_state.epoch_start_timestamp_ms as i64,
                    ..Default::default()
                },
                system_state: system_state.into(),
                stable_pools,
                validators,
                validator_stakes,
                last_epoch_stable_rate: validator_set.get_stable_rates(),
            }))
        } else if let Some(end_of_epoch_data) = &checkpoint.end_of_epoch_data {
            let system_state = get_sui_system_state(data)?;
            let validator_set = ValidatorSet::from_system_state(&system_state);
            let validator_stakes = validator_set.parse_stake(self.http_client.clone()).await?;
            let stable_pools = extract_stable_stakes(&validator_stakes);
            let system_state: SuiSystemStateSummary = system_state.into_sui_system_state_summary();
            let epoch_event = transactions.iter().find_map(|tx| {
                tx.events.data.iter().find(|ev| {
                    ev.type_.address == SUI_SYSTEM_ADDRESS
                        && ev.type_.module.as_ident_str() == ident_str!("sui_system_state_inner")
                        && ev.type_.name.as_ident_str() == ident_str!("SystemEpochInfoEvent")
                })
            });

            let event = epoch_event
                .map(|e| bcs::from_bytes::<SystemEpochInfoEvent>(&e.bcs))
                .transpose()?;

            let validators = system_state
                .active_validators
                .iter()
                .map(|v| (system_state.epoch, v.clone()).into())
                .collect();

            let epoch_commitments = end_of_epoch_data
                .epoch_commitments
                .iter()
                .map(|c| match c {
                    CheckpointCommitment::ECMHLiveObjectSetDigest(d) => {
                        Some(d.digest.into_inner().to_vec())
                    }
                })
                .collect();

            let (next_epoch_committee, next_epoch_committee_stake) =
                end_of_epoch_data.next_epoch_committee.iter().fold(
                    (vec![], vec![]),
                    |(mut names, mut stakes), (name, stake)| {
                        names.push(Some(name.as_bytes().to_vec()));
                        stakes.push(Some(*stake as i64));
                        (names, stakes)
                    },
                );

            let event = event.as_ref();

            let last_epoch = system_state.epoch as i64 - 1;
            let network_tx_count_prev_epoch = self
                .state
                .get_network_total_transactions_previous_epoch(last_epoch)
                .await?;
            Ok(Some(TemporaryEpochStore {
                last_epoch: Some(DBEpochInfo {
                    epoch: last_epoch,
                    first_checkpoint_id: 0,
                    last_checkpoint_id: Some(checkpoint.sequence_number as i64),
                    epoch_start_timestamp: 0,
                    epoch_end_timestamp: Some(checkpoint.timestamp_ms as i64),
                    epoch_total_transactions: checkpoint.network_total_transactions as i64
                        - network_tx_count_prev_epoch,
                    next_epoch_version: Some(
                        end_of_epoch_data.next_epoch_protocol_version.as_u64() as i64,
                    ),
                    next_epoch_committee,
                    next_epoch_committee_stake,
                    stake_subsidy_amount: event.map(|e| e.stake_subsidy_amount),
                    reference_gas_price: event.map(|e| e.reference_gas_price),
                    storage_fund_balance: event.map(|e| e.storage_fund_balance),
                    total_gas_fees: event.map(|e| e.total_gas_fees),
                    total_stake_rewards_distributed: event
                        .map(|e| e.total_stake_rewards_distributed),
                    total_stake: event.map(|e| e.total_stake),
                    storage_fund_reinvestment: event.map(|e| e.storage_fund_reinvestment),
                    storage_charge: event.map(|e| e.storage_charge),
                    protocol_version: event.map(|e| e.protocol_version),
                    storage_rebate: event.map(|e| e.storage_rebate),
                    leftover_storage_fund_inflow: event.map(|e| e.leftover_storage_fund_inflow),
                    epoch_commitments,
                }),
                new_epoch: DBEpochInfo {
                    epoch: system_state.epoch as i64,
                    first_checkpoint_id: checkpoint.sequence_number as i64 + 1,
                    epoch_start_timestamp: system_state.epoch_start_timestamp_ms as i64,
                    ..Default::default()
                },
                system_state: system_state.into(),
                validators,
                stable_pools,
                validator_stakes,
                last_epoch_stable_rate: validator_set.get_stable_rates(),
            }))
        } else {
            Ok(None)
        }
    }

    fn index_packages(
        transactions: &[CheckpointTransactionBlockResponse],
        changed_objects: &[(ObjectStatus, SuiObjectData)],
    ) -> Result<Vec<Package>, IndexerError> {
        let object_map = changed_objects
            .iter()
            .filter_map(|(_, o)| {
                if let SuiRawData::Package(p) = &o
                    .bcs
                    .as_ref()
                    .expect("Expect the content field to be non-empty from data fetching")
                {
                    Some((o.object_id, p))
                } else {
                    None
                }
            })
            .collect::<BTreeMap<_, _>>();

        transactions
            .iter()
            .flat_map(|tx| {
                tx.effects.created().iter().map(|oref| {
                    object_map
                        .get(&oref.reference.object_id)
                        .map(|o| Package::try_from(*tx.transaction.data.sender(), o))
                })
            })
            .flatten()
            .collect()
    }

    fn spawn_periodical_tasks(&self) {
        let handler = self.clone();
        spawn_monitored_task!(async move {
            handler.start_persisting_bfc_history_price().await;
        });
        let handler = self.clone();
        spawn_monitored_task!(async move {
            handler.start_settling_minging_nft_profits().await;
        });
    }

    async fn start_settling_minging_nft_profits(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(600));
        loop {
            interval.tick().await;
            info!("Indexer starts to settle the historic profits of mining NFTs...");
            if let Err(err) = self.settle_mining_nft_profits().await {
                warn!(
                    "Failed to settle the historic profits of mining NFTs with erorr: {:?}",
                    err
                );
            }
        }
    }

    async fn settle_mining_nft_profits(&self) -> Result<(), IndexerError> {
        let dt_timestamp_ms = benfen::get_yesterday_started_at();
        let pendings = self.state.get_unsettle_mining_nfts(dt_timestamp_ms).await?;
        let mut results = vec![];
        let price = benfen::get_bfc_price_in_usd(self.http_client.clone()).await?;
        let mut total_pending_bfc = 0u64;
        for nft in pendings.iter() {
            let current = self
                .state
                .get_mining_nft_profit(nft, dt_timestamp_ms)
                .await?;
            let previous = self
                .state
                .get_mining_nft_profit(nft, dt_timestamp_ms - 86_400_000)
                .await?;
            let pending_bfc = if nft.mining_ticket_id.is_some() {
                benfen::get_mining_nft_pending_reward(
                    self.http_client.clone(),
                    &self.config.mining_nft_contract,
                    &self.config.mining_nft_global,
                    &nft.mining_ticket_id.clone().unwrap_or_default(),
                )
                .await?
            } else {
                0
            };
            total_pending_bfc += pending_bfc;
            let mint_bfc = if current.claimed_reward > 0 {
                current.claimed_reward - previous.pending_reward + pending_bfc as i64
            } else {
                pending_bfc as i64 - previous.pending_reward
            };
            let mint_usd = (mint_bfc as f64 * price) as i64;
            results.push(MiningNFTHistoryProfit {
                owner: nft.owner.clone(),
                miner_id: nft.miner_id.clone(),
                dt_timestamp_ms,
                mint_bfc,
                mint_usd,
                pending_reward: pending_bfc as i64,
                claimed_reward: current.claimed_reward,
                cost_bfc: nft.cost_bfc,
            });
        }
        if results.len() > 0 {
            self.state.persist_mining_nft_profits(results).await?;
            self.state
                .calculate_mining_nft_overall(dt_timestamp_ms, total_pending_bfc)
                .await?;
        }
        Ok(())
    }

    async fn start_persisting_bfc_history_price(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(600));
        loop {
            interval.tick().await;
            info!("Indexer starts to maintain BFC price history periodically...");
            match benfen::get_bfc_price_in_usd(self.http_client.clone()).await {
                Ok(price) => {
                    let result = self
                        .state
                        .persist_price(PriceHistory {
                            ts: Utc::now().timestamp_millis(),
                            coin: "BFC".to_owned(),
                            price: (price * 10000f64) as i64,
                        })
                        .await;
                    if let Err(err) = result {
                        warn!("Failed to get BFC price with error: {:?}", err);
                    }
                }
                Err(err) => warn!("Failed to get BFC price with error: {:?}", err),
            }
        }
    }

    async fn index_mining_nfts(
        &self,
        indexed_checkpoint: TemporaryCheckpointStore,
    ) -> Result<(), IndexerError> {
        let TemporaryCheckpointStore {
            checkpoint,
            transactions: _,
            events,
            object_changes,
            packages: _,
            input_objects: _,
            changed_objects: _,
            move_calls: _,
            recipients: _,
        } = indexed_checkpoint;

        let mut extracted_nfts =
            mining_nft::extract_from_events(&self.config.mining_nft_event_package, &events)?;
        let operations = mining_nft::extract_operations_from_events(
            &self.config.mining_nft_event_package,
            &events,
        )?;
        // So we should avoid to extract transfer nfts if there're operations in the txn.
        if operations.len() == 0 && extracted_nfts.len() == 0 {
            let transfered_nfts = object_changes
                .iter()
                .flat_map(|x| x.changed_objects.iter())
                .filter(|x| mining_nft::is_miner(&self.config.mining_nft_contract, x))
                .map(|object| object.clone().into())
                .collect::<Vec<mining_nft::ExtractedMiningNFT>>();
            extracted_nfts.extend(transfered_nfts);
        }

        let mut flag = false;
        for e in extracted_nfts.into_iter() {
            let mut mining_nft: MiningNFT = (checkpoint.clone(), e).into();
            let object_id = ObjectID::from_hex_literal(&mining_nft.miner_id)?;
            let display = benfen::get_nft_display(self.http_client.clone(), object_id).await?;
            mining_nft.miner_name = display.name.clone();
            mining_nft.miner_url = display.image_url.clone();
            mining_nft.cost_bfc =
                benfen::get_mining_nft_cost_in_bfc(self.config.clone(), self.http_client.clone())
                    .await? as i64;
            flag = true;
            self.state
                .persist_mining_nft(crate::store::MiningNFTOperation::Creation(mining_nft), checkpoint.sequence_number)
                .await?;
        }
        for p in operations.into_iter() {
            flag = true;
            self.state
                .persist_mining_nft(crate::store::MiningNFTOperation::Operation(p), checkpoint.sequence_number)
                .await?;
        }
        if flag {
            self.state.refresh_mining_nft().await?;
        }

        let liq_admin_addrs: HashSet<AccountAddress> = self
            .config
            .mining_nft_liquidity_admins
            .iter()
            .map(|x| SuiAddress::from_str(x).unwrap_or_default().into())
            .collect();

        let liq_events: Vec<_> = events
            .iter()
            .filter(|x| {
                liq_admin_addrs.contains(
                    &AccountAddress::from_hex_literal(&x.sender).unwrap_or(AccountAddress::ZERO),
                )
            })
            .cloned()
            .collect();
        if liq_events.len() > 0 {
            let liquidities = mining_nft::extract_liquidities_from_event(&liq_events)?;
            let mut mls = vec![];
            for liq in liquidities.into_iter() {
                let base_price_gte = benfen::get_price_at_tick(
                    self.http_client.clone(),
                    &self.config.mining_nft_dex_contract,
                    liq.1.tick_lower,
                )
                .await?;
                let base_price_lte = benfen::get_price_at_tick(
                    self.http_client.clone(),
                    &self.config.mining_nft_dex_contract,
                    liq.1.tick_upper,
                )
                .await?;

                let mut ml: mining_nft::MiningNFTLiquiditiy = (checkpoint.clone(), liq).into();
                ml.base_price_gte = (base_price_gte * mining_nft::PRICE_TO_INT_SCALE) as i64;
                ml.base_price_lte = (base_price_lte * mining_nft::PRICE_TO_INT_SCALE) as i64;
                mls.push(ml);
            }
            if mls.len() > 0 {
                self.state.persist_mining_nft_liquidities(mls).await?;
            }
        }
        Ok(())
    }

    async fn check_epoch_staking(&self, checkpoint_id: i64) -> Result<(), IndexerError> {
        if checkpoint_id == 0 {
            return Ok(());
        }
        info!("check_epoch_staking begin {:?}", checkpoint_id);
        let data = self.download_checkpoint_data(checkpoint_id as u64).await?;
        let stop_epoch = data.checkpoint.epoch;
        let latest = self.state.get_last_epoch_stake().await?.unwrap_or_default();
        let start_epoch = latest.epoch as u64;
        let staking_sender_guard = self.staking_sender.lock().await;
        for epoch in start_epoch..stop_epoch {
            info!("Start indexing epoch staking {:?}", epoch);
            staking_sender_guard
                .send((epoch, None))
                .await
                .map_err(|e| {
                    error!(
                        "Failed to send epoch to staking commit handler with error: {}",
                        e.to_string()
                    );
                    IndexerError::MpscChannelError(e.to_string())
                })?;
        }
        drop(staking_sender_guard);
        Ok(())
    }
}

// TODO(gegaowp): re-orgnize object util functions below
pub fn get_object_changes(
    effects: &SuiTransactionBlockEffects,
) -> Vec<(ObjectID, SequenceNumber, ObjectStatus)> {
    let created = effects.created().iter().map(|o: &OwnedObjectRef| {
        (
            o.reference.object_id,
            o.reference.version,
            ObjectStatus::Created,
        )
    });
    let mutated = effects.mutated().iter().map(|o: &OwnedObjectRef| {
        (
            o.reference.object_id,
            o.reference.version,
            ObjectStatus::Mutated,
        )
    });
    let unwrapped = effects.unwrapped().iter().map(|o: &OwnedObjectRef| {
        (
            o.reference.object_id,
            o.reference.version,
            ObjectStatus::Unwrapped,
        )
    });
    created.chain(mutated).chain(unwrapped).collect()
}

pub async fn fetch_changed_objects(
    http_client: HttpClient,
    object_changes: Vec<(ObjectID, SequenceNumber, ObjectStatus)>,
) -> Result<Vec<(ObjectStatus, SuiObjectData)>, IndexerError> {
    join_all(object_changes.chunks(MULTI_GET_CHUNK_SIZE).map(|objects| {
        let wanted_past_object_statuses: Vec<ObjectStatus> =
            objects.iter().map(|(_, _, status)| *status).collect();

        let wanted_past_object_request = objects
            .iter()
            .map(|(id, seq_num, _)| SuiGetPastObjectRequest {
                object_id: *id,
                version: *seq_num,
            })
            .collect();
        http_client
            .try_multi_get_past_objects(
                wanted_past_object_request,
                Some(SuiObjectDataOptions::bcs_lossless()),
            )
            .map(move |resp| (resp, wanted_past_object_statuses))
    }))
    .await
    .into_iter()
    .try_fold(vec![], |mut acc, chunk| {
        let object_data = chunk.0?.into_iter().try_fold(vec![], |mut acc, resp| {
            let object_data = resp.into_object()?;
            acc.push(object_data);
            Ok::<Vec<SuiObjectData>, Error>(acc)
        })?;
        let mutated_object_chunk = chunk.1.into_iter().zip(object_data);
        acc.extend(mutated_object_chunk);
        Ok::<_, Error>(acc)
    })
    .map_err(|e| {
        IndexerError::SerdeError(format!(
            "Failed to generate changed objects of checkpoint with err {:?}",
            e
        ))
    })
}

// TODO(gegaowp): temp. disable fast-path
// pub fn to_changed_db_objects(
//     changed_objects: Vec<(ObjectStatus, SuiObjectData)>,
//     epoch: u64,
//     checkpoint: Option<CheckpointSequenceNumber>,
// ) -> Vec<Object> {
//     changed_objects
//         .into_iter()
//         .map(|(status, o)| Object::from(epoch, checkpoint.map(<u64>::from), &status, &o))
//         .collect::<Vec<_>>()
// }

pub fn get_deleted_db_objects(
    effects: &SuiTransactionBlockEffects,
    epoch: EpochId,
    checkpoint: Option<CheckpointSequenceNumber>,
) -> Vec<DeletedObject> {
    let deleted = effects.deleted().iter();
    let deleted = deleted.map(|o| (ObjectStatus::Deleted, o));
    let wrapped = effects.wrapped().iter();
    let wrapped = wrapped.map(|o| (ObjectStatus::Wrapped, o));
    let unwrapped_then_deleted = effects.unwrapped_then_deleted().iter();
    let unwrapped_then_deleted =
        unwrapped_then_deleted.map(|o| (ObjectStatus::UnwrappedThenDeleted, o));
    deleted
        .chain(wrapped)
        .chain(unwrapped_then_deleted)
        .map(|(status, oref)| {
            DeletedObject::from(
                epoch,
                checkpoint.map(<u64>::from),
                oref,
                effects.transaction_digest(),
                &status,
            )
        })
        .collect::<Vec<_>>()
}
