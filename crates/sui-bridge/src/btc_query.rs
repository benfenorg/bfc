use anyhow::Error;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use tracing::{info, error};
use crate::retry_with_max_elapsed_time;

#[derive(Serialize, Deserialize, Debug)]
pub struct BtcQuery {
    txid: String,
    version: u32,
    locktime: u32,
    vout: Vec<Vout>,
    size: u32,
    weight: u32,
    fee: u64,
    // vin: Vec<Vin>,
    status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vin {
    txid: String,
    vout: u32,
    prevout: Prevout,
    scriptsig: String,
    scriptsig_asm: String,
    witness: Vec<String>,
    is_coinbase: bool,
    sequence: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prevout {
    scriptpubkey: String,
    scriptpubkey_asm: String,
    scriptpubkey_type: String,
    scriptpubkey_address: Option<String>,  // 变为 Option
    value: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vout {
    scriptpubkey: String,
    scriptpubkey_asm: String,
    scriptpubkey_type: String,
    scriptpubkey_address: Option<String>,  // 变为 Option
    value: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    confirmed: bool,
    block_height: u32,
    block_hash: String,
    block_time: u64,
}

/// Check if the given Bitcoin transaction is confirmed.
pub async fn check_btc_txn(txn_id: &str, address: &str, amount: u64) -> bool {
    if txn_id.len() != 64 {
        return false;
    }
    match retry_with_max_elapsed_time!(btc_query(txn_id, address, amount), Duration::from_secs(5)) {
        Ok(result) => result.unwrap_or_else(|_| false),
        Err(e) => {
            error!("Error checking BTC txn: {:?}", e);
            false
        },
    }
}

const ERROR_REQUEST_FAILED: &str = "Request failed";
const ERROR_RESPONSE_TEXT_FAILED: &str = "Failed to get response text";
const ERROR_AMOUNT_NOT_CORRECT: &str = "Amount not correct";
const ERROR_TARGET_ADDRESS_NOT_CORRECT: &str = "Target address not correct";
const ERROR_JSON_PARSE_FAILED: &str = "Failed to parse JSON response";

#[allow(unused)]
/// 这里定义一个结构体来解析token的返回值
#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[allow(unused)]
/// 这里定义一个结构体来缓存token
struct TokenCache {
    token: String,
    created_at: Instant,
}

#[allow(unused)]
static TOKEN_CACHE: Lazy<Mutex<Option<TokenCache>>> = Lazy::new(|| Mutex::new(None));
#[allow(unused)]
const TOKEN_VALIDITY: Duration = Duration::from_secs(5 * 60); // 5分钟有效期

#[allow(unused)]
async fn get_access_token(client: reqwest::Client) -> Result<String, Error> {
    // 尝试从缓存获取token
    {
        let cache = TOKEN_CACHE.lock().unwrap();
        if let Some(cache_data) = cache.as_ref() {
            if cache_data.created_at.elapsed() < TOKEN_VALIDITY {
                info!("Using cached access token");
                return Ok(cache_data.token.clone());
            }
            // Token已过期，需要重新获取
        }
    }
    //todo add client id to config
    let client_id = std::env::var("BTC_CLIENT_ID").unwrap_or_else(|_| "".to_string());
    let client_secret = std::env::var("BTC_CLIENT_SECRET").unwrap_or_else(|_| "".to_string());

    if client_id.is_empty() || client_secret.is_empty() {
        return Err(anyhow::anyhow!("CLIENT_ID or CLIENT_SECRET not set"));
    }

    let url = "https://login.blockstream.com/realms/blockstream-public/protocol/openid-connect/token";

    let params = [
        ("client_id", &client_id),
        ("client_secret", &client_secret),
        ("grant_type", &"client_credentials".to_string()),
        ("scope", &"openid".to_string()),
    ];

    let response = match client.post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await {
        Ok(response) => response,
        Err(_) => return Err(anyhow::anyhow!("Failed to send token request")),
    };

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Token request failed with status: {}", response.status()));
    }

    let response_text = match response.text().await {
        Ok(text) => text,
        Err(_) => return Err(anyhow::anyhow!("Failed to get token response text")),
    };

    let token_response: TokenResponse = match serde_json::from_str(&response_text) {
        Ok(parsed) => parsed,
        Err(_) => return Err(anyhow::anyhow!("Failed to parse token JSON response")),
    };

    let token = token_response.access_token;

    // 更新缓存
    {
        let mut cache = TOKEN_CACHE.lock().unwrap();
        *cache = Some(TokenCache {
            token: token.clone(),
            created_at: Instant::now(),
        });
    }

    info!("Successfully retrieved new access token");
    Ok(token)
}

  pub async fn btc_query(txn_id: &str, address: &str, amount: u64) -> Result<bool, Error> {
      let client = reqwest::Client::new();
      // let token = get_access_token(client.clone()).await?;  //enterprise.blockstream.info
      let url = format!("https://mempool.space/testnet/api/tx/{}", txn_id);
      let response = match
          client
              .get(&url)
              .send()
            .await {
                Ok(response) => response,
                Err(_) => return Err(anyhow::anyhow!(ERROR_REQUEST_FAILED)),
            };

     let response_text = match response.text().await {
         Ok(text) => text,
         Err(_) => return Err(anyhow::anyhow!(ERROR_RESPONSE_TEXT_FAILED)),
     };
       // println!(" text： {:#?}", response_text);
     let parsed_response: BtcQuery = match serde_json::from_str(&response_text) {
         Ok(parsed) => parsed,
         Err(e) => {
             // println!("error: {:?}", e);
             return Err(anyhow::anyhow!(ERROR_JSON_PARSE_FAILED))
         },
     };

      let mut found = false;
      for v in &parsed_response.vout {
          if v.scriptpubkey_address.as_deref() == Some(address) {
              if v.value != amount {
                  return Err(anyhow::anyhow!(ERROR_AMOUNT_NOT_CORRECT));
              }
              found = true;
              break;
          }
      }
      if !found {
          return Err(anyhow::anyhow!(ERROR_TARGET_ADDRESS_NOT_CORRECT));
      }

      // println!("response：{:#?}", parsed_response);
      info!("btc query: {:?}", parsed_response);
     if parsed_response.status.confirmed && parsed_response.status.block_height > 0 {
         Ok(true)
     } else {
         Ok(false)
     }
 }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_btc_txn() {
        let result = check_btc_txn(
            "20c04f56b8dc0f507f8ca7d208fff8f7ca6ca7508bb2a334bbcbf7ec99804941",
            "n1sfLwoLTnLFxj2BT8kNETsLDM8xMecYn3",
            10).await;
        assert_eq!(result, true);
    }
}