// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::type_complexity)]

use std::collections::BTreeMap;
use crate::btc_query::check_btc_txn;
use crate::abi::EthToSuiTokenBridgeV1;
use crate::crypto::{BridgeAuthorityKeyPair, BridgeAuthoritySignInfo};
use crate::error::{BridgeError, BridgeResult};
use crate::eth_client::EthClient;
use crate::fast_path::{FastPathConfig, FastPathSelector};
use crate::metrics::BridgeMetrics;
use crate::sui_client::{SuiClient, SuiClientInner};
use crate::types::{BridgeAction, BridgeActionType, EthToSuiBridgeAction, SignedBridgeAction};
use crate::tron_query::check_tron_txn;
use crate::solana_query::check_solana_txn;
use crate::solana_client::SolanaClient;
use crate::config::ExternalChainRpcConfig;
use async_trait::async_trait;
use axum::Json;
use ethers::providers::JsonRpcClient;
use ethers::types::{BigEndianHash, TxHash, U256};
use lru::LruCache;
use num_enum::TryFromPrimitive;
use std::num::NonZeroUsize;
use std::str::FromStr;
use std::sync::Arc;
use sui_types::digests::TransactionDigest;
use tap::TapFallible;
use tokio::sync::{oneshot, Mutex};
use tracing::info;
use tracing::log::error;
use sui_types::bridge::BridgeChainId;
use sui_types::bridge::BridgeChainId::SuiMainnet;
use super::governance_verifier::GovernanceVerifier;

#[async_trait]
pub trait BridgeRequestHandlerTrait {
    /// Handles a request to sign a BridgeAction that bridges assets
    /// from Ethereum to Sui. The inputs are a transaction hash on Ethereum
    /// that emitted the bridge event and the Event index in that transaction
    async fn handle_eth_tx_hash(
        &self,
        chain_id: u8,
        tx_hash_hex: String,
        event_idx: u16,
        fast_path_selector: u8,
    ) -> Result<Json<SignedBridgeAction>, BridgeError>;

    /// Handles a request to sign a BridgeAction that bridges assets
    /// from Solana to Sui. The inputs are a transaction signature on Solana
    /// that emitted the bridge event and the Event index in that transaction
    async fn handle_solana_tx_signature(
        &self,
        chain_id: u8,
        tx_signature: String,
        event_idx: u16,
        fast_path_selector: u8,
    ) -> Result<Json<SignedBridgeAction>, BridgeError>;
    /// Handles a request to sign a BridgeAction that bridges assets
    /// from Sui to Ethereum. The inputs are a transaction digest on Sui
    /// that emitted the bridge event and the Event index in that transaction
    async fn handle_sui_tx_digest(
        &self,
        tx_digest_base58: String,
        event_idx: u16,
    ) -> Result<Json<SignedBridgeAction>, BridgeError>;

    async fn handle_send_back_tx_digest(
        &self,
        tx_digest_base58: String,
        event_idx: u16,
    ) -> Result<Json<SignedBridgeAction>, BridgeError>;

    async fn handle_external_coin_tx_digest(
        &self,
        tx_digest_base58: String,
        event_idx: u16,
    ) -> Result<Json<SignedBridgeAction>, BridgeError>;

    /// Handles a request to sign a governance action.
    async fn handle_governance_action(
        &self,
        action: BridgeAction,
    ) -> Result<Json<SignedBridgeAction>, BridgeError>;
}

#[async_trait::async_trait]
pub trait ActionVerifier<K>: Send + Sync {
    // Name of the verifier, used for metrics
    fn name(&self) -> &'static str;
    async fn verify(&self, key: K) -> BridgeResult<BridgeAction>;
}

struct SuiActionVerifier<C> {
    sui_client: Arc<SuiClient<C>>,
}

struct EthActionVerifier<P> {
    eth_client: Arc<EthClient<P>>,
    evm_clients: BTreeMap<BridgeChainId, Arc<EthClient<P>>>,
    fast_path_config: FastPathConfig,
}

struct SolanaActionVerifier {
    solana_client: Arc<SolanaClient>,
}

struct SendBackActionVerifier<C, P> {
    sui_client: Arc<SuiClient<C>>,
    eth_client: Arc<EthClient<P>>,
    evm_clients: BTreeMap<BridgeChainId, Arc<EthClient<P>>>,
    fast_path_config: FastPathConfig,
    solana_base_url: Option<String>,// for decentralization request
}

struct ExternalCoinVerifier<C> {
    sui_client: Arc<SuiClient<C>>,
    external_rpc: Option<Arc<ExternalChainRpcConfig>>,
}

#[async_trait::async_trait]
impl<C> ActionVerifier<(u8, TransactionDigest, u16)> for SuiActionVerifier<C>
where
    C: SuiClientInner + Send + Sync + 'static,
{
    fn name(&self) -> &'static str {
        "SuiActionVerifier"
    }

    async fn verify(&self, key: (u8, TransactionDigest, u16)) -> BridgeResult<BridgeAction> {
        let (_, tx_digest, event_idx) = key;
        self.sui_client
            .get_bridge_action_by_tx_digest_and_event_idx_maybe(&tx_digest, event_idx)
            .await
            .tap_ok(|action| info!("Sui action found: {:?}", action))
    }
}

#[async_trait::async_trait]
impl<C> ActionVerifier<(u8, TxHash, u16, u8)> for EthActionVerifier<C>
where
    C: JsonRpcClient + Send + Sync + 'static,
{
    fn name(&self) -> &'static str {
        "EthActionVerifier"
    }

    async fn verify(&self, key: (u8, TxHash, u16, u8)) -> BridgeResult<BridgeAction> {
        let (chain_id, tx_hash, event_idx, fast_path_selector) = key;
        let bridge_chain_id = BridgeChainId::try_from(chain_id)?;
        let fast_path_selector = FastPathSelector::try_from_primitive(fast_path_selector).unwrap();
        match bridge_chain_id {
            BridgeChainId::TronMainnet | BridgeChainId::TronTestnet |
            BridgeChainId::SolanaMainnet | BridgeChainId::SolanaTestnet |
            BridgeChainId::LTCMainnet | BridgeChainId::LTCTestnet |
            BridgeChainId::DogeMainnet | BridgeChainId::DogeTestnet |
            BridgeChainId::SuiMainnet | BridgeChainId::SuiTestnet | BridgeChainId::SuiCustom |
            BridgeChainId::BtcMainnet | BridgeChainId::BtcTestnet => {
                unreachable!()
            }
            BridgeChainId::EthMainnet | BridgeChainId::EthSepolia | BridgeChainId::EthCustom => {
                let action = self.eth_client
                    .get_bridge_action_maybe(tx_hash, event_idx, self.fast_path_config.clone(), Some(fast_path_selector))
                    .await
                    .tap_ok(|action| info!("Eth action found: {:?}", action));
                if let Err(e) = action {
                    return Err(e);
                }
                let action = action.unwrap();
                let mut action_clone = action.clone();
                if let BridgeAction::EthToSuiBridgeAction(ref mut action_inner) = action_clone {
                    action_inner.eth_bridge_event.set_fast_path_selector(fast_path_selector);
                };
                info!("bbking action_clone mut: {:?}", action_clone);
                Ok(action_clone)
            }

            // Add all other evm chains here
            BridgeChainId::BscMainnet | BridgeChainId::BscTestnet | BridgeChainId::BscCustom |
            BridgeChainId::BaseMainnet | BridgeChainId::BaseTestnet | BridgeChainId::BaseCustom |
            BridgeChainId::ArbMainnet | BridgeChainId::ArbTestnet | BridgeChainId::ArbCustom |
            BridgeChainId::AvaxMainnet | BridgeChainId::AvaxTestnet | BridgeChainId::AvaxCustom |
            BridgeChainId::PolMainnet | BridgeChainId::PolTestnet | BridgeChainId::PolCustom |
            BridgeChainId::OPMainnet | BridgeChainId::OPTestnet | BridgeChainId::OPCustom => {
                let client = self.evm_clients.get(&bridge_chain_id);
                if client.is_none() {
                    error!( "ERC20 client not found chain id  {:?}", bridge_chain_id);
                    return Err(BridgeError::Generic(format!("ERC20 client not found chain id  {:?}", bridge_chain_id).to_string()));
                }

                let client = client.unwrap();
                let action = client
                    .get_bridge_action_maybe(tx_hash, event_idx, self.fast_path_config.clone(), Some(fast_path_selector))
                    .await
                    .tap_ok(|action| info!("ERC20 action found: {:?}", action));
                if let Err(e) = action {
                    return Err(e);
                }
                let action = action.unwrap();
                let mut action_clone = action.clone();
                if let BridgeAction::EthToSuiBridgeAction(ref mut action_inner) = action_clone {
                    action_inner.eth_bridge_event.set_fast_path_selector(fast_path_selector);
                };
                info!("bbking action_clone mut: {:?}", action_clone);
                Ok(action_clone)
            }
            BridgeChainId::AptosMainnet| BridgeChainId::AptosTestnet => {
                // Aptos is not supported yet
                Err(BridgeError::Generic(
                    "Aptos chain is not supported yet".to_string(),
                ))
            }
            BridgeChainId::SuiOfficialMainnet | BridgeChainId::SuiOfficialTestnet => {
                // Sui is not supported yet
                Err(BridgeError::Generic(
                    "Sui chain is not supported yet".to_string(),
                ))
            }
        }
    }
}

