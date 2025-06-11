use serde::{Deserialize, Serialize};
use sui_types::bridge::BridgeChainId;
use tracing::{error, info};

const MAINNET_URL: &str = "https://go.getblock.io/249475ea717441e8b4ddd7d5aa4f5dfc";
const TESTNET_URL: &str = MAINNET_URL;

// USDC & USDT mint addresses on mainnet
const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const USDT_MINT: &str = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";

#[derive(Debug, Serialize)]
struct JsonRpcRequest<T> {
    jsonrpc: String,
    method: String,
    params: T,
    id: String,
}

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
    #[serde(default, rename = "preTokenBalances")]
    pre_token_balances: Vec<TokenBalance>,
    #[serde(default, rename = "postTokenBalances")]
    post_token_balances: Vec<TokenBalance>,
}

#[derive(Debug, Deserialize)]
struct UiTokenAmount {
    #[serde(rename = "amount")]
    amount: String, // Use string to avoid u64 overflow
}

#[derive(Debug, Deserialize)]
struct TokenBalance {
    #[serde(rename = "accountIndex")]
    account_index: usize,
    #[serde(rename = "mint")]
    mint: String,
    #[serde(rename = "owner")]
    owner: String,
    #[serde(rename = "uiTokenAmount")]
    ui_token_amount: UiTokenAmount,
}


