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
use crate::utils::{get_object_owneraddress, verify_zklogin_signature, ZkVerifyRequest};
use crate::utils::public_key_bytes_to_sui_address;
use clap::Parser;
use ed25519_dalek::ed25519::signature::digest;
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
use fastcrypto::hash::HashFunction;
use mpc_transmission::get_zklogin_rpc_address_from_config;
const PERSONAL_MESSAGE_PREFIX: &[u8; 3] = &[3, 0, 0];


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
    value1: Vec<u8>,
    value2: Vec<u8>,
    signature: Vec<u8>,
    objectid: String,
    publickey: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AnonymousRestoreZKLoginParams {
    value1: Vec<u8>,
    value2: Vec<u8>,
    signature: ZkVerifyRequest,
    objectid: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AnonymousRestoreArrayParamsZKLoginParams {
    anonymous_restore_array: Vec<AnonymousRestoreElementParams>,
    signature: ZkVerifyRequest,
    object_ids: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AnonymousRestoreElementParams {
    value1: Vec<u8>,
    value2: Vec<u8>,
    objectid: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AnonymousRestoreArrayParams {
    anonymous_restore_array: Vec<AnonymousRestoreElementParams>,
    signature: Vec<u8>,
    publickey: Vec<u8>,
}


#[derive(Debug)]
#[allow(dead_code)]
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



fn create_error_response(requestid : serde_json::Value, code: i32, message: String, data: Option<serde_json::Value>) -> JsonRpcResponse {
    let result = JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: requestid,
        result: None,
        error: Some(JsonRpcError {
            code,
            message,
            data: data,
        }),
    };
    result
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
        "bfcx_getAnonymousEncodeData" => handle_anonymous_split_to_two_value(request).await,
        "bfcx_getAnonymousRestoreValue" => handle_anonymous_restore_value(request).await,
        "bfcx_getAnonymousRestoreValueArray" => handle_anonymous_restore_value_array(request).await,
        "bfcx_getAnonymousRestoreValueForZKloginAddress" => handle_anonymous_restore_value_for_zklogin_address(request).await,
        "bfcx_getAnonymousRestoreValueArrayForZKloginAddress" => handle_anonymous_restore_value_array_for_zklogin_address(request).await,
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
        return Err(warp::reject::custom(RpcError( anyhow!("handle_rpc_request failed, caused by {}", response.error.unwrap().message))))
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
                    return create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": "invalid params"})));
                }

                let args_result = Args::try_parse();
                let mut config_path : Option<String> = None;
                if args_result.is_ok() {
                    config_path = Some(args_result.unwrap().config);
                }

                let mask_secret = match get_mask_secret_from_config(config_path) {
                    Ok(secret) => secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Internal error: Failed to load configuration".to_string(),
                                                     Some(serde_json::json!({"error": e.to_string()})));
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
                        create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
                    }
                }
            }
            Err(e) => {
                warn!("Invalid parameters for bfcx_getAnonymousAdd: {}", e);
                create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
            }
        },
        None => {
            create_error_response(request.id, -32602, "Missing params".to_string(), None)
        }
    }
}