#[async_trait::async_trait]
impl<C> ActionVerifier<(u8, TransactionDigest, u16)> for ExternalCoinVerifier<C>
where
    C: SuiClientInner + Send + Sync + 'static,
{
    fn name(&self) -> &'static str {
        "ExternalCoinVerifier"
    }

    async fn verify(&self, key: (u8, TransactionDigest, u16)) -> BridgeResult<BridgeAction> {
        let (_, tx_digest, event_idx) = key;
        let result = self
            .sui_client
            .get_bridge_action_by_tx_digest_and_event_idx_maybe(&tx_digest, event_idx)
            .await
            .tap_ok(|action| info!("Sui action found: {:?}", action));
        if let Err(e) = result {
            return Err(e);
        }
        let action_rs: BridgeAction = result.unwrap();
        if let BridgeAction::ExternalDepositStartBridgeAction(ref external_action) = action_rs {
            let tx_hash = &external_action.sui_bridge_event.tx_hash;
            let amount = external_action.sui_bridge_event.amount;
            let chain_id = external_action.sui_bridge_event.source_chain;

            // check target address in whitelist
            let summary = self.sui_client.get_bridge_summary().await;
            if summary.is_err() {
                return Err(BridgeError::Generic("Get bridge summary failed".to_string()));
            }
            let mut whitelist: Vec<String> = vec![];
            for (_, v) in summary.unwrap().treasury.external_coin_target_address {
                whitelist.append(&mut v.clone());
            }
            info!("whitelist: {:#?}", &whitelist);


            match chain_id {
                BridgeChainId::BtcMainnet | BridgeChainId::BtcTestnet => {
                    // check btc txn
                    let ok = check_btc_txn(chain_id, tx_hash, whitelist, amount).await;
                    if ok {
                        return Ok(action_rs);
                    }
                }
                BridgeChainId::TronMainnet | BridgeChainId::TronTestnet => {
                    // check tron txn: only support TRC20
                    // readme: amount is benfen amount, not tron amount, so we need to convert it
                    let tron_amount = amount / 1_000;
                    let external_rpc = self.external_rpc.as_ref().ok_or_else(|| {
                        BridgeError::Generic("External RPC config not found".to_string())
                    })?;
                    let ok = check_tron_txn(chain_id, tx_hash, whitelist, tron_amount, false, &external_rpc.tron).await;
                    if ok {
                        return Ok(action_rs);
                    }
                }
                BridgeChainId::SolanaMainnet | BridgeChainId::SolanaTestnet => {
                    // check solana txn: only support USDC/USDT

                    // readme: amount is benfen amount, not solana amount, so we need to convert it
                    let sol_amount = amount / 1_000;
                    let external_rpc = self.external_rpc.as_ref().ok_or_else(|| {
                        BridgeError::Generic("External RPC config not found".to_string())
                    })?;
                    let ok = check_solana_txn(chain_id, tx_hash, whitelist, sol_amount, false, &external_rpc.solana).await;
                    if ok {
                        return Ok(action_rs);
                    }
                }
                _ => {
                    return Err(BridgeError::Generic(
                        format!("Unsupported External Coin chain ID({})", chain_id)
                    ));
                }
            }

            return Err(BridgeError::Generic(
                format!("External Coin txn({:#?}) is not valid for chain {}", tx_hash, chain_id)
            ));
        }

        Err(BridgeError::ActionIsNotGovernanceAction(action_rs))
    }
}

