mod client_test;
mod database;
mod utils;

mod bfc_object;
mod signature;

use std::net::SocketAddr;
use anyhow::anyhow;
use warp::Rejection;

use crate::bfc_object::parse_response;
use crate::signature::verify_signature;
use crate::utils::get_object_owneraddress;
use crate::utils::public_key_bytes_to_sui_address;
use clap::Parser;
use move_core_types::account_address::AccountAddress;
use mpc_transmission::{get_sui_config_directory, two_party_share::{
    add_two_shared_secrets, mul_two_shared_secrets, recover_two_shares, recover_value,
    split_to_two_value, sub_two_shared_secrets,
}};
use mpc_transmission::{get_mask_secret_from_config};

use serde::{Deserialize, Serialize};
use sui_types::base_types_bfc::bfc_address_util::convert_to_evm_address;
use tracing::{info, warn};
use tracing_subscriber::fmt;
use warp::Filter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 9010)]
    port: u16,

    /// Host to bind to
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    #[clap(short, long, default_value = "")]
    config: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: serde_json::Value,
    method: String,
    params: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: serde_json::Value,
    result: Option<serde_json::Value>,
    error: Option<JsonRpcError>,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    data: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AnonymousAddParams {
    value1: String,
    value2: String,
    value3: String,
    value4: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AnonymousMinusParams {
    value1: String,
    value2: String,
    value3: String,
    value4: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AnonymousMultiplyParams {
    value1: String,
    value2: String,
    value3: String,
    value4: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AnonymousCompareParams {
    value1: String,
    value2: String,
    value3: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct AnonymousSplitValueParams {
    value: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct AnonymousRestoreValueParams {
    value1: String,
    value2: String,
    signature: Vec<u8>,
    objectid: String,
    publickey: Vec<u8>,
}

#[derive(Debug)]
struct RpcError(anyhow::Error);

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
    //todo: 1. import annoymous private key ,
    // open log system.
    // db cache system.
    //
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
        .and_then(handle_rpc_request)
        .with(cors.clone());

    let health_route = warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::with_status("OK", warp::http::StatusCode::OK))
        .with(cors.clone());

    let info_route = warp::path::end()
        .and(warp::get())
        .map(|| {
            let info = serde_json::json!({
                "service": "BFC Anonymous Server",
                "version": "0.1.0",
                "endpoints": {
                    "rpc": "/rpc",
                    "health": "/health"
                }
            });
            warp::reply::json(&info)
        })
        .with(cors);

    rpc_route.or(health_route).or(info_route)
}

async fn handle_rpc_request(request: JsonRpcRequest) -> Result<impl warp::Reply, Rejection> {

    info!(
        "Received RPC request: method={}, id={:?}",
        request.method, request.id
    );

    let response = match request.method.as_str() {
        "bfcx_getAnonymousAdd" => handle_anonymous_add(request).await,
        "bfcx_getAnonymousMinus" => handle_anonymous_minus(request).await,
        "bfcx_getAnonymousMultiply" => handle_anonymous_multiply(request).await,
        "bfcx_getAnonymousCompare" => handle_anonymous_compare(request).await,
        "bfcx_getAnonymousSplitValue" => handle_anonymous_split_to_two_value(request).await,
        "bfcx_getAnonymousRestoreValue" => handle_anonymous_restore_value(request).await,
        "bfcx_ping" => handle_ping(request).await,
        _ => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: None,
            error: Some(JsonRpcError {
                code: -32601,
                message: "Method not found".to_string(),
                data: None,
            }),
        },
    };

    if response.error.is_some() {
        return Err(warp::reject::custom(RpcError( anyhow!("handle_rpc_request failed"))))
    }
    Ok(warp::reply::json(&response))
}


async fn handle_anonymous_add(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousAddParams>(params) {
            Ok(add_params) => {
                println!("add_params.value1:{}", add_params.value1);
                println!("add_params.value2:{}", add_params.value2);
                println!("add_params.value3:{}", add_params.value3);
                println!("add_params.value4:{}", add_params.value4);
                let value1_share = recover_two_shares(add_params.value1, add_params.value2);
                let value2_share = recover_two_shares(add_params.value3, add_params.value4);
                if value1_share.is_err() || value2_share.is_err() {
                    return JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32602,
                            message: "Invalid params".to_string(),
                            data: Some(serde_json::json!({"error": "invalid params"})),
                        }),
                    };
                }

                let args = Args::parse();
                let mask_secret = match get_mask_secret_from_config(Some(args.config)) {
                    Ok(secret) => secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32603,
                                message: "Internal error: Failed to load configuration".to_string(),
                                data: Some(serde_json::json!({"error": e.to_string()})),
                            }),
                        };
                    }
                };
                
                match add_two_shared_secrets(
                    value1_share.unwrap(),
                    value2_share.unwrap(),
                    mask_secret,
                ) {
                    Ok(result) => {
                        let (result1, result2) = split_to_two_value(result, mask_secret);
                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: Some(serde_json::json!({
                                "result1": result1,
                                "result2": result2,
                                "operation": "anonymous_add",
                                "timestamp": chrono::Utc::now().timestamp()
                            })),
                            error: None,
                        }
                    }
                    Err(e) => {
                        warn!("Invalid parameters for bfcx_getAnonymousAdd: {}", e);
                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32602,
                                message: "Invalid params".to_string(),
                                data: Some(serde_json::json!({"error": e.to_string()})),
                            }),
                        }
                    }
                }
            }
            Err(e) => {
                warn!("Invalid parameters for bfcx_getAnonymousAdd: {}", e);
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: "Invalid params".to_string(),
                        data: Some(serde_json::json!({"error": e.to_string()})),
                    }),
                }
            }
        },
        None => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: None,
            error: Some(JsonRpcError {
                code: -32602,
                message: "Missing params".to_string(),
                data: None,
            }),
        },
    }
}

