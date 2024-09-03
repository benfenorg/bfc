// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::indexer_reader::IndexerReader;
use diesel::r2d2::R2D2Connection;
use jsonrpsee::{core::RpcResult, RpcModule};
use sui_json_rpc::SuiRpcModule;
use sui_json_rpc_api::{validate_limit, ExtendedApiServer, QUERY_MAX_RESULT_LIMIT_CHECKPOINTS};
use sui_json_rpc_types::{CheckpointedObjectID, EpochInfo, EpochPage, NetworkMetrics, Page, QueryObjectsPage, StakeRewardHistory, SuiMiningNFTList, SuiObjectResponseQuery, SuiOwnedTicketList};
use sui_json_rpc_types::{NetworkOverview, DaoProposalFilter, SuiDaoProposal, SuiOwnedMiningNFTFilter, ClassicPage, SuiOwnedMiningNFTOverview};
use sui_json_rpc_types::{SuiMiningNFT, SuiOwnedMiningNFTProfit, StakeMetrics, IndexedStake, SuiMiningNFTLiquidity, MoveCallMetrics, AddressMetrics, NFTStakingOverview};
use sui_open_rpc::Module;
use sui_types::sui_serde::BigInt;
use sui_types::base_types::{SequenceNumber, SuiAddress};

pub(crate) struct ExtendedApi<T: R2D2Connection + 'static> {
    inner: IndexerReader<T>,
}

impl<T: R2D2Connection> ExtendedApi<T> {
    pub fn new(inner: IndexerReader<T>) -> Self {
        Self { inner }
    }
}

#[async_trait::async_trait]
impl<T: R2D2Connection + 'static> ExtendedApiServer for ExtendedApi<T> {

    async fn get_epochs(
        &self,
        cursor: Option<BigInt<u64>>,
        limit: Option<usize>,
        descending_order: Option<bool>,
    ) -> RpcResult<EpochPage> {
        let limit = validate_limit(limit, QUERY_MAX_RESULT_LIMIT_CHECKPOINTS)?;
        let mut epochs = self
            .inner
            .spawn_blocking(move |this| {
                this.get_epochs(
                    cursor.map(|x| *x),
                    limit + 1,
                    descending_order.unwrap_or(false),
                )
            })
            .await?;

        let has_next_page = epochs.len() > limit;
        epochs.truncate(limit);
        let next_cursor = epochs.last().map(|e| e.epoch);
        Ok(Page {
            data: epochs,
            next_cursor: next_cursor.map(|id| id.into()),
            has_next_page,
        })
    }

    async fn get_current_epoch(&self) -> RpcResult<EpochInfo> {
        let stored_epoch = self
            .inner
            .spawn_blocking(|this| this.get_latest_epoch_info_from_db())
            .await?;
        EpochInfo::try_from(stored_epoch).map_err(Into::into)
    }

    async fn query_objects(
        &self,
        _query: SuiObjectResponseQuery,
        _cursor: Option<CheckpointedObjectID>,
        _limit: Option<usize>,
    ) -> RpcResult<QueryObjectsPage> {
        Err(jsonrpsee::types::error::CallError::Custom(
            jsonrpsee::types::error::ErrorCode::MethodNotFound.into(),
        )
        .into())
    }

    async fn get_total_transactions(&self) -> RpcResult<BigInt<u64>> {
        let latest_checkpoint = self
            .inner
            .spawn_blocking(|this| this.get_latest_checkpoint())
            .await?;
        Ok(latest_checkpoint.network_total_transactions.into())
    }

    async fn get_network_metrics(&self) -> RpcResult<NetworkMetrics> {
        todo!()
    }

    async fn get_network_overview(&self) -> RpcResult<NetworkOverview> {
        todo!()
    }

    async fn get_owned_mining_nfts(
        &self,
        _address: SuiAddress,
        _page: Option<usize>,
        _limit: Option<usize>,
        _filter: Option<SuiOwnedMiningNFTFilter>,
    ) -> RpcResult<ClassicPage<SuiMiningNFT>> {
        todo!()
    }

    async fn get_owned_mining_nft_overview(
        &self,
        _address: SuiAddress,
    ) -> RpcResult<SuiOwnedMiningNFTOverview> {
        todo!()
    }

    async fn get_owned_mining_nft_profits(
        &self,
        _address: SuiAddress,
        _limit: usize,
    ) -> RpcResult<Vec<SuiOwnedMiningNFTProfit>> {
        todo!()
    }

    async fn get_dao_proposals(
        &self,
        _filter: Option<DaoProposalFilter>,
    ) -> RpcResult<Vec<SuiDaoProposal>> {
        todo!()
    }

    async fn get_stake_metrics(&self, _epoch: Option<SequenceNumber>) -> RpcResult<StakeMetrics> {
        todo!()
    }

    async fn get_indexed_stakes(&self, _owner: SuiAddress) -> RpcResult<Vec<IndexedStake>> {
        todo!()
    }

    async fn get_mining_nft_recent_liquidities(
        &self,
        _base_coin: String,
    ) -> RpcResult<Vec<SuiMiningNFTLiquidity>> {
        todo!()
    }

    async fn get_move_call_metrics(&self) -> RpcResult<MoveCallMetrics> {
        todo!()
    }

    async fn get_latest_address_metrics(&self) -> RpcResult<AddressMetrics> {
        todo!()
    }

    async fn get_checkpoint_address_metrics(&self, _checkpoint: u64) -> RpcResult<AddressMetrics> {
        todo!()
    }

    async fn get_all_epoch_address_metrics(
        &self,
        _descending_order: Option<bool>,
    ) -> RpcResult<Vec<AddressMetrics>> {
        todo!()
    }

    async fn get_nft_staking_overview(&self) -> RpcResult<NFTStakingOverview> {
        todo!()
    }

    async fn get_stake_reward_history(&self, _address: SuiAddress, _page: Option<usize>, _limit: Option<usize>) -> RpcResult<ClassicPage<StakeRewardHistory>> {
        todo!()
    }

    async fn get_owned_mining_nfts_idle(&self, _address: SuiAddress) -> RpcResult<SuiMiningNFTList> {
        todo!()
    }

    async fn get_owned_mining_nft_overview2(&self, _address: SuiAddress) -> RpcResult<SuiOwnedMiningNFTOverview> {
        todo!()
    }

    async fn get_owned_ticket_list(&self, _address: SuiAddress) -> RpcResult<SuiOwnedTicketList> {
        todo!()
    }

    async fn get_owned_ticket_list2(&self, _address: SuiAddress) -> RpcResult<SuiOwnedTicketList> {
        todo!()
    }

    async fn init_stake_reward(&self, _usd_rate: f64, _jpy_rate: f64, _epoch: u64, _first_epoch_end_ms: u64, _p: String) -> RpcResult<String> {
        todo!()
    }

    async fn init_nft(&self, _reward_per_power: u64) -> RpcResult<String> {
        todo!()
    }

    fn into_rpc(self) -> RpcModule<Self>
    where
    {
        todo!()
    }
}

impl<T: R2D2Connection> SuiRpcModule for ExtendedApi<T> {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        sui_json_rpc_api::ExtendedApiOpenRpc::module_doc()
    }
}
