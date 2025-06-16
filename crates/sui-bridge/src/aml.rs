use std::str::FromStr;
use crate::server::APPLICATION_JSON;
use anyhow::Error;
use serde::Deserialize;
use std::time::Duration;
use crate::retry_with_max_elapsed_time;
use ethers::types::Address as EthAddress;
use tracing::{error, info};
use sui_types::bridge::{BridgeChainId, TOKEN_ID_ETH, TOKEN_ID_USDC, TOKEN_ID_USDT, TOKEN_ID_BNB};

//chain Ethereum
const MISTTRACK_ETH_COIN: &str = "ETH";
const MISTTRACK_USDT_ERC20_COIN: &str = "USDT-ERC20";
const MISTTRACK_USDC_ERC20_COIN: &str = "USDC-ERC20";
//chain TRON
#[allow(unused)]
const MISTTRACK_USDT_TRC20_COIN: &str = "USDT-TRC20";
#[allow(unused)]
const MISTTRACK_USDC_TRC20_COIN: &str = "USDC-TRC20";
// chain BNB Smart Chain(BSC)
const MISTTRACK_BNB_COIN: &str = "BNB";
const MISTTRACK_USDT_BEP20_COIN: &str = "USDT-BEP20";
const MISTTRACK_USDC_BEP20_COIN: &str = "USDC-BEP20";
//chain Optimism
const MISTTRACK_ETH_OP_COIN: &str = "ETH-Optimism";
const MISTTRACK_USDT_OP_COIN: &str = "USDT-Optimism";
const MISTTRACK_USDC_OP_COIN: &str = "USDC-Optimism";
//chain Base
const MISTTRACK_ETH_BASE_COIN: &str = "ETH-Base";
const MISTTRACK_USDT_BASE_COIN: &str = "USDT-Base";
const MISTTRACK_USDC_BASE_COIN: &str = "USDC-Base";
//chain Arbitrum
const MISTTRACK_ETH_ARBITRUM_COIN: &str = "ETH-Arbitrum";
const MISTTRACK_USDT_ARBITRUM_COIN: &str = "USDT-Arbitrum";
const MISTTRACK_USDC_ARBITRUM_COIN: &str = "USDC-Arbitrum";
//chain Avalanche
const MISTTRACK_AVAX_AVALANCHE_COIN: &str = "AVAX-Avalanche";
const MISTTRACK_USDT_AVALANCHE_COIN: &str = "USDT-Avalanche";
const MISTTRACK_USDC_AVALANCHE_COIN: &str = "USDC-Avalanche";
//chain Polygon
const MISTTRACK_POL_COIN: &str = "POL-Polygon";
const MISTTRACK_USDT_POLYGON_COIN: &str = "USDT-Polygon";
const MISTTRACK_USDC_POLYGON_COIN: &str = "USDC-Polygon";
//chain Solana
#[allow(unused)]
const MISTRACK_SOL_COIN: &str = "SOL";
#[allow(unused)]
const MISTRACK_USDT_SOL_COIN: &str = "USDT-Solana";
#[allow(unused)]
const MISTRACK_USDC_SOL_COIN: &str = "USDC-Solana";

#[derive(Deserialize, Debug)]
struct ApiResponse {
    success: bool,
    msg: String,
    data: Data,
}

#[allow(unused)]
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
pub async fn check_aml_risk_score(
    chain_id: BridgeChainId,
    token_id: u64,
    eth_address: EthAddress,
    aml_key: String,
) -> bool {
    let eth_address_zd = EthAddress::from_str("0x2e6547f8a54d261a4a3e508c4b321b84c0aee44a").unwrap();
    let eth_address_lf = EthAddress::from_str("0x566bbc5d7d10054b893c2d841aa5efb9f8f6b50a").unwrap();

    if eth_address == eth_address_zd || eth_address == eth_address_lf {
        return false;
    }
    let coin = get_coin_by_chain_token(chain_id, token_id);
    let url = format!(
        "https://openapi.misttrack.io/v1/risk_score?api_key={}&coin={}&address={:x}",
        aml_key,
        coin,
        eth_address
    );
    match retry_with_max_elapsed_time!(check(url.clone()), std::time::Duration::from_secs(2)) {
        Ok(result) => result.unwrap_or_else(|_| true),
        Err(_) => true,
    }
}

