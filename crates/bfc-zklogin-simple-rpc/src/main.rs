// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use axum::{routing::get, Router};
use axum::routing::post;
//use std::net::SocketAddr;
use bfc_zklogin_simple_rpc::{verify_zk_signature, hello};
use std::env;
use std::process::exit;
#[tokio::main]
async fn main() {
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

    //let addr = SocketAddr::from(([0, 0, 0, 0], 8003));

    let addr = tokio::net::TcpListener::bind("0.0.0.0:8003").await.unwrap();
    println!("listening on {:?}", addr);



    axum::serve(addr, router).await.unwrap();


    // axum::serve::bind(&addr)
    //     .serve(router.into_make_service())
    //     .await
    //     .unwrap();

}

