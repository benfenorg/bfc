// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use axum::{routing::get, Router};
use axum::routing::post;
use std::net::SocketAddr;
use sui_simple_rpc::{verify_zk_signature, hello};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(hello))
        .route("/verify_zk_login_sig", post(verify_zk_signature));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8003));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

