// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use axum::{middleware, routing::get, Router};
use axum::routing::post;
use axum::body::Body;
use axum::middleware::Next;
use axum::response::Response as AxumResponse;
use http::{HeaderValue, Request};
use std::net::SocketAddr;
use std::time::Instant;
use bfc_zklogin_simple_rpc::{verify_zk_signature, hello};
use std::env;
use std::process::exit;
use tracing::{info, Level, Instrument};
use tracing_subscriber::fmt;
use tower_http::request_id::{MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer};
use uuid::Uuid;


#[tokio::main]
async fn main() {
    let subscriber = fmt::Subscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
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
        })
        .layer(middleware::from_fn(custom_trace_layer))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeSimpleUuid));


    let addr = SocketAddr::from(([0, 0, 0, 0], 8003));
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
    info!("listening on {}", addr);


    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

async fn custom_trace_layer(req: Request<Body>, next: Next) -> AxumResponse {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let req_id = req.headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("-")
        .to_string();
    let span = tracing::info_span!("http", req_id = %req_id);
    let start = Instant::now();

    async move {
        let response = next.run(req).await;
        let latency = start.elapsed().as_millis();
        let status = response.status().as_u16();

        info!(
            method = %method,
            uri = %uri,
            status = status,
            latency_ms = latency,
            "http response"
        );

        response
    }
    .instrument(span)
    .await
}

#[derive(Clone, Copy)]
pub struct MakeSimpleUuid;

impl MakeRequestId for MakeSimpleUuid {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let uuid = Uuid::new_v4();
        let id_str = uuid.simple().to_string();
        let header_value = HeaderValue::from_str(&id_str).ok()?;
        Some(RequestId::new(header_value))
    }
}