async fn handle_anonymous_minus(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousMinusParams>(params) {
            Ok(minus_params) => {
                let value1_share = recover_two_shares(minus_params.value1, minus_params.value2);
                let value2_share = recover_two_shares(minus_params.value3, minus_params.value4);
                if value1_share.is_err() || value2_share.is_err() {
                    return JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32602,
                            message: "Invalid params".to_string(),
                            data: Some(serde_json::json!({"error": "invalid params"})),
                        }),
                    };
                }

                let args = Args::parse();
                let mask_secret = match get_mask_secret_from_config(Some(args.config)) {
                    Ok(secret) => secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32603,
                                message: "Internal error: Failed to load configuration".to_string(),
                                data: Some(serde_json::json!({"error": e.to_string()})),
                            }),
                        };
                    }
                };
                
                match sub_two_shared_secrets(
                    value1_share.unwrap(),
                    value2_share.unwrap(),
                    mask_secret,
                ) {
                    Ok(result) => {
                        let (result1, result2) = split_to_two_value(result, mask_secret);
                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: Some(serde_json::json!({
                                "result1": result1,
                                "result2": result2,
                                "operation": "anonymous_minus",
                                "timestamp": chrono::Utc::now().timestamp()
                            })),
                            error: None,
                        }
                    }
                    Err(e) => {
                        warn!("Invalid parameters for bfcx_getAnonymousMinus: {}", e);
                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32602,
                                message: "Invalid params".to_string(),
                                data: Some(serde_json::json!({"error": e.to_string()})),
                            }),
                        }
                    }
                }
            }
            Err(e) => {
                warn!("Invalid parameters for bfcx_getAnonymousMinus: {}", e);
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: "Invalid params".to_string(),
                        data: Some(serde_json::json!({"error": e.to_string()})),
                    }),
                }
            }
        },
        None => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: None,
            error: Some(JsonRpcError {
                code: -32602,
                message: "Missing params".to_string(),
                data: None,
            }),
        },
    }
}

