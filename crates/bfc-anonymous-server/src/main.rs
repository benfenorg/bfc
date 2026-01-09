mod client_test;
mod database;
mod utils;

mod bfc_object;
mod signature;
mod anonymous_server_v1;
mod server_utils;
mod anonymous_server_v2;
mod bfc_transaction;

use std::net::SocketAddr;
use crate::bfc_object::parse_response_and_check_balance;
use clap::Parser;
use mpc_transmission::get_sui_config_directory;
use serde::{Deserialize, Serialize};
use tracing::{info};
use tracing_subscriber::fmt;
use warp::Filter;
use crate::server_utils::{create_error_response, Args};

use crate::anonymous_server_v2::{handle_rpc_request_for_client, handle_rpc_request_internal_v2};

use crate::anonymous_server_v1::handle_rpc_request_internal_v1;

impl warp::reject::Reject for RpcError {}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousServer {
    config_path_string: String
}
impl AnonymousServer {
    pub fn new(input_config:Option<String>) -> Self {
        // Initialize any necessary resources here
        let mut config_path = input_config.unwrap_or("".to_string());
        if config_path.len()==0 {
            let default_sui_path = get_sui_config_directory();
            config_path = default_sui_path.to_string_lossy().parse().unwrap();
        }
        info!("Starting BFC Anonymous Server using config path {}", config_path);

        AnonymousServer {
            config_path_string: config_path
        }
    }
    pub async fn start(&self, addr: SocketAddr) -> anyhow::Result<()> {
        info!("Starting BFC Anonymous Server on {}", addr);

        let routes = create_routes();

        warp::serve(routes).run(addr).await;

        Ok(())
    }
}





#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = fmt::Subscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    let args = Args::parse();
    let addr: SocketAddr = format!("{}:{}", args.host, args.port).parse()?;
    let server = AnonymousServer::new(None);
    server.start(addr).await?;

    Ok(())
}

fn create_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["POST", "GET", "OPTIONS"]);

    let rpc_route = warp::path("rpc")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_rpc_request_for_client)
        .with(cors.clone());

    let health_route = warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::with_status("OK", warp::http::StatusCode::OK))
        .with(cors.clone());

    let rpc_route_internal_v2 = warp::path("rpc_internal_v2")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_rpc_request_internal_v2)
        .with(cors.clone());

    let rpc_route_internal_v1 = warp::path("rpc_internal_v1")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_rpc_request_internal_v1)
        .with(cors.clone());

    let info_route = warp::path::end()
        .and(warp::get())
        .map(|| {
            let info = serde_json::json!({
                "service": "BFC Anonymous Server",
                "version": "0.1.0",
                "endpoints": {
                    "rpc": "/rpc",
                    "health": "/health",
                    "rpc_internal_v1" : "/rpc_internal_v1",
                    "rpc_internal_v2" : "/rpc_internal_v2",

                }
            });
            warp::reply::json(&info)
        })
        .with(cors);


    rpc_route.or(health_route).or(info_route).or(rpc_route_internal_v2).or(rpc_route_internal_v1)
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct RpcError(anyhow::Error);