#[async_trait::async_trait]
impl<C, P> ActionVerifier<(u8, TransactionDigest, u16)> for SendBackActionVerifier<C, P>
where
    C: SuiClientInner + Send + Sync + 'static,
    P: JsonRpcClient + Send + Sync + 'static,
{
    fn name(&self) -> &'static str {
        "SendBackActionVerifier"
    }

    async fn verify(&self, key: (u8, TransactionDigest, u16)) -> BridgeResult<BridgeAction> {
        let (_, tx_digest, event_idx) = key;
        let result = self
            .sui_client
            .get_bridge_action_by_tx_digest_and_event_idx_maybe(&tx_digest, event_idx)
            .await
            .tap_ok(|action| info!("Sui action found: {:?}", action));
        if let Err(e) = result {
            return Err(e);
        }
        let action_rs = result.unwrap();
        if let BridgeAction::EthSendBackBridgeAction(ref send_back_action) = action_rs {
            let tx_hash_bytes = send_back_action.sui_bridge_event.tx_hash.to_vec();
            let tx_hash = U256::from_big_endian(&tx_hash_bytes);
            let event_idx = send_back_action.sui_bridge_event.event_idx;

            let result = match send_back_action.sui_bridge_event.eth_chain_id {
                BridgeChainId::TronMainnet | BridgeChainId::TronTestnet |
                BridgeChainId::SolanaMainnet | BridgeChainId::SolanaTestnet |
                BridgeChainId::LTCMainnet | BridgeChainId::LTCTestnet |
                BridgeChainId::DogeMainnet | BridgeChainId::DogeTestnet |
                BridgeChainId::SuiMainnet | BridgeChainId::SuiTestnet | BridgeChainId::SuiCustom |
                BridgeChainId::BtcMainnet | BridgeChainId::BtcTestnet => {
                    unreachable!()
                }
                BridgeChainId::EthMainnet | BridgeChainId::EthSepolia | BridgeChainId::EthCustom => {
                    self.eth_client
                        .get_bridge_action_maybe(TxHash::from_uint(&tx_hash), event_idx, self.fast_path_config.clone(), Some(FastPathSelector::Finalized))
                        .await
                }

                // Add all other evm chains here
                BridgeChainId::BscMainnet | BridgeChainId::BscTestnet | BridgeChainId::BscCustom |
                BridgeChainId::BaseMainnet | BridgeChainId::BaseTestnet | BridgeChainId::BaseCustom |
                BridgeChainId::ArbMainnet | BridgeChainId::ArbTestnet | BridgeChainId::ArbCustom |
                BridgeChainId::AvaxMainnet | BridgeChainId::AvaxTestnet | BridgeChainId::AvaxCustom |
                BridgeChainId::PolMainnet | BridgeChainId::PolTestnet | BridgeChainId::PolCustom |
                BridgeChainId::OPMainnet | BridgeChainId::OPTestnet | BridgeChainId::OPCustom => {
                    let client = self.evm_clients.get(&send_back_action.sui_bridge_event.eth_chain_id);
                    if client.is_none() {
                        error!( "ERC20 client not found chain id  {:?}", send_back_action.sui_bridge_event.eth_chain_id);
                        return Err(BridgeError::Generic(format!("ERC20 client not found chain id  {:?}", send_back_action.sui_bridge_event.eth_chain_id).to_string()));
                    }

                    let client = client.unwrap();
                    client
                        .get_bridge_action_maybe(TxHash::from_uint(&tx_hash), event_idx, self.fast_path_config.clone(), Some(FastPathSelector::Finalized))
                        .await
                }
                BridgeChainId::AptosMainnet| BridgeChainId::AptosTestnet => {
                    // Aptos is not supported yet
                    Err(BridgeError::Generic(
                        "Aptos chain is not supported yet".to_string(),
                    ))
                }
                BridgeChainId::SuiOfficialMainnet | BridgeChainId::SuiOfficialTestnet => {
                    // Sui is not supported yet
                    Err(BridgeError::Generic(
                        "Sui chain is not supported yet".to_string(),
                    ))
                }
            };


            if let Err(e) = result {
                return Err(e);
            }
            let action = result.unwrap();
            if action.action_type() != BridgeActionType::TokenTransfer {
                return Err(BridgeError::Generic(format!(
                    "Expected EthToSuiBridgeAction, got {:?}",
                    action.action_type()
                )));
            }
            // check amount, token_id, target_address
            if let BridgeAction::EthToSuiBridgeAction(ref eth_to_sui_action) = action {
                if eth_to_sui_action.eth_bridge_event.sui_adjusted_amount
                    != send_back_action.sui_bridge_event.amount_sui_adjusted
                {
                    return Err(BridgeError::Generic(format!(
                        "Amount mismatch: expected {}, got {}",
                        send_back_action.sui_bridge_event.amount_sui_adjusted,
                        eth_to_sui_action.eth_bridge_event.sui_adjusted_amount
                    )));
                }
                if eth_to_sui_action.eth_bridge_event.token_id
                    != send_back_action.sui_bridge_event.token_id
                {
                    return Err(BridgeError::Generic(format!(
                        "Token ID mismatch: expected {}, got {}",
                        send_back_action.sui_bridge_event.token_id,
                        eth_to_sui_action.eth_bridge_event.token_id
                    )));
                }
                if eth_to_sui_action.eth_bridge_event.eth_address
                    != send_back_action.sui_bridge_event.eth_address
                {
                    return Err(BridgeError::Generic(format!(
                        "Target address mismatch: expected {}, got {}",
                        send_back_action.sui_bridge_event.eth_address,
                        eth_to_sui_action.eth_bridge_event.eth_address
                    )));
                }
            }
            return Ok(action_rs);
        }
        if let BridgeAction::SolanaSendBackBridgeAction(ref send_back_action) = action_rs {
            let tx_hash_bytes = send_back_action.sui_bridge_event.tx_hash.to_vec();
            let event_idx = send_back_action.sui_bridge_event.event_idx;

            let base_url = self.solana_base_url.as_ref().ok_or_else(|| {
                BridgeError::Generic("Solana base URL not found".to_string())
            })?;
            let solana_client = SolanaClient::new(base_url);

            // tx_hash 字段在 Solana send-back 事件里承载原始 Solana tx signature（bytes）。
            // 优先按 UTF-8 解析（兼容直接存字符串），失败则 fallback 到 base58。
            let mut tx_signature = std::str::from_utf8(&tx_hash_bytes)
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|_| bs58::encode(&tx_hash_bytes).into_string());
            if tx_signature.is_empty() {
                tx_signature = bs58::encode(&tx_hash_bytes).into_string();
            }

            let action = solana_client
                .get_bridge_action_maybe(&tx_signature, event_idx)
                .await
                .tap_ok(|action| info!("Solana action found: {:?}", action))?;

            if action.action_type() != BridgeActionType::TokenTransfer {
                return Err(BridgeError::Generic(format!(
                    "Expected SolanaToSuiBridgeAction, got {:?}",
                    action.action_type()
                )));
            }
            if let BridgeAction::SolanaToSuiBridgeAction(ref solana_to_sui_action) = action {
                if solana_to_sui_action.solana_bridge_event.sui_adjusted_amount
                    != send_back_action.sui_bridge_event.amount_sui_adjusted
                {
                    return Err(BridgeError::Generic(format!(
                        "Amount mismatch: expected {}, got {}",
                        send_back_action.sui_bridge_event.amount_sui_adjusted,
                        solana_to_sui_action.solana_bridge_event.sui_adjusted_amount
                    )));
                }
                if solana_to_sui_action.solana_bridge_event.token_id
                    != send_back_action.sui_bridge_event.token_id
                {
                    return Err(BridgeError::Generic(format!(
                        "Token ID mismatch: expected {}, got {}",
                        send_back_action.sui_bridge_event.token_id,
                        solana_to_sui_action.solana_bridge_event.token_id
                    )));
                }
                if solana_to_sui_action.solana_bridge_event.solana_chain_id
                    != send_back_action.sui_bridge_event.solana_chain_id
                {
                    return Err(BridgeError::Generic(format!(
                        "Solana chain ID mismatch: expected {:?}, got {:?}",
                        send_back_action.sui_bridge_event.solana_chain_id,
                        solana_to_sui_action.solana_bridge_event.solana_chain_id
                    )));
                }
                if solana_to_sui_action.solana_bridge_event.solana_address
                    != send_back_action.sui_bridge_event.solana_address
                {
                    return Err(BridgeError::Generic(format!(
                        "Target address mismatch: expected {}, got {}",
                        send_back_action.sui_bridge_event.solana_address,
                        solana_to_sui_action.solana_bridge_event.solana_address
                    )));
                }
            } else {
                return Err(BridgeError::Generic(format!(
                    "Expected SolanaToSuiBridgeAction, got {:?}",
                    action.action_type()
                )));
            }

            return Ok(action_rs);
        }
        Err(BridgeError::Generic(format!(
            "Expected EthSendBackBridgeAction or SolanaSendBackBridgeAction, got {:?}",
            action_rs.action_type()
        )))
    }
}

struct SignerWithCache<K> {
    signer: Arc<BridgeAuthorityKeyPair>,
    verifier: Arc<dyn ActionVerifier<K>>,
    mutex: Arc<Mutex<()>>,
    cache: LruCache<K, Arc<Mutex<Option<BridgeResult<SignedBridgeAction>>>>>,
    metrics: Arc<BridgeMetrics>,
}

impl<K> SignerWithCache<K>
where
    K: std::hash::Hash + Eq + Clone + Send + Sync + 'static,
{
    fn new(
        signer: Arc<BridgeAuthorityKeyPair>,
        verifier: impl ActionVerifier<K> + 'static,
        metrics: Arc<BridgeMetrics>,
    ) -> Self {
        Self {
            signer,
            verifier: Arc::new(verifier),
            mutex: Arc::new(Mutex::new(())),
            cache: LruCache::new(NonZeroUsize::new(1000).unwrap()),
            metrics,
        }
    }

    fn spawn(
        mut self,
        mut rx: mysten_metrics::metered_channel::Receiver<(
            K,
            oneshot::Sender<BridgeResult<SignedBridgeAction>>,
        )>,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                let (key, tx) = rx
                    .recv()
                    .await
                    .unwrap_or_else(|| panic!("Server signer's channel is closed"));
                info!("bbking100 recv message from channel");
                let result = self.sign(key).await;
                // The receiver may be dropped before the sender (client connection was dropped for example),
                // we ignore the error in that case.
                let _ = tx.send(result);
            }
        })
    }

    async fn get_cache_entry(
        &mut self,
        key: K,
    ) -> Arc<Mutex<Option<BridgeResult<SignedBridgeAction>>>> {
        // This mutex exists to make sure everyone gets the same entry, namely no double insert
        let _ = self.mutex.lock().await;
        self.cache
            .get_or_insert(key, || Arc::new(Mutex::new(None)))
            .clone()
    }

    async fn sign(&mut self, key: K) -> BridgeResult<SignedBridgeAction> {
        let signer = self.signer.clone();
        let verifier = self.verifier.clone();
        let verifier_name = verifier.name();
        let entry = self.get_cache_entry(key.clone()).await;
        let mut guard = entry.lock().await;
        if let Some(result) = &*guard {
            self.metrics
                .signer_with_cache_hit
                .with_label_values(&[verifier_name])
                .inc();
            return result.clone();
        }
        self.metrics
            .signer_with_cache_miss
            .with_label_values(&[verifier_name])
            .inc();
        match verifier.verify(key.clone()).await {
            Ok(bridge_action) => {
                let bridge_action = if bridge_action.is_stable_coin() {
                    match bridge_action {
                        BridgeAction::EthToSuiBridgeAction(action_inner) => {
                            let action = EthToSuiBridgeAction {
                                eth_tx_hash: action_inner.eth_tx_hash,
                                eth_event_index: action_inner.eth_event_index,
                                eth_bridge_event: EthToSuiTokenBridgeV1::try_from(
                                    &action_inner.eth_bridge_event,
                                )
                                .unwrap(),
                            };
                            BridgeAction::EthToSuiBridgeAction(action)
                        }
                        BridgeAction::SolanaToSuiBridgeAction(action_inner) => {
                            BridgeAction::SolanaToSuiBridgeAction(crate::types::SolanaToSuiBridgeAction {
                                solana_tx_signature: action_inner.solana_tx_signature,
                                solana_event_index: action_inner.solana_event_index,
                                solana_bridge_event: crate::types::SolanaToSuiTokenBridgeV1::try_from(
                                    &action_inner.solana_bridge_event,
                                )
                                .unwrap(),
                            })
                        }
                        _ => {
                            return Err(BridgeError::Generic("Not a stable coin".to_string()));
                        }
                    }
                } else {
                    bridge_action
                };

                let sig = BridgeAuthoritySignInfo::new(&bridge_action, &signer);
                let result = SignedBridgeAction::new_from_data_and_sig(bridge_action, sig);
                info!("bbking SignedBridgeAction: {:?}", result);
                // Cache result if Ok
                *guard = Some(Ok(result.clone()));
                Ok(result)
            }
            Err(e) => {
                match e {
                    // Only cache non-transient errors
                    BridgeError::GovernanceActionIsNotApproved { .. }
                    | BridgeError::ActionIsNotGovernanceAction(..)
                    | BridgeError::BridgeEventInUnrecognizedSuiPackage
                    | BridgeError::BridgeEventInUnrecognizedEthContract
                    | BridgeError::BridgeEventNotActionable
                    | BridgeError::NoBridgeEventsInTxPosition => {
                        *guard = Some(Err(e.clone()));
                    }
                    _ => (),
                }
                Err(e)
            }
        }
    }

    #[cfg(test)]
    async fn get_testing_only(
        &mut self,
        key: K,
    ) -> Option<&Arc<Mutex<Option<BridgeResult<SignedBridgeAction>>>>> {
        let _ = self.mutex.lock().await;
        self.cache.get(&key)
    }
}