async fn handle_anonymous_multiply(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousMultiplyParams>(params) {
            Ok(multiply_params) => {
                let value1_share =
                    recover_two_shares(multiply_params.value1, multiply_params.value2);
                let value2_share =
                    recover_two_shares(multiply_params.value3, multiply_params.value4);
                if value1_share.is_err() || value2_share.is_err() {
                    return JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32602,
                            message: "Invalid params".to_string(),
                            data: Some(serde_json::json!({"error": "invalid params"})),
                        }),
                    };
                }
                let args = Args::parse();

                let mask_secret = match get_mask_secret_from_config(Some(args.config)) {
                    Ok(secret) => secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32603,
                                message: "Internal error: Failed to load configuration".to_string(),
                                data: Some(serde_json::json!({"error": e.to_string()})),
                            }),
                        };
                    }
                };
                
                match mul_two_shared_secrets(
                    value1_share.unwrap(),
                    value2_share.unwrap(),
                    mask_secret,
                ) {
                    Ok(result) => {
                        let (result1, result2) = split_to_two_value(result, mask_secret);
                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: Some(serde_json::json!({
                                "result1": result1,
                                "result2": result2,
                                "operation": "anonymous_multiply",
                                "timestamp": chrono::Utc::now().timestamp()
                            })),
                            error: None,
                        }
                    }
                    Err(e) => {
                        warn!("Invalid parameters for bfcx_getAnonymousMultiply: {}", e);
                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32602,
                                message: "Invalid params".to_string(),
                                data: Some(serde_json::json!({"error": e.to_string()})),
                            }),
                        }
                    }
                }
            }
            Err(e) => {
                warn!("Invalid parameters for bfcx_getAnonymousMultiply: {}", e);
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: "Invalid params".to_string(),
                        data: Some(serde_json::json!({"error": e.to_string()})),
                    }),
                }
            }
        },
        None => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: None,
            error: Some(JsonRpcError {
                code: -32602,
                message: "Missing params".to_string(),
                data: None,
            }),
        },
    }
}

#[warn(unused_assignments)]
async fn handle_anonymous_restore_value(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => {
            match serde_json::from_value::<AnonymousRestoreValueParams>(params) {
                Ok(restore_value_params) => {
                    let signature = restore_value_params.signature;
                    let objectid = restore_value_params.objectid;
                    let mut pass_verify_signature = verify_signature(
                        &restore_value_params.publickey,
                        &*signature,
                        objectid.as_bytes(),
                    )
                    .is_ok();
                    info!("temporary skip check, important todo need object ownership check to continue restore value!!!!!");
                    if pass_verify_signature == true {
                        match get_object_owneraddress(objectid.clone()).await {
                            Ok(owner_address_value) => {
                                let sui_address_from_send = public_key_bytes_to_sui_address(
                                    restore_value_params.publickey.clone(),
                                );
                                let owner_address_from_send =
                                    AccountAddress::from(sui_address_from_send);
                                let evm_addr_from_system =
                                    convert_to_evm_address(owner_address_value.clone());
                                pass_verify_signature = evm_addr_from_system
                                    == owner_address_from_send.to_hex_with_hex_head();
                            }
                            Err(error) => {
                                info!("failed get owner address: {}", error);
                                pass_verify_signature = false;
                            }
                        }
                    }

                    if pass_verify_signature == false {
                        return JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32603,
                                message: "Verify signature false".to_string(),
                                data: Some(serde_json::json!({"error": "verify signature false"})),
                            }),
                        };
                    }

                    //todo,signature check,address.
                    // edd25519 signature check

                    let args = Args::parse();
                    let mask_secret = match get_mask_secret_from_config(Some(args.config)) {
                        Ok(secret) => secret,
                        Err(e) => {
                            warn!("Failed to get mask secret from config: {}", e);
                            return JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                id: request.id,
                                result: None,
                                error: Some(JsonRpcError {
                                    code: -32603,
                                    message: "Internal error: Failed to load configuration".to_string(),
                                    data: Some(serde_json::json!({"error": e.to_string()})),
                                }),
                            };
                        }
                    };
                    
                    let data1 = restore_value_params.value1;
                    let data2 = restore_value_params.value2;
                    match recover_value(data1, data2, mask_secret) {
                        Ok(value) => JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: Some(serde_json::json!({
                                "result1": value,
                                "result2": 0,
                                "operation": "anonymous_restore_value",
                                "timestamp": chrono::Utc::now().timestamp()
                            })),
                            error: None,
                        },
                        Err(e) => {
                            warn!(
                                "Invalid parameters for bfcx_getAnonymousRestoreValue: {}",
                                e
                            );
                            JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                id: request.id,
                                result: None,
                                error: Some(JsonRpcError {
                                    code: -32602,
                                    message: "Invalid params".to_string(),
                                    data: Some(serde_json::json!({"error": e.to_string()})),
                                }),
                            }
                        }
                    }
                }
                Err(e) => {
                    warn!(
                        "Invalid parameters for bfcx_getAnonymousRestoreValue: {}",
                        e
                    );
                    JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32602,
                            message: "Invalid params".to_string(),
                            data: Some(serde_json::json!({"error": e.to_string()})),
                        }),
                    }
                }
            }
        }
        None => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: None,
            error: Some(JsonRpcError {
                code: -32602,
                message: "Missing params".to_string(),
                data: None,
            }),
        },
    }
}