async fn handle_anonymous_minus(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousMinusParams>(params) {
            Ok(minus_params) => {
                let value1_share = recover_two_shares(minus_params.value1, minus_params.value2);
                let value2_share = recover_two_shares(minus_params.value3, minus_params.value4);
                if value1_share.is_err() || value2_share.is_err() {
                    return create_error_response(request.id, -32602, "Invalid params".to_string(),
                                                 Some(serde_json::json!({"error": "invalid params"})));
                }

                let args_result = Args::try_parse();
                let mut config_path : Option<String> = None;
                if args_result.is_ok() {
                    config_path = Some(args_result.unwrap().config);
                }

                let mask_secret = match get_mask_secret_from_config(config_path) {
                    Ok(secret) => secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,-32603 , "Internal error: Failed to load configuration".to_string(), Some(serde_json::json!({"error": e.to_string()})))
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
                        create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
                    }
                }
            }
            Err(e) => {
                warn!("Invalid parameters for bfcx_getAnonymousMinus: {}", e);
                create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
            }
        },
        None => {
            create_error_response(request.id, -32602, "Missing params".to_string(), None)
        }
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
                    return create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": "invalid params"})));

                }

                let args_result = Args::try_parse();
                let mut config_path : Option<String> = None;
                if args_result.is_ok() {
                    config_path = Some(args_result.unwrap().config);
                }
                let mask_secret = match get_mask_secret_from_config(config_path) {
                    Ok(secret) => secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Internal error: Failed to load configuration".to_string(),
                                                     Some(serde_json::json!({"error": e.to_string()})));
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
                        create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
                    }
                }
            }
            Err(e) => {
                warn!("Invalid parameters for bfcx_getAnonymousMultiply: {}", e);
                create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
            }
        },
        None => {
            create_error_response(request.id, -32602, "Missing params".to_string(), None)
        }
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

                    let mut intent_data = Vec::new();
                    intent_data.extend_from_slice(PERSONAL_MESSAGE_PREFIX);
                    let len = objectid.len() as u64;
                    let mut buffer = [0u8; 10];
                    let length = write_unsigned_leb128(&mut buffer, len);
                    intent_data.extend_from_slice(&buffer[..length]);
                    intent_data.extend_from_slice(objectid.as_bytes());

                    let digest = fastcrypto::hash::Blake2b256::digest(intent_data);
                    let logdata = hex::encode(digest.as_ref());

                    info!("handle_anonymous_restore_value intent data: {}", logdata);

                    let mut pass_verify_signature = verify_signature(
                        &restore_value_params.publickey,
                        &*signature,
                        digest.as_ref()
                    )
                        .is_ok();
                    info!("temporary skip check, important todo need object ownership check to continue restore value!!!!!");
                    if pass_verify_signature == true {
                        info!("handle_anonymous_restore_value pass verify signature");
                        match get_object_owneraddress(objectid.clone()).await {
                            Ok(owner_address_value) => {
                                let sui_address_from_send_result = public_key_bytes_to_sui_address(
                                    restore_value_params.publickey.clone(),
                                );
                                if sui_address_from_send_result.is_err() {
                                    info!("failed public key to sui address: {:?}", sui_address_from_send_result.err());
                                    pass_verify_signature = false;
                                } else {
                                    let sui_address_from_send = sui_address_from_send_result.unwrap();
                                    let owner_address_from_send =
                                        AccountAddress::from(sui_address_from_send);
                                    let evm_addr_from_system =
                                        convert_to_evm_address(owner_address_value.clone());
                                    pass_verify_signature = evm_addr_from_system
                                        == owner_address_from_send.to_hex_with_hex_head();
                                }
                            }
                            Err(error) => {
                                info!("failed get owner address: {}", error);
                                pass_verify_signature = false;
                            }
                        }
                    }

                    if pass_verify_signature == false {
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Verify signature or get owner address failed".to_string(),
                                                     Some(serde_json::json!({"error": "verify signature or get owner address failed"})));
                    }


                    let args_result = Args::try_parse();
                    let mut config_path : Option<String> = None;
                    if args_result.is_ok() {
                        config_path = Some(args_result.unwrap().config);
                    }
                    let mask_secret = match get_mask_secret_from_config(config_path) {
                        Ok(secret) => secret,
                        Err(e) => {
                            warn!("Failed to get mask secret from config: {}", e);
                            return create_error_response(request.id,
                                                         -32603,
                                                         "Internal error: Failed to load configuration".to_string(),
                                                         Some(serde_json::json!({"error": e.to_string()})));
                        }
                    };

                    let data1 = restore_value_params.value1;
                    let data2 = restore_value_params.value2;
                    info!("data1 len: {}, data2 len: {}", data1.len(), data2.len());

                    let data_str1 = String::from_utf8(data1).unwrap_or_default();
                    let data_str2 = String::from_utf8(data2).unwrap_or_default();
                    info!("=== data_str1: {}, data_str2: {} ===", data_str1, data_str2);

                    match recover_value(data_str1, data_str2, mask_secret) {
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
                                "process recover_value error, caused by: {}",
                                e
                            );
                            create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
                        }
                    }
                }
                Err(e) => {
                    warn!(
                        "Invalid parameters for bfcx_getAnonymousRestoreValue: {}",
                        e
                    );
                    create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
                }
            }
        }
        None => {
            create_error_response(request.id, -32602, "Missing params".to_string(), None)
        }
    }
}