pub struct BridgeRequestHandler {
    sui_signer_tx: mysten_metrics::metered_channel::Sender<(
        (u8, TransactionDigest, u16),
        oneshot::Sender<BridgeResult<SignedBridgeAction>>,
    )>,
    external_coin_signer_tx: mysten_metrics::metered_channel::Sender<(
        (u8, TransactionDigest, u16),
        oneshot::Sender<BridgeResult<SignedBridgeAction>>,
    )>,
    send_back_signer_tx: mysten_metrics::metered_channel::Sender<(
        (u8, TransactionDigest, u16),
        oneshot::Sender<BridgeResult<SignedBridgeAction>>,
    )>,
    eth_signer_tx: mysten_metrics::metered_channel::Sender<(
        (u8, TxHash, u16, u8),
        oneshot::Sender<BridgeResult<SignedBridgeAction>>,
    )>,
    solana_signer_tx: mysten_metrics::metered_channel::Sender<(
        (u8, String, u16, u8),
        oneshot::Sender<BridgeResult<SignedBridgeAction>>,
    )>,
    governance_signer_tx: mysten_metrics::metered_channel::Sender<(
        BridgeAction,
        oneshot::Sender<BridgeResult<SignedBridgeAction>>,
    )>,
}

impl BridgeRequestHandler {
    pub fn new<
        SC: SuiClientInner + Send + Sync + 'static,
        EP: JsonRpcClient + Send + Sync + 'static,
    >(
        signer: BridgeAuthorityKeyPair,
        sui_client: Arc<SuiClient<SC>>,
        eth_client: Arc<EthClient<EP>>,
        evm_clients: BTreeMap<BridgeChainId, Arc<EthClient<EP>>>,
        approved_governance_actions: Vec<BridgeAction>,
        metrics: Arc<BridgeMetrics>,
        fast_path_config: FastPathConfig,
        external_rpc: Option<crate::config::ExternalChainRpcConfig>,
        solana_base_url: Option<String>,// for decentralization request
    ) -> Self {
        let external_rpc = external_rpc.map(Arc::new);
        info!("bbking100 external_rpc: {:?}", external_rpc);
        let (sui_signer_tx, sui_rx) = mysten_metrics::metered_channel::channel(
            1000,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["server_sui_action_signing_queue"]),
        );

        let (send_back_signer_tx, send_back_rx) = mysten_metrics::metered_channel::channel(
            1000,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["server_send_back_action_signing_queue"]),
        );
        let (eth_signer_tx, eth_rx) = mysten_metrics::metered_channel::channel::<((u8, TxHash, u16, u8), oneshot::Sender<BridgeResult<SignedBridgeAction>>)>(
            1000,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["server_eth_action_signing_queue"]),
        );
        let (solana_signer_tx, solana_rx) = mysten_metrics::metered_channel::channel::<(
            (u8, String, u16, u8),
            oneshot::Sender<BridgeResult<SignedBridgeAction>>,
        )>(
            1000,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["server_solana_action_signing_queue"]),
        );
        let (external_coin_signer_tx, external_coin_rx) = mysten_metrics::metered_channel::channel(
            1000,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["server_sui_action_signing_queue"]),
        );

        let (governance_signer_tx, governance_rx) = mysten_metrics::metered_channel::channel(
            1000,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["server_governance_action_signing_queue"]),
        );
        let signer = Arc::new(signer);

        SignerWithCache::new(
            signer.clone(),
            SuiActionVerifier {
                sui_client: sui_client.clone(),
            },
            metrics.clone(),
        )
            .spawn(sui_rx);

        SignerWithCache::new(
            signer.clone(),
            ExternalCoinVerifier {
                sui_client: sui_client.clone(),
                external_rpc: external_rpc.clone(),
            },
            metrics.clone(),
        )
            .spawn(external_coin_rx);

        SignerWithCache::new(
            signer.clone(),
            EthActionVerifier {
                eth_client: eth_client.clone(),
                evm_clients: evm_clients.clone(),
                fast_path_config: fast_path_config.clone(),
            },
            metrics.clone(),
        )
            .spawn(eth_rx);

        if let Some(solana_base_url) = solana_base_url.clone() {
            let solana_client = Arc::new(SolanaClient::new(&solana_base_url));
            SignerWithCache::new(
                signer.clone(),
                SolanaActionVerifier { solana_client },
                metrics.clone(),
            )
            .spawn(solana_rx);
        } else {
            // If Solana RPC config is missing, we still keep the channel, but requests will error.
            // This avoids panics in environments that do not enable Solana.
            tokio::spawn(async move {
                let mut solana_rx = solana_rx;
                while let Some((_, resp)) = solana_rx.recv().await {
                    let _ = resp.send(Err(BridgeError::Generic(
                        "Solana RPC config not found".to_string(),
                    )));
                }
            });
        }

        SignerWithCache::new(
            signer.clone(),
            GovernanceVerifier::new(approved_governance_actions).unwrap(),
            metrics.clone(),
        )
            .spawn(governance_rx);

        SignerWithCache::new(
            signer.clone(),
            SendBackActionVerifier {
                sui_client: sui_client.clone(),
                eth_client: eth_client.clone(),
                evm_clients: evm_clients.clone(),
                fast_path_config: fast_path_config.clone(),
                solana_base_url: solana_base_url.clone(),
            },
            metrics.clone(),
        )
            .spawn(send_back_rx);

        Self {
            sui_signer_tx,
            external_coin_signer_tx,
            send_back_signer_tx,
            eth_signer_tx,
            solana_signer_tx,
            governance_signer_tx,
        }
    }
}

#[async_trait::async_trait]
impl ActionVerifier<(u8, String, u16, u8)> for SolanaActionVerifier {
    fn name(&self) -> &'static str {
        "SolanaActionVerifier"
    }

    async fn verify(&self, key: (u8, String, u16, u8)) -> BridgeResult<BridgeAction> {
        let (chain_id, tx_signature, event_idx, fast_path_selector) = key;
        info!("bbking100 verify solana action, chain_id: {:?}, tx_signature: {:?}, event_idx: {:?}, fast_path_selector: {:?}", chain_id, tx_signature, event_idx, fast_path_selector);
        let bridge_chain_id = BridgeChainId::try_from(chain_id)?;
        if !bridge_chain_id.is_solana_chain() {
            return Err(BridgeError::Generic(format!(
                "Invalid chain id for SolanaActionVerifier: {:?}",
                bridge_chain_id
            )));
        }

        let mut action = self
            .solana_client
            .get_bridge_action_maybe(&tx_signature, event_idx)
            .await
            .tap_ok(|action| info!("Solana action found: {:?}", action))?;

        if let BridgeAction::SolanaToSuiBridgeAction(ref mut inner) = action {
            inner.solana_bridge_event.set_fast_path_selector(
                FastPathSelector::try_from_primitive(fast_path_selector).unwrap(),
            );
        }

        Ok(action)
    }
}

#[async_trait]
impl BridgeRequestHandlerTrait for BridgeRequestHandler {
    async fn handle_eth_tx_hash(
        &self,
        chain_id: u8,
        tx_hash_hex: String,
        event_idx: u16,
        fast_path_selector: u8,
    ) -> Result<Json<SignedBridgeAction>, BridgeError> {
        let tx_hash = TxHash::from_str(&tx_hash_hex).map_err(|_| BridgeError::InvalidTxHash)?;

        let (tx, rx) = oneshot::channel();
        self.eth_signer_tx
            .send(((chain_id, tx_hash, event_idx, fast_path_selector), tx))
            .await
            .unwrap_or_else(|_| panic!("Server eth signing channel is closed"));
        let signed_action = rx
            .await
            .unwrap_or_else(|_| panic!("Server signing task's oneshot channel is dropped"))?;
        Ok(Json(signed_action))
    }

