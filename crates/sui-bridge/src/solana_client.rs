use anyhow::anyhow;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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
    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::routing::post;
    use axum::{Json, Router};
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
}