#[warn(unused_assignments)]
async fn handle_anonymous_restore_value_array_for_zklogin_address(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => {
            match serde_json::from_value::<AnonymousRestoreArrayParamsZKLoginParams>(params) {
                Ok(restore_value_params) => {
                    let signature = restore_value_params.signature;
                    let args_result = Args::try_parse();
                    let mut config_path: Option<String> = None;
                    if args_result.is_ok() {
                        config_path = Some(args_result.unwrap().config);
                    }

                    let zklogin_address = match get_zklogin_rpc_address_from_config(config_path.clone()) {
                        Ok(address) => address,
                        Err(e) => {
                            warn!("Failed to get zklogin address from config: {}", e);
                            return create_error_response(request.id, -32603, "Internal error: Failed to load configuration".to_string(), Some(serde_json::json!({"error": e.to_string()})))
                        }
                    };

                    let mut pass_verify_signature = verify_zklogin_signature(
                        signature,
                        zklogin_address
                    ).await.is_ok();
                    info!("temporary skip check, important todo need object ownership check to continue restore value!!!!!");

                    if pass_verify_signature == false {
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Verify signature or get owner address failed".to_string(),
                                                     Some(serde_json::json!({"error": "verify signature or get owner address failed"})));
                    }

                    let mask_secret = match get_mask_secret_from_config(config_path) {
                        Ok(secret) => secret,
                        Err(e) => {
                            warn!("Failed to get mask secret from config: {}", e);
                            return create_error_response(request.id,
                                                         -32603,
                                                         "Internal error: Failed to load configuration".to_string(),
                                                         Some(serde_json::json!({"error": e.to_string()})));
                        }
                    };
                    let mut restore_result_array = Vec::new();

                    let mut object_ids =  String::new();

                    for anonymous_restore_value in &restore_value_params.anonymous_restore_array {
                        let data1 = anonymous_restore_value.value1.clone();
                        let data2 = anonymous_restore_value.value2.clone();
                        object_ids = format!("{}{}", object_ids, anonymous_restore_value.objectid);
                        info!("data1 len: {}, data2 len: {}", data1.len(), data2.len());

                        let data_str1 = String::from_utf8(data1).unwrap_or_default();
                        let data_str2 = String::from_utf8(data2).unwrap_or_default();
                        info!("=== data_str1: {}, data_str2: {} ===", data_str1, data_str2);

                        match recover_value(data_str1, data_str2, mask_secret) {
                            Ok(value) => {
                                restore_result_array.push(value);
                            },
                            Err(e) => {
                                restore_result_array.push(0);
                                warn!("process recover_value error, caused by: {}",e);
                            }
                        }
                    }

                    if object_ids != restore_value_params.object_ids {
                        warn!(
                        "authentication failed for bfcx_getAnonymousRestoreArrayParamsZKLoginParams"
                        );
                        return create_error_response(request.id, -32602, "authentication failed".to_string(), None);
                    }

                    JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: Some(serde_json::json!({
                        "result1": restore_result_array,
                        "result2": 0,
                        "operation": "anonymous_multiply",
                        "timestamp": chrono::Utc::now().timestamp()
                    })),
                        error: None,
                    }
                }

                Err(e) => {
                    warn!(
                        "Invalid parameters for bfcx_getAnonymousRestoreArrayParamsZKLoginParams: {}",
                        e
                    );
                    create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
                }
            }
        }
        None => {
            create_error_response(request.id, -32602, "Missing params".to_string(), None)
        }
    }
}


