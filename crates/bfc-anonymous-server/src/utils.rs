use crate::parse_response;
use anyhow::anyhow;
use fastcrypto::ed25519::Ed25519PublicKey;
use fastcrypto::traits::ToFromBytes;
use serde::{Deserialize, Serialize};
use serde_json::{json};
use tracing::info;
use mpc_transmission::get_sui_config_directory;
use sui_config::anonymous_privatekey_config::AnonymousPrivateKeyConfig;
use sui_types::base_types::SuiAddress;
use fastcrypto::hash::{HashFunction};

const PERSONAL_MESSAGE_PREFIX: &[u8; 3] = &[3, 0, 0];
#[derive(Debug, Deserialize, Serialize)]
pub struct ZkVerifyRequest {
    pub signature: String,
    pub bytes: String, // "hello world" ==> base64
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

pub fn write_unsigned_leb128(out: &mut [u8], mut value: u64) -> usize {
    let mut i = 0;
    loop {
        if value < 0x80 {
            out[i] = value as u8;
            i += 1;
            break;
        } else {
            out[i] = ((value & 0x7F) | 0x80) as u8;
            value >>= 7;
            i += 1;
        }
    }
    i
}

pub fn create_sign_message(message: String) -> Vec<u8> {
    let mut intent_data = Vec::new();
    intent_data.extend_from_slice(PERSONAL_MESSAGE_PREFIX);
    let len = message.len() as u64;
    let mut buffer = [0u8; 10];
    let length = write_unsigned_leb128(&mut buffer, len);
    intent_data.extend_from_slice(&buffer[..length]);
    intent_data.extend_from_slice(message.as_bytes());

    let digest = fastcrypto::hash::Blake2b256::digest(intent_data);
    return digest.to_vec();
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

pub fn convert_value_array_to_string(value_array: &Vec<String>) -> String {
    let with_comma: String = value_array.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    with_comma
}

#[cfg(test)]
mod tests {
    use crate::utils::convert_value_array_to_string;

    #[tokio::test]
    async fn test_convert_value_array_to_string() {
        let input : Vec<u64> = vec![1, 2, 3];
        let result = "1,2,3";
        assert_eq!(convert_value_array_to_string(&input), result);

        let input : Vec<u64> = vec![1];
        let result = "1";
        assert_eq!(convert_value_array_to_string(&input), result);

        let input : Vec<u64> = vec![1000,10000, 100000];
        let result = "1000,10000,100000";
        assert_eq!(convert_value_array_to_string(&input), result);
    }
}
