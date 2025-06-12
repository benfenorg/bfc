use serde::{Deserialize, Serialize};
use sui_types::bridge::BridgeChainId;
use tracing::{debug, error, info};
use bs58;
use sha2::{Sha256, Digest};
use hex;

const MAINNET_URL: &str = "https://go.getblock.io/8703fc5554244851be7ee8d84c338177";
const TESTNET_URL: &str = "https://api.shasta.trongrid.io";

#[derive(Debug, Serialize)]
struct JsonRpcRequest<T> {
    jsonrpc: String,
    method: String,
    params: T,
    id: String,
}

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
    value: String,
    block_hash: Option<String>,
    block_number: Option<String>,
    transaction_index: Option<String>,
    gas: Option<String>,
    gas_price: Option<String>,
    input: Option<String>,
    nonce: Option<String>,
    v: Option<String>,
    r: Option<String>,
    s: Option<String>,
}

fn parse_trc20_transfer_input(input: &str) -> Option<(String, u64)> {
    // TRC20 transfer input:
    // 0xa9059cbb + recipient address (32 bytes) + amount (32 bytes)
    // for example: 0xa9059cbb000000000000000000000000<recipient address>000000000000000000000000<amount>
    // 0xa9059cbb00000000000000000000000010800ca2a4458149eb947eafed8b2c99cf96a10d000000000000000000000000000000000000000000000000000000001dcd6500

    if !input.starts_with("0xa9059cbb") || input.len() < 138 {
        return None;
    }

    let address_hex = &input[10..74];
    let actual_address = format!("0x{}", &address_hex[24..]);

    let amount_hex = &input[74..138];
    let amount = match u64::from_str_radix(amount_hex, 16) {
        Ok(v) => v,
        Err(_) => return None,
    };

    Some((actual_address, amount))
}

pub async fn check_tron_txn(
    chain_id: BridgeChainId,
    tx_hash: &str,
    whitelist: Vec<String>,
    expected_amount: u64,
    native_token: bool,
) -> bool {
    info!(
        "Checking tron txn: chain_id: {:?}, tx_hash: {}, whitelist: {:?}, expected_amount: {}",
        chain_id, tx_hash, whitelist, expected_amount
    );

    let base_url = match chain_id {
        BridgeChainId::TronMainnet => MAINNET_URL,
        BridgeChainId::TronTestnet => TESTNET_URL,
        _ => {
            error!("Unsupported Tron chain id: {:?}", chain_id);
            return false;
        }
    };
    let url = format!("{}/jsonrpc", base_url);

    let request_body = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "eth_getTransactionByHash".to_string(),
        params: vec![tx_hash.to_string()],
        id: "getblock.io".to_string(),
    };

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

    if !native_token {
        // check TRC20
        if let Some(input) = &tx.input {
            if let Some((recipient, amount)) = parse_trc20_transfer_input(input) {
                let is_whitelisted = whitelist.iter().any(|whitelist_addr| {
                    is_address_match(whitelist_addr, &recipient)
                });

                if !is_whitelisted {
                    error!("TRC20 recipient address {} is not in whitelist", recipient);
                    return false;
                }

                if amount != expected_amount {
                    error!("TRC20 amount mismatch: expected {}, got {}", expected_amount, amount);
                    return false;
                }

                info!("Tron TRC20 transaction {} verified successfully.", tx_hash);
                return true;
            }
        }
    } else {
        // check TRX
        if let Some(to_addr) = &tx.to {
            let value = match u64::from_str_radix(tx.value.trim_start_matches("0x"), 16) {
                Ok(v) => v,
                Err(e) => {
                    error!("Failed to parse value: {:?}", e);
                    return false;
                }
            };

            if value != expected_amount {
                error!("TRX amount mismatch: expected {}, got {}", expected_amount, value);
                return false;
            }

            let is_whitelisted = whitelist.iter().any(|whitelist_addr| {
                is_address_match(whitelist_addr, to_addr)
            });
            if is_whitelisted {
                info!("Tron native transaction {} verified successfully.", tx_hash);
                return true;
            }
        }
    }

    error!("Transaction {} is neither a valid TRX transfer nor a valid TRC20 transfer", tx_hash);
    false
}

pub fn tron_to_eth_address(tron_address: &str) -> Option<String> {
    if !tron_address.starts_with('T') {
        return None;
    }

    let decoded = match bs58::decode(tron_address).into_vec() {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Failed to decode Tron address: {:?}", e);
            return None;
        }
    };

    if decoded.len() != 25 {
        error!("Invalid Tron address length: {}", decoded.len());
        return None;
    }

    if decoded[0] != 0x41 {
        error!("Invalid Tron address prefix: {:x}", decoded[0]);
        return None;
    }

    let mut hasher = Sha256::new();
    hasher.update(&decoded[0..21]);
    let hash1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(&hash1);
    let hash2 = hasher.finalize();

    for i in 0..4 {
        if decoded[21 + i] != hash2[i] {
            error!("Checksum verification failed for Tron address");
            return None;
        }
    }

    let eth_address = format!("0x{}", hex::encode(&decoded[1..21]));
    Some(eth_address)
}

pub fn eth_to_tron_address(eth_address: &str) -> Option<String> {
    if !eth_address.starts_with("0x") {
        return None;
    }

    let hex_str = &eth_address[2..];
    let eth_bytes = match hex::decode(hex_str) {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Failed to decode Ethereum address: {:?}", e);
            return None;
        }
    };

    if eth_bytes.len() != 20 {
        error!("Invalid Ethereum address length: {}", eth_bytes.len());
        return None;
    }

    let mut tron_bytes = vec![0x41];
    tron_bytes.extend_from_slice(&eth_bytes);

    let mut hasher = Sha256::new();
    hasher.update(&tron_bytes);
    let hash1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(&hash1);
    let hash2 = hasher.finalize();

    let mut address_with_checksum = tron_bytes.clone();
    address_with_checksum.extend_from_slice(&hash2[0..4]);

    let tron_address = bs58::encode(address_with_checksum).into_string();
    Some(tron_address)
}

pub fn is_address_match(address1: &str, address2: &str) -> bool {
    if address1.eq_ignore_ascii_case(address2) {
        debug!("Addresses match: {} and {}", address1, address2);
        return true;
    }

    if address1.starts_with('T') {
        if let Some(eth_addr) = tron_to_eth_address(address1) {
            debug!("Try convert tron address to eth address: Addresses match: {} and {}", eth_addr, address2);
            if eth_addr.eq_ignore_ascii_case(address2) {
                return true;
            }
        }
    }

    if address1.starts_with("0x") {
        if let Some(tron_addr) = eth_to_tron_address(address1) {
            debug!("Try convert eth address to tron address: Addresses match: {} and {}", tron_addr, address2);
            if tron_addr.eq_ignore_ascii_case(address2) {
                return true;
            }
        }
    }

    false
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use tracing_test::traced_test;

    #[traced_test]
    #[tokio::test]
    async fn test_check_tron_trx_txn_testnet() {
        let tx_hash = "8e38c6374a6cd2840218e3f68d42ce63dea83f62c919eb603b286e931325e2b5";
        let to_address = "TBuPPYvuanuSdCSMafrWAfRhyTr4NWe8KU".to_string();
        let whitelist = vec![to_address];
        let amount: u64 = 1_000000;

        let result = check_tron_txn(
            BridgeChainId::TronTestnet,
            tx_hash,
            whitelist,
            amount,
            false,
        ).await;

        assert!(result, "TRX transaction verification failed");
    }

    #[traced_test]
    #[tokio::test]
    async fn test_check_tron_trx_txn() {
        // https://tronscan.org/#/transaction/2d6333be3786c53895df14b7fd1222f3fe9d9522770abdb50f2c869772739d35
        let tx_hash = "2d6333be3786c53895df14b7fd1222f3fe9d9522770abdb50f2c869772739d35";
        let to_address = "TSxxRXrcwWe1zzAizx4uJogZeVv5555555".to_string();
        let whitelist = vec![to_address];
        let amount: u64 = 3_000000;

        let result = check_tron_txn(
            BridgeChainId::TronMainnet,
            tx_hash,
            whitelist,
            amount,
            true,
        ).await;

        assert!(result, "TRX transaction verification failed");
    }

    #[traced_test]
    #[tokio::test]
    async fn test_check_tron_txn_real() {
        let tx_hash = "aa05a6faef0ddb124482028084007a71ef8a80540eca957741bb23442e47b3cd";
        let to_address = "TBUTD9rrESu8A1Q4za6qCsv92vdA58Y4BS".to_string();
        let whitelist = vec![to_address];
        let amount: u64 = 500_000000;

        let result = check_tron_txn(
            BridgeChainId::TronMainnet,
            tx_hash,
            whitelist,
            amount,
            false,
        ).await;

        assert!(result, "Transaction verification failed");

        let tx_hash = "aa05a6faef0ddb124482028084007a71ef8a80540eca957741bb23442e47b3cd";
        let to_address = "0x10800ca2a4458149eb947eafed8b2c99cf96a10d".to_string();
        let whitelist = vec![to_address];
        let amount: u64 = 500000000; // 0x1dcd6500

        let result = check_tron_txn(
            BridgeChainId::TronMainnet,
            tx_hash,
            whitelist,
            amount,
            false,
        ).await;

        assert!(result, "TRC20 transaction verification failed");
    }

    #[traced_test]
    #[tokio::test]
    async fn test_check_tron_txn_with_eth_address_in_whitelist() {
        let tx_hash = "aa05a6faef0ddb124482028084007a71ef8a80540eca957741bb23442e47b3cd";
        let to_address = "0x10800CA2A4458149eb947eAFeD8B2c99cF96a10D".to_string();
        let whitelist = vec![to_address];
        let amount: u64 = 500000000;

        let result = check_tron_txn(
            BridgeChainId::TronMainnet,
            tx_hash,
            whitelist,
            amount,
            false,
        ).await;

        assert!(result, "Transaction verification with ETH address in whitelist failed");
    }

    #[traced_test]
    #[tokio::test]
    async fn test_check_tron_txn_with_mixed_whitelist() {
        let tx_hash = "aa05a6faef0ddb124482028084007a71ef8a80540eca957741bb23442e47b3cd";
        let whitelist = vec![
            "TBUTD9rrESu8A1Q4za6qCsv92vdA58Y4BS".to_string(),
            "0x0000000000000000000000000000000000000001".to_string(),
            "0x10800CA2A4458149eb947eAFeD8B2c99cF96a10D".to_string(),
        ];
        let amount: u64 = 500000000;

        let result = check_tron_txn(
            BridgeChainId::TronMainnet,
            tx_hash,
            whitelist,
            amount,
            false,
        ).await;

        assert!(result, "Transaction verification with mixed whitelist failed");
    }

    #[test]
    fn test_parse_trc20_transfer_input() {
        let input = "0xa9059cbb00000000000000000000000010800ca2a4458149eb947eafed8b2c99cf96a10d000000000000000000000000000000000000000000000000000000001dcd6500";

        let (recipient, amount) = parse_trc20_transfer_input(input).unwrap();
        assert_eq!(recipient, "0x10800ca2a4458149eb947eafed8b2c99cf96a10d");
        assert_eq!(amount, 500000000);

        let invalid_input = "0x12345678";
        assert!(parse_trc20_transfer_input(invalid_input).is_none());
    }

    #[test]
    fn test_eth_tron_conversion() {
        let eth_address = "0x10800CA2A4458149eb947eAFeD8B2c99cF96a10D";
        let expected_tron = "TBUTD9rrESu8A1Q4za6qCsv92vdA58Y4BS";

        let converted = eth_to_tron_address(eth_address).unwrap();
        assert_eq!(converted, expected_tron, "ETH to Tron conversion failed for standard address");

        let eth_address_upper = "0x10800CA2A4458149EB947EAFED8B2C99CF96A10D";
        let converted = eth_to_tron_address(eth_address_upper).unwrap();
        assert_eq!(converted, expected_tron, "ETH to Tron conversion failed for uppercase address");

        let invalid_eth = "0x123";
        assert!(eth_to_tron_address(invalid_eth).is_none(), "Should reject invalid ETH address");

        let non_eth = "not-an-eth-address";
        assert!(eth_to_tron_address(non_eth).is_none(), "Should reject non-ETH format address");
    }

    #[test]
    fn test_tron_to_eth_conversion() {
        let tron_address = "TBUTD9rrESu8A1Q4za6qCsv92vdA58Y4BS";
        let expected_eth = "0x10800CA2A4458149eb947eAFeD8B2c99cF96a10D";

        let converted = tron_to_eth_address(tron_address).unwrap();
        assert_eq!(converted.to_uppercase(), expected_eth.to_uppercase(), "Tron to ETH conversion failed for standard address");

        let invalid_tron = "T123!@#";
        assert!(tron_to_eth_address(invalid_tron).is_none(), "Should reject invalid Tron address");

        let non_tron = "0xa614f803b6fd780986a42c78ec9c7f77e6ded13c";
        assert!(tron_to_eth_address(non_tron).is_none(), "Should reject non-Tron format address");
    }

    #[test]
    fn test_address_matching() {
        let eth1 = "0xa614f803b6fd780986a42c78ec9c7f77e6ded13c";
        let eth2 = "0xa614f803b6fd780986a42c78ec9c7f77e6ded13c";
        assert!(is_address_match(eth1, eth2), "Same ETH addresses should match");

        let tron1 = "TBUTD9rrESu8A1Q4za6qCsv92vdA58Y4BS";
        let tron2 = "TBUTD9rrESu8A1Q4za6qCsv92vdA58Y4BS";
        assert!(is_address_match(tron1, tron2), "Same Tron addresses should match");

        let eth = "0x10800CA2A4458149eb947eAFeD8B2c99cF96a10D";
        let tron = "TBUTD9rrESu8A1Q4za6qCsv92vdA58Y4BS";
        assert!(is_address_match(eth, tron), "ETH and corresponding Tron address should match");

        let eth_upper = "0x10800CA2A4458149EB947EAFED8B2C99CF96A10D";
        assert!(is_address_match(eth_upper, tron), "Case-insensitive matching should work");

        let other_eth = "0x0000000000000000000000000000000000000001";
        assert!(!is_address_match(other_eth, tron), "Different addresses should not match");
    }

    #[test]
    fn test_roundtrip_conversion() {
        let original_eth = "0x10800CA2A4458149eb947eAFeD8B2c99cF96a10D";
        let tron = eth_to_tron_address(original_eth).unwrap();
        let back_to_eth = tron_to_eth_address(&tron).unwrap();

        assert_eq!(original_eth.to_lowercase(), back_to_eth.to_lowercase(),
                   "Round-trip conversion ETH->Tron->ETH should preserve the address");
    }

    #[test]
    fn test_real_address_pairs() {
        let eth1 = "0x10800CA2A4458149eb947eAFeD8B2c99cF96a10D";
        let tron1 = "TBUTD9rrESu8A1Q4za6qCsv92vdA58Y4BS";
        assert!(is_address_match(eth1, tron1), "Real address pair 1 should match");
    }
}