    async fn handle_solana_tx_signature(
        &self,
        chain_id: u8,
        tx_signature: String,
        event_idx: u16,
        fast_path_selector: u8,
    ) -> Result<Json<SignedBridgeAction>, BridgeError> {
        info!("bbking100 handle_solana_tx_signature: {:?}", tx_signature);
        let (tx, rx) = oneshot::channel();
        self.solana_signer_tx
            .send(((chain_id, tx_signature, event_idx, fast_path_selector), tx))
            .await
            .unwrap_or_else(|_| panic!("Server solana signing channel is closed"));
        let signed_action = rx
            .await
            .unwrap_or_else(|_| panic!("Server signing task's oneshot channel is dropped"))?;
        Ok(Json(signed_action))
    }

    async fn handle_sui_tx_digest(
        &self,
        tx_digest_base58: String,
        event_idx: u16,
    ) -> Result<Json<SignedBridgeAction>, BridgeError> {
        let tx_digest = TransactionDigest::from_str(&tx_digest_base58)
            .map_err(|_e| BridgeError::InvalidTxHash)?;
        let (tx, rx) = oneshot::channel();
        self.sui_signer_tx
            .send(((SuiMainnet as u8, tx_digest, event_idx), tx))
            .await
            .unwrap_or_else(|_| panic!("Server sui signing channel is closed"));
        let signed_action = rx
            .await
            .unwrap_or_else(|_| panic!("Server signing task's oneshot channel is dropped"))?;
        Ok(Json(signed_action))
    }

    async fn handle_send_back_tx_digest(
        &self,
        tx_digest_base58: String,
        event_idx: u16,
    ) -> Result<Json<SignedBridgeAction>, BridgeError> {
        let tx_digest = TransactionDigest::from_str(&tx_digest_base58)
            .map_err(|_e| BridgeError::InvalidTxHash)?;
        let (tx, rx) = oneshot::channel();
        self.send_back_signer_tx
            .send(((SuiMainnet as u8, tx_digest, event_idx), tx))
            .await
            .unwrap_or_else(|_| panic!("Server sui signing channel is closed"));
        let signed_action = rx
            .await
            .unwrap_or_else(|_| panic!("Server signing task's oneshot channel is dropped"))?;
        Ok(Json(signed_action))
    }

    async fn handle_external_coin_tx_digest(
        &self,
        tx_digest_base58: String,
        event_idx: u16,
    ) -> Result<Json<SignedBridgeAction>, BridgeError> {
        let tx_digest = TransactionDigest::from_str(&tx_digest_base58)
            .map_err(|_e| BridgeError::InvalidTxHash)?;
        let (tx, rx) = oneshot::channel();
        self.external_coin_signer_tx
            .send(((SuiMainnet as u8, tx_digest, event_idx), tx))
            .await
            .unwrap_or_else(|_| panic!("Server sui signing channel is closed"));
        let signed_action = rx
            .await
            .unwrap_or_else(|_| panic!("Server signing task's oneshot channel is dropped"))?;
        Ok(Json(signed_action))
    }

