// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use diesel::prelude::*;

use fastcrypto::traits::EncodeDecodeBase64;
use sui_json_rpc_types::Checkpoint as RpcCheckpoint;
use sui_types::base_types::TransactionDigest;
use sui_types::crypto::AggregateAuthoritySignature;
use sui_types::digests::CheckpointDigest;
use sui_types::gas::{BASE_RATE, DEFAULT_BASE_POINT_FOR_BFC, GasCostSummary};
use sui_types::messages_checkpoint::EndOfEpochData;

use crate::errors::IndexerError;
use crate::schema::checkpoints::{self};

#[derive(Queryable, Insertable, Debug, Clone, Default)]
#[diesel(table_name = checkpoints)]
pub struct Checkpoint {
    pub sequence_number: i64,
    pub checkpoint_digest: String,
    pub epoch: i64,
    pub transactions: Vec<Option<String>>,
    pub previous_checkpoint_digest: Option<String>,
    pub end_of_epoch: bool,
    // total_gas_cost can be negative,
    // which means that overall rebate is greater than overall cost.
    pub total_gas_cost: i64,
    pub total_computation_cost: i64,
    pub total_storage_cost: i64,
    pub total_storage_rebate: i64,
    // NOTE: we should use the following formula as tps_total_transactions for TPS calculation:
    // total_successful_transactions + total_transaction_blocks - total_successful_transaction_blocks
    // such that:
    /// - a successful transaction block is counted as (number of commands in the block);
    /// - a failed transaction block is counted as 1.
    pub total_transaction_blocks: i64,
    pub total_transactions: i64,
    pub total_successful_transaction_blocks: i64,
    pub total_successful_transactions: i64,
    pub network_total_transactions: i64,
    pub timestamp_ms: i64,
    pub validator_signature: String,
    pub total_transact_bfc: i64,
    pub system_tick: bool,
}

impl Checkpoint {
    pub fn from(
        rpc_checkpoint: &RpcCheckpoint,
        total_transactions: i64,
        total_successful_transactions: i64,
        total_successful_transaction_blocks: i64,
        total_transact_bfc: i64,
        system_tick: bool,
    ) -> Result<Self, IndexerError> {
        let total_gas_cost = rpc_checkpoint
            .epoch_rolling_bfc_gas_cost_summary
            .computation_cost as i64
            + rpc_checkpoint.epoch_rolling_bfc_gas_cost_summary.storage_cost as i64
            - rpc_checkpoint.epoch_rolling_bfc_gas_cost_summary.storage_rebate as i64;

        let checkpoint_transactions: Vec<Option<String>> = rpc_checkpoint
            .transactions
            .iter()
            .map(|t| Some(t.base58_encode()))
            .collect();

        Ok(Checkpoint {
            sequence_number: rpc_checkpoint.sequence_number as i64,
            checkpoint_digest: rpc_checkpoint.digest.base58_encode(),
            epoch: rpc_checkpoint.epoch as i64,
            transactions: checkpoint_transactions,
            previous_checkpoint_digest: rpc_checkpoint.previous_digest.map(|d| d.base58_encode()),
            end_of_epoch: rpc_checkpoint.end_of_epoch_data.is_some(),
            total_gas_cost,
            total_computation_cost: rpc_checkpoint
                .epoch_rolling_bfc_gas_cost_summary
                .computation_cost as i64,
            total_storage_cost: rpc_checkpoint.epoch_rolling_bfc_gas_cost_summary.storage_cost as i64,
            total_storage_rebate: rpc_checkpoint.epoch_rolling_bfc_gas_cost_summary.storage_rebate
                as i64,
            total_transaction_blocks: rpc_checkpoint.transactions.len() as i64,
            total_transactions,
            total_successful_transaction_blocks,
            total_successful_transactions,
            network_total_transactions: rpc_checkpoint.network_total_transactions as i64,
            timestamp_ms: rpc_checkpoint.timestamp_ms as i64,
            validator_signature: rpc_checkpoint.validator_signature.encode_base64(),
            total_transact_bfc,
            system_tick,
        })
    }

    pub fn into_rpc(
        self,
        end_of_epoch_data: Option<EndOfEpochData>,
    ) -> Result<RpcCheckpoint, IndexerError> {
        let parsed_digest = self
            .checkpoint_digest
            .parse::<CheckpointDigest>()
            .map_err(|e| {
                IndexerError::SerdeError(format!(
                    "Failed to decode checkpoint digest: {:?} with err: {:?}",
                    self.checkpoint_digest, e
                ))
            })?;

        let parsed_previous_digest = self
            .previous_checkpoint_digest
            .map(|digest| {
                digest.parse::<CheckpointDigest>().map_err(|e| {
                    IndexerError::SerdeError(format!(
                        "Failed to decode previous checkpoint digest: {:?} with err: {:?}",
                        digest, e
                    ))
                })
            })
            .transpose()?;
        let parsed_tx_digests: Vec<TransactionDigest> = self
            .transactions
            .into_iter()
            .filter_map(|tx| {
                tx.map(|tx| {
                    tx.parse().map_err(|e| {
                        IndexerError::SerdeError(format!(
                            "Failed to decode transaction digest: {:?} with err: {:?}",
                            tx, e
                        ))
                    })
                })
            })
            .collect::<Result<Vec<TransactionDigest>, IndexerError>>()?;
        let validator_sig = AggregateAuthoritySignature::decode_base64(&self.validator_signature)
            .map_err(|e| {
            IndexerError::SerdeError(format!(
                "Failed to decode validator signature: {:?} with err: {:?}",
                self.validator_signature, e
            ))
        })?;

        Ok(RpcCheckpoint {
            epoch: self.epoch as u64,
            sequence_number: self.sequence_number as u64,
            digest: parsed_digest,
            previous_digest: parsed_previous_digest,
            end_of_epoch_data,
            validator_signature: validator_sig,
            epoch_rolling_bfc_gas_cost_summary: GasCostSummary {
                base_point: DEFAULT_BASE_POINT_FOR_BFC,
                rate: BASE_RATE,
                computation_cost: self.total_computation_cost as u64,
                storage_cost: self.total_storage_cost as u64,
                storage_rebate: self.total_storage_rebate as u64,
                non_refundable_storage_fee: 0,
            },
            epoch_rolling_stable_gas_cost_summary_map: HashMap::new(),
            network_total_transactions: self.network_total_transactions as u64,
            timestamp_ms: self.timestamp_ms as u64,
            transactions: parsed_tx_digests,
            checkpoint_commitments: vec![],
        })
    }
}
