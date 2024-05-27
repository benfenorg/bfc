// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::RpcModule;

use sui_json_rpc::api::{GovernanceReadApiClient, GovernanceReadApiServer};
use sui_json_rpc::SuiRpcModule;
use sui_json_rpc_types::SuiCommittee;
use sui_json_rpc_types::{DelegatedStake, ValidatorApys};
use sui_open_rpc::Module;
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::dynamic_field::DynamicFieldInfo;
use sui_types::sui_serde::BigInt;
use sui_types::sui_system_state::sui_system_state_summary::SuiSystemStateSummary;
use sui_types::proposal::Proposal;

pub(crate) struct GovernanceReadApi {
    fullnode: HttpClient,
}

impl GovernanceReadApi {
    pub fn new(fullnode_client: HttpClient) -> Self {
        Self {
            fullnode: fullnode_client,
        }
    }
}

#[async_trait]
impl GovernanceReadApiServer for GovernanceReadApi {
    async fn get_stakes_by_ids(
        &self,
        staked_sui_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>> {
        self.fullnode.get_stakes_by_ids(staked_sui_ids).await
    }
    async fn get_stakes(&self, owner: SuiAddress) -> RpcResult<Vec<DelegatedStake>> {
        self.fullnode.get_stakes(owner).await
    }

    async fn get_committee_info(&self, epoch: Option<BigInt<u64>>) -> RpcResult<SuiCommittee> {
        self.fullnode.get_committee_info(epoch).await
    }

    async fn get_latest_sui_system_state(&self) -> RpcResult<SuiSystemStateSummary> {
        self.fullnode.get_latest_sui_system_state().await
    }

    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>> {
        self.fullnode.get_reference_gas_price().await
    }

    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys> {
        self.fullnode.get_validators_apy().await
    }

    async fn get_proposal(&self, owner: SuiAddress) -> RpcResult<Proposal> {
        self.fullnode.get_proposal(owner).await
    }
    async fn get_stable_pools(&self, owner: SuiAddress) -> RpcResult<Vec<(ObjectID, DynamicFieldInfo)>> {
        self.fullnode.get_stable_pools(owner).await
    }
    async fn get_stable_rate(&self, tag: String) -> RpcResult<BigInt<u64>> {
        self.fullnode.get_stable_rate(tag).await
    }
}

impl SuiRpcModule for GovernanceReadApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        sui_json_rpc::api::GovernanceReadApiOpenRpc::module_doc()
    }
}
