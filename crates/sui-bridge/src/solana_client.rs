use anyhow::anyhow;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::error::{BridgeError, BridgeResult};
use crate::solana_events::SolanaBridgeEvent;
use crate::types::BridgeAction;

#[derive(Clone)]
pub struct SolanaClient {
    client: reqwest::Client,
    base_url: String,
}

impl SolanaClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.to_string(),
        }
    }

    async fn send(&self, method: &str, params: Value) -> Result<Value> {
        let body = json!({ "jsonrpc": "2.0", "id": 1, "method": method, "params": params });
        let resp = self
            .client
            .post(&self.base_url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;
        let status = resp.status();
        let v: Value = resp.json().await?;
        if !status.is_success() {
            return Err(anyhow!(format!("http {}", status)));
        }
        if v.get("error").is_some() {
            return Err(anyhow!(format!("{}", v)));
        }
        Ok(v)
    }

    pub async fn get_block_height(&self, commitment: Option<&str>) -> Result<u64> {
        let params = match commitment {
            Some(c) => json!([ { "commitment": c } ]),
            None => json!([null]),
        };
        let v = self.send("getBlockHeight", params).await?;
        let result = v.get("result").ok_or_else(|| anyhow!("empty result"))?;
        let n = result
            .as_u64()
            .ok_or_else(|| anyhow!("invalid result type"))?;
        Ok(n)
    }

    pub async fn get_slot(&self, commitment: Option<&str>) -> Result<u64> {
        let params = match commitment {
            Some(c) => json!([{ "commitment": c }]),
            None => json!([null]),
        };
        let v = self.send("getSlot", params).await?;
        let result = v.get("result").ok_or_else(|| anyhow!("empty result"))?;
        let n = result
            .as_u64()
            .ok_or_else(|| anyhow!("invalid result type"))?;
        Ok(n)
    }

    pub async fn get_signatures_for_address(
        &self,
        address: &str,
        config: Option<GetSignaturesConfig>,
    ) -> Result<Vec<SignatureInfo>> {
        let params = match config {
            Some(c) => json!([address, c]),
            None => json!([address]),
        };
        let v = self.send("getSignaturesForAddress", params).await?;
        let result = v.get("result").ok_or_else(|| anyhow!("empty result"))?;
        let arr = result
            .as_array()
            .ok_or_else(|| anyhow!("invalid result type"))?;
        let mut out = vec![];
        for item in arr {
            let info: SignatureInfo = serde_json::from_value(item.clone())?;
            out.push(info);
        }
        Ok(out)
    }

    pub async fn get_transaction(&self, signature: &str) -> Result<SolanaTransaction> {
        let config = GetTransactionConfig {
            encoding: Some("json".to_string()),
            commitment: Some("finalized".to_string()),
        };

        let params = json!([signature, config]);
        let v = self.send("getTransaction", params).await?;
        let result = v.get("result").ok_or_else(|| anyhow!("empty result"))?;
        let tx: SolanaTransaction = serde_json::from_value(result.clone())?;
        Ok(tx)
    }

    /// Returns BridgeAction from a Solana Transaction with transaction signature
    /// and the event index.
    pub async fn get_bridge_action_maybe(
        &self,
        tx_signature: &str,
        event_idx: u16,
    ) -> BridgeResult<BridgeAction> {
        let tx = self
            .get_transaction(tx_signature)
            .await
            .map_err(|e| BridgeError::ProviderError(e.to_string()))?;

        // Parse events from transaction logs
        let events = SolanaBridgeEvent::try_from_client_transaction(&tx);
        if events.is_empty() {
            return Err(BridgeError::NoBridgeEventsInTxPosition);
        }

        // Get the event at the specified index
        let event = events
            .get(event_idx as usize)
            .ok_or(BridgeError::NoBridgeEventsInTxPosition)?;

        // Convert event to bridge action
        event
            .try_into_bridge_action(tx_signature.to_string(), event_idx)?
            .ok_or(BridgeError::BridgeEventNotActionable)
    }
}

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSignaturesConfig {
    pub commitment: Option<String>,
    pub min_context_slot: Option<u64>,
    pub limit: Option<u64>,
    pub before: Option<String>,
    pub until: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SignatureInfo {
    pub signature: String,
    pub slot: u64,
    pub err: Option<Value>,
    pub memo: Option<String>,
    pub block_time: Option<i64>,
    pub confirmation_status: Option<String>,
}

#[derive(Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionConfig {
    pub encoding: Option<String>,
    pub commitment: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SolanaTransaction {
    #[serde(default)]
    pub transaction: Option<SolanaTransactionData>,
    #[serde(default)]
    pub meta: Option<SolanaTransactionMeta>,
    #[serde(default)]
    pub slot: Option<u64>,
    #[serde(default)]
    pub block_time: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SolanaTransactionData {
    #[serde(default, rename = "message")]
    pub message: Option<SolanaTransactionMessage>,
    #[serde(rename = "signatures")]
    pub signatures: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SolanaTransactionMessage {
    #[serde(rename = "accountKeys")]
    pub account_keys: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SolanaTransactionMeta {
    #[serde(default, rename = "preBalances")]
    pub pre_balances: Vec<u64>,
    #[serde(default, rename = "postBalances")]
    pub post_balances: Vec<u64>,
    #[serde(default, rename = "preTokenBalances")]
    pub pre_token_balances: Vec<TokenBalance>,
    #[serde(default, rename = "postTokenBalances")]
    pub post_token_balances: Vec<TokenBalance>,
    #[serde(default, rename = "logMessages")]
    pub log_messages: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UiTokenAmount {
    #[serde(rename = "amount")]
    pub amount: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TokenBalance {
    #[serde(rename = "accountIndex")]
    pub account_index: usize,
    #[serde(rename = "mint")]
    pub mint: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "uiTokenAmount")]
    pub ui_token_amount: UiTokenAmount,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solana_events::SolanaBridgeEvent;
    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::routing::post;
    use axum::{Json, Router};
    use sha2::digest::crypto_common::rand_core::le;
    use std::collections::HashMap;
    use std::net::SocketAddr;
    use tokio::sync::oneshot;
    use tokio::task::JoinHandle;

    fn real_solana_rpc_url() -> Option<String> {
        std::env::var("SOLANA_RPC_URL")
            .ok()
            .or_else(|| std::env::var("GETBLOCK_SOLANA_RPC_URL").ok())
    }

    #[derive(Clone)]
    struct MockState;

    async fn handler(
        State(_): State<MockState>,
        Json(body): Json<Value>,
    ) -> (StatusCode, Json<Value>) {
        let method = body
            .get("method")
            .and_then(|m| m.as_str())
            .unwrap_or_default();
        match method {
            "getBlockHeight" => {
                let rsp = json!({ "jsonrpc": "2.0", "id": 1, "result": 12345 });
                (StatusCode::OK, Json(rsp))
            }
            "getSignaturesForAddress" => {
                let rsp = json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": [
                        {
                            "signature": "sig1",
                            "slot": 114,
                            "err": null,
                            "memo": null,
                            "blockTime": 1700000000,
                            "confirmationStatus": "finalized"
                        }
                    ]
                });
                (StatusCode::OK, Json(rsp))
            }
            "getTransaction" => {
                let rsp = json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "slot": 123456,
                        "transaction": {
                            "signatures": ["abc"]
                        },
                        "meta": {
                            "err": null,
                            "fee": 5000
                        }
                    }
                });
                (StatusCode::OK, Json(rsp))
            }
            _ => {
                let rsp = json!({ "jsonrpc": "2.0", "id": 1, "error": { "code": -32601, "message": "Method not found" }});
                (StatusCode::OK, Json(rsp))
            }
        }
    }

    async fn start_mock_server() -> (SocketAddr, JoinHandle<()>, oneshot::Sender<()>) {
        let state = MockState;
        let app = Router::new().route("/", post(handler)).with_state(state);
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        let local_addr = listener.local_addr().unwrap();
        let (tx, rx) = oneshot::channel();
        let handle = tokio::spawn(async move {
            let server = axum::serve(listener, app);
            tokio::select! {
                _ = server => {},
                _ = rx => {},
            }
        });
        (local_addr, handle, tx)
    }

    #[tokio::test]
    async fn test_get_block_height() {
        let (addr, handle, tx) = start_mock_server().await;
        let base_url = format!("http://{addr}/");
        let client = SolanaClient::new(&base_url);
        let n = client.get_block_height(None).await.unwrap();
        assert_eq!(n, 12345);
        tx.send(()).ok();
        handle.abort();
    }

    #[tokio::test]
    async fn test_get_signatures_for_address() {
        let (addr, handle, tx) = start_mock_server().await;
        let base_url = format!("http://{addr}/");
        let client = SolanaClient::new(&base_url);
        let cfg = GetSignaturesConfig {
            limit: Some(1),
            ..Default::default()
        };
        let v = client
            .get_signatures_for_address("Vote111111111111111111111111111111111111111", Some(cfg))
            .await
            .unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].signature, "sig1");
        assert_eq!(v[0].slot, 114);
        tx.send(()).ok();
        handle.abort();
    }

    #[tokio::test]
    async fn test_get_transaction() {
        let (addr, handle, tx) = start_mock_server().await;
        let base_url = format!("http://{addr}/");
        let client = SolanaClient::new(&base_url);
        let v = client.get_transaction("abc").await.unwrap();
        assert!(v.slot.is_some());
        tx.send(()).ok();
        handle.abort();
    }

    // TODO: delete this test, it is just a real solana client test
    #[tokio::test]
    async fn test_real_solana_client() {
        let Some(base_url) = real_solana_rpc_url() else {
            eprintln!("Skipping real Solana RPC test: set SOLANA_RPC_URL or GETBLOCK_SOLANA_RPC_URL");
            return;
        };
        let client = SolanaClient::new(&base_url);

        // test get block height
        let n = client.get_block_height(None).await.unwrap();
        assert!(n > 364267566);

        // test get slot
        let n = client.get_slot(None).await.unwrap();
        assert!(n > 364267566);
        println!("slot: {:?}", n);

        let client = SolanaClient::new(&base_url);

        // test get signatures for address
        let cfg = GetSignaturesConfig {
            limit: Some(1),
            ..Default::default()
        };
        let v = client
            .get_signatures_for_address("Vote111111111111111111111111111111111111111", Some(cfg))
            .await
            .unwrap();
        assert!(v.len() > 0);
        println!("{:?}", v);

        // test get transaction
        let v = client.get_transaction(&v[0].signature).await.unwrap();
        assert!(v.slot.is_some());
        assert!(v.transaction.is_some());
        assert!(v.meta.is_some());

        println!("{:?}", v);
        assert!(!v.meta.as_ref().unwrap().log_messages.is_empty());
    }

    #[tokio::test]
    async fn test_real_get_signatures_for_address_ordering() {
        let Some(base_url) = real_solana_rpc_url() else {
            eprintln!("Skipping real Solana RPC test: set SOLANA_RPC_URL or GETBLOCK_SOLANA_RPC_URL");
            return;
        };

        let client = SolanaClient::new(&base_url);

        // Use a well-known address with lots of history.
        let address = "Vote111111111111111111111111111111111111111";

        // Page 1: default query should return newest -> oldest (descending by slot/time)
        let cfg = GetSignaturesConfig {
            commitment: Some("finalized".to_string()),
            limit: Some(20),
            ..Default::default()
        };

        let page1 = client
            .get_signatures_for_address(address, Some(cfg.clone()))
            .await
            .unwrap();
        assert!(page1.len() >= 5);

        for w in page1.windows(2) {
            assert!(
                w[0].slot >= w[1].slot,
                "expected slots to be non-increasing (newest->oldest), got {} then {}",
                w[0].slot,
                w[1].slot
            );

            if let (Some(t0), Some(t1)) = (w[0].block_time, w[1].block_time) {
                assert!(
                    t0 >= t1,
                    "expected block_time to be non-increasing (newest->oldest), got {} then {}",
                    t0,
                    t1
                );
            }
        }

        // Page 2: using `before` should move further back in history.
        let last_sig_page1 = page1.last().unwrap().signature.clone();
        let cfg2 = GetSignaturesConfig {
            before: Some(last_sig_page1),
            ..cfg
        };
        let page2 = client
            .get_signatures_for_address(address, Some(cfg2))
            .await
            .unwrap();
        assert!(!page2.is_empty());

        assert!(
            page1.last().unwrap().slot >= page2.first().unwrap().slot,
            "expected page2 to be older than page1 when using before; got page1_last_slot={} page2_first_slot={}",
            page1.last().unwrap().slot,
            page2.first().unwrap().slot
        );
    }

    #[tokio::test]
    async fn test_real_get_signatures_for_address_min_context_slot_no_stop_with_pagination() {
        // min_context_slot is a lower bound on the slots returned. slots >= min_context_slot.
        let Some(base_url) = real_solana_rpc_url() else {
            eprintln!("Skipping real Solana RPC test: set SOLANA_RPC_URL or GETBLOCK_SOLANA_RPC_URL");
            return;
        };

        let client = SolanaClient::new(&base_url);
        let address = "Vote111111111111111111111111111111111111111";

        let seed_cfg = GetSignaturesConfig {
            commitment: Some("finalized".to_string()),
            limit: Some(10),
            ..Default::default()
        };
        let seed = client
            .get_signatures_for_address(address, Some(seed_cfg))
            .await
            .unwrap();
        assert!(seed.len() == 10, "expected enough history to run pagination test");

        // API returns newest -> oldest.
        let candidate_idx = seed.len() - 1;
        let min_slot = seed[candidate_idx].slot;
        if !seed.iter().any(|s| s.slot >= min_slot) {
            eprintln!(
                "Skipping min_context_slot test: seed window does not include any slots below chosen min_slot={}, (candidate_idx={})",
                min_slot, candidate_idx
            );
            assert!(false);
            return;
        }

        let limit: u64 = 12;
        let mut before_with_min: Option<String> = None;
        let mut pages_with_min: usize = 0;
        let mut saw_slot_below_min_with_min_context_slot: bool = false;
        let mut last_slot: Option<u64> = None;

        for _ in 0..100 {
            let cfg = GetSignaturesConfig {
                commitment: Some("finalized".to_string()),
                min_context_slot: Some(min_slot),
                limit: Some(limit),
                before: before_with_min.clone(),
                ..Default::default()
            };

            let page = client
                .get_signatures_for_address(address, Some(cfg))
                .await
                .unwrap();

            if page.is_empty() {
                break;
            }

            pages_with_min += 1;

            assert!(
                page.iter().all(|s| s.slot >= min_slot),
                "expected all returned slots to be >= min_slot when min_context_slot is set; min_slot={} slots={:?}",
                min_slot,
                page.iter().map(|s| s.slot).collect::<Vec<_>>()
            );

            assert!(min_slot <= page.last().unwrap().slot);
            if page.iter().any(|s| s.slot < min_slot) {
                saw_slot_below_min_with_min_context_slot = true;
                break;
            }

            before_with_min = page.last().map(|s| s.signature.clone());

            if page.len() < limit as usize {
                assert_eq!(page.last().unwrap().slot, min_slot);
                last_slot = Some(page.last().unwrap().slot);
                break;
            }

            last_slot = Some(page.last().unwrap().slot);
        }
        assert!(
            pages_with_min >= 2,
            "expected to exercise pagination with min_context_slot; pages_with_min={}",
            pages_with_min
        );
        assert!(
            !saw_slot_below_min_with_min_context_slot,
            "expected min_context_slot to prevent returning slots below min_slot when paginating; min_slot={} pages_with_min={}",
            min_slot,
            pages_with_min
        );

        assert!(
            last_slot.is_some(),
            "expected to have seen last_slot after pagination with min_context_slot"
        );
        assert!(
            last_slot.unwrap() == min_slot,
            "expected pagination with min_context_slot to end exactly at min_slot; last_slot={} min_slot={}",
            last_slot.unwrap(),
            min_slot
        );
    }

    #[tokio::test]
    async fn test_real_get_signatures_for_address_before_until_range_with_pagination() {
        // results between (until < result < before), exclusive
        let Some(base_url) = real_solana_rpc_url() else {
            eprintln!("Skipping real Solana RPC test: set SOLANA_RPC_URL or GETBLOCK_SOLANA_RPC_URL");
            return;
        };

        let client = SolanaClient::new(&base_url);
        let address = "Vote111111111111111111111111111111111111111";

        // Seed a window large enough to pick `before` and `until` that are far apart.
        let seed_cfg = GetSignaturesConfig {
            commitment: Some("finalized".to_string()),
            limit: Some(100),
            ..Default::default()
        };
        let seed = client
            .get_signatures_for_address(address, Some(seed_cfg))
            .await
            .unwrap();
        assert!(seed.len() >= 90, "expected enough history to run range test");

        // API returns newest -> oldest. Choose `before` newer, `until` older.
        let before_idx: usize = 5;
        let until_idx: usize = 80;
        let before_sig = seed[before_idx].signature.clone();
        let until_sig = seed[until_idx].signature.clone();

        let mut sig_to_index: HashMap<String, usize> = HashMap::new();
        for (idx, s) in seed.iter().enumerate() {
            sig_to_index.insert(s.signature.clone(), idx);
        }

        let limit: u64 = 7;
        let mut cursor_sig = before_sig.clone();
        let mut cursor_idx = before_idx;
        let mut collected: Vec<SignatureInfo> = Vec::new();

        for i in 0..50 {
            let cfg = GetSignaturesConfig {
                commitment: Some("finalized".to_string()),
                before: Some(cursor_sig.clone()),
                until: Some(until_sig.clone()),
                limit: Some(limit),
                ..Default::default()
            };

            let page = client
                .get_signatures_for_address(address, Some(cfg))
                .await
                .unwrap();

            if page.is_empty() {
                assert!(
                    i > 0,
                    "expected at least one page of results between (until, before) range"
                );
                break;
            }

            for s in &page {
                assert_ne!(
                    s.signature, before_sig,
                    "before signature must be exclusive and not appear in results"
                );
                assert_ne!(
                    s.signature, until_sig,
                    "until signature must be exclusive and not appear in results"
                );

                let idx = *sig_to_index
                    .get(&s.signature)
                    .expect("expected signature to be within the seeded range");
                assert!(
                    idx > before_idx,
                    "expected idx to be after before_idx (older), got idx={} before_idx={} sig={}",
                    idx,
                    before_idx,
                    s.signature
                );
                assert!(
                    idx < until_idx,
                    "expected idx to be before until_idx (newer), got idx={} until_idx={} sig={}",
                    idx,
                    until_idx,
                    s.signature
                );
                assert!(
                    idx > cursor_idx,
                    "expected pagination to progress to older signatures; got idx={} cursor_idx={} sig={}",
                    idx,
                    cursor_idx,
                    s.signature
                );
            }

            collected.extend(page.clone());

            cursor_sig = page.last().unwrap().signature.clone();
            cursor_idx = *sig_to_index
                .get(&cursor_sig)
                .expect("expected cursor signature to be within the seeded range");

            if page.len() < limit as usize {
                break;
            }
        }

        assert!(
            !collected.is_empty(),
            "expected some results for the (until, before) range"
        );
        assert!(
            collected.len() > limit as usize,
            "expected pagination to be exercised; collected_len={} limit={}",
            collected.len(),
            limit
        );
    }

    #[tokio::test]
    async fn test_get_bridge_action_maybe_with_mock() {
        // Mock server handler for bridge action test
        async fn bridge_handler(
            State(_): State<MockState>,
            Json(body): Json<Value>,
        ) -> (StatusCode, Json<Value>) {
            let method = body
                .get("method")
                .and_then(|m| m.as_str())
                .unwrap_or_default();
            match method {
                "getTransaction" => {
                    // Return a transaction with TokensDeposited event log
                    let rsp = json!({
                        "jsonrpc": "2.0",
                        "id": 1,
                        "result": {
                            "slot": 123456,
                            "transaction": {
                                "signatures": ["test_signature"]
                            },
                            "meta": {
                                "err": null,
                                "fee": 5000,
                                "logMessages": [
                                    "Program FVaTThSeeX4G5dHXqhTdqby9W6u77WDRMtBnfUpQasHm invoke [1]",
                                    "Program log: Instruction: MockCross",
                                    "Program log: emit TokensDeposited",
                                    "Program data: xNnHWCN1PGABAAAAAAAAADMBAwAAAAAAAABAQg8AAAAAAOXaYE6RS0pYLywuTnxDVWpNC5vNpLb2ZR5oXqMkmrq8IAAAAK6o6kznyCufMoNfXO4QV6GdxnPPcbMT248r8B8cx6ke",
                                    "Program FVaTThSeeX4G5dHXqhTdqby9W6u77WDRMtBnfUpQasHm consumed 1333 of 200000 compute units",
                                    "Program FVaTThSeeX4G5dHXqhTdqby9W6u77WDRMtBnfUpQasHm success"
                                ]
                            }
                        }
                    });
                    (StatusCode::OK, Json(rsp))
                }
                _ => {
                    let rsp = json!({ "jsonrpc": "2.0", "id": 1, "error": { "code": -32601, "message": "Method not found" }});
                    (StatusCode::OK, Json(rsp))
                }
            }
        }

        let state = MockState;
        let app = Router::new()
            .route("/", post(bridge_handler))
            .with_state(state);
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        let local_addr = listener.local_addr().unwrap();
        let (tx, rx) = oneshot::channel();
        let handle = tokio::spawn(async move {
            let server = axum::serve(listener, app);
            tokio::select! {
                _ = server => {},
                _ = rx => {},
            }
        });

        let base_url = format!("http://{local_addr}/");
        let client = SolanaClient::new(&base_url);

        // Test get_bridge_action_maybe
        let result = client.get_bridge_action_maybe("test_signature", 0).await;
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);

        let action = result.unwrap();
        match action {
            BridgeAction::SolanaToSuiBridgeAction(solana_action) => {
                println!("SolanaToSuiBridgeAction: {:?}", solana_action);
                assert_eq!(solana_action.solana_tx_signature, "test_signature");
                assert_eq!(solana_action.solana_event_index, 0);
            }
            _ => panic!("Expected SolanaToSuiBridgeAction"),
        }

        tx.send(()).ok();
        handle.abort();
    }

    #[tokio::test]
    async fn test_get_bridge_action_maybe_no_events() {
        // Mock server handler that returns transaction without bridge events
        async fn no_event_handler(
            State(_): State<MockState>,
            Json(body): Json<Value>,
        ) -> (StatusCode, Json<Value>) {
            let method = body
                .get("method")
                .and_then(|m| m.as_str())
                .unwrap_or_default();
            match method {
                "getTransaction" => {
                    let rsp = json!({
                        "jsonrpc": "2.0",
                        "id": 1,
                        "result": {
                            "slot": 123456,
                            "transaction": {
                                "signatures": ["test_signature"]
                            },
                            "meta": {
                                "err": null,
                                "fee": 5000,
                                "logMessages": [
                                    "Program log: some other log"
                                ]
                            }
                        }
                    });
                    (StatusCode::OK, Json(rsp))
                }
                _ => {
                    let rsp = json!({ "jsonrpc": "2.0", "id": 1, "error": { "code": -32601, "message": "Method not found" }});
                    (StatusCode::OK, Json(rsp))
                }
            }
        }

        let state = MockState;
        let app = Router::new()
            .route("/", post(no_event_handler))
            .with_state(state);
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        let local_addr = listener.local_addr().unwrap();
        let (tx, rx) = oneshot::channel();
        let handle = tokio::spawn(async move {
            let server = axum::serve(listener, app);
            tokio::select! {
                _ = server => {},
                _ = rx => {},
            }
        });

        let base_url = format!("http://{local_addr}/");
        let client = SolanaClient::new(&base_url);

        // Test get_bridge_action_maybe with no events
        let result = client.get_bridge_action_maybe("test_signature", 0).await;
        assert!(matches!(result, Err(BridgeError::NoBridgeEventsInTxPosition)));

        tx.send(()).ok();
        handle.abort();
    }

    /// Test parsing real Solana log data provided by user
    #[test]
    fn test_parse_real_solana_log() {
        // Real log data from user:
        // Program FVaTThSeeX4G5dHXqhTdqby9W6u77WDRMtBnfUpQasHm invoke [1]
        // Program log: Instruction: MockCross
        // In Program log: emit TokensDeposited
        // Program data: xNnHWCN1PGABAAAAAAAAAD0CAwAAAAAAAABAQg8AAAAAAOXaYE6RS0pYLywuTnxDVWpNC5vNpLb2ZR5oXqMkmrq8IAAAAK6o6kznyCufMoNfXO4QV6GdxnPPcbMT248r8B8cx6ke
        // Program FVaTThSeeX4G5dHXqhTdqby9W6u77WDRMtBnfUpQasHm consumed 1333 of 200000 compute units
        // Program FVaTThSeeX4G5dHXqhTdqby9W6u77WDRMtBnfUpQasHm success

        let base64_data = "xNnHWCN1PGABAAAAAAAAAD0CAwAAAAAAAABAQg8AAAAAAOXaYE6RS0pYLywuTnxDVWpNC5vNpLb2ZR5oXqMkmrq8IAAAAK6o6kznyCufMoNfXO4QV6GdxnPPcbMT248r8B8cx6ke";
        let log_msg = format!("Program data: {}", base64_data);

        let events = SolanaBridgeEvent::test_try_from_logs(&log_msg);
        assert_eq!(events.len(), 1, "Should parse exactly one event");

        match &events[0] {
            SolanaBridgeEvent::TokensDeposited(tokens_deposited) => {
                // Verify discriminator (TokensDeposited)
                assert_eq!(
                    tokens_deposited.discriminator,
                    [196, 217, 199, 88, 35, 117, 60, 96],
                    "Discriminator should match TokensDeposited"
                );

                // Verify parsed fields
                assert_eq!(tokens_deposited.nonce, 1, "nonce should be 1");
                assert_eq!(tokens_deposited.source_chain_id, 61, "source_chain_id should be 61 (0x3D)");
                assert_eq!(tokens_deposited.target_chain_id, 2, "target_chain_id should be 2");
                assert_eq!(tokens_deposited.token_id, 3, "token_id should be 3");
                assert_eq!(tokens_deposited.amount, 1000000, "amount should be 1000000");
                
                // Print actual values for verification
                println!("== Parsed TokensDeposited ==");
                println!("nonce: {}", tokens_deposited.nonce);
                println!("source_chain_id: {}", tokens_deposited.source_chain_id);
                println!("target_chain_id: {}", tokens_deposited.target_chain_id);
                println!("token_id: {}", tokens_deposited.token_id);
                println!("amount: {}", tokens_deposited.amount);
                println!("sender_address(base58): {}", tokens_deposited.sender_base58());
                println!("recipient_length: {}", tokens_deposited.recipient_length);
                println!("recipient_address(hex): {}", tokens_deposited.recipient_hex());

                // Verify recipient length
                assert_eq!(tokens_deposited.recipient_length, 32, "recipient_length should be 32");
                assert_eq!(tokens_deposited.recipient_bytes.len(), 32, "recipient_bytes should be 32 bytes");
            }
            SolanaBridgeEvent::RawEvent { discriminator, .. } => {
                panic!("Expected TokensDeposited event, got RawEvent with discriminator: {:?}", discriminator);
            }
        }
    }
}
