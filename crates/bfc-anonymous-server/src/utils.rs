use crate::parse_response;
use anyhow::anyhow;
use fastcrypto::ed25519::Ed25519PublicKey;
use fastcrypto::traits::ToFromBytes;
use serde_json::json;
use std::path::PathBuf;
use sui_config::anonymous_privatekey_config::AnonymousPrivateKeyConfig;
use sui_types::base_types::SuiAddress;

pub async fn get_object_owneraddress(
    object_id: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let path = get_sui_config_directory().join("bfc_anonymous_config.yaml");
    let config = AnonymousPrivateKeyConfig::from_yaml_file(&path)
        .unwrap_or(AnonymousPrivateKeyConfig::default());
    let fullnode_rpc = config
        .fullnode_rpc_path
        .unwrap_or("https://devrpc4.openblock.vip".to_string());

    let client = reqwest::Client::new();
    let response = client
        .post(fullnode_rpc)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "bfc_getObject",
            "params": [
                object_id,
                    {
                        "showType": true,
                        "showOwner": true,
                        "showContent": true,
                        "showDisplay": true,
                        "showBcs": false,
                        "showStorageRebate": false
                    }
            ]
        }))
        .send()
        .await?;

    let result = response.text().await?;

    let object_id = parse_response(&result.clone());
    match object_id {
        Some(val) => return Ok(val),
        None => return Err(anyhow!("object owner not exit").into()),
    }
}

pub fn public_key_bytes_to_sui_address(pubkey_bytes: Vec<u8>) -> SuiAddress {
    let ed25519_pk =
        Ed25519PublicKey::from_bytes(&pubkey_bytes).expect("Valid Ed25519 public key bytes");
    SuiAddress::from(&ed25519_pk)
}

/// Get mask secret from configuration file
///
/// Reads the anonymous private key from the BFC configuration file and converts it to a u64 value.
/// The private key should be a hexadecimal string that can be parsed as a u64.
///
/// # Returns
/// - `Ok(u64)`: The parsed mask secret value
/// - `Err`: If the config file doesn't exist, is invalid, or the private key cannot be parsed
pub fn get_mask_secret_from_config() -> Result<u64, Box<dyn std::error::Error>> {
    let path = get_sui_config_directory().join("bfc_anonymous_config.yaml");
    
    // Load config file, return error if it doesn't exist or is invalid
    let config = AnonymousPrivateKeyConfig::from_yaml_file(&path)?;
    
    // Get the private key string from config
    let private_key_str = config.anonymous_privatekey
        .ok_or_else(|| anyhow!("Anonymous private key not found in configuration"))?;
    
    // Parse the private key string to u64
    // Handle both hex format (0x...) and decimal format
    let mask_secret = if private_key_str.starts_with("0x") || private_key_str.starts_with("0X") {
        // Parse as hexadecimal
        u64::from_str_radix(&private_key_str[2..], 16)
            .map_err(|e| anyhow!("Failed to parse private key as hex: {}", e))?
    } else {
        // Parse as decimal
        private_key_str.parse::<u64>()
            .map_err(|e| anyhow!("Failed to parse private key as decimal: {}", e))?
    };
    
    Ok(mask_secret)
}

fn get_sui_config_directory() -> PathBuf {
    match dirs::home_dir() {
        Some(v) => v.join(".bfc").join("bfc_config"),
        None => panic!("Cannot obtain home directory path"),
    }
}
