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

// eth_getTransactionByHash 返回结构体
#[derive(Debug, Deserialize)]
struct EthTxByHashResponse {
    jsonrpc: String,
    id: String,
    result: Option<EthTransaction>,
}

#[derive(Debug, Deserialize)]
struct EthTransaction {
    hash: String,
    from: String,
    to: Option<String>,
    value: String, // 十六进制字符串
    blockHash: Option<String>,
    blockNumber: Option<String>,
    transactionIndex: Option<String>,
    gas: Option<String>,
    gasPrice: Option<String>,
    input: Option<String>,
    nonce: Option<String>,
    v: Option<String>,
    r: Option<String>,
    s: Option<String>,
}

/// 检查 Tron 交易
/// 验证交易是否为有效的转账交易，交易金额是否符合预期，接收地址是否在白名单中
pub async fn check_tron_txn(
    chain_id: BridgeChainId,
    tx_hash: &str,
    whitelist: Vec<String>,
    expected_amount: u64,
) -> bool {
    info!(
        "Checking tron txn: chain_id: {:?}, tx_hash: {}, whitelist: {:?}, expected_amount: {}",
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
        BridgeChainId::TronMainnet => "mainnet",
        BridgeChainId::TronTestnet => "testnet", // Shasta测试网络
        _ => {
            error!("Unsupported Tron chain id: {:?}", chain_id);
            return false;
        }
    };
    
    // 构建API URL，格式为 https://go.getblock.io/{API_KEY}/jsonrpc
    let url = format!("https://go.getblock.io/{}/jsonrpc", api_key);

    // 创建请求对象
    let request_body = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "eth_getTransactionByHash".to_string(),
        params: vec![tx_hash.to_string()],
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

    let parsed: EthTxByHashResponse = match serde_json::from_str(&text) {
        Ok(resp) => resp,
        Err(e) => {
            error!("Failed to parse eth_getTransactionByHash response: {:?}", e);
            return false;
        }
    };

    let tx = match parsed.result {
        Some(tx) => tx,
        None => {
            error!("No transaction found for hash: {}", tx_hash);
            return false;
        }
    };

    // 验证 to 地址是否在白名单
    let to_addr = match tx.to {
        Some(addr) => addr,
        None => {
            error!("Transaction has no 'to' address");
            return false;
        }
    };
    let is_whitelisted = whitelist.iter().any(|addr| addr.eq_ignore_ascii_case(&to_addr));
    if !is_whitelisted {
        error!("Receiver address {} is not in whitelist", to_addr);
        return false;
    }

    // 验证金额（value为16进制字符串，单位为wei）
    let value = match u64::from_str_radix(tx.value.trim_start_matches("0x"), 16) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to parse value: {:?}", e);
            return false;
        }
    };
    if value != expected_amount {
        error!("Amount mismatch: expected {}, got {}", expected_amount, value);
        return false;
    }

    info!("Tron transaction {} verified successfully.", tx_hash);
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_check_tron_txn_real() {
        // 这是一个真实的交易ID，需要替换为实际存在的交易
        let tx_hash = "your_real_transaction_hash";
        let to_address = "real_recipient_address".to_string();
        let whitelist = vec![to_address];
        let amount: u64 = 1000000; // 假设金额为1 TRX (1,000,000 SUN)

        let result = check_tron_txn(
            BridgeChainId::TronMainnet,
            tx_hash,
            whitelist,
            amount,
        ).await;

        assert!(result, "Transaction verification failed");
    }
}