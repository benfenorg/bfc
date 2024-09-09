// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;

use sui_json_rpc_types::{
    AddressMetrics, CheckpointedObjectID, ClassicPage, DaoProposalFilter, EpochInfo, EpochPage,
    IndexedStake, MoveCallMetrics, NFTStakingOverview, NetworkMetrics, NetworkOverview,
    QueryObjectsPage, StakeMetrics, SuiDaoProposal, SuiMiningNFT, SuiMiningNFTLiquidity,
    SuiObjectResponseQuery, SuiOwnedMiningNFTFilter, SuiOwnedMiningNFTOverview,
    SuiOwnedMiningNFTProfit, SuiOwnedTicketList, SuiMiningNFTList, StakeRewardHistory
};
use sui_open_rpc_macros::open_rpc;
use sui_types::{
    base_types::{SequenceNumber, SuiAddress},
    sui_serde::BigInt,
};

#[open_rpc(namespace = "bfcx", tag = "Extended API")]
#[rpc(server, client, namespace = "bfcx")]
pub trait ExtendedApi {
    /// Return a list of epoch info
    #[method(name = "getEpochs")]
    async fn get_epochs(
        &self,
        /// optional paging cursor
        cursor: Option<BigInt<u64>>,
        /// maximum number of items per page
        limit: Option<usize>,
        /// flag to return results in descending order
        descending_order: Option<bool>,
    ) -> RpcResult<EpochPage>;

    /// Return current epoch info
    #[method(name = "getCurrentEpoch")]
    async fn get_current_epoch(&self) -> RpcResult<EpochInfo>;

    /// Return the list of queried objects. Note that this is an enhanced full node only api.
    #[method(name = "queryObjects")]
    async fn query_objects(
        &self,
        /// the objects query criteria.
        query: SuiObjectResponseQuery,
        /// An optional paging cursor. If provided, the query will start from the next item after the specified cursor. Default to start from the first item if not specified.
        cursor: Option<CheckpointedObjectID>,
        /// Max number of items returned per page, default to [QUERY_MAX_RESULT_LIMIT] if not specified.
        limit: Option<usize>,
    ) -> RpcResult<QueryObjectsPage>;

    /// Return Network metrics
    #[method(name = "getNetworkMetrics")]
    async fn get_network_metrics(&self) -> RpcResult<NetworkMetrics>;

    /// Return Network overview
    #[method(name = "getNetworkOverview")]
    async fn get_network_overview(&self) -> RpcResult<NetworkOverview>;

    /// Return the list of dao proposals
    #[method(name = "getDaoProposals")]
    async fn get_dao_proposals(
        &self,
        filter: Option<DaoProposalFilter>,
    ) -> RpcResult<Vec<SuiDaoProposal>>;

    /// Return the stake metrics.
    #[method(name = "getStakeMetrics")]
    async fn get_stake_metrics(&self, epoch: Option<SequenceNumber>) -> RpcResult<StakeMetrics>;

    /// Return all [IndexedStake].
    #[method(name = "getIndexedStakes")]
    async fn get_indexed_stakes(&self, owner: SuiAddress) -> RpcResult<Vec<IndexedStake>>;

    /// Return Network metrics
    #[method(name = "getMoveCallMetrics")]
    async fn get_move_call_metrics(&self) -> RpcResult<MoveCallMetrics>;

    /// Address related metrics
    #[method(name = "getLatestAddressMetrics")]
    async fn get_latest_address_metrics(&self) -> RpcResult<AddressMetrics>;
    #[method(name = "getCheckpointAddressMetrics")]
    async fn get_checkpoint_address_metrics(&self, checkpoint: u64) -> RpcResult<AddressMetrics>;
    #[method(name = "getAllEpochAddressMetrics")]
    async fn get_all_epoch_address_metrics(
        &self,
        descending_order: Option<bool>,
    ) -> RpcResult<Vec<AddressMetrics>>;

    #[method(name = "getTotalTransactions")]
    async fn get_total_transactions(&self) -> RpcResult<BigInt<u64>>;

    #[method(name = "getNFTStakingOverview")]
    async fn get_nft_staking_overview(&self) -> RpcResult<NFTStakingOverview>;

    #[method(name = "getOwnedMiningNFTs")]
    async fn get_owned_mining_nfts(
        &self,
        address: SuiAddress,
        /// optional current page
        page: Option<usize>,
        /// maximum number of items per page
        limit: Option<usize>,
        filter: Option<SuiOwnedMiningNFTFilter>,
    ) -> RpcResult<ClassicPage<SuiMiningNFT>>;

    #[method(name = "getOwnedMiningNFTOverview")]
    async fn get_owned_mining_nft_overview(
        &self,
        address: SuiAddress,
    ) -> RpcResult<SuiOwnedMiningNFTOverview>;

    #[method(name = "getOwnedMiningNFTProfits")]
    async fn get_owned_mining_nft_profits(
        &self,
        address: SuiAddress,
        limit: usize,
    ) -> RpcResult<Vec<SuiOwnedMiningNFTProfit>>;

    #[method(name = "getMiningNFTRecentLiquidities")]
    async fn get_mining_nft_recent_liquidities(
        &self,
        base_coin: String,
    ) -> RpcResult<Vec<SuiMiningNFTLiquidity>>;

    #[method(name = "getStakeRewardHistory")]
    async fn get_stake_reward_history(
        &self,
        address: SuiAddress,
        /// optional current page
        page: Option<usize>,
        /// maximum number of items per page
        limit: Option<usize>
    ) -> RpcResult<ClassicPage<StakeRewardHistory>>;

    #[method(name = "getOwnedMiningNFTsIdel")]
    async fn get_owned_mining_nfts_idle(
        &self,
        address: SuiAddress,
    ) -> RpcResult<SuiMiningNFTList>;

    #[method(name = "getOwnedMiningNFTOverview2")]
    async fn get_owned_mining_nft_overview2(
        &self,
        address: SuiAddress,
    ) -> RpcResult<SuiOwnedMiningNFTOverview>;

    #[method(name = "getOwnedTicketList")]
    async fn get_owned_ticket_list(
        &self,
        address: SuiAddress,
    ) -> RpcResult<SuiOwnedTicketList>;

    #[method(name = "getOwnedTicketList2")]
    async fn get_owned_ticket_list2(
        &self,
        address: SuiAddress,
    ) -> RpcResult<SuiOwnedTicketList>;

    #[method(name = "initStakeReward")]
    async fn init_stake_reward(&self, usd_rate: f64, jpy_rate: f64, epoch: u64, first_epoch_end_ms: u64, p: String) -> RpcResult<String>;

    #[method(name = "initNft")]
    async fn init_nft(&self, reward_per_power: u64) -> RpcResult<String>;
}