pub async fn check_solana_txn(
    chain_id: BridgeChainId,
    tx_hash: &str,
    whitelist: Vec<String>,
    expected_amount: u64,
    native_token: bool,
) -> bool {
    info!(
        "Checking solana txn: chain_id: {:?}, tx_hash: {}, whitelist: {:?}, expected_amount: {}",
        chain_id, tx_hash, whitelist, expected_amount
    );

    let base_url = match chain_id {
        BridgeChainId::SolanaMainnet => MAINNET_URL,
        BridgeChainId::SolanaTestnet => TESTNET_URL,
        _ => {
            error!("Unsupported Solana chain id: {:?}", chain_id);
            return false;
        }
    };

    let request_body = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "getTransaction".to_string(),
        params: serde_json::json!([
            tx_hash,
            {
                "encoding": "json",
                "commitment": "finalized"
            }
        ]),
        id: "getblock.io".to_string(),
    };

    let client = reqwest::Client::new();
    let res = match client.post(base_url)
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

    let text = res.text().await.unwrap_or_default();
    info!("GetBlock response: {}", text);

    let response: JsonRpcResponse<SolanaTransaction> = match serde_json::from_str(&text) {
        Ok(resp) => resp,
        Err(e) => {
            error!("Failed to parse response from getblock: {:?}", e);
            return false;
        }
    };
    info!("Parsed response: {:#?}", response);

    let Some(transaction) = response.result else {
        error!("No result in getblock response for tx: {}", tx_hash);
        return false;
    };

    let (Some(tx_data), Some(meta)) = (transaction.transaction, transaction.meta) else {
        error!("Missing transaction or meta field in getblock response for tx: {}", tx_hash);
        return false;
    };

    if !native_token {
        // check SPL Token (USDC/USDT)
        for post_token in &meta.post_token_balances {
            if post_token.mint != USDC_MINT && post_token.mint != USDT_MINT {
                continue;
            }
            let owner = &post_token.owner;
            if !whitelist.contains(owner) {
                continue;
            }
            let pre_token = meta.pre_token_balances.iter().find(|t| t.account_index == post_token.account_index && t.mint == post_token.mint && &t.owner == owner);
            let pre_amount: u64 = pre_token.map(|t| t.ui_token_amount.amount.parse().unwrap_or(0)).unwrap_or(0);
            let post_amount: u64 = post_token.ui_token_amount.amount.parse().unwrap_or(0);
            info!("Token {} account owner: {}, pre: {}, post: {}", post_token.mint, owner, pre_amount, post_amount);
            if post_amount > pre_amount {
                let received_amount = post_amount - pre_amount;
                if received_amount == expected_amount {
                    info!("Solana {} transaction {} verified successfully for owner {}.", post_token.mint, tx_hash, owner);
                    return true;
                } else {
                    error!("{} Amount mismatch for owner {}: expected {}, got {}", post_token.mint, owner, expected_amount, received_amount);
                    return false;
                }
            }
        }
    } else {
        // check SOL
        for (i, key) in tx_data.message.account_keys.iter().enumerate() {
            info!("Account key: {}", key);
            if whitelist.contains(key) {
                info!("Found whitelisted address: {}", key);
                let pre_balance = meta.pre_balances.get(i).cloned().unwrap_or(0);
                let post_balance = meta.post_balances.get(i).cloned().unwrap_or(0);
                info!("Pre balance: {}, post balance: {}", pre_balance, post_balance);
                if post_balance > pre_balance {
                    let received_amount = post_balance - pre_balance;
                    if received_amount == expected_amount {
                        info!("Solana transaction {} verified successfully for address {}.", tx_hash, key);
                        return true;
                    } else {
                        error!("Amount mismatch for address {}: expected {}, got {}", key, expected_amount, received_amount);
                        return false;
                    }
                }
            }
        }
    }

    error!("No whitelisted address found as receiver in transaction {}", tx_hash);
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use tracing_test::traced_test;

    #[traced_test]
    #[tokio::test]
    async fn test_check_solana_txn_sol() {
        let tx_hash = "3cEXCHqLqNJHx3XbjpLgGRwecyV9QaDbMipsy9k7uMYY5H2BRX1ABgT8QwTFW7eLPBMJHDa5c6unAtifQqgixvLy";
        let to_address = "EhXz9TGdmupxiToVtjiJvotPicimamNuJR5BmXCKR8nq".to_string();
        let whitelist = vec![to_address];
        let amount: u64 = 1_0000000;

        let result = check_solana_txn(
            BridgeChainId::SolanaMainnet,
            tx_hash,
            whitelist,
            amount,
            true,
        ).await;
        assert!(result, "Transaction verification failed");
    }

    #[traced_test]
    #[tokio::test]
    async fn test_check_solana_txn_benfen() {
        // https://solscan.io/tx/26hmqjiz9PSeXtCKPquUREAvF4VuMUJdSTpUs4dFehNu2tfMGWPMJmvCNfsZxjgYwqKJ9Ymf3vJWSiUVZX2xKDJU
        let tx_hash = "26hmqjiz9PSeXtCKPquUREAvF4VuMUJdSTpUs4dFehNu2tfMGWPMJmvCNfsZxjgYwqKJ9Ymf3vJWSiUVZX2xKDJU";
        let to_address = "93rJovDUhd1Fn24Ue5TdgKQ5yTbCBM5S5a8uPcr9eZiu".to_string();
        let whitelist = vec![to_address];
        let amount: u64 = 1_000;

        let result = check_solana_txn(
            BridgeChainId::SolanaMainnet,
            tx_hash,
            whitelist,
            amount,
            false,
        ).await;
        assert!(result, "Transaction verification failed");

        // benfen test in mainnet
        let tx_hash = "26hmqjiz9PSeXtCKPquUREAvF4VuMUJdSTpUs4dFehNu2tfMGWPMJmvCNfsZxjgYwqKJ9Ymf3vJWSiUVZX2xKDJU";
        let to_address = "93rJovDUhd1Fn24Ue5TdgKQ5yTbCBM5S5a8uPcr9eZiu".to_string();
        let whitelist = vec![to_address];
        let amount: u64 = 1_000;

        let result = check_solana_txn(
            BridgeChainId::SolanaTestnet,
            tx_hash,
            whitelist,
            amount,
            false,
        ).await;
        assert!(result, "Transaction verification failed");
    }

    #[traced_test]
    #[tokio::test]
    async fn test_check_solana_txn_real() {
        // USDC
        // https://solscan.io/tx/3Szcy1koGpMe1cKjHDamG587BRAkTGGKwT9GR6w4NCe6VSbHUwcCKa8xdfhtJoWVfqFL4S6dvsKyqd55oFvVEY2C
        let tx_hash = "3Szcy1koGpMe1cKjHDamG587BRAkTGGKwT9GR6w4NCe6VSbHUwcCKa8xdfhtJoWVfqFL4S6dvsKyqd55oFvVEY2C";
        let to_address = "EUpoDjWzaxfhKBfHHLuabJSwp9nXDYJqxTuxLn454yPk".to_string();
        let whitelist = vec![to_address];
        let amount: u64 = 1340000;

        let result = check_solana_txn(
            BridgeChainId::SolanaMainnet,
            tx_hash,
            whitelist,
            amount,
            false,
        ).await;

        assert!(result, "Transaction verification failed");

        // USDT
        // https://solscan.io/tx/24e6A7DxZAqzxNixALDf8nFyB7aNPHv5VLtAyrZY7pdW47RYeA8YoZAZEqxH2xLAYmnCJvkAwUy8kgoDNVS4EFpS
        let tx_hash = "24e6A7DxZAqzxNixALDf8nFyB7aNPHv5VLtAyrZY7pdW47RYeA8YoZAZEqxH2xLAYmnCJvkAwUy8kgoDNVS4EFpS";
        let to_address = "Cv1u7R1xdbNrHroRZGY5U1WRXMZTnBB32FDSFi6zVq1Y".to_string();
        let whitelist = vec![to_address];
        let amount: u64 = 3000000000;

        let result = check_solana_txn(
            BridgeChainId::SolanaMainnet,
            tx_hash,
            whitelist,
            amount,
            false,
        ).await;

        assert!(result, "Transaction verification failed");
    }
}