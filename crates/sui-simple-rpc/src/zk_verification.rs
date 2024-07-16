use std::str::FromStr;
use anyhow::anyhow;
use axum::Json;
use axum::response::IntoResponse;
use fastcrypto::encoding::{Base64, Encoding};
use fastcrypto::traits::ToFromBytes;
use fastcrypto_zkp::bn254::zk_login::{fetch_jwks, JWK, JwkId, OIDCProvider};
use fastcrypto_zkp::bn254::zk_login_api::ZkLoginEnv;
use http::StatusCode;
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
    pub cur_epoch: u64,
    // sui address
    pub author: String,
}

impl IntoResponse for ZkVerifyResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub async fn verify_zk_login_sig(
    sig: String, bytes: String, intent_scope: u8, cur_epoch: u64, author: String
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

    let verify_params =
        VerifyParams::new(parsed, vec![], env, true, true, Some(30000));

    let (_serialized, res) = match IntentScope::try_from(intent_scope)
        .map_err(|_| anyhow!("Invalid scope"))?
    {
        IntentScope::TransactionData => {
            let tx_data: TransactionData = bcs::from_bytes(
                &Base64::decode(&bytes)
                    .map_err(|e| anyhow!("Invalid base64 tx data: {:?}", e))?,
            )?;

            let sig = GenericSignature::ZkLoginAuthenticator(zk.clone());
            let res = sig.verify_authenticator(
                &IntentMessage::new(Intent::sui_transaction(), tx_data.clone()),
                author_address,
                cur_epoch,
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
                cur_epoch,
                &verify_params,
            );
            (serde_json::to_string(&data)?, res)
        }
        _ => return Err(anyhow!("Invalid intent scope")),
    };

    Ok(res)
}