#[warn(unused_assignments)]
async fn handle_anonymous_restore_value_for_zklogin_address(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => {
            match serde_json::from_value::<AnonymousRestoreZKLoginParams>(params) {
                Ok(restore_value_params) => {
                    let author = restore_value_params.signature.author.clone();
                    let signature = restore_value_params.signature;
                    let objectid = restore_value_params.objectid;
                    let args_result = Args::try_parse();
                    let mut config_path: Option<String> = None;
                    if args_result.is_ok() {
                        config_path = Some(args_result.unwrap().config);
                    }

                    let zklogin_address = match get_zklogin_rpc_address_from_config(config_path) {
                        Ok(address) => address,
                        Err(e) => {
                            warn!("Failed to get zklogin address from config: {}", e);
                            return create_error_response(request.id, -32603, "Internal error: Failed to load configuration".to_string(), Some(serde_json::json!({"error": e.to_string()})))
                        }
                    };


                    let mut pass_verify_signature = verify_zklogin_signature(
                        signature,
                        zklogin_address
                    ).await.is_ok();
                    info!("temporary skip check, important todo need object ownership check to continue restore value!!!!!");

                    if pass_verify_signature == true {
                        info!("handle_anonymous_restore_value pass verify signature");
                        match get_object_owneraddress(objectid.clone()).await {
                            Ok(owner_address_value) => {
                                pass_verify_signature = author.eq(&owner_address_value);
                            }
                            Err(error) => {
                                info!("failed get owner address: {}", error);
                                pass_verify_signature = false;
                            }
                        }
                    }
                    if pass_verify_signature == false {
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Verify signature or get owner address failed".to_string(),
                                                     Some(serde_json::json!({"error": "verify signature or get owner address failed"})));
                    }


                    let args_result = Args::try_parse();
                    let mut config_path: Option<String> = None;
                    if args_result.is_ok() {
                        config_path = Some(args_result.unwrap().config);
                    }
                    let mask_secret = match get_mask_secret_from_config(config_path) {
                        Ok(secret) => secret,
                        Err(e) => {
                            warn!("Failed to get mask secret from config: {}", e);
                            return create_error_response(request.id,
                                                         -32603,
                                                         "Internal error: Failed to load configuration".to_string(),
                                                         Some(serde_json::json!({"error": e.to_string()})));
                        }
                    };

                    let data1 = restore_value_params.value1;
                    let data2 = restore_value_params.value2;
                    info!("data1 len: {}, data2 len: {}", data1.len(), data2.len());

                    let data_str1 = String::from_utf8(data1).unwrap_or_default();
                    let data_str2 = String::from_utf8(data2).unwrap_or_default();
                    info!("=== data_str1: {}, data_str2: {} ===", data_str1, data_str2);

                    match recover_value(data_str1, data_str2, mask_secret) {
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
                            "process recover_value error, caused by: {}",
                            e
                        );
                            create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
                        }
                    }
                }
                Err(e) => {
                    warn!(
                    "Invalid parameters for bfcx_getAnonymousRestoreValue: {}",
                    e
                );
                    create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
                }
            }
        }
        None => {
            create_error_response(request.id, -32602, "Missing params".to_string(), None)
        }
    }
}


