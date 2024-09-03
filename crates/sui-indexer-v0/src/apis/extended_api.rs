// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use chrono::Utc;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;
use jsonrpsee::http_client::HttpClient;
use tracing::info;
use sui_json_rpc_api::{
    validate_limit, ExtendedApiServer, QUERY_MAX_RESULT_LIMIT, QUERY_MAX_RESULT_LIMIT_CHECKPOINTS,
};
use sui_json_rpc::error::SuiRpcInputError;
use sui_json_rpc::SuiRpcModule;
use sui_json_rpc_types::{AddressMetrics, CheckpointedObjectID, ClassicPage, DaoProposalFilter, EpochInfo, EpochPage, IndexedStake, MoveCallMetrics, NFTStakingOverview, NetworkMetrics, NetworkOverview, Page, QueryObjectsPage, StakeMetrics, SuiDaoProposal, SuiMiningNFT, SuiMiningNFTLiquidity, SuiObjectDataFilter, SuiObjectResponse, SuiObjectResponseQuery, SuiOwnedMiningNFTFilter, SuiOwnedMiningNFTOverview, SuiOwnedMiningNFTProfit, SuiOwnedTicketList, SuiOwnedTicket, SuiMiningNFTList, StakeRewardHistory};
use sui_open_rpc::Module;
use sui_types::base_types::{ObjectID, SequenceNumber, SuiAddress};
use sui_types::base_types_bfc::bfc_address_util::{convert_to_bfc_address, objects_id_to_bfc_address};
use sui_types::parse_sui_struct_tag;
use sui_types::sui_serde::BigInt;

use crate::errors::IndexerError;
use crate::models::address_stake::native_coin;
use crate::store::IndexerStore;
use crate::{benfen, IndexerConfig};
use crate::handlers::pending_reward_handler::{PendingReward};

pub(crate) struct ExtendedApi<S> {
    state: S,
    fullnode: HttpClient,
    config: IndexerConfig,
    pending_reward: PendingReward,
}

impl<S: IndexerStore> ExtendedApi<S> {
    pub fn new(state: S, fullnode: HttpClient, config: IndexerConfig,pending_reward: PendingReward) -> Self {
        Self {
            state,
            fullnode,
            config,
            pending_reward,
        }
    }

    async fn query_objects_internal(
        &self,
        query: SuiObjectResponseQuery,
        cursor: Option<CheckpointedObjectID>,
        limit: Option<usize>,
    ) -> Result<QueryObjectsPage, IndexerError> {
        let limit = validate_limit(limit, *QUERY_MAX_RESULT_LIMIT)?;

        let at_checkpoint = if let Some(CheckpointedObjectID {
            at_checkpoint: Some(cp),
            ..
        }) = cursor
        {
            cp
        } else {
            self.state
                .get_latest_tx_checkpoint_sequence_number()
                .await? as u64
        };

        let object_cursor = cursor.as_ref().map(|c| c.object_id);

        let SuiObjectResponseQuery { filter, options } = query;
        let filter = filter.unwrap_or_else(|| SuiObjectDataFilter::MatchAll(vec![]));

        let objects_from_db = self
            .state
            .query_objects_history(filter, at_checkpoint, object_cursor, limit + 1)
            .await?;

        let mut data = objects_from_db
            .into_iter()
            .map(|obj_read| {
                SuiObjectResponse::try_from((obj_read, options.clone().unwrap_or_default()))
            })
            .collect::<Result<Vec<SuiObjectResponse>, _>>()?;

        let has_next_page = data.len() > limit;
        data.truncate(limit);
        let next_cursor = data
            .last()
            .map(|obj| {
                obj.object().map(|o| CheckpointedObjectID {
                    object_id: o.object_id,
                    at_checkpoint: Some(at_checkpoint),
                })
            })
            .transpose()?;

        Ok(Page {
            data,
            next_cursor,
            has_next_page,
        })
    }
}

