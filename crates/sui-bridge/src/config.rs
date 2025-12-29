// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::abi::EthBridgeConfig;
use crate::crypto::BridgeAuthorityKeyPair;
use crate::error::BridgeError;
use crate::eth_client::EthClient;
use crate::metered_eth_provider::new_metered_eth_provider;
use crate::metered_eth_provider::MeteredEthHttpProvier;
use crate::metrics::BridgeMetrics;
use crate::sui_client::SuiClient;
use crate::types::{is_route_valid, BridgeAction};
use crate::utils::get_eth_contract_addresses;
use anyhow::anyhow;
use ethers::providers::Middleware;
use ethers::types::Address as EthAddress;
use futures::{future, StreamExt};
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use url::Url;
use solana_sdk::pubkey::Pubkey;
use sui_config::Config;
use sui_json_rpc_types::Coin;
use sui_keys::keypair_file::read_key;
use sui_sdk::apis::CoinReadApi;
use sui_sdk::{SuiClient as SuiSdkClient, SuiClientBuilder};
use sui_types::base_types::ObjectRef;
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::bridge::BridgeChainId;
use sui_types::crypto::KeypairTraits;
use sui_types::crypto::{get_key_pair_from_rng, NetworkKeyPair, SuiKeyPair};
use sui_types::digests::{get_mainnet_chain_identifier, get_testnet_chain_identifier};
use sui_types::event::EventID;
use sui_types::object::Owner;
use tracing::info;

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EthConfig {
    /// Rpc url for Eth fullnode, used for query stuff.
    pub eth_rpc_url: String,
    /// The proxy address of SuiBridge
    pub eth_bridge_proxy_address: String,
    /// The expected BridgeChainId on Eth side.
    pub eth_bridge_chain_id: u8,
    /// The starting block for EthSyncer to monitor eth contracts.
    /// It is required when `run_client` is true. Usually this is
    /// the block number when the bridge contracts are deployed.
    /// When BridgeNode starts, it reads the contract watermark from storage.
    /// If the watermark is not found, it will start from this fallback block number.
    /// If the watermark is found, it will start from the watermark.
    /// this v.s.`eth_contracts_start_block_override`:
    pub eth_contracts_start_block_fallback: Option<u64>,
    /// The starting block for EthSyncer to monitor eth contracts. It overrides
    /// the watermark in storage. This is useful when we want to reprocess the events
    /// from a specific block number.
    /// Note: this field has to be reset after starting the BridgeNode, otherwise it will
    /// reprocess the events from this block number every time it starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eth_contracts_start_block_override: Option<u64>,
    /// latest 快速到账阈值
    pub latest_fast_path_threshold: Option<u64>,
    /// safe 快速到账阈值
    pub safe_fast_path_threshold: Option<u64>,
    /// 是否开启快速到账
    pub enable_fast_path_latest: bool,
    pub enable_fast_path_safe: bool,
    pub enable_fast_path_finalized: bool,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SuiConfig {
    /// Rpc url for Sui fullnode, used for query stuff and submit transactions.
    pub sui_rpc_url: String,
    /// The expected BridgeChainId on Sui side.
    pub sui_bridge_chain_id: u8,
    /// Path of the file where bridge client key (any SuiKeyPair) is stored.
    /// If `run_client` is true, and this is None, then use `bridge_authority_key_path` as client key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_client_key_path: Option<PathBuf>,
    /// The gas object to use for paying for gas fees for the client. It needs to
    /// be owned by the address associated with bridge client key. If not set
    /// and `run_client` is true, it will query and use the gas object with highest
    /// amount for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_client_gas_object: Option<ObjectID>,
    /// Override the last processed EventID for bridge module `bridge`.
    /// When set, SuiSyncer will start from this cursor (exclusively) instead of the one in storage.
    /// If the cursor is not found in storage or override, the query will start from genesis.
    /// Key: sui module, Value: last processed EventID (tx_digest, event_seq).
    /// Note 1: This field should be rarely used. Only use it when you understand how to follow up.
    /// Note 2: the EventID needs to be valid, namely it must exist and matches the filter.
    /// Otherwise, it will miss one event because of fullnode Event query semantics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sui_bridge_module_last_processed_event_id_override: Option<EventID>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BridgeNodeConfig {
    /// The port that the server listens on.
    pub server_listen_port: u16,
    /// The port that for metrics server.
    pub metrics_port: u16,
    /// Path of the file where bridge authority key (Secp256k1) is stored.
    pub bridge_authority_key_path: PathBuf,
    /// Whether to run client. If true, `sui.bridge_client_key_path`
    /// and `db_path` needs to be provided.
    pub run_client: bool,
    /// Path of the client storage. Required when `run_client` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_path: Option<PathBuf>,
    /// A list of approved governance actions. Action in this list will be signed when requested by client.
    pub approved_governance_actions: Vec<BridgeAction>,
    /// Sui configuration
    pub sui: SuiConfig,
    /// Eth configuration
    pub eth: EthConfig,
    /// List of evm
    pub evm: Vec<EthConfig>,
    /// AML key used for AML checking
    pub aml_key: String,
    /// Network key used for metrics pushing
    #[serde(default = "default_ed25519_key_pair")]
    pub metrics_key_pair: NetworkKeyPair,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<MetricsConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchdog_config: Option<WatchdogConfig>,
    pub solana: SolanaConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit_db_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_rpc: Option<ExternalChainRpcConfig>,
}

pub fn default_ed25519_key_pair() -> NetworkKeyPair {
    get_key_pair_from_rng(&mut rand::rngs::OsRng).1
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetricsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_interval_seconds: Option<u64>,
    pub push_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct WatchdogConfig {
    /// Total supplies to watch on Sui. Mapping from coin name to coin type tag
    pub total_supplies: BTreeMap<String, String>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SolanaConfig {
    pub getblock_base_url: String,
    pub bridge_proxy_address: String,
    pub bridge_chain_id: u8,
    pub contracts_start_slot_fallback: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts_start_slot_override: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ExternalChainRpcConfig {
    pub solana: ChainRpcUrls,
    pub tron: ChainRpcUrls,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ChainRpcUrls {
    pub mainnet_url: String,
    pub testnet_url: String,
}

impl Config for BridgeNodeConfig {}

impl BridgeNodeConfig {
    pub async fn validate(
        &self,
        metrics: Arc<BridgeMetrics>,
    ) -> anyhow::Result<(BridgeServerConfig, Option<BridgeClientConfig>)> {
        if !is_route_valid(
            BridgeChainId::try_from(self.sui.sui_bridge_chain_id)?,
            BridgeChainId::try_from(self.eth.eth_bridge_chain_id)?,
        ) {
            return Err(anyhow!(
                "Route between Sui chain id {} and Eth chain id {} is not valid",
                self.sui.sui_bridge_chain_id,
                self.eth.eth_bridge_chain_id,
            ));
        };

        let bridge_authority_key = match read_key(&self.bridge_authority_key_path, true)? {
            SuiKeyPair::Secp256k1(key) => key,
            _ => unreachable!("we required secp256k1 key in `read_key`"),
        };

        // we do this check here instead of `prepare_for_sui` below because
        // that is only called when `run_client` is true.
        let sui_client =
            Arc::new(SuiClient::<SuiSdkClient>::new(&self.sui.sui_rpc_url, metrics.clone()).await?);
        let bridge_committee = sui_client
            .get_bridge_committee()
            .await
            .map_err(|e| anyhow!("Error getting bridge committee: {:?}", e))?;
        if !bridge_committee.is_active_member(&bridge_authority_key.public().into()) {
            return Err(anyhow!(
                "Bridge authority key is not part of bridge committee"
            ));
        }

        let (eth_client, eth_contracts) = self.prepare_for_eth(metrics.clone()).await?;
        let (mut evm_clients, mut evm_contracts) = self.prepare_for_evm(metrics.clone()).await?;
        if BridgeChainId::is_custom_chain_by_id(self.eth.eth_bridge_chain_id) {
            // Custom chains use eth config from BridgeNodeConfig
            // We need to insert the eth client and contracts for custom chain
            // so that it can be used in the bridge server.
            evm_clients.insert(
                BridgeChainId::BscCustom,
                eth_client.clone(),
            );
            evm_contracts.insert(
                BridgeChainId::BscCustom,
                eth_contracts.clone(),
            );
        }

        // Validate external_rpc configuration
        let external_rpc = self.external_rpc.as_ref().ok_or_else(|| {
            anyhow!(
                "external_rpc configuration is required but not found in config file. \
                Please add the following section to your config:\n\
                external-rpc:\n\
                  solana:\n\
                    mainnet-url: \"https://your_solana_mainnet_rpc_url\"\n\
                    testnet-url: \"https://your_solana_testnet_rpc_url\"\n\
                  tron:\n\
                    mainnet-url: \"https://your_tron_mainnet_rpc_url\"\n\
                    testnet-url: \"https://your_tron_testnet_rpc_url\""
            )
        })?;

        // Validate solana configuration
        if self.solana.getblock_base_url.trim().is_empty() {
            anyhow::bail!("solana.getblock_base_url must be set to a valid http(s) url");
        }
        let parsed = Url::parse(&self.solana.getblock_base_url)
            .map_err(|e| anyhow!("Invalid solana.getblock_base_url: {}", e))?;
        if parsed.scheme() != "http" && parsed.scheme() != "https" {
            anyhow::bail!("solana.getblock_base_url must use http or https scheme");
        }
        if self.run_client && self.solana.contracts_start_slot_fallback.is_none() {
            anyhow::bail!("solana.contracts_start_slot_fallback is required when run_client is true");
        }
        BridgeChainId::try_from(self.solana.bridge_chain_id)
            .map_err(|_| anyhow!("Invalid solana.bridge_chain_id: {}", self.solana.bridge_chain_id))?;
        if self.solana.bridge_proxy_address.trim().is_empty() {
            anyhow::bail!("solana.bridge_proxy_address must be set");
        }
        Pubkey::from_str(&self.solana.bridge_proxy_address)
            .map_err(|_| anyhow!("solana.bridge_proxy_address must be a valid Solana address (base58)"))?;

        let bridge_summary = sui_client
            .get_bridge_summary()
            .await
            .map_err(|e| anyhow!("Error getting bridge summary: {:?}", e))?;
        if bridge_summary.chain_id != self.sui.sui_bridge_chain_id {
            anyhow::bail!(
                "Bridge chain id mismatch: expected {}, but connected to {}",
                self.sui.sui_bridge_chain_id,
                bridge_summary.chain_id
            );
        }

        // Validate approved actions that must be governace actions
        for action in &self.approved_governance_actions {
            if !action.is_governace_action() {
                anyhow::bail!(format!(
                    "{:?}",
                    BridgeError::ActionIsNotGovernanceAction(action.clone())
                ));
            }
        }
        let approved_governance_actions = self.approved_governance_actions.clone();

        let bridge_server_config = BridgeServerConfig {
            key: bridge_authority_key,
            metrics_port: self.metrics_port,
            eth_bridge_proxy_address: eth_contracts[0], // the first contract is bridge proxy
            server_listen_port: self.server_listen_port,
            sui_client: sui_client.clone(),
            eth_client: eth_client.clone(),
            evm_clients: evm_clients.clone(),
            approved_governance_actions,
            external_rpc: Some(external_rpc.clone()),
        };
        // if !self.run_client {
        //     return Ok((bridge_server_config, None));
        // }

        // If client is enabled, prepare client config
        let (bridge_client_key, client_sui_address, gas_object_ref) =
            self.prepare_for_sui(sui_client.clone(), metrics).await?;

        let db_path = self
            .db_path
            .clone()
            .ok_or(anyhow!("`db_path` is required when `run_client` is true"))?;

        let mut evm_client_configs: BTreeMap<BridgeChainId, BridgeClientEvmConfig> = BTreeMap::new();
        for evm_config in &self.evm {
            info!("chain_id: {}, read evm_config: {:#?}", evm_config.eth_bridge_chain_id, evm_config);

            let chain_id = BridgeChainId::try_from(evm_config.eth_bridge_chain_id)?;
            evm_client_configs.insert(
                chain_id.clone(),
                BridgeClientEvmConfig {
                    contracts: evm_contracts.get(&chain_id).unwrap().clone(),
                    contracts_start_block_fallback: evm_config.eth_contracts_start_block_fallback.unwrap(),
                    contracts_start_block_override: evm_config.eth_contracts_start_block_override,
                    latest_fast_path_threshold: evm_config.latest_fast_path_threshold,
                    safe_fast_path_threshold: evm_config.safe_fast_path_threshold,
                    enable_fast_path_latest: evm_config.enable_fast_path_latest,
                    enable_fast_path_safe: evm_config.enable_fast_path_safe,
                    enable_fast_path_finalized: evm_config.enable_fast_path_finalized,
                },
            );
        }

        let bridge_client_config = BridgeClientConfig {
            sui_address: client_sui_address,
            key: bridge_client_key,
            gas_object_ref,
            metrics_port: self.metrics_port,
            sui_client: sui_client.clone(),
            eth_client: eth_client.clone(),
            db_path,
            eth_contracts,
            // in `prepare_for_eth` we check if this is None when `run_client` is true. Safe to unwrap here.
            eth_contracts_start_block_fallback: self
                .eth
                .eth_contracts_start_block_fallback
                .unwrap(),
            eth_contracts_start_block_override: self.eth.eth_contracts_start_block_override,
            sui_bridge_module_last_processed_event_id_override: self
                .sui
                .sui_bridge_module_last_processed_event_id_override,
            eth_latest_fast_path_threshold: self.eth.latest_fast_path_threshold,
            eth_safe_fast_path_threshold: self.eth.safe_fast_path_threshold,
            eth_enable_fast_path_latest: self.eth.enable_fast_path_latest,
            eth_enable_fast_path_safe: self.eth.enable_fast_path_safe,
            eth_enable_fast_path_finalized: self.eth.enable_fast_path_finalized,
            aml_key: self.aml_key.clone(),
            evm_clients,
            evm_client_configs,
            run_client: self.run_client,
            user_limit_db_url: self.user_limit_db_url.clone(),
        };

        Ok((bridge_server_config, Some(bridge_client_config)))
    }

    async fn prepare_for_evm(
        &self,
        metrics: Arc<BridgeMetrics>,
    ) -> anyhow::Result<(BTreeMap<BridgeChainId, Arc<EthClient<MeteredEthHttpProvier>>>, BTreeMap<BridgeChainId, Vec<EthAddress>>)> {
        let mut eth_clients: BTreeMap<BridgeChainId, Arc<EthClient<MeteredEthHttpProvier>>> = BTreeMap::new();
        let mut eth_contracts: BTreeMap<BridgeChainId, Vec<EthAddress>> = BTreeMap::new();

        for evm_config in &self.evm {
            if BridgeChainId::is_custom_chain_by_id(evm_config.eth_bridge_chain_id) {
                // custom chains use eth config from BridgeNodeConfig
                continue;
            }
            let bridge_proxy_address =
                EthAddress::from_str(&evm_config.eth_bridge_proxy_address)?;

            let provider =
                Arc::new(
                    new_metered_eth_provider(&evm_config.eth_rpc_url, metrics.clone())
                        .unwrap()
                        .interval(std::time::Duration::from_millis(2000)),
                );
            let chain_id = provider.get_chainid().await?;
            let (
                committee_address,
                limiter_address,
                vault_address,
                config_address,
                _weth_address,
                _usdt_address,
            ) = get_eth_contract_addresses(bridge_proxy_address, &provider).await?;
            let config = EthBridgeConfig::new(config_address, provider.clone());

            if self.run_client && evm_config.eth_contracts_start_block_fallback.is_none() {
                return Err(anyhow!(
                "eth_contracts_start_block_fallback is required when run_client is true"
            ));
            }

            let bridge_chain_id: u8 = config.chain_id().call().await?;
            // if evm_config.eth_bridge_chain_id != bridge_chain_id {
            //     return Err(anyhow!(
            //     "Bridge chain id mismatch: expected {}, but connected to {}",
            //     evm_config.eth_bridge_chain_id,
            //     bridge_chain_id
            // ));
            // }

            info!("Connected to Eth chain: {}, Bridge chain id: {}", chain_id.as_u64(), bridge_chain_id);

            let eth_client =
                Arc::new(
                    EthClient::<MeteredEthHttpProvier>::new(
                        &evm_config.eth_rpc_url,
                        HashSet::from_iter(vec![
                            bridge_proxy_address,
                            committee_address,
                            config_address,
                            limiter_address,
                            vault_address,
                        ]),
                        metrics.clone(),
                        chain_id,
                        BridgeChainId::try_from_primitive(evm_config.eth_bridge_chain_id).unwrap(),
                    )
                        .await?,
                )
                ;
            let contract_addresses = vec![
                bridge_proxy_address,
                committee_address,
                config_address,
                limiter_address,
                vault_address,
            ];

            let chain_id = BridgeChainId::try_from(bridge_chain_id)?;
            eth_clients.insert(chain_id, eth_client.clone());
            eth_contracts.insert(chain_id, contract_addresses);
        }

        Ok((eth_clients, eth_contracts))
    }

    async fn prepare_for_eth(
        &self,
        metrics: Arc<BridgeMetrics>,
    ) -> anyhow::Result<(Arc<EthClient<MeteredEthHttpProvier>>, Vec<EthAddress>)> {
        let bridge_proxy_address = EthAddress::from_str(&self.eth.eth_bridge_proxy_address)?;
        let provider = Arc::new(
            new_metered_eth_provider(&self.eth.eth_rpc_url, metrics.clone())
                .unwrap()
                .interval(std::time::Duration::from_millis(2000)),
        );
        let chain_id = provider.get_chainid().await?;
        let (
            committee_address,
            limiter_address,
            vault_address,
            config_address,
            _weth_address,
            _usdt_address,
        ) = get_eth_contract_addresses(bridge_proxy_address, &provider).await?;
        let config = EthBridgeConfig::new(config_address, provider.clone());

        if self.run_client && self.eth.eth_contracts_start_block_fallback.is_none() {
            return Err(anyhow!(
                "eth_contracts_start_block_fallback is required when run_client is true"
            ));
        }

        // If bridge chain id is Eth Mainent or Sepolia, we expect to see chain
        // identifier to match accordingly.
        let bridge_chain_id: u8 = config.chain_id().call().await?;

        // // TODO: BridgeTestCluster can't be used here, because it doesn't support evm contracts
        // if self.eth.eth_bridge_chain_id != bridge_chain_id {
        //     return Err(anyhow!(
        //         "Bridge chain id mismatch: expected {}, but connected to {}",
        //         self.eth.eth_bridge_chain_id,
        //         bridge_chain_id
        //     ));
        // }
        if bridge_chain_id == BridgeChainId::EthMainnet as u8 && chain_id.as_u64() != 1 {
            anyhow::bail!(
                "Expected Eth chain id 1, but connected to {}",
                chain_id.as_u64()
            );
        }
        if bridge_chain_id == BridgeChainId::EthSepolia as u8 && chain_id.as_u64() != 11155111 {
            anyhow::bail!(
                "Expected Eth chain id 11155111, but connected to {}",
                chain_id.as_u64()
            );
        }
        info!(
            "Connected to Eth chain: {}, Bridge chain id: {}",
            chain_id.as_u64(),
            bridge_chain_id,
        );

        let eth_client = Arc::new(
            EthClient::<MeteredEthHttpProvier>::new(
                &self.eth.eth_rpc_url,
                HashSet::from_iter(vec![
                    bridge_proxy_address,
                    committee_address,
                    config_address,
                    limiter_address,
                    vault_address,
                ]),
                metrics,
                chain_id,
                BridgeChainId::try_from_primitive(self.eth.eth_bridge_chain_id).unwrap(),
            )
                .await?,
        );
        let contract_addresses = vec![
            bridge_proxy_address,
            committee_address,
            config_address,
            limiter_address,
            vault_address,
        ];
        Ok((eth_client, contract_addresses))
    }

    async fn prepare_for_sui(
        &self,
        sui_client: Arc<SuiClient<SuiSdkClient>>,
        metrics: Arc<BridgeMetrics>,
    ) -> anyhow::Result<(SuiKeyPair, SuiAddress, ObjectRef)> {
        tracing::info!("prepare_for_sui bridge_client_key_path {:?}", &self.sui.bridge_client_key_path);
        let bridge_client_key = match &self.sui.bridge_client_key_path {
            None => read_key(&self.bridge_authority_key_path, true),
            Some(path) => read_key(path, false),
        }?;

        // If bridge chain id is Sui Mainent or Testnet, we expect to see chain
        // identifier to match accordingly.
        let sui_identifier = sui_client
            .get_chain_identifier()
            .await
            .map_err(|e| anyhow!("Error getting chain identifier from Sui: {:?}", e))?;
        if self.sui.sui_bridge_chain_id == BridgeChainId::SuiMainnet as u8
            && sui_identifier != get_mainnet_chain_identifier().to_string()
        {
            anyhow::bail!(
                "Expected sui chain identifier {}, but connected to {}",
                self.sui.sui_bridge_chain_id,
                sui_identifier
            );
        }
        if self.sui.sui_bridge_chain_id == BridgeChainId::SuiTestnet as u8
            && sui_identifier != get_testnet_chain_identifier().to_string()
        {
            anyhow::bail!(
                "Expected sui chain identifier {}, but connected to {}",
                self.sui.sui_bridge_chain_id,
                sui_identifier
            );
        }
        info!(
            "Connected to Sui chain: {}, Bridge chain id: {}",
            sui_identifier, self.sui.sui_bridge_chain_id,
        );
        let client_sui_address = SuiAddress::from(&bridge_client_key.public());

        let gas_object_id = match self.sui.bridge_client_gas_object {
            Some(id) => id,
            None => {
                let sui_client = SuiClientBuilder::default()
                    .build(&self.sui.sui_rpc_url)
                    .await?;
                let coin =
                    // Minimum balance for gas object is 10 SUI
                    pick_highest_balance_coin(sui_client.coin_read_api(), client_sui_address, 10_000_000_000)
                        .await?;
                coin.coin_object_id
            }
        };
        let (gas_coin, gas_object_ref, owner) = sui_client
            .get_gas_data_panic_if_not_gas(gas_object_id)
            .await;
        if owner != Owner::AddressOwner(client_sui_address) {
            return Err(anyhow!("Gas object {:?} is not owned by bridge client key's associated sui address {:?}, but {:?}", gas_object_id, client_sui_address, owner));
        }
        let balance = gas_coin.value();
        metrics.gas_coin_balance.set(balance as i64);
        info!(
            "Starting bridge client with address: {:?}, gas object {:?}, balance: {}",
            client_sui_address, gas_object_ref.0, balance,
        );

        Ok((bridge_client_key, client_sui_address, gas_object_ref))
    }
}

pub struct BridgeServerConfig {
    pub key: BridgeAuthorityKeyPair,
    pub server_listen_port: u16,
    pub eth_bridge_proxy_address: EthAddress,
    pub metrics_port: u16,
    pub sui_client: Arc<SuiClient<SuiSdkClient>>,
    pub eth_client: Arc<EthClient<MeteredEthHttpProvier>>,
    pub evm_clients: BTreeMap<BridgeChainId, Arc<EthClient<MeteredEthHttpProvier>>>,
    /// A list of approved governance actions. Action in this list will be signed when requested by client.
    pub approved_governance_actions: Vec<BridgeAction>,
    pub external_rpc: Option<ExternalChainRpcConfig>,
}

pub struct BridgeClientConfig {
    pub sui_address: SuiAddress,
    pub key: SuiKeyPair,
    pub gas_object_ref: ObjectRef,
    pub metrics_port: u16,
    pub sui_client: Arc<SuiClient<SuiSdkClient>>,
    pub eth_client: Arc<EthClient<MeteredEthHttpProvier>>,
    pub evm_clients: BTreeMap<BridgeChainId, Arc<EthClient<MeteredEthHttpProvier>>>,

    pub db_path: PathBuf,
    pub eth_contracts: Vec<EthAddress>,
    // See `BridgeNodeConfig` for the explanation of following two fields.
    pub eth_contracts_start_block_fallback: u64,
    pub eth_contracts_start_block_override: Option<u64>,
    /// latest 快速到账阈值
    pub eth_latest_fast_path_threshold: Option<u64>,
    /// safe 快速到账阈值
    pub eth_safe_fast_path_threshold: Option<u64>,
    /// 是否开启快速到账
    pub eth_enable_fast_path_latest: bool,
    pub eth_enable_fast_path_safe: bool,
    pub eth_enable_fast_path_finalized: bool,

    pub evm_client_configs: BTreeMap<BridgeChainId, BridgeClientEvmConfig>,

    pub sui_bridge_module_last_processed_event_id_override: Option<EventID>,
    // The following fields are used for AML checking authorization key
    pub aml_key: String,
    pub run_client: bool,
    pub user_limit_db_url: Option<String>,
}

#[derive(Debug)]
pub struct BridgeClientEvmConfig {
    pub contracts: Vec<EthAddress>,
    // See `BridgeNodeConfig` for the explanation of following two fields.
    pub contracts_start_block_fallback: u64,
    pub contracts_start_block_override: Option<u64>,
    /// latest 快速到账阈值
    pub latest_fast_path_threshold: Option<u64>,
    /// safe 快速到账阈值
    pub safe_fast_path_threshold: Option<u64>,
    /// 是否开启快速到账
    pub enable_fast_path_latest: bool,
    pub enable_fast_path_safe: bool,
    pub enable_fast_path_finalized: bool,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct BridgeCommitteeConfig {
    pub bridge_authority_port_and_key_path: Vec<(u64, PathBuf)>,
}

impl Config for BridgeCommitteeConfig {}

pub async fn pick_highest_balance_coin(
    coin_read_api: &CoinReadApi,
    address: SuiAddress,
    minimal_amount: u64,
) -> anyhow::Result<Coin> {
    let mut highest_balance = 0;
    let mut highest_balance_coin = None;
    coin_read_api
        .get_coins_stream(address, None)
        .for_each(|coin: Coin| {
            if coin.balance > highest_balance {
                highest_balance = coin.balance;
                highest_balance_coin = Some(coin.clone());
            }
            future::ready(())
        })
        .await;
    if highest_balance_coin.is_none() {
        return Err(anyhow!("No Sui coins found for address {:?}", address));
    }
    if highest_balance < minimal_amount {
        return Err(anyhow!(
            "Found no single coin that has >= {} balance Sui for address {:?}",
            minimal_amount,
            address,
        ));
    }
    Ok(highest_balance_coin.unwrap())
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EthContractAddresses {
    pub sui_bridge: EthAddress,
    pub bridge_committee: EthAddress,
    pub bridge_config: EthAddress,
    pub bridge_limiter: EthAddress,
    pub bridge_vault: EthAddress,
}
