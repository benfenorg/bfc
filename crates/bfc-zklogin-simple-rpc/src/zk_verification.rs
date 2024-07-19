use std::str::FromStr;
use anyhow::anyhow;
use axum::Json;
use axum::response::IntoResponse;
use fastcrypto::encoding::{Base64, Encoding};
use fastcrypto::traits::ToFromBytes;
use fastcrypto_zkp::bn254::zk_login::{fetch_jwks, JWK, JwkId, OIDCProvider};
use fastcrypto_zkp::bn254::zk_login_api::ZkLoginEnv;
use http::{HeaderMap, HeaderValue, StatusCode};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use shared_crypto::intent::{Intent, IntentMessage, IntentScope, PersonalMessage};
use sui_types::base_types::SuiAddress;
use sui_types::base_types_bfc::bfc_address_util::convert_to_evm_address;
use sui_types::error::SuiResult;
use sui_types::signature::{GenericSignature, VerifyParams};
use sui_types::transaction::TransactionData;
use sui_types::zk_login_authenticator::ZkLoginAuthenticator;
use im::hashmap::HashMap as ImHashMap;
use serde_json::{json, Value};
use tracing::info;

/// A response struct for the zk verification.
#[derive(Deserialize, Serialize, Debug)]
pub struct ZkVerifyResponse {
    pub result: bool,
    pub message: String,
}

/// A request struct for the zk verification.
#[derive(Serialize, Deserialize, Debug)]
pub struct ZkVerifyRequest {
    pub signature: String,
    pub bytes: String,
    // 0 - TransactionData 3 - personal message
    pub intent_scope: u8,
    pub cur_epoch: Option<u64>,
    pub cur_rpc_url : Option<String>,
    // sui address
    pub author: String,
}

impl IntoResponse for ZkVerifyResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub async fn verify_zk_login_sig(
    sig: String, bytes: String, intent_scope: u8, cur_epoch: Option<u64>, cur_rpc_url: Option<String>, author: String
) -> Result<SuiResult, anyhow::Error> {
    let mut address_string = author.to_string();
    if author.starts_with("bfc") || author.starts_with("BFC") {
        address_string = convert_to_evm_address(address_string);
    }
    let author_address = SuiAddress::from_str(&address_string)?;

    let sig_decode_bytes = &Base64::decode(&sig).map_err(|e| anyhow!("Invalid base64 sig: {:?}", e))?;
    let zk = ZkLoginAuthenticator::from_bytes(sig_decode_bytes)?;

    let client = Client::new();
    let provider = OIDCProvider::from_iss(zk.get_iss())
        .map_err(|_| anyhow!("Invalid iss"))?;
    let jwks = fetch_jwks(&provider, &client).await?;
    let parsed: ImHashMap<JwkId, JWK> = jwks.clone().into_iter().collect();
    // TODO  adjust env by environment variable
    let env = ZkLoginEnv::Test;

    let cur_epoch_id = match cur_rpc_url {
        Some(url) => {
            if url.starts_with("http") || url.starts_with("https") {
                get_current_epoch(url).await?
            } else {
                return Err(anyhow!("url pattern error"));
            }
        },
        None => {
            let epoch_id = cur_epoch.ok_or_else(|| anyhow!("cur_epoch was None"))?;
            epoch_id
        }
    };

    let verify_params =
        VerifyParams::new(parsed, vec![], env, true, true, Some(30));

    let (_serialized, res) = match IntentScope::try_from(intent_scope)
        .map_err(|_| anyhow!("Invalid scope"))? {
        IntentScope::TransactionData => {
            let tx_data: TransactionData = bcs::from_bytes(
                &Base64::decode(&bytes)
                    .map_err(|e| anyhow!("Invalid base64 tx data: {:?}", e))?,
            )?;

            let sig = GenericSignature::ZkLoginAuthenticator(zk.clone());
            let res = sig.verify_authenticator(
                &IntentMessage::new(Intent::sui_transaction(), tx_data.clone()),
                author_address,
                cur_epoch_id,
                &verify_params,
            );
            (serde_json::to_string(&tx_data)?, res)
        }
        IntentScope::PersonalMessage => {
            let data = PersonalMessage {
                message: Base64::decode(&bytes).map_err(|e| {
                    anyhow!("Invalid base64 personal message data: {:?}", e)
                })?,
            };

            let sig = GenericSignature::ZkLoginAuthenticator(zk.clone());
            let res = sig.verify_authenticator(
                &IntentMessage::new(Intent::personal_message(), data.clone()),
                author_address,
                cur_epoch_id,
                &verify_params,
            );
            (serde_json::to_string(&data)?, res)
        }
        _ => return Err(anyhow!("Invalid intent scope")),
    };

    Ok(res)
}

pub async fn get_current_epoch(rpc_url: String) -> Result<u64, anyhow::Error>  {
    let json = json!({"jsonrpc":"2.0", "id":"0", "method":"bfcx_getLatestSuiSystemState", "params":[]});

    let response = post_with_body(rpc_url.as_str(), json.to_string()).await?;

    let response_obj = response.as_object().ok_or_else(|| anyhow!("response format is not json"))?;
    let result_obj = response_obj.get("result").ok_or_else(|| anyhow!("result is not present"))?;
    let epoch = result_obj.get("epoch").ok_or_else(|| anyhow!("epoch is not present"))?
        .as_str().ok_or_else(|| anyhow!("epoch is not a valid u64"))?;

    return epoch.parse::<u64>().map_err(|e| anyhow!("epoch not a integer: {:?}", e));
}

pub async fn post_with_body(url: &str, body_data: String) ->  Result<Value, anyhow::Error>  {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    info!("post url={}", url);
    let response = client.post(url)
        .headers(headers).body(body_data)
        .send().await?;
    let body: Value = response.json::<Value>().await?;
    // println!("response body={:?}", &body);

    Ok(body)
}

