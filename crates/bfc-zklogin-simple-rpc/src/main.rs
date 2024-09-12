// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use axum::{routing::get, Router};
use axum::routing::post;
use std::net::SocketAddr;
use bfc_zklogin_simple_rpc::{verify_zk_signature, hello};
use std::env;
use std::process::exit;
use tracing::info;
use tracing_subscriber::fmt;


#[tokio::main]
async fn main() {
    let subscriber = fmt::Subscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: bfc-zklogin-simple-rpc <env>(test,prod)");
        exit(0);
    }

    let router = Router::new().route("/", get(hello))
        .route("/verify_zk_login_sig", {
            let env_var = args[1].clone();
            post(move |request| verify_zk_signature(env_var, request))
        });


    let addr = SocketAddr::from(([0, 0, 0, 0], 8003));
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
    info!("listening on {}", addr);


    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

