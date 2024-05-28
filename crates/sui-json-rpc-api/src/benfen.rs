// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;

use sui_json_rpc_types::{
    AddressMetrics, ClassicPage, DaoProposalFilter, MoveCallMetrics, NFTStakingOverview,
    NetworkMetrics, NetworkOverview, SuiDaoProposal, SuiMiningNFT, SuiOwnedMiningNFTFilter,
    SuiOwnedMiningNFTOverview, SuiOwnedMiningNFTProfit,
};
use sui_open_rpc_macros::open_rpc;
use sui_types::base_types::SuiAddress;

#[open_rpc(namespace = "bfcx", tag = "Benfen API")]
#[rpc(server, client, namespace = "bfcx")]
pub trait BenfenApi {
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
}