    async fn handle_governance_action(
        &self,
        action: BridgeAction,
    ) -> Result<Json<SignedBridgeAction>, BridgeError> {
        if !action.is_governace_action() {
            return Err(BridgeError::ActionIsNotGovernanceAction(action));
        }
        let (tx, rx) = oneshot::channel();
        self.governance_signer_tx
            .send((action, tx))
            .await
            .unwrap_or_else(|_| panic!("Server governance action signing channel is closed"));
        let signed_action = rx.await.unwrap_or_else(|_| {
            panic!("Server governance action task's oneshot channel is dropped")
        })?;
        Ok(Json(signed_action))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::{
        eth_mock_provider::EthMockProvider,
        events::{
            init_all_struct_tags, ExternalDepositStartBridgeV1, MoveExternalDepositStartEvent,
            MoveTokenDepositedEvent, SuiToEthTokenBridgeV1,
            MoveTokenDepositedEventV2,SuiToEthTokenBridgeV2
        },
        sui_mock_client::SuiMockClient,
        test_utils::{
            get_test_external_bridge_action, get_test_log_and_action,
            get_test_sui_to_eth_bridge_action, mock_last_finalized_block,
        },
        types::{
            AddExternalCoinAdminAction, AddExternalCoinTargetAction, AddExternalCoinWitnessAction,
            EmergencyAction, EmergencyActionType, LimitUpdateAction, RefundAdminAction,
            RemoveExternalCoinAdminAction, RemoveExternalCoinTargetAction,
            RemoveExternalCoinWitnessAction,
        },
    };
    use ethers::types::{Address as EthAddress, TransactionReceipt};
    use sui_json_rpc_types::{BcsEvent, SuiEvent};
    use sui_types::bridge::{BridgeChainId, TOKEN_ID_BTC, TOKEN_ID_USDC};
    use sui_types::{base_types::SuiAddress, crypto::get_key_pair};

    #[tokio::test]
    async fn test_sui_signer_with_cache() {
        let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
        let signer = Arc::new(kp);
        let sui_client_mock = SuiMockClient::default();
        let sui_verifier = SuiActionVerifier {
            sui_client: Arc::new(SuiClient::new_for_testing(sui_client_mock.clone())),
        };
        let metrics = Arc::new(BridgeMetrics::new_for_testing());
        let mut sui_signer_with_cache = SignerWithCache::new(signer.clone(), sui_verifier, metrics);

        // Test `get_cache_entry` creates a new entry if not exist
        let sui_tx_digest = TransactionDigest::random();
        let sui_event_idx = 42;
        assert!(sui_signer_with_cache
            .get_testing_only((0, sui_tx_digest, sui_event_idx))
            .await
            .is_none());
        let entry = sui_signer_with_cache
            .get_cache_entry((0, sui_tx_digest, sui_event_idx))
            .await;
        let entry_ = sui_signer_with_cache
            .get_testing_only((0, sui_tx_digest, sui_event_idx))
            .await;
        assert!(entry_.unwrap().lock().await.is_none());

        let action = get_test_sui_to_eth_bridge_action(
            Some(sui_tx_digest),
            Some(sui_event_idx),
            None,
            None,
            None,
            None,
            None,
        );
        let sig = BridgeAuthoritySignInfo::new(&action, &signer);
        let signed_action = SignedBridgeAction::new_from_data_and_sig(action.clone(), sig);
        entry.lock().await.replace(Ok(signed_action));
        let entry_ = sui_signer_with_cache
            .get_testing_only((0, sui_tx_digest, sui_event_idx))
            .await;
        assert!(entry_.unwrap().lock().await.is_some());

        // Test `sign` caches Err result
        let sui_tx_digest = TransactionDigest::random();
        let sui_event_idx = 0;

        // Mock an non-cacheable error such as rpc error
        sui_client_mock.add_events_by_tx_digest_error(sui_tx_digest);
        sui_signer_with_cache
            .sign((0, sui_tx_digest, sui_event_idx))
            .await
            .unwrap_err();
        let entry_ = sui_signer_with_cache
            .get_testing_only((0, sui_tx_digest, sui_event_idx))
            .await;
        assert!(entry_.unwrap().lock().await.is_none());

        // Mock a cacheable error such as no bridge events in tx position (empty event list)
        sui_client_mock.add_events_by_tx_digest(sui_tx_digest, vec![]);
        assert!(matches!(
            sui_signer_with_cache
                .sign((0,sui_tx_digest, sui_event_idx))
                .await,
            Err(BridgeError::NoBridgeEventsInTxPosition)
        ));
        let entry_ = sui_signer_with_cache
            .get_testing_only((0, sui_tx_digest, sui_event_idx))
            .await;
        assert_eq!(
            entry_.unwrap().lock().await.clone().unwrap().unwrap_err(),
            BridgeError::NoBridgeEventsInTxPosition,
        );

        // TODO: test BridgeEventInUnrecognizedSuiPackage, SuiBridgeEvent::try_from_sui_event
        // and BridgeEventNotActionable to be cached

        // Test `sign` caches Ok result
        let emitted_event_1 = MoveTokenDepositedEvent {
            seq_num: 1,
            source_chain: BridgeChainId::SuiCustom as u8,
            sender_address: SuiAddress::random_for_testing_only().to_vec(),
            target_chain: BridgeChainId::EthCustom as u8,
            target_address: EthAddress::random().as_bytes().to_vec(),
            token_type: TOKEN_ID_USDC,
            amount_sui_adjusted: 12345,
        };

        init_all_struct_tags();

        let mut sui_event_1 = SuiEvent::random_for_testing();
        sui_event_1.type_ = SuiToEthTokenBridgeV1.get().unwrap().clone();
        sui_event_1.bcs = BcsEvent::new(bcs::to_bytes(&emitted_event_1).unwrap());
        let sui_tx_digest = sui_event_1.id.tx_digest;

        let mut sui_event_2 = SuiEvent::random_for_testing();
        sui_event_2.type_ = SuiToEthTokenBridgeV1.get().unwrap().clone();
        sui_event_2.bcs = BcsEvent::new(bcs::to_bytes(&emitted_event_1).unwrap());
        let sui_event_idx_2 = 1;
        sui_client_mock.add_events_by_tx_digest(sui_tx_digest, vec![sui_event_2.clone()]);

        sui_client_mock.add_events_by_tx_digest(
            sui_tx_digest,
            vec![sui_event_1.clone(), sui_event_2.clone()],
        );
        let signed_1 = sui_signer_with_cache
            .sign((0, sui_tx_digest, sui_event_idx))
            .await
            .unwrap();
        let signed_2 = sui_signer_with_cache
            .sign((0, sui_tx_digest, sui_event_idx_2))
            .await
            .unwrap();

        // Because the result is cached now, the verifier should not be called again.
        // Even though we remove the `add_events_by_tx_digest` mock, we will still get the same result.
        sui_client_mock.add_events_by_tx_digest(sui_tx_digest, vec![]);
        assert_eq!(
            sui_signer_with_cache
                .sign((0, sui_tx_digest, sui_event_idx))
                .await
                .unwrap(),
            signed_1
        );
        assert_eq!(
            sui_signer_with_cache
                .sign((0, sui_tx_digest, sui_event_idx_2))
                .await
                .unwrap(),
            signed_2
        );
    }
    #[tokio::test]
    async fn test_sui_signer_with_cache_v2() {
        let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
        let signer = Arc::new(kp);
        let sui_client_mock = SuiMockClient::default();
        let sui_verifier = SuiActionVerifier {
            sui_client: Arc::new(SuiClient::new_for_testing(sui_client_mock.clone())),
        };
        let metrics = Arc::new(BridgeMetrics::new_for_testing());
        let mut sui_signer_with_cache = SignerWithCache::new(signer.clone(), sui_verifier, metrics);

        // Test `get_cache_entry` creates a new entry if not exist
        let sui_tx_digest = TransactionDigest::random();
        let sui_event_idx = 42;
        assert!(sui_signer_with_cache
            .get_testing_only((0, sui_tx_digest, sui_event_idx))
            .await
            .is_none());
        let entry = sui_signer_with_cache
            .get_cache_entry((0, sui_tx_digest, sui_event_idx))
            .await;
        let entry_ = sui_signer_with_cache
            .get_testing_only((0, sui_tx_digest, sui_event_idx))
            .await;
        assert!(entry_.unwrap().lock().await.is_none());

        let action = get_test_sui_to_eth_bridge_action(
            Some(sui_tx_digest),
            Some(sui_event_idx),
            None,
            None,
            None,
            None,
            None,
        );
        let sig = BridgeAuthoritySignInfo::new(&action, &signer);
        let signed_action = SignedBridgeAction::new_from_data_and_sig(action.clone(), sig);
        entry.lock().await.replace(Ok(signed_action));
        let entry_ = sui_signer_with_cache
            .get_testing_only((0, sui_tx_digest, sui_event_idx))
            .await;
        assert!(entry_.unwrap().lock().await.is_some());

        // Test `sign` caches Err result
        let sui_tx_digest = TransactionDigest::random();
        let sui_event_idx = 0;

        // Mock an non-cacheable error such as rpc error
        sui_client_mock.add_events_by_tx_digest_error(sui_tx_digest);
        sui_signer_with_cache
            .sign((0, sui_tx_digest, sui_event_idx))
            .await
            .unwrap_err();
        let entry_ = sui_signer_with_cache
            .get_testing_only((0, sui_tx_digest, sui_event_idx))
            .await;
        assert!(entry_.unwrap().lock().await.is_none());

        // Mock a cacheable error such as no bridge events in tx position (empty event list)
        sui_client_mock.add_events_by_tx_digest(sui_tx_digest, vec![]);
        assert!(matches!(
            sui_signer_with_cache
                .sign((0,sui_tx_digest, sui_event_idx))
                .await,
            Err(BridgeError::NoBridgeEventsInTxPosition)
        ));
        let entry_ = sui_signer_with_cache
            .get_testing_only((0, sui_tx_digest, sui_event_idx))
            .await;
        assert_eq!(
            entry_.unwrap().lock().await.clone().unwrap().unwrap_err(),
            BridgeError::NoBridgeEventsInTxPosition,
        );

        // TODO: test BridgeEventInUnrecognizedSuiPackage, SuiBridgeEvent::try_from_sui_event
        // and BridgeEventNotActionable to be cached

        // Test `sign` caches Ok result
        let emitted_event_1 = MoveTokenDepositedEventV2 {
            seq_num: 1,
            source_chain: BridgeChainId::SuiCustom as u8,
            sender_address: SuiAddress::random_for_testing_only().to_vec(),
            target_chain: BridgeChainId::EthCustom as u8,
            target_address: EthAddress::random().as_bytes().to_vec(),
            token_type: TOKEN_ID_USDC,
            amount_before_fee: 0,
            amount_after_fee: 12345,
        };

        init_all_struct_tags();

        let mut sui_event_1 = SuiEvent::random_for_testing();
        sui_event_1.type_ = SuiToEthTokenBridgeV2.get().unwrap().clone();
        sui_event_1.bcs = BcsEvent::new(bcs::to_bytes(&emitted_event_1).unwrap());
        let sui_tx_digest = sui_event_1.id.tx_digest;

        let mut sui_event_2 = SuiEvent::random_for_testing();
        sui_event_2.type_ = SuiToEthTokenBridgeV2.get().unwrap().clone();
        sui_event_2.bcs = BcsEvent::new(bcs::to_bytes(&emitted_event_1).unwrap());
        let sui_event_idx_2 = 1;
        sui_client_mock.add_events_by_tx_digest(sui_tx_digest, vec![sui_event_2.clone()]);

        sui_client_mock.add_events_by_tx_digest(
            sui_tx_digest,
            vec![sui_event_1.clone(), sui_event_2.clone()],
        );
        let signed_1 = sui_signer_with_cache
            .sign((0, sui_tx_digest, sui_event_idx))
            .await
            .unwrap();
        let signed_2 = sui_signer_with_cache
            .sign((0, sui_tx_digest, sui_event_idx_2))
            .await
            .unwrap();

        // Because the result is cached now, the verifier should not be called again.
        // Even though we remove the `add_events_by_tx_digest` mock, we will still get the same result.
        sui_client_mock.add_events_by_tx_digest(sui_tx_digest, vec![]);
        assert_eq!(
            sui_signer_with_cache
                .sign((0, sui_tx_digest, sui_event_idx))
                .await
                .unwrap(),
            signed_1
        );
        assert_eq!(
            sui_signer_with_cache
                .sign((0, sui_tx_digest, sui_event_idx_2))
                .await
                .unwrap(),
            signed_2
        );
    }


    #[tokio::test]
    async fn test_external_coin_signer_with_cache() {
        let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
        let signer = Arc::new(kp);
        let sui_client_mock = SuiMockClient::default();
        let external_verifier = ExternalCoinVerifier {
            sui_client: Arc::new(SuiClient::new_for_testing(sui_client_mock.clone())),
            external_rpc: None,
        };
        let metrics: Arc<BridgeMetrics> = Arc::new(BridgeMetrics::new_for_testing());
        let mut external_signer_with_cache =
            SignerWithCache::new(signer.clone(), external_verifier, metrics);

        // Test `get_cache_entry` creates a new entry if not exist
        let sui_tx_digest = TransactionDigest::random();
        let sui_event_idx = 42;
        assert!(external_signer_with_cache
            .get_testing_only((0, sui_tx_digest, sui_event_idx))
            .await
            .is_none());
        let entry = external_signer_with_cache
            .get_cache_entry((0, sui_tx_digest, sui_event_idx))
            .await;
        let entry_ = external_signer_with_cache
            .get_testing_only((0, sui_tx_digest, sui_event_idx))
            .await;
        assert!(entry_.unwrap().lock().await.is_none());

        let action =
            get_test_external_bridge_action(Some(sui_tx_digest), Some(sui_event_idx), None, SuiAddress::random_for_testing_only(), None);
        let sig = BridgeAuthoritySignInfo::new(&action, &signer);
        let signed_action = SignedBridgeAction::new_from_data_and_sig(action.clone(), sig);
        entry.lock().await.replace(Ok(signed_action));
        let entry_ = external_signer_with_cache
            .get_testing_only((0, sui_tx_digest, sui_event_idx))
            .await;
        assert!(entry_.unwrap().lock().await.is_some());

        // Test `sign` caches Err result
        let sui_tx_digest = TransactionDigest::random();
        let sui_event_idx = 0;

        // Mock an non-cacheable error such as rpc error
        sui_client_mock.add_events_by_tx_digest_error(sui_tx_digest);
        external_signer_with_cache
            .sign((0, sui_tx_digest, sui_event_idx))
            .await
            .unwrap_err();
        let entry_ = external_signer_with_cache
            .get_testing_only((0, sui_tx_digest, sui_event_idx))
            .await;
        assert!(entry_.unwrap().lock().await.is_none());

        // Mock a cacheable error such as no bridge events in tx position (empty event list)
        sui_client_mock.add_events_by_tx_digest(sui_tx_digest, vec![]);
        assert!(matches!(
            external_signer_with_cache
                .sign((0,sui_tx_digest, sui_event_idx))
                .await,
            Err(BridgeError::NoBridgeEventsInTxPosition)
        ));
        let entry_ = external_signer_with_cache
            .get_testing_only((0, sui_tx_digest, sui_event_idx))
            .await;
        assert_eq!(
            entry_.unwrap().lock().await.clone().unwrap().unwrap_err(),
            BridgeError::NoBridgeEventsInTxPosition,
        );

        // TODO: test BridgeEventInUnrecognizedSuiPackage, SuiBridgeEvent::try_from_sui_event
        // and BridgeEventNotActionable to be cached

        // Test `sign` caches Ok result
        let emitted_event_1 = MoveExternalDepositStartEvent {
            seq_num: 1,
            tx_hash: "20c04f56b8dc0f507f8ca7d208fff8f7ca6ca7508bb2a334bbcbf7ec99804941".to_string(),
            source_chain: BridgeChainId::BtcTestnet as u8,
            target_chain: BridgeChainId::SuiCustom as u8,
            source_address: SuiAddress::random_for_testing_only().to_vec(),
            target_address: "tb1p3436xedsqrxfd3gqr3rcrgavytgtrus83plndht05afsssw23q3sxejagc".into(),
            token_id: TOKEN_ID_BTC,
            amount: 10,
        };

        init_all_struct_tags();

        let mut sui_event_1 = SuiEvent::random_for_testing();
        sui_event_1.type_ = ExternalDepositStartBridgeV1.get().unwrap().clone();
        sui_event_1.bcs = BcsEvent::new(bcs::to_bytes(&emitted_event_1).unwrap());
        let sui_tx_digest = sui_event_1.id.tx_digest;

        let mut sui_event_2 = SuiEvent::random_for_testing();
        sui_event_2.type_ = ExternalDepositStartBridgeV1.get().unwrap().clone();
        sui_event_2.bcs = BcsEvent::new(bcs::to_bytes(&emitted_event_1).unwrap());
        let sui_event_idx_2 = 1;
        sui_client_mock.add_events_by_tx_digest(sui_tx_digest, vec![sui_event_2.clone()]);

        sui_client_mock.add_events_by_tx_digest(
            sui_tx_digest,
            vec![sui_event_1.clone(), sui_event_2.clone()],
        );
        let signed_1 = external_signer_with_cache
            .sign((0, sui_tx_digest, sui_event_idx))
            .await
            .unwrap();
        let signed_2 = external_signer_with_cache
            .sign((0, sui_tx_digest, sui_event_idx_2))
            .await
            .unwrap();

        // Because the result is cached now, the verifier should not be called again.
        // Even though we remove the `add_events_by_tx_digest` mock, we will still get the same result.
        sui_client_mock.add_events_by_tx_digest(sui_tx_digest, vec![]);
        assert_eq!(
            external_signer_with_cache
                .sign((0, sui_tx_digest, sui_event_idx))
                .await
                .unwrap(),
            signed_1
        );
        assert_eq!(
            external_signer_with_cache
                .sign((0, sui_tx_digest, sui_event_idx_2))
                .await
                .unwrap(),
            signed_2
        );
    }

    #[tokio::test]
    async fn test_eth_signer_with_cache() {
        let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
        let signer = Arc::new(kp);
        let eth_mock_provider = EthMockProvider::default();
        let contract_address = EthAddress::random();
        let eth_client = EthClient::new_mocked(
            eth_mock_provider.clone(),
            HashSet::from_iter(vec![contract_address]),
        );

        let eth_verifier = EthActionVerifier {
            eth_client: Arc::new(eth_client),
            evm_clients: Default::default(),
            fast_path_config: Default::default(),
        };
        let metrics = Arc::new(BridgeMetrics::new_for_testing());
        let mut eth_signer_with_cache =
            SignerWithCache::new(signer.clone(), eth_verifier, metrics.clone());

        // Test `get_cache_entry` creates a new entry if not exist
        let eth_tx_hash = TxHash::random();
        let eth_event_idx = 42;
        assert!(eth_signer_with_cache
            .get_testing_only((0, eth_tx_hash, eth_event_idx, 2))
            .await
            .is_none());
        let entry = eth_signer_with_cache
            .get_cache_entry((0, eth_tx_hash, eth_event_idx, 2))
            .await;
        let entry_ = eth_signer_with_cache
            .get_testing_only((0, eth_tx_hash, eth_event_idx, 2))
            .await;
        // first unwrap should not pacic because the entry should have been inserted by `get_cache_entry`
        assert!(entry_.unwrap().lock().await.is_none());

        let (_, action) = get_test_log_and_action(contract_address, eth_tx_hash, eth_event_idx);
        let sig = BridgeAuthoritySignInfo::new(&action, &signer);
        let signed_action = SignedBridgeAction::new_from_data_and_sig(action.clone(), sig);
        entry.lock().await.replace(Ok(signed_action.clone()));
        let entry_ = eth_signer_with_cache
            .get_testing_only((0, eth_tx_hash, eth_event_idx, 2))
            .await;
        assert_eq!(
            entry_.unwrap().lock().await.clone().unwrap().unwrap(),
            signed_action
        );

        // Test `sign` caches Ok result
        let eth_tx_hash = TxHash::random();
        let eth_event_idx = 0;
        let (log, _action) = get_test_log_and_action(contract_address, eth_tx_hash, eth_event_idx);
        eth_mock_provider
            .add_response::<[TxHash; 1], TransactionReceipt, TransactionReceipt>(
                "eth_getTransactionReceipt",
                [log.transaction_hash.unwrap()],
                TransactionReceipt {
                    block_number: log.block_number,
                    logs: vec![log.clone()],
                    ..Default::default()
                },
            )
            .unwrap();
        mock_last_finalized_block(&eth_mock_provider, log.block_number.unwrap().as_u64());

        eth_signer_with_cache
            .sign((BridgeChainId::EthCustom as u8, eth_tx_hash, eth_event_idx, 2))
            .await
            .unwrap();
        let entry_ = eth_signer_with_cache
            .get_testing_only((BridgeChainId::EthCustom as u8, eth_tx_hash, eth_event_idx, 2))
            .await;
        entry_.unwrap().lock().await.clone().unwrap().unwrap();
    }

    #[tokio::test]
    async fn test_signer_with_governace_verifier() {
        let action_1 = BridgeAction::EmergencyAction(EmergencyAction {
            chain_id: BridgeChainId::EthCustom,
            nonce: 1,
            action_type: EmergencyActionType::Pause,
        });
        let action_2 = BridgeAction::LimitUpdateAction(LimitUpdateAction {
            chain_id: BridgeChainId::EthCustom,
            sending_chain_id: BridgeChainId::SuiCustom,
            nonce: 1,
            new_usd_limit: 10000,
        });

        let verifier = GovernanceVerifier::new(vec![action_1.clone(), action_2.clone()]).unwrap();
        assert_eq!(
            verifier.verify(action_1.clone()).await.unwrap(),
            action_1.clone()
        );
        assert_eq!(
            verifier.verify(action_2.clone()).await.unwrap(),
            action_2.clone()
        );

        let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
        let signer = Arc::new(kp);
        let metrics = Arc::new(BridgeMetrics::new_for_testing());
        let mut signer_with_cache = SignerWithCache::new(signer.clone(), verifier, metrics.clone());

        // action_1 is signable
        signer_with_cache.sign(action_1.clone()).await.unwrap();
        // signed action is cached
        let entry_ = signer_with_cache.get_testing_only(action_1.clone()).await;
        assert_eq!(
            entry_
                .unwrap()
                .lock()
                .await
                .clone()
                .unwrap()
                .unwrap()
                .data(),
            &action_1
        );

        // alter action_1 to action_3
        let action_3 = BridgeAction::EmergencyAction(EmergencyAction {
            chain_id: BridgeChainId::EthCustom,
            nonce: 1,
            action_type: EmergencyActionType::Unpause,
        });
        // action_3 is not signable
        assert!(matches!(
            signer_with_cache.sign(action_3.clone()).await.unwrap_err(),
            BridgeError::GovernanceActionIsNotApproved { .. }
        ));
        // error is cached
        let entry_ = signer_with_cache.get_testing_only(action_3.clone()).await;
        assert!(matches!(
            entry_.unwrap().lock().await.clone().unwrap().unwrap_err(),
            BridgeError::GovernanceActionIsNotApproved { .. }
        ));

        // Non governace action is not signable
        let action_4 = get_test_sui_to_eth_bridge_action(None, None, None, None, None, None, None);
        assert!(matches!(
            signer_with_cache.sign(action_4.clone()).await.unwrap_err(),
            BridgeError::ActionIsNotGovernanceAction(..)
        ));
        // error is cached
        let entry_ = signer_with_cache.get_testing_only(action_4.clone()).await;
        assert!(matches!(
            entry_.unwrap().lock().await.clone().unwrap().unwrap_err(),
            BridgeError::ActionIsNotGovernanceAction { .. }
        ));
    }

    #[tokio::test]
    async fn test_add_remove_external_admin_action() {
        let action_1 = BridgeAction::AddExternalCoinAdminAction(AddExternalCoinAdminAction {
            chain_id: BridgeChainId::SuiCustom,
            nonce: 1,
            coin_type: "test".to_string(),
            admin_address: SuiAddress::random_for_testing_only().to_string(),
        });
        let action_2 = BridgeAction::RemoveExternalCoinAdminAction(RemoveExternalCoinAdminAction {
            chain_id: BridgeChainId::SuiCustom,
            nonce: 1,
            coin_type: "test".to_string(),
            admin_address: SuiAddress::random_for_testing_only().to_string(),
        });
        let verifier = GovernanceVerifier::new(vec![action_1.clone(), action_2.clone()]).unwrap();
        assert_eq!(
            verifier.verify(action_1.clone()).await.unwrap(),
            action_1.clone()
        );
        let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
        let signer = Arc::new(kp);
        let metrics = Arc::new(BridgeMetrics::new_for_testing());
        let mut signer_with_cache = SignerWithCache::new(signer.clone(), verifier, metrics.clone());

        // action_1 is signable
        signer_with_cache.sign(action_1.clone()).await.unwrap();
        // signed action is cached
        let entry_ = signer_with_cache.get_testing_only(action_1.clone()).await;
        assert_eq!(
            entry_
                .unwrap()
                .lock()
                .await
                .clone()
                .unwrap()
                .unwrap()
                .data(),
            &action_1
        );

        // action_2 is signable
        signer_with_cache.sign(action_2.clone()).await.unwrap();
        // signed action is cached
        let entry_ = signer_with_cache.get_testing_only(action_2.clone()).await;
        assert_eq!(
            entry_
                .unwrap()
                .lock()
                .await
                .clone()
                .unwrap()
                .unwrap()
                .data(),
            &action_2
        );
    }

    #[tokio::test]
    async fn test_add_remove_external_target_action() {
        let action_1 = BridgeAction::AddExternalCoinTargetAction(AddExternalCoinTargetAction {
            chain_id: BridgeChainId::SuiCustom,
            nonce: 1,
            coin_type: "test".to_string(),
            target_address: SuiAddress::random_for_testing_only().to_string(),
        });
        let action_2 =
            BridgeAction::RemoveExternalCoinTargetAction(RemoveExternalCoinTargetAction {
                chain_id: BridgeChainId::SuiCustom,
                nonce: 1,
                coin_type: "test".to_string(),
                target_address: SuiAddress::random_for_testing_only().to_string(),
            });
        let verifier = GovernanceVerifier::new(vec![action_1.clone(), action_2.clone()]).unwrap();
        assert_eq!(
            verifier.verify(action_1.clone()).await.unwrap(),
            action_1.clone()
        );
        let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
        let signer = Arc::new(kp);
        let metrics = Arc::new(BridgeMetrics::new_for_testing());
        let mut signer_with_cache = SignerWithCache::new(signer.clone(), verifier, metrics.clone());

        // action_1 is signable
        signer_with_cache.sign(action_1.clone()).await.unwrap();
        // signed action is cached
        let entry_ = signer_with_cache.get_testing_only(action_1.clone()).await;
        assert_eq!(
            entry_
                .unwrap()
                .lock()
                .await
                .clone()
                .unwrap()
                .unwrap()
                .data(),
            &action_1
        );

        // action_2 is signable
        signer_with_cache.sign(action_2.clone()).await.unwrap();
        // signed action is cached
        let entry_ = signer_with_cache.get_testing_only(action_2.clone()).await;
        assert_eq!(
            entry_
                .unwrap()
                .lock()
                .await
                .clone()
                .unwrap()
                .unwrap()
                .data(),
            &action_2
        );
    }

    #[tokio::test]
    async fn test_add_remove_external_witness_action() {
        let hex_str = EthAddress::from_str("7518085822fAA839EeB59035a74A87b4220C6629").unwrap();

        let action_1 = BridgeAction::AddExternalCoinWitnessAction(AddExternalCoinWitnessAction {
            chain_id: BridgeChainId::SuiCustom,
            nonce: 1,
            coin_type: "test".to_string(),
            witness_address: hex_str.clone(),
        });
        let action_2 =
            BridgeAction::RemoveExternalCoinWitnessAction(RemoveExternalCoinWitnessAction {
                chain_id: BridgeChainId::SuiCustom,
                nonce: 1,
                coin_type: "test".to_string(),
                witness_address: hex_str.clone(),
            });
        let verifier = GovernanceVerifier::new(vec![action_1.clone(), action_2.clone()]).unwrap();
        assert_eq!(
            verifier.verify(action_1.clone()).await.unwrap(),
            action_1.clone()
        );
        let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
        let signer = Arc::new(kp);
        let metrics = Arc::new(BridgeMetrics::new_for_testing());
        let mut signer_with_cache = SignerWithCache::new(signer.clone(), verifier, metrics.clone());

        // action_1 is signable
        signer_with_cache.sign(action_1.clone()).await.unwrap();
        // signed action is cached
        let entry_ = signer_with_cache.get_testing_only(action_1.clone()).await;
        assert_eq!(
            entry_
                .unwrap()
                .lock()
                .await
                .clone()
                .unwrap()
                .unwrap()
                .data(),
            &action_1
        );

        // action_2 is signable
        signer_with_cache.sign(action_2.clone()).await.unwrap();
        // signed action is cached
        let entry_ = signer_with_cache.get_testing_only(action_2.clone()).await;
        assert_eq!(
            entry_
                .unwrap()
                .lock()
                .await
                .clone()
                .unwrap()
                .unwrap()
                .data(),
            &action_2
        );
    }

    #[tokio::test]
    async fn test_refund_admin_action() {
        let action_1 = BridgeAction::RefundAdminAction(RefundAdminAction {
            chain_id: BridgeChainId::SuiCustom,
            nonce: 1,
            op_type: 0,
            sui_address: SuiAddress::random_for_testing_only().to_string(),
        });
        let verifier = GovernanceVerifier::new(vec![action_1.clone()]).unwrap();
        assert_eq!(
            verifier.verify(action_1.clone()).await.unwrap(),
            action_1.clone()
        );
        let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
        let signer = Arc::new(kp);
        let metrics = Arc::new(BridgeMetrics::new_for_testing());
        let mut signer_with_cache = SignerWithCache::new(signer.clone(), verifier, metrics.clone());

        // action_1 is signable
        signer_with_cache.sign(action_1.clone()).await.unwrap();
        // signed action is cached
        let entry_ = signer_with_cache.get_testing_only(action_1.clone()).await;
        assert_eq!(
            entry_
                .unwrap()
                .lock()
                .await
                .clone()
                .unwrap()
                .unwrap()
                .data(),
            &action_1
        );
    }
    // TODO: add tests for BridgeRequestHandler (need to hook up local eth node)
}
