mod client_test;
mod database;
mod utils;

mod signature;
mod bfc_object;

use std::convert::Infallible;
use std::net::SocketAddr;

use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use warp::Filter;
use tracing_subscriber::fmt;
use serde_json::json;
use crate::bfc_object::parse_response;
use anyhow::anyhow;
use crate::signature::verify_signature;
use crate::utils::get_object_owneraddress;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 9010)]
    port: u16,

    /// Host to bind to
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
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
    value1: u64,
    value2: u64,
    value3: u64,
    value4: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct AnonymousMinusParams {
    value1: u64,
    value2: u64,
    value3: u64,
    value4: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct AnonymousMultiplyParams {
    value1: u64,
    value2: u64,
    value3: u64,
    value4: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct AnonymousCompareParams {
    value1: u64,
    value2: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct AnonymousSplitValueParams {
    value: u64,
}

#[derive(Debug, Deserialize, Serialize)]
struct AnonymousRestoreValueParams {
    value1: u64,
    value2: u64,
    signature: Vec<u8>,
    Objectid : String,

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

    info!("Starting BFC Anonymous Server on {}", addr);

    let routes = create_routes();

    warp::serve(routes)
        .run(addr)
        .await;

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

async fn handle_rpc_request(request: JsonRpcRequest) -> Result<impl warp::Reply, Infallible> {
    info!("Received RPC request: method={}, id={:?}", request.method, request.id);

    let response = match request.method.as_str() {
        "bfcx_getAnonymousAdd" => handle_anonymous_add(request).await,
        "bfcx_getAnonymousMinus" => handle_anonymous_minus(request).await,
        "bfcx_getAnonymousMultiply" => handle_anonymous_multiply(request).await,
        "bfcx_getAnonymousCompare" => handle_anonymous_compare(request).await,
        "bfcx_getAnonymousSplitValue"  => handle_anonymous_split_value(request).await,
        "bfcx_getAnonymousRestoreValue"  => handle_anonymous_restore_value(request).await,

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

    Ok(warp::reply::json(&response))
}

async fn handle_anonymous_add(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => {
            match serde_json::from_value::<AnonymousAddParams>(params) {
                Ok(add_params) => {
                    let data1 = add_params.value1 + add_params.value2;
                    let data2 = add_params.value3 + add_params.value4;

                    if data1.checked_add(data2) == None {
                        return JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32602,
                                message: "Arithmetic overflow".to_string(),
                                data: Some(serde_json::json!({"error": "arithmetic overflow"})),
                            }),
                        }
                    }
                    let result = data1 + data2;
                    let result1 = result / 2;
                    let result2 = result - result1;
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
        None => {
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32602,
                    message: "Missing params".to_string(),
                    data: None,
                }),
            }
        }
    }
}

async fn handle_anonymous_minus(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => {
            match serde_json::from_value::<AnonymousMinusParams>(params) {
                Ok(minus_params) => {

                    let data1 = minus_params.value1 + minus_params.value2;
                    let data2 = minus_params.value3 + minus_params.value4;

                    if data1.checked_sub(data2) == None {
                        return JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32602,
                                message: "Arithmetic overflow".to_string(),
                                data: Some(serde_json::json!({"error": "arithmetic overflow"})),
                            }),
                        };
                    }
                    let result = data1 - data2;
                    let result1 = result / 2;
                    let result2 = result - result1;
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
        None => {
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32602,
                    message: "Missing params".to_string(),
                    data: None,
                }),
            }
        }
    }
}

async fn handle_anonymous_multiply(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => {
            match serde_json::from_value::<AnonymousMultiplyParams>(params) {
                Ok(multiply_params) => {
                    let data1 = multiply_params.value1 + multiply_params.value2;
                    let data2 = multiply_params.value3 + multiply_params.value4;

                    if data1.checked_mul(data2) == None {
                        return JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32602,
                                message: "Arithmetic overflow".to_string(),
                                data: Some(serde_json::json!({"error": "arithmetic overflow"})),
                            }),
                        };
                    }

                    let result = data1 * data2;

                    let result1 = result / 2;
                    let result2 = result - result1;

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
        None => {
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32602,
                    message: "Missing params".to_string(),
                    data: None,
                }),
            }
        }
    }
}


async fn handle_anonymous_restore_value(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => {
            match serde_json::from_value::<AnonymousRestoreValueParams>(params) {
                Ok(restore_value_params) => {
                    let signature = restore_value_params.signature;
                    let objectid = restore_value_params.Objectid;

                    let owner_address = get_object_owneraddress(objectid.clone()).await;
                    let mut pass_verify_signature = true;
                    match owner_address {
                        Ok(value) => {
                            info!("get owner address: {}", value);
                            let result = verify_signature(value.as_bytes(), &*signature, objectid.as_bytes());
                            pass_verify_signature = result.is_ok();
                        },
                        Err(error) => {
                            info!("failed get owner address: {}", error);
                            pass_verify_signature = false;
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


                    info!("handle_anonymous_restore_value get signature{:?} object id{:?}", signature, objectid);
                    info!("temporary skip check, important todo need object ownership check to continue restore value!!!!!");

                    let data1 = restore_value_params.value1;
                    let data2 = restore_value_params.value2;
                    if data1.checked_add(data2) == None {
                        return JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32602,
                                message: "Arithmetic overflow".to_string(),
                                data: Some(serde_json::json!({"error": "arithmetic overflow"})),
                            }),
                        };
                    }
                    let value = data1 + data2;

                    JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: Some(serde_json::json!({
                            "result1": value,
                            "result2": 0,
                            "operation": "anonymous_restore_value",
                            "timestamp": chrono::Utc::now().timestamp()
                        })),
                        error: None,
                    }

                }
                Err(e) => {
                    warn!("Invalid parameters for bfcx_getAnonymousRestoreValue: {}", e);
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
        None => {
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32602,
                    message: "Missing params".to_string(),
                    data: None,
                }),
            }
        }
    }
}

async fn handle_anonymous_split_value(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => {
            match serde_json::from_value::<AnonymousSplitValueParams>(params) {
                Ok(split_value_params) => {
                    let value = split_value_params.value;
                    let value1 = value/2;
                    let value2 = value - value1;

                    JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: Some(serde_json::json!({
                            "result1": value1,
                            "result2": value2,
                            "operation": "anonymous_split_value",
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
            }
        }
        None => {
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32602,
                    message: "Missing params".to_string(),
                    data: None,
                }),
            }
        }
    }
}

async fn handle_anonymous_compare(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => {
            match serde_json::from_value::<AnonymousCompareParams>(params) {
                Ok(compare_params) => {
                    let comparison = if compare_params.value1 > compare_params.value2 {
                        1
                    } else if compare_params.value1 < compare_params.value2 {
                        2
                    } else {
                        0
                    };
                    
                    info!("Anonymous compare: {} vs {} = {}", compare_params.value1, compare_params.value2, comparison);
                    
                    JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: Some(serde_json::json!({
                            "result": comparison,
                            "value1": compare_params.value1,
                            "value2": compare_params.value2,
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
        None => {
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32602,
                    message: "Missing params".to_string(),
                    data: None,
                }),
            }
        }
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