use crate::parse_response;
use anyhow::anyhow;
use fastcrypto::ed25519::Ed25519PublicKey;
use fastcrypto::hash::HashFunction;
use fastcrypto::traits::ToFromBytes;
use move_core_types::account_address::AccountAddress;
use serde::{Deserialize, Serialize};
use serde_json::{json};
use tracing::info;
use mpc_transmission::get_sui_config_directory;
use sui_config::anonymous_privatekey_config::AnonymousPrivateKeyConfig;
use sui_types::base_types::SuiAddress;

#[derive(Debug, Deserialize, Serialize)]
pub struct ZkVerifyRequest {
    pub signature: String,
    pub bytes: String, // abfc token objectid
    pub intent_scope: u8,
    pub cur_epoch: Option<u64>,
    pub cur_rpc_url : Option<String>,
    pub author: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ZkVerifyResponse {
    result: bool,
    message: String,
}

pub async fn verify_zklogin_signature(
    signature: ZkVerifyRequest,
    zklogin_rpc: String
) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .post(zklogin_rpc)
        .json(&signature)
        .send()
        .await?;

    let result = response.text().await?;

    match serde_json::from_str::<ZkVerifyResponse>(result.as_str()) {
        Ok(response) => {
            if response.result == false {
                Err(anyhow!("verify failed",).into())
            } else {
                Ok(true)
            }

        }
        Err(e) => {
            info!("resolving failed, caused by: {}", e);
            Err(anyhow!("resolving failed, caused by {}", e).into())
        }
    }
}

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

pub fn public_key_bytes_to_sui_address(pubkey_bytes: Vec<u8>) -> Result<SuiAddress, Box<dyn std::error::Error>> {
    let ed25519_pk =
        Ed25519PublicKey::from_bytes(&pubkey_bytes)?;
    Ok(SuiAddress::from(&ed25519_pk))
}