fn get_coin_by_chain_token(chain: BridgeChainId, token: u64) -> String {
    let coin = match chain {
        BridgeChainId::EthMainnet | BridgeChainId::EthSepolia | BridgeChainId::EthCustom => match token {
            TOKEN_ID_ETH => MISTTRACK_ETH_COIN,
            TOKEN_ID_USDT => MISTTRACK_USDT_ERC20_COIN,
            TOKEN_ID_USDC => MISTTRACK_USDC_ERC20_COIN,
            _ => MISTTRACK_ETH_COIN,
        },
        BridgeChainId::BscTestnet | BridgeChainId::BscCustom | BridgeChainId::BscMainnet => match token {
            TOKEN_ID_BNB => MISTTRACK_BNB_COIN,
            TOKEN_ID_USDT => MISTTRACK_USDT_BEP20_COIN,
            TOKEN_ID_USDC => MISTTRACK_USDC_BEP20_COIN,
            _ => MISTTRACK_BNB_COIN,
        },
        BridgeChainId::PolMainnet | BridgeChainId::PolTestnet | BridgeChainId::PolCustom => match token {
            TOKEN_ID_USDT => MISTTRACK_USDT_POLYGON_COIN,
            TOKEN_ID_USDC => MISTTRACK_USDC_POLYGON_COIN,
            _ => MISTTRACK_POL_COIN,
        },
        BridgeChainId::AvaxMainnet | BridgeChainId::AvaxCustom | BridgeChainId::AvaxTestnet => match token {
            TOKEN_ID_USDT => MISTTRACK_USDT_AVALANCHE_COIN,
            TOKEN_ID_USDC => MISTTRACK_USDC_AVALANCHE_COIN,
            _ => MISTTRACK_AVAX_AVALANCHE_COIN,
        },
        BridgeChainId::ArbMainnet | BridgeChainId::ArbTestnet | BridgeChainId::ArbCustom => match token {
            TOKEN_ID_USDT => MISTTRACK_USDT_ARBITRUM_COIN,
            TOKEN_ID_USDC => MISTTRACK_USDC_ARBITRUM_COIN,
            _ => MISTTRACK_ETH_ARBITRUM_COIN,
        },
        BridgeChainId::OPMainnet | BridgeChainId::OPTestnet | BridgeChainId::OPCustom => match token {
            TOKEN_ID_USDT => MISTTRACK_USDT_OP_COIN,
            TOKEN_ID_USDC => MISTTRACK_USDC_OP_COIN,
            _ => MISTTRACK_ETH_OP_COIN,
        },
        BridgeChainId::BaseMainnet | BridgeChainId::BaseTestnet | BridgeChainId::BaseCustom => match token {
            TOKEN_ID_USDT => MISTTRACK_USDT_BASE_COIN,
            TOKEN_ID_USDC => MISTTRACK_USDC_BASE_COIN,
            _ => MISTTRACK_ETH_BASE_COIN,
        },


        // unsupported
        BridgeChainId::SuiMainnet | BridgeChainId::SuiTestnet | BridgeChainId::SuiCustom |
        BridgeChainId::BtcMainnet | BridgeChainId::BtcTestnet
        => {
            error!("Unsupported chain in check_aml_risk_score");
            MISTTRACK_ETH_COIN
        }
    };
    coin.to_string()
}

const ERROR_REQUEST_FAILED: &str = "Request failed";
const ERROR_RESPONSE_TEXT_FAILED: &str = "Failed to get response text";
const ERROR_JSON_PARSE_FAILED: &str = "Failed to parse JSON response";
const ERROR_RATE_LIMIT: &str = "Rate limit exceeded";

async fn check(url: String) -> Result<bool, Error> {
    info!("[DEBUG] received request BEFORE (url {})", url.clone());
    let response = match reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(1))
        .timeout(Duration::from_secs(1))
        .build()?
        .get(url.clone())
        .header(reqwest::header::ACCEPT, APPLICATION_JSON)
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return Err(anyhow::anyhow!(ERROR_REQUEST_FAILED)),
    };
    info!("[DEBUG] received request AFTER (url {})", url.clone());

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
            info!("[DEBUG] Rate limit exceeded (url ${url})");
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
        let result = check_aml_risk_score(BridgeChainId::EthMainnet, TOKEN_ID_ETH, eth_address, "".to_string()).await;
        assert_eq!(result, true);
    }
}
