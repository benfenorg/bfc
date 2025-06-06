mod client_test;
mod database;

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;

use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use warp::Filter;
use tracing_subscriber::fmt;


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






#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    //todo: 1. import annoymous private key ,
    // open log system.
    // db cache system.
    //
    let subscriber = fmt::Subscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");


    let args = Args::parse();
    let addr: SocketAddr = format!("{}:{}", args.host, args.port).parse()?;

    info!("Starting BFC Anonymous Server on {}", addr);

    // 创建路由
    let routes = create_routes();

    // 启动服务器
    warp::serve(routes)
        .run(addr)
        .await;

    Ok(())
}

fn create_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // CORS 配置
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["POST", "GET", "OPTIONS"]);

    // JSON-RPC 路由
    let rpc_route = warp::path("rpc")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_rpc_request)
        .with(cors.clone());

    // 健康检查路由
    let health_route = warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::with_status("OK", warp::http::StatusCode::OK))
        .with(cors.clone());

    // 根路径信息
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
    println!("handle_anonymous_add here");
    match request.params {
        Some(params) => {
            match serde_json::from_value::<AnonymousAddParams>(params) {
                Ok(add_params) => {
                    let result1 = add_params.value1 + add_params.value2;
                    info!("Anonymous add: {} + {} = {}", add_params.value1, add_params.value2, result1);

                    let result2 = add_params.value3 + add_params.value4;
                    info!("Anonymous add: {} + {} = {}", add_params.value3, add_params.value4, result2);
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
                    let result1 = minus_params.value1 - minus_params.value2;
                    info!("Anonymous minus: {} - {} = {}", minus_params.value1, minus_params.value2, result1);

                    let result2 = minus_params.value3 - minus_params.value4;
                    info!("Anonymous minus: {} - {} = {}", minus_params.value3, minus_params.value4, result2);
                    JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: Some(serde_json::json!({
                            "result1": result1,
                            "result2": result1,
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
                    let result1 = multiply_params.value1 * multiply_params.value2;
                    info!("Anonymous multiply: {} * {} = {}", multiply_params.value1, multiply_params.value2, result1);

                    let result2 = multiply_params.value3 * multiply_params.value4;
                    info!("Anonymous multiply: {} * {} = {}", multiply_params.value1, multiply_params.value2, result2);

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