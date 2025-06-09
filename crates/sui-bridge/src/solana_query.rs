use serde::{Deserialize, Serialize};
use sui_types::bridge::BridgeChainId;
use tracing::{error, info};

// API Key from environment variable
fn get_getblock_api_key() -> Result<String, std::env::VarError> {
    std::env::var("GETBLOCK_API_KEY")
}

#[derive(Debug, Serialize)]
struct JsonRpcRequest<T> {
    jsonrpc: String,
    method: String,
    params: T,
    id: String,
}

// Solana交易响应结构定义
#[derive(Debug, Deserialize)]
struct JsonRpcResponse<T> {
    jsonrpc: String,
    id: String,
    result: Option<T>,
}

#[derive(Debug, Deserialize)]
struct SolanaTransaction {
    #[serde(default)]
    transaction: Option<SolanaTransactionData>,
    #[serde(default)]
    meta: Option<SolanaTransactionMeta>,
    #[serde(default)]
    slot: Option<u64>,
    #[serde(default)]
    block_time: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct SolanaTransactionData {
    #[serde(rename = "message")]
    message: SolanaTransactionMessage,
    #[serde(rename = "signatures")]
    signatures: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SolanaTransactionMessage {
    #[serde(rename = "accountKeys")]
    account_keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SolanaTransactionMeta {
    #[serde(rename = "preBalances")]
    pre_balances: Vec<u64>,
    #[serde(rename = "postBalances")]
    post_balances: Vec<u64>,
}

/// 检查 Solana 交易
/// 验证交易中的余额变化是否符合预期，接收地址是否在白名单中
pub async fn check_solana_txn(
    chain_id: BridgeChainId,
    tx_hash: &str,
    whitelist: Vec<String>,
    expected_amount: u64,
) -> bool {
    info!(
        "Checking solana txn: chain_id: {:?}, tx_hash: {}, whitelist: {:?}, expected_amount: {}",
        chain_id, tx_hash, whitelist, expected_amount
    );

    let api_key = match get_getblock_api_key() {
        Ok(key) => key,
        Err(_) => {
            error!("GETBLOCK_API_KEY not set");
            return false;
        }
    };

    // 确定正确的网络URL
    let network = match chain_id {
        BridgeChainId::SolanaMainnet => "mainnet",
        BridgeChainId::SolanaTestnet => "testnet",
        _ => {
            error!("Unsupported Solana chain id: {:?}", chain_id);
            return false;
        }
    };

    // 构建API URL，格式为 https://go.getblock.io/{API_KEY}/mainnet
    let url = format!("https://go.getblock.io/{}/{}", api_key, network);

    // 创建请求对象
    let request_body = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "getTransaction".to_string(), // 正确的Solana API方法名
        params: serde_json::json!([
            tx_hash,
            {
                "encoding": "json",
                "commitment": "finalized" // 确保交易已最终确认
            }
        ]),
        id: "getblock.io".to_string(),
    };

    // 发送请求
    let client = reqwest::Client::new();
    let res = match client.post(&url)
        .json(&request_body)
        .send()
        .await {
        Ok(res) => res,
        Err(e) => {
            error!("Failed to send request to getblock: {:?}", e);
            return false;
        }
    };

    if !res.status().is_success() {
        error!("GetBlock request failed with status: {}", res.status());
        let body = res.text().await.unwrap_or_default();
        error!("Response body: {}", body);
        return false;
    }

    // 解析响应
    let text = res.text().await.unwrap_or_default();
    info!("GetBlock response: {}", text);

    let response: JsonRpcResponse<SolanaTransaction> = match serde_json::from_str(&text) {
        Ok(resp) => resp,
        Err(e) => {
            error!("Failed to parse response from getblock: {:?}", e);
            return false;
        }
    };

    let Some(transaction) = response.result else {
        error!("No result in getblock response for tx: {}", tx_hash);
        return false;
    };

    let (Some(tx_data), Some(meta)) = (transaction.transaction, transaction.meta) else {
        error!("Missing transaction or meta field in getblock response for tx: {}", tx_hash);
        return false;
    };

    // 分析交易数据，查找白名单中的地址是否是接收方，并验证金额变动
    for (i, key) in tx_data.message.account_keys.iter().enumerate() {
        if whitelist.contains(key) {
            // 获取地址在交易前后的余额
            let pre_balance = meta.pre_balances.get(i).cloned().unwrap_or(0);
            let post_balance = meta.post_balances.get(i).cloned().unwrap_or(0);

            // 如果余额增加了，说明这个地址是接收者
            if post_balance > pre_balance {
                let received_amount = post_balance - pre_balance;
                // 验证接收金额是否符合预期
                if received_amount == expected_amount {
                    info!("Solana transaction {} verified successfully for address {}.", tx_hash, key);
                    return true;
                } else {
                    error!(
                        "Amount mismatch for address {}: expected {}, got {}",
                        key, expected_amount, received_amount
                    );
                    return false;
                }
            }
        }
    }

    error!(
        "No whitelisted address found as receiver in transaction {}",
        tx_hash
    );
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_check_solana_txn_real() {
        // 这是一个真实的交易ID，需要替换为实际存在的交易
        let tx_hash = "your_real_transaction_hash";
        let to_address = "real_recipient_address".to_string();
        let whitelist = vec![to_address];
        let amount: u64 = 1_000_000_000; // 假设金额为1 SOL (1,000,000,000 Lamports)

        let result = check_solana_txn(
            BridgeChainId::SolanaMainnet,
            tx_hash,
            whitelist,
            amount,
        ).await;

        assert!(result, "Transaction verification failed");
    }
}