#[warn(unused_assignments)]
async fn handle_anonymous_restore_value_array(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => {
            let response  = serde_json::from_value(params);
            if response.is_err() {
                warn!(
                        "Invalid parameters for bfcx_getAnonymousRestoreValueArray: {:?}",
                        response.err()
                    );
                return create_error_response(request.id, -32602, "Invalid params".to_string(), None)
            }

            let anonymous_restore_value_array: AnonymousRestoreArrayParams = response.unwrap();
            let mut object_ids = String::new();
            for anonymous_restore_value in &anonymous_restore_value_array.anonymous_restore_array {
                object_ids = format!("{}{}", object_ids, anonymous_restore_value.objectid);
            }

            let mut intent_data = Vec::new();
            intent_data.extend_from_slice(PERSONAL_MESSAGE_PREFIX);
            let len = object_ids.len() as u64;
            let mut buffer = [0u8; 10];
            let length = write_unsigned_leb128(&mut buffer, len);
            intent_data.extend_from_slice(&buffer[..length]);
            intent_data.extend_from_slice(object_ids.as_bytes());
            println!("value array before {:?}", hex::encode(intent_data.clone()));

            let digest = fastcrypto::hash::Blake2b256::digest(intent_data);
            println!("value array after{:?}", hex::encode(digest));

            let mut pass_authentication = verify_signature(
                &anonymous_restore_value_array.publickey,
                &*anonymous_restore_value_array.signature,
                digest.as_ref(),
            ).is_ok();
            if pass_authentication == false {
                warn!(
                        "Invalid authentication for bfcx_getAnonymousRestoreValueArray",
                    );
                return create_error_response(request.id, -32602, "Invalid authentication".to_string(), None)
            }

            info!("temporary skip check, important todo need object ownership check to continue restore value!!!!!");
            let mut restore_result_array = Vec::new();

            for anonymous_restore_value in &anonymous_restore_value_array.anonymous_restore_array {
                let objectid = anonymous_restore_value.objectid.clone();
                match get_object_owneraddress(objectid.clone()).await {
                    Ok(owner_address_value) => {
                        let sui_address_from_send_result = public_key_bytes_to_sui_address(
                            anonymous_restore_value_array.publickey.clone(),
                        );
                        if sui_address_from_send_result.is_err() {
                            info!("failed public key to sui address: {:?}", sui_address_from_send_result.err());
                            pass_authentication = false;
                        } else {
                            let sui_address_from_send = sui_address_from_send_result.unwrap();
                            let owner_address_from_send =
                                AccountAddress::from(sui_address_from_send);
                            let evm_addr_from_system =
                                convert_to_evm_address(owner_address_value.clone());
                            pass_authentication = evm_addr_from_system
                                == owner_address_from_send.to_hex_with_hex_head();
                        }
                    }
                    Err(error) => {
                        info!("failed get owner address: {}", error);
                        pass_authentication = false;
                    }
                }

                if pass_authentication == false {
                    restore_result_array.push(0);
                    continue;
                }

                let args_result = Args::try_parse();
                let mut config_path : Option<String> = None;
                if args_result.is_ok() {
                    config_path = Some(args_result.unwrap().config);
                }
                let mask_secret = match get_mask_secret_from_config(config_path) {
                    Ok(secret) => secret,
                    Err(e) => {
                        info!("get_mask_secret_from_config failed, caused by: {}", e);
                        restore_result_array.push(0);
                        continue;
                    }
                };

                let data1 = anonymous_restore_value.value1.clone();
                let data2 = anonymous_restore_value.value2.clone();
                info!("data1 len: {}, data2 len: {}", data1.len(), data2.len());

                let data_str1 = String::from_utf8(data1).unwrap_or_default();
                let data_str2 = String::from_utf8(data2).unwrap_or_default();
                info!("=== data_str1: {}, data_str2: {} ===", data_str1, data_str2);

                match recover_value(data_str1, data_str2, mask_secret) {
                    Ok(value) => {
                        restore_result_array.push(value);
                    },
                    Err(e) => {
                        restore_result_array.push(0);
                        warn!("process recover_value error, caused by: {}",e);
                    }
                }

            }
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(serde_json::json!({
                                "result1": restore_result_array,
                                "result2": 0,
                                "operation": "anonymous_multiply",
                                "timestamp": chrono::Utc::now().timestamp()
                            })),
                error: None,
            }
        }
        None => {
            create_error_response(request.id, -32602, "Missing params".to_string(), None)
        }
    }
}



async fn handle_anonymous_split_to_two_value(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousSplitValueParams>(params) {
            Ok(split_to_two_value_params) => {

                let args_result = Args::try_parse();
                let mut config_path : Option<String> = None;
                if args_result.is_ok() {
                    config_path = Some(args_result.unwrap().config);
                }
                let mask_secret = match get_mask_secret_from_config(config_path) {
                    Ok(secret) => secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Internal error: Failed to load configuration".to_string(),
                                                     Some(serde_json::json!({"error": e.to_string()})));
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
                warn!("Invalid parameters for bfcx_getAnonymousEncodeData: {}", e);
                create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
            }
        },
        None => {
            create_error_response(request.id, -32602, "Missing params".to_string(), None)
        }
    }
}

async fn handle_anonymous_compare(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousCompareParams>(params) {
            Ok(compare_params) => {
                let args_result = Args::try_parse();
                let mut config_path : Option<String> = None;
                if args_result.is_ok() {
                    config_path = Some(args_result.unwrap().config);
                }

                let mask_secret = match get_mask_secret_from_config(config_path) {
                    Ok(secret) => secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Internal error: Failed to load configuration".to_string(),
                                                     Some(serde_json::json!({"error": e.to_string()})));
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
                        create_error_response(request.id,
                                              -32602,
                                              "Invalid params".to_string(),
                                              Some(serde_json::json!({"error": e.to_string()})))
                    }
                }
            }
            Err(e) => {
                warn!("Invalid parameters for bfcx_getAnonymousCompare: {}", e);
                create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
            }
        },
        None => {
            create_error_response(request.id, -32602, "Missing params".to_string(), None)
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

pub fn write_unsigned_leb128(out: &mut [u8], mut value: u64) -> usize {
    let mut i = 0;
    loop {
        if value < 0x80 {
            out[i] = value as u8;
            i += 1;
            break;
        } else {
            out[i] = ((value & 0x7F) | 0x80) as u8;
            value >>= 7;
            i += 1;
        }
    }
    i
}