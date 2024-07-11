// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use axum::Json;
use axum::response::IntoResponse;
use tracing::info;
use crate::zk_verification::{verify_zk_login_sig, ZkVerifyRequest, ZkVerifyResponse};

pub mod zk_verification;


pub async fn hello() -> &'static str {
    "Hello, this is simple rpc!"
}

pub async fn verify_zk_signature(Json(req): Json<ZkVerifyRequest>) -> impl IntoResponse {
    info!("verify_zk_signature req={:?}", &req);
    let ZkVerifyRequest { signature: sig, bytes, intent_scope, cur_epoch, author } = req;

    let result = verify_zk_login_sig(sig, bytes, intent_scope, cur_epoch, author).await;

    match result {
        Ok(sui_result) => {
            match sui_result {
                Ok(_) => ZkVerifyResponse {result: true, message: "success".to_string()},
                Err(e) => ZkVerifyResponse {result: false, message: e.to_string()},
            }
        },
        Err(any_error) => ZkVerifyResponse {result: false, message: any_error.to_string()}
    }

}