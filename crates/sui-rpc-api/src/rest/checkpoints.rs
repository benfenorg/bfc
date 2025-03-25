// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::Query;
use axum::extract::{Path, State};
use axum::Json;
use sui_sdk_types::CheckpointSequenceNumber;
use sui_types::storage::ReadStore;

use super::{ApiEndpoint, RouteHandler};
use crate::reader::StateReader;
use crate::rest::PageCursor;
use crate::service::checkpoints::CheckpointId;
use crate::types::{CheckpointResponse, GetCheckpointOptions};
use crate::Result;
use crate::{Direction, RpcService};

/// Fetch a Checkpoint
///
/// Fetch a checkpoint either by `CheckpointSequenceNumber` (checkpoint height) or by
/// `CheckpointDigest` and optionally request its contents.
///
/// If the checkpoint has been pruned and is not available, a 410 will be returned.
pub struct GetCheckpoint;

impl ApiEndpoint<RpcService> for GetCheckpoint {
    fn method(&self) -> axum::http::Method {
        axum::http::Method::GET
    }

    fn path(&self) -> &'static str {
        "/checkpoints/{checkpoint}"
    }

    fn handler(&self) -> RouteHandler<RpcService> {
        RouteHandler::new(self.method(), get_checkpoint)
    }
}

async fn get_checkpoint(
    Path(checkpoint_id): Path<CheckpointId>,
    Query(options): Query<GetCheckpointOptions>,
    State(state): State<RpcService>,
) -> Result<Json<CheckpointResponse>> {
    state.get_checkpoint(Some(checkpoint_id), options).map(Json)
}

/// Query parameters for the GetCheckpoint endpoint
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GetCheckpointQueryParameters {
    /// Request `CheckpointContents` be included in the response
    #[serde(default)]
    pub contents: bool,
}

/// List Checkpoints
///
/// Request a page of checkpoints, and optionally their contents, ordered by
/// `CheckpointSequenceNumber`.
///
/// If the requested page is below the Node's `lowest_available_checkpoint`, a 410 will be
/// returned.
pub struct ListCheckpoints;

impl ApiEndpoint<RpcService> for ListCheckpoints {
    fn method(&self) -> axum::http::Method {
        axum::http::Method::GET
    }

    fn path(&self) -> &'static str {
        "/checkpoints"
    }

    fn handler(&self) -> RouteHandler<RpcService> {
        RouteHandler::new(self.method(), list_checkpoints)
    }
}