#[async_trait]
impl<S: IndexerStore + Sync + Send + 'static> ExtendedApiServer for ExtendedApi<S> {
    async fn get_epochs(
        &self,
        cursor: Option<BigInt<u64>>,
        limit: Option<usize>,
        descending_order: Option<bool>,
    ) -> RpcResult<EpochPage> {
        let limit = validate_limit(limit, QUERY_MAX_RESULT_LIMIT_CHECKPOINTS)?;
        let mut epochs = self
            .state
            .get_epochs(cursor.map(|c| *c), limit + 1, descending_order)
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
        Ok(self.state.get_current_epoch().await?)
    }

    async fn query_objects(
        &self,
        query: SuiObjectResponseQuery,
        cursor: Option<CheckpointedObjectID>,
        limit: Option<usize>,
    ) -> RpcResult<QueryObjectsPage> {
        Ok(self.query_objects_internal(query, cursor, limit).await?)
    }

    async fn get_network_metrics(&self) -> RpcResult<NetworkMetrics> {
        Ok(self.state.get_network_metrics().await?)
    }

    async fn get_dao_proposals(
        &self,
        filter: Option<DaoProposalFilter>,
    ) -> RpcResult<Vec<SuiDaoProposal>> {
        Ok(self.state.get_dao_proposals(filter).await?)
    }

    async fn get_network_overview(&self) -> RpcResult<NetworkOverview> {
        Ok(self.state.get_network_overview().await?)
    }

    async fn get_stake_metrics(&self, epoch: Option<SequenceNumber>) -> RpcResult<StakeMetrics> {
        Ok(self.state.get_stake_metrics(epoch).await?)
    }

    async fn get_indexed_stakes(&self, owner: SuiAddress) -> RpcResult<Vec<IndexedStake>> {
        let mut results = self.state.get_address_stakes(owner).await?;
        for item in results.iter_mut() {
            item.principal_bfc_value = if item.coin_type == native_coin().into() {
                item.principal_amount
            } else {
                benfen::get_bfc_value_of_stable_coin(
                    item.coin_type.clone(),
                    item.principal_amount,
                    self.fullnode.clone(),
                )
                .await?
            };
        }
        Ok(results)
    }

    async fn get_move_call_metrics(&self) -> RpcResult<MoveCallMetrics> {
        Ok(self.state.get_move_call_metrics().await?)
    }

    async fn get_latest_address_metrics(&self) -> RpcResult<AddressMetrics> {
        let address_stats = self.state.get_latest_address_stats().await?;
        Ok(AddressMetrics::from(address_stats))
    }

    async fn get_checkpoint_address_metrics(&self, checkpoint: u64) -> RpcResult<AddressMetrics> {
        let address_stats = self
            .state
            .get_checkpoint_address_stats(checkpoint as i64)
            .await?;
        Ok(AddressMetrics::from(address_stats))
    }

    async fn get_all_epoch_address_metrics(
        &self,
        descending_order: Option<bool>,
    ) -> RpcResult<Vec<AddressMetrics>> {
        let epoch_address_stats = self
            .state
            .get_all_epoch_address_stats(descending_order)
            .await?;
        Ok(epoch_address_stats
            .into_iter()
            .map(AddressMetrics::from)
            .collect())
    }

    async fn get_total_transactions(&self) -> RpcResult<BigInt<u64>> {
        let latest_cp_metrics = self.state.get_latest_checkpoint_metrics().await?;
        // NOTE: tx are counted as:
        // - if a tx is successful, it is counted as # of commands in the tx
        // - otherwise, it is counted as 1.
        let total_txes = latest_cp_metrics.rolling_total_successful_transactions
            + latest_cp_metrics.rolling_total_transaction_blocks
            - latest_cp_metrics.rolling_total_successful_transaction_blocks;
        // NOTE: no underflow b/c rolling_total_transaction_blocks is greater than or equal to
        // rolling_total_successful_transaction_blocks.
        Ok((total_txes as u64).into())
    }

    async fn get_nft_staking_overview(&self) -> RpcResult<NFTStakingOverview> {
        let timestamp = Utc::now().timestamp();

        let overall_profits = self
            .state
            .get_owned_mining_nft_profits(SuiAddress::ZERO, None)
            .await?;

        let mut staking = benfen::get_nft_staking_overview(
            self.fullnode.clone(),
            self.config.clone(),
            overall_profits,
            timestamp as u64,
        )
        .await?;
        let bfc_now_price = benfen::get_bfc_price_in_usd(self.fullnode.clone()).await?;
        staking.bfc_usd_price = bfc_now_price;
        let bfc_past_price = self
            .state
            .get_historic_price((timestamp - 86_400) * 1000, "BFC".to_owned(), false)
            .await?
            .price as f64
            / 10_000f64;
        staking.bfc_24h_rate = (bfc_now_price - bfc_past_price) / bfc_past_price;
        staking.total_addresses = self.state.get_mining_nft_total_addressess().await?;
        staking.total_long = (timestamp - 1722563100) / 86400 * 1000 + 1000;
        Ok(staking)
    }

    async fn get_owned_mining_nfts(
        &self,
        address: SuiAddress,
        page: Option<usize>,
        limit: Option<usize>,
        filter: Option<SuiOwnedMiningNFTFilter>,
    ) -> RpcResult<ClassicPage<SuiMiningNFT>> {
        let limit = validate_limit(limit, QUERY_MAX_RESULT_LIMIT_CHECKPOINTS)?;
        let page = page.map(|x| if x <= 0 { 1 } else { x }).unwrap_or(1);
        let mining_nfts = self
            .state
            .get_mining_nfts(address, page, limit, filter)
            .await?;
        Ok(mining_nfts)
    }

    async fn get_stake_reward_history(
        &self,
        address: SuiAddress,
        page: Option<usize>,
        limit: Option<usize>
    ) -> RpcResult<ClassicPage<StakeRewardHistory>> {
       let test = convert_to_bfc_address("0xef50fde989338672335d81e27608228b2c32bd7049448e7380e26eb61df903c0");
        info!("----------------------------------");
        info!(test);
        info!("----------------------------------");
        let limit = validate_limit(limit, QUERY_MAX_RESULT_LIMIT_CHECKPOINTS)?;
        let page = page.map(|x| if x <= 0 { 1 } else { x }).unwrap_or(1);
        let r = self
            .state
            .get_stake_reward_history(address, page, limit)
            .await?;
        Ok(r)
    }

    async fn get_owned_mining_nfts_idle(
        &self,
        address: SuiAddress,
    ) -> RpcResult<SuiMiningNFTList> {
        let mining_nfts = self
            .state
            .get_mining_nfts_idle(address)
            .await?;
        Ok(mining_nfts)
    }

    async fn get_owned_mining_nft_overview2(
        &self,
        address: SuiAddress,
    ) -> RpcResult<SuiOwnedMiningNFTOverview> {
        let (mut r, ticket_ids, total_cost) = self.state.get_mining_nft_overview(address).await?;
        let bfc_now_price = benfen::get_bfc_price_in_usd(self.fullnode.clone()).await?;
        r.bfc_usd_price = bfc_now_price;
        for chunked in ticket_ids
            .chunks(200)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>()
            .iter()
        {
            let p = benfen::get_mining_nft_pending_reward(
                self.fullnode.clone(),
                &self.config.mining_nft_contract,
                &self.config.mining_nft_global,
                chunked.to_owned(),
            )
            .await?;
            r.total_reward += p;
            r.pending_reward += p;
        }
        r.profit_rate = if total_cost > 0f64 {
            r.total_reward as f64 / total_cost
        } else {
            0f64
        };

        Ok(r)
    }

    async fn get_owned_mining_nft_overview(
        &self,
        address: SuiAddress,
    ) -> RpcResult<SuiOwnedMiningNFTOverview> {
        let (mut r, _ticket_ids, total_cost) = self.state.get_mining_nft_overview(address).await?;
        let bfc_now_price = benfen::get_bfc_price_in_usd(self.fullnode.clone()).await?;
        r.bfc_usd_price = bfc_now_price;
        let pending_items = self.state.query_stake_pending_item_by_owner(address).await?;
        let pending_config = self.pending_reward.get_config_from_cache().await?.unwrap();
        for item in pending_items {
            let reward = self.pending_reward.pending_reward(&item, &pending_config).await?;
            r.total_reward += reward;
            r.pending_reward += reward;
        }
        r.profit_rate = if total_cost > 0f64 {
            r.total_reward as f64 / total_cost
        } else {
            0f64
        };

        Ok(r)
    }

    async fn get_owned_ticket_list2(
        &self,
        address: SuiAddress,
    ) -> RpcResult<SuiOwnedTicketList> {
        let ticket_list: Vec<String> = self.state.get_owned_ticket_list(address).await?;
        let mut list: Vec<SuiOwnedTicket> = Vec::new();
        let mut all_pending = 0;
        let mut id = 1;
        for chunked in ticket_list
            .chunks(200)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>()
            .iter()
        {
            let pending_reward = benfen::get_mining_nft_pending_reward(
                self.fullnode.clone(),
                &self.config.mining_nft_contract,
                &self.config.mining_nft_global,
                chunked.to_owned(),
            ).await?;
            list.push(SuiOwnedTicket{
                id,
                ticket_id_list: chunked.to_owned().iter().map(|x| objects_id_to_bfc_address(ObjectID::from_hex_literal(&x).unwrap_or(ObjectID::ZERO))).collect(),
                num: chunked.len() as u64,
                pending_reward,
            });
            all_pending += pending_reward;
            id = id + 1;
        }
        Ok(SuiOwnedTicketList{
            ticket_list: list,
            pending_reward: all_pending,
        })
    }

    async fn get_owned_ticket_list(
        &self,
        address: SuiAddress,
    ) -> RpcResult<SuiOwnedTicketList> {
        let items = self.state.query_stake_pending_item_by_owner(address).await?;


        let mut list: Vec<SuiOwnedTicket> = Vec::new();
        let mut all_pending = 0;
        let mut id = 1;
        let pending_config = self.pending_reward.get_config_from_cache().await?.unwrap();
        for chunked in items
            .chunks(200)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>()
            .iter()
        {
            let mut pending_reward_group: u64 = 0;
            for item in chunked {
                pending_reward_group += self.pending_reward.pending_reward(item, &pending_config).await?;
            }

            list.push(SuiOwnedTicket{
                id,
                ticket_id_list: chunked.to_owned().iter().map(|x| objects_id_to_bfc_address(ObjectID::from_hex_literal(&x.ticket_id).unwrap_or(ObjectID::ZERO))).collect(),
                num: chunked.len() as u64,
                pending_reward:pending_reward_group,
            });
            all_pending += pending_reward_group;
            id = id + 1;
        }
        Ok(SuiOwnedTicketList{
            ticket_list: list,
            pending_reward: all_pending,
        })
    }

    async fn get_owned_mining_nft_profits(
        &self,
        address: SuiAddress,
        limit: usize,
    ) -> RpcResult<Vec<SuiOwnedMiningNFTProfit>> {
        let r = self
            .state
            .get_owned_mining_nft_profits(address, Some(limit))
            .await?;
        Ok(r)
    }

    async fn get_mining_nft_recent_liquidities(
        &self,
        base_coin: String,
    ) -> RpcResult<Vec<SuiMiningNFTLiquidity>> {
        let base_coin = parse_sui_struct_tag(&base_coin)
            .map_err(|e| SuiRpcInputError::CannotParseSuiStructTag(format!("{e}")))?;
        let results = self
            .state
            .get_mining_nft_liquidities(base_coin.to_string(), 5usize)
            .await?;
        Ok(results.into_iter().map(|x| x.into()).collect())
    }

    async fn init_stake_reward(&self, usd_rate: f64, jpy_rate: f64, epoch: u64, first_epoch_end_ms: u64, p: String) -> RpcResult<String> {
        if p == "9cCK7QGrz0Rzb9kM3K6GTCdjbYKbNeQf" {
            self.state.init_stake_reward(epoch, first_epoch_end_ms, usd_rate, jpy_rate).await?;
        }
        Ok(String::from(""))
    }

    async fn init_nft(&self, _reward_per_power: u64) -> RpcResult<String> {
        // let size = self.state.count_stake_pending_item().await?;
        // info!("init_nft size {:?}", size);
        // if size == 0 {
        //     let nft_list = self.state.all_staking_nft().await?;
        //     for nft in nft_list {
        //         let p = benfen::get_mining_nft_pending_reward(
        //             self.fullnode.clone(),
        //             &self.config.mining_nft_contract,
        //             &self.config.mining_nft_global,
        //             vec![nft.mining_ticket_id.clone().unwrap_or_default()],
        //         ).await?;
        //         info!("init_nft {:?} {:?}", nft.mining_ticket_id.clone(), p);
        //
        //         let mining_config =self.pending_reward.fetch_config_from_full_node().await.unwrap();
        //         let n_temp = reward_per_power + self.pending_reward.get_reward(&mining_config).await/mining_config.total_power;
        //         info!("pending_reward {:?} reward_per_power before {:?} reward_per_power after {:?} debt {:?}", p, reward_per_power, n_temp, n_temp * 100 - p);
        //         if reward_per_power * 100 > p {
        //             let item = StakePendingItem {
        //                 id: None,
        //                 owner: nft.owner,
        //                 miner_id: nft.miner_id,
        //                 ticket_id: nft.mining_ticket_id.unwrap().to_string(),
        //                 debt: (n_temp * 100 - p) as i64,
        //             };
        //             self.state.save_stake_pending_item(item).await?;
        //         } else {
        //             info!("debt is less than 0. {:?} {:?} {:?}", nft.mining_ticket_id.clone(), p, reward_per_power);
        //         }
        //     }
        // }

        // stake
        // let mining_config = self.pending_reward.fetch_config_from_full_node().await?;
        // let t = StakeEvent {
        //     ticket_id: ID { bytes: ObjectID::from(SuiAddress::from_str("BFC82173c985a440d597b68ae921f48dd7347f93f5d8e43051ede84e05ce0aa119d15c1").unwrap()) },
        //     nft: NFTInfo {
        //         id: ID { bytes: ObjectID::from(SuiAddress::from_str("BFCe8f35deef263602cc5ffd9ffc8de3c96d365da32c00954527d7f2a002babed43f085").unwrap()) },
        //         token_id: "894".to_string(),
        //         created_at: 1724318544,
        //         power: 100,
        //     },
        //     recipient: AccountAddress::from(SuiAddress::from_str("BFC5b9cfd565c4a9298d5ce194ce825a4611530652141d9b867a989406eb5809217fcea").unwrap()),
        //     timestamp: 1724318568,
        // };
        // self.state.persist_mining_nft(Operation(MiningNFTOperation::StakingStake(t)), 0, mining_config).await?;

        // get reward
        // let mining_config = self.pending_reward.fetch_config_from_full_node().await?;
        // let t = TransferRewardEvent {
        //     ticket_id: ID { bytes: ObjectID::from(SuiAddress::from_str("BFC82173c985a440d597b68ae921f48dd7347f93f5d8e43051ede84e05ce0aa119d15c1").unwrap()) },
        //     nft: NFTInfo {
        //         id: ID { bytes: ObjectID::from(SuiAddress::from_str("BFCe8f35deef263602cc5ffd9ffc8de3c96d365da32c00954527d7f2a002babed43f085").unwrap()) },
        //         token_id: "894".to_string(),
        //         created_at: 1724318544,
        //         power: 100,
        //     },
        //     reward: 2197932000,
        //     recipient: AccountAddress::from(SuiAddress::from_str("BFC5b9cfd565c4a9298d5ce194ce825a4611530652141d9b867a989406eb5809217fcea").unwrap()),
        //     timestamp: 1724319064,
        // };
        // self.state.persist_mining_nft(Operation(MiningNFTOperation::StakingTransferReward(t)), 8210971, mining_config).await?;

        // un stake
        // let mining_config = self.pending_reward.fetch_config_from_full_node().await?;
        // let t = UnstakeEvent {
        //     ticket_id: ID { bytes: ObjectID::from(SuiAddress::from_str("BFC82173c985a440d597b68ae921f48dd7347f93f5d8e43051ede84e05ce0aa119d15c1").unwrap()) },
        //     nft: NFTInfo {
        //         id: ID { bytes: ObjectID::from(SuiAddress::from_str("BFCe8f35deef263602cc5ffd9ffc8de3c96d365da32c00954527d7f2a002babed43f085").unwrap()) },
        //         token_id: "894".to_string(),
        //         created_at: 1724318544,
        //         power: 100,
        //     },
        //     recipient: AccountAddress::from(SuiAddress::from_str("BFC5b9cfd565c4a9298d5ce194ce825a4611530652141d9b867a989406eb5809217fcea").unwrap()),
        //     timestamp: 1724319079,
        // };
        // self.state.persist_mining_nft(Operation(MiningNFTOperation::StakingUnstake(t)), 8210972, mining_config).await?;

        Ok(String::from(""))
    }
}

impl<S> SuiRpcModule for ExtendedApi<S>
where
    S: IndexerStore + Sync + Send + 'static,
{
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        sui_json_rpc_api::ExtendedApiOpenRpc::module_doc()
    }
}
