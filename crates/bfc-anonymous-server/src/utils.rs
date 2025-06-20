
use crate::parse_response;
use serde_json::json;
use anyhow::anyhow;
use fastcrypto::ed25519::Ed25519PublicKey;
use fastcrypto::traits::ToFromBytes;
use sui_types::base_types::SuiAddress;

pub async fn get_object_owneraddress(object_id: String) ->  Result<String, Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();

    let response = client
        .post("https://devrpc4.openblock.vip")
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
        None => return Err(anyhow!("object owner not exit").into())
    }
}

pub fn public_key_bytes_to_sui_address(pubkey_bytes: Vec<u8>) -> SuiAddress {
    let ed25519_pk = Ed25519PublicKey::from_bytes(&pubkey_bytes)
        .expect("Valid Ed25519 public key bytes");
    SuiAddress::from(&ed25519_pk)
}