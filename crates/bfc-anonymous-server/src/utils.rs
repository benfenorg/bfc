use crate::parse_response_and_check_balance;
use anyhow::anyhow;
use fastcrypto::ed25519::Ed25519PublicKey;
use fastcrypto::traits::ToFromBytes;
use serde::{Deserialize, Serialize};
use serde_json::{json};
use tracing::info;
use mpc_transmission::get_sui_config_directory;
use sui_config::anonymous_privatekey_config::AnonymousPrivateKeyConfig;
use sui_types::base_types::SuiAddress;
use fastcrypto::hash::{HashFunction, Sha256};
use crate::bfc_object::parse_response_and_return_balance;

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




pub fn create_deep_compress_message(raw_message: String) -> Vec<u8> {
    // Create SHA-256 hasher
    let mut hasher = Sha256::new();
    // Write input data
    hasher.update(raw_message.as_bytes());
    // Get hash result
    let raw_message_digest = hasher.finalize();


    let mut intent_data = Vec::new();
    intent_data.extend_from_slice(PERSONAL_MESSAGE_PREFIX);
    let len = raw_message_digest.size() as u64;
    let mut buffer = [0u8; 10];
    let length = write_unsigned_leb128(&mut buffer, len);
    intent_data.extend_from_slice(&buffer[..length]);
    intent_data.extend_from_slice(raw_message_digest.to_vec().as_ref());


    let digest = fastcrypto::hash::Blake2b256::digest(intent_data);
    digest.to_vec()
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

pub async fn get_object_owner_address(
    object_id: String, value1: Vec<u8>, value2: Vec<u8>
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

    let object_id = parse_response_and_check_balance(&result.clone(), value1, value2);
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




pub async fn get_object_value1_and_value2(
    object_id: String
) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
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

    let value_array = parse_response_and_return_balance(&result.clone());
    match value_array.0.clone() {
        Some(val) => return Ok((value_array.0.unwrap(), value_array.1.unwrap())),
        None => return Err(anyhow!("object owner not exit").into()),
    }
}

#[cfg(test)]
mod tests {
    use fastcrypto::hash::{HashFunction, Sha256};
    use crate::utils::{convert_value_array_to_string, create_deep_compress_message, create_sign_message};

    #[tokio::test]
    async fn test_convert_value_array_to_string() {
        let input : Vec<String> = vec![String::from("1"), String::from("2"), String::from("3")];
        let result = "1,2,3";
        assert_eq!(convert_value_array_to_string(&input), result);

        let input : Vec<String> = vec![String::from("1")];
        let result = "1";
        assert_eq!(convert_value_array_to_string(&input), result);

        let input : Vec<String> = vec![String::from("1000"),String::from("10000"), String::from("100000")];
        let result = "1000,10000,100000";
        assert_eq!(convert_value_array_to_string(&input), result);
    }

    #[tokio::test]
    async fn test_deep_compress_message(){
        let raw_message = "BFC000be30a4678893081f67ea8fc9d4b6dda10d1f5d46250ee699d55691b2ee2766801\
        BFC115fe6543d2ad9c8a7dba16a8aa09b791a459e1fc4026217ed1a49b2aa2dec417893\
        BFC117d3029807425f4865f803370c346049da64a32b9b4f2cda0341c9b78eba4618b35\
        BFC119055c683365a1690bc4d9e03006600aa60b4c855bb893498e675b96d0e0dbf9b0d\
        BFC11bc9fe5e2d962278ea463b13fd036155853616470a70185904f28ea031f607385b5\
        BFC11c4c0ab31400a66b1ff11f20a446df0da47791a99f51a7bf6ed4a568f5454bd795c\
        BFC11d7d0ab70630b58c5b2da200e7ff0502e4cd0301d811cc62ee8e1056375f97e638b\
        BFC11dd74b7956232691a64ebcca13ec0fce23efb45465a7a649ddbd2634ba41a585697\
        BFC11e9edcba46c6e7822e39ffab972fdeec65c4cb843cded5e109d9fda02cab14377ec\
        BFC11f9a4d1e0ccf8c34d8bfe3c79e6a3c37f7b4e6d12e1baaee40cf5cbfa7c56e73417\
        BFC12af3ea9b440c62b235a63a3f0acacbd1ff449ac62dde6c81ae1954a1a62e7354778\
        BFC12c2568f452b71273beb446fe51ee5f3754291e0947374df46a51d1237f0028ef85c\
        BFC12c2907286cbbc298c5f2da48582c0d7288cbc89fee03a143b66ed84a186e9a91826\
        BFC12ca6b8059e11380618d7417baad7549bbc0d95842d45c74ea750868299009b615ea\
        BFC13011d4c8e0b1b87e04c5c70400d3397ab4ca3d222109735a48befaf1a77be465acb\
        BFC130381b7e9c19b3b315113fd34ca64e6cb2a4d767e38aeb7589aecc5f2f15b588166";



        let compress_message = create_deep_compress_message(raw_message.to_string());
        let hexx = hex::encode(compress_message);
        println!("Compress message: {}", hexx);
        assert_eq!(hexx, "8cb79897739dbc829f818cc65d881df45cb3edfdd870235c76304cca29d50089");
    }

}