async fn list_checkpoints(
    Query(parameters): Query<ListCheckpointsPaginationParameters>,
    Query(options): Query<GetCheckpointOptions>,
    State(state): State<StateReader>,
) -> Result<(
    PageCursor<CheckpointSequenceNumber>,
    Json<Vec<CheckpointResponse>>,
)> {
    let latest_checkpoint = state.inner().get_latest_checkpoint()?.sequence_number;
    let oldest_checkpoint = state.inner().get_lowest_available_checkpoint()?;
    let limit = parameters.limit();
    let start = parameters.start(latest_checkpoint);
    let direction = parameters.direction();

    if start < oldest_checkpoint {
        return Err(crate::RpcServiceError::new(
            axum::http::StatusCode::GONE,
            "Old checkpoints have been pruned",
        ));
    }

    let checkpoints = state
        .checkpoint_iter(direction, start)
        .take(limit)
        .map(|result| {
            result
                .map_err(Into::into)
                .and_then(|(checkpoint, contents)| {

                    let contents = if options.include_contents() {
                        Some(contents.try_into()?)
                    } else {
                        None
                    };
                    Ok(CheckpointResponse {
                        sequence_number: *checkpoint.sequence_number(),
                        digest: (*checkpoint.digest()).into(),
                        summary: Some(sui_sdk_types::CheckpointSummary {
                            epoch: checkpoint.epoch,
                            sequence_number: *checkpoint.sequence_number(),
                            network_total_transactions: checkpoint.network_total_transactions,
                            content_digest: sui_sdk_types::CheckpointContentsDigest::new(*checkpoint.content_digest.inner()),
                            previous_digest: checkpoint.previous_digest.map(|d | sui_sdk_types::CheckpointDigest::new(*d.inner())),
                            epoch_rolling_bfc_gas_cost_summary: sui_sdk_types::GasCostSummary {
                                base_point: checkpoint.epoch_rolling_bfc_gas_cost_summary.base_point,
                                rate: checkpoint.epoch_rolling_bfc_gas_cost_summary.rate,
                                computation_cost: checkpoint.epoch_rolling_bfc_gas_cost_summary.computation_cost,
                                storage_cost: checkpoint.epoch_rolling_bfc_gas_cost_summary.storage_cost,
                                storage_rebate: checkpoint.epoch_rolling_bfc_gas_cost_summary.storage_rebate,
                                non_refundable_storage_fee: checkpoint.epoch_rolling_bfc_gas_cost_summary.non_refundable_storage_fee,
                            },
                            timestamp_ms: checkpoint.timestamp_ms,
                            checkpoint_commitments: checkpoint.checkpoint_commitments.clone().into_iter().map(|c |
                                sui_sdk_types::CheckpointCommitment::EcmhLiveObjectSet{digest: match c {
                                    sui_types::messages_checkpoint::CheckpointCommitment::ECMHLiveObjectSetDigest(ecmh_live_object_set_digest) =>
                                        sui_sdk_types::Digest::new(*ecmh_live_object_set_digest.digest.inner()
                                        ),
                                }}).collect(),
                            end_of_epoch_data: checkpoint.end_of_epoch_data.clone().map(|c | sui_sdk_types::EndOfEpochData {
                                next_epoch_committee: c.next_epoch_committee.into_iter().map(|next_epoch_committee | {
                                    sui_sdk_types::ValidatorCommitteeMember {
                                        public_key: sui_sdk_types::Bls12381PublicKey::new(next_epoch_committee.0.0),
                                        stake: next_epoch_committee.1,
                                    }
                                }).collect(),
                                next_epoch_protocol_version: c.next_epoch_protocol_version.as_u64(),
                                epoch_commitments: c.epoch_commitments.clone().into_iter().map(|epoch_commitment |
                                    sui_sdk_types::CheckpointCommitment::EcmhLiveObjectSet{digest: match epoch_commitment {
                                        sui_types::messages_checkpoint::CheckpointCommitment::ECMHLiveObjectSetDigest(ecmh_live_object_set_digest) =>
                                            sui_sdk_types::Digest::new(*ecmh_live_object_set_digest.digest.inner()
                                            ),
                                    }}).collect(),
                            }),
                            version_specific_data: checkpoint.version_specific_data.clone(),
                        }),
                        signature: Some(checkpoint.into_sig().into()),
                        contents,
                        summary_bcs: None,
                        contents_bcs: None,
                    })
                })
        })
        .collect::<Result<Vec<_>>>()?;

    let cursor = checkpoints.last().and_then(|checkpoint| match direction {
        Direction::Ascending => checkpoint.sequence_number.checked_add(1),
        Direction::Descending => {
            let cursor = checkpoint.sequence_number.checked_sub(1);
            // If we've exhausted our available checkpoint range then there are no more pages left
            if cursor < Some(oldest_checkpoint) {
                None
            } else {
                cursor
            }
        }
    });

    Ok((PageCursor(cursor), Json(checkpoints)))
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ListCheckpointsPaginationParameters {
    /// Page size limit for the response.
    ///
    /// Defaults to `50` if not provided with a maximum page size of `100`.
    pub limit: Option<u32>,
    /// The checkpoint to start listing from.
    ///
    /// Defaults to the latest checkpoint if not provided.
    pub start: Option<CheckpointSequenceNumber>,
    /// The direction to paginate in.
    ///
    /// Defaults to `descending` if not provided.
    pub direction: Option<Direction>,
}

impl ListCheckpointsPaginationParameters {
    pub fn limit(&self) -> usize {
        self.limit
            .map(|l| (l as usize).clamp(1, crate::rest::MAX_PAGE_SIZE))
            .unwrap_or(crate::rest::DEFAULT_PAGE_SIZE)
    }

    pub fn start(&self, default: CheckpointSequenceNumber) -> CheckpointSequenceNumber {
        self.start.unwrap_or(default)
    }

    pub fn direction(&self) -> Direction {
        self.direction.unwrap_or(Direction::Descending)
    }
}