async fn handle_anonymous_split_to_two_value(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousSplitValueParams>(params) {
            Ok(split_to_two_value_params) => {
                let args = Args::parse();
                let mask_secret = match get_mask_secret_from_config(Some(args.config)) {
                    Ok(secret) => secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32603,
                                message: "Internal error: Failed to load configuration".to_string(),
                                data: Some(serde_json::json!({"error": e.to_string()})),
                            }),
                        };
                    }
                };
                
                let value = split_to_two_value_params.value;
                let (result1, result2) = split_to_two_value(value, mask_secret);
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(serde_json::json!({
                        "result1": result1,
                        "result2": result2,
                        "operation": "anonymous_split_to_two_value",
                        "timestamp": chrono::Utc::now().timestamp()
                    })),
                    error: None,
                }
            }
            Err(e) => {
                warn!("Invalid parameters for bfcx_getAnonymousSplitValue: {}", e);
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: "Invalid params".to_string(),
                        data: Some(serde_json::json!({"error": e.to_string()})),
                    }),
                }
            }
        },
        None => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: None,
            error: Some(JsonRpcError {
                code: -32602,
                message: "Missing params".to_string(),
                data: None,
            }),
        },
    }
}

async fn handle_anonymous_compare(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousCompareParams>(params) {
            Ok(compare_params) => {
                let args = Args::parse();
                let mask_secret = match get_mask_secret_from_config(Some(args.config)) {
                    Ok(secret) => secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32603,
                                message: "Internal error: Failed to load configuration".to_string(),
                                data: Some(serde_json::json!({"error": e.to_string()})),
                            }),
                        };
                    }
                };
                
                match recover_value(compare_params.value1, compare_params.value2, mask_secret) {
                    Ok(value_a) => {
                        let value_b = compare_params.value3;
                        let comparison = if value_a > value_b {
                            "1"
                        } else if value_a < value_b {
                            "2"
                        } else {
                            "0"
                        };
                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: Some(serde_json::json!({
                                "result1": comparison.to_string(),
                                "result2": "0".to_string(),
                                "operation": "anonymous_compare",
                                "timestamp": chrono::Utc::now().timestamp()
                            })),
                            error: None,
                        }
                    }
                    Err(e) => {
                        warn!("Invalid parameters for bfcx_getAnonymousCompare: {}", e);
                        JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32602,
                                message: "Invalid params".to_string(),
                                data: Some(serde_json::json!({"error": e.to_string()})),
                            }),
                        }
                    }
                }
            }
            Err(e) => {
                warn!("Invalid parameters for bfcx_getAnonymousCompare: {}", e);
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: "Invalid params".to_string(),
                        data: Some(serde_json::json!({"error": e.to_string()})),
                    }),
                }
            }
        },
        None => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: None,
            error: Some(JsonRpcError {
                code: -32602,
                message: "Missing params".to_string(),
                data: None,
            }),
        },
    }
}

async fn handle_ping(request: JsonRpcRequest) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: request.id,
        result: Some(serde_json::json!({
            "message": "pong",
            "timestamp": chrono::Utc::now().timestamp()
        })),
        error: None,
    }
}
