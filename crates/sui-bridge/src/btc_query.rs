use anyhow::Error;
use std::time::Duration;
use serde::{Deserialize, Serialize};
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
const ERROR_ADDRESS_NOT_IN_TXN: &str = "Address not in txn";
const ERROR_AMOUNT_NOT_CORRECT: &str = "Amount not correct";
const ERROR_TARGET_ADDRESS_NOT_CORRECT: &str = "Target address not correct";
const ERROR_JSON_PARSE_FAILED: &str = "Failed to parse JSON response";

//todo add obc addresses to config
const OBC_ADDRESSES: [&str; 2] = ["n1sfLwoLTnLFxj2BT8kNETsLDM8xMecYn3", "n1sfLwoLTnLFxj2BT8kNETsLDM8xMecYn3"];

  pub async fn btc_query(txn_id: &str, address: &str, amount: u64) -> Result<bool, Error> {
     let url = format!("https://blockstream.info/testnet/api/tx/{}", txn_id);
     let response = match reqwest::get(&url).await {
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
                println!("error: {:?}", e);
             return Err(anyhow::anyhow!(ERROR_JSON_PARSE_FAILED))
         },
     };

      if parsed_response.vout.iter().any(|v| v.scriptpubkey_address.as_deref() == Some(address)) {
      } else {
          return Err(anyhow::anyhow!(ERROR_ADDRESS_NOT_IN_TXN))
      }

      let mut found = false;
      for v in &parsed_response.vout {
          if OBC_ADDRESSES.contains(&v.scriptpubkey_address.as_deref().unwrap_or("")) {
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

      println!("response：{:#?}", parsed_response);
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
            "tb1p3436xedsqrxfd3gqr3rcrgavytgtrus83plndht05afsssw23q3sxejagc",
            10).await;
        assert_eq!(result, true);
    }
}