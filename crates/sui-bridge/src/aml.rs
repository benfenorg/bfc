use std::str::FromStr;
use crate::server::APPLICATION_JSON;
use anyhow::Error;
use ethers::types::Address;
use serde::Deserialize;
use std::time::Duration;
use crate::retry_with_max_elapsed_time;
use ethers::types::Address as EthAddress;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    success: bool,
    msg: String,
    data: Data,
}

#[derive(Deserialize, Debug)]
struct Data {
    score: u32,
    hacking_event: String,
    detail_list: Vec<String>,
    risk_level: String,
    risk_detail: Vec<String>,
}

/// Check if the given Ethereum address is an AML address.
/// result: false if the address is an AML address, true otherwise.
/// https://openapi.misttrack.io/v1/risk_score
//    ?coin=ETH
//    &address={address}
//    &txid={txn hash}
//    &api_key=YourApiKey
pub async fn check_aml_eth(eth_address: EthAddress) -> bool {
    let eth_address_zd =EthAddress::from_str("0x2e6547f8a54d261a4a3e508c4b321b84c0aee44a").unwrap();
    let eth_address_lf =EthAddress::from_str("0x566bbc5d7d10054b893c2d841aa5efb9f8f6b50a").unwrap();

    if eth_address == eth_address_zd || eth_address == eth_address_lf {
        return false;
    }

    let url = format!(
        "https://openapi.misttrack.io/v1/risk_score?api_key=&coin=ETH&address={:x}",
        eth_address
    );
    match retry_with_max_elapsed_time!(check(url.clone()), std::time::Duration::from_secs(5)) {
        Ok(result) => result.unwrap_or_else(|_| true),
        Err(_) => true,
    }
}



const ERROR_REQUEST_FAILED: &str = "Request failed";
const ERROR_RESPONSE_TEXT_FAILED: &str = "Failed to get response text";
const ERROR_JSON_PARSE_FAILED: &str = "Failed to parse JSON response";
const ERROR_RATE_LIMIT: &str = "Rate limit exceeded";

async fn check(url: String) -> Result<bool, Error> {
    let response = match reqwest::Client::new()
        .get(url.clone())
        .header(reqwest::header::ACCEPT, APPLICATION_JSON)
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return Err(anyhow::anyhow!(ERROR_REQUEST_FAILED)),
    };

    let response_text = match response.text().await {
        Ok(text) => text,
        Err(_) => return Err(anyhow::anyhow!(ERROR_RESPONSE_TEXT_FAILED)),
    };

    let parsed_response: ApiResponse = match serde_json::from_str(&response_text) {
        Ok(parsed) => parsed,
        Err(_) => return Err(anyhow::anyhow!(ERROR_JSON_PARSE_FAILED)),
    };

    if parsed_response.success {
        if parsed_response.data.score > 30 {
            return Ok(false);
        }
        if parsed_response.data.risk_level.to_lowercase() != "low" {
            return Ok(false);
        }
    } else {
        if parsed_response.msg.contains("MaxRateLimit") {
            tokio::time::sleep(tokio::time::Duration::from_micros(800)).await;
            return Err(anyhow::anyhow!(ERROR_RATE_LIMIT));
        }
    }
    Ok(true)
}

/// Check if the given Bitcoin address is an AML address.
/// result: false if the address is an AML address, true otherwise.
pub async fn check_aml_btc(btc_address: &str) -> bool {
    let url = format!(
        "https://openapi.misttrack.io/v1/risk_score?api_key=&coin=BTC&address={}",
        btc_address
    );
    if let Ok(result) = retry_with_max_elapsed_time!(check(url.clone()), std::time::Duration::from_secs(5)) {
        result.unwrap_or_else(|_| true)
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_check_aml() {
        let eth_address = EthAddress::from_str("0x2e6547f8a54d261a4a3e508c4b321b84c0aee44b").unwrap();
        let result = check_aml_eth(eth_address).await;
        assert_eq!(result, true);
    }
}
