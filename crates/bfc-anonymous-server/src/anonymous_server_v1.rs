use clap::Parser;
use log::{info, warn};
use mpc_transmission::{get_mask_secret_and_coord_seed_from_config, get_user_address_salt, two_party_share::{
    add_two_shared_secrets, mul_two_shared_secrets, recover_two_shares, recover_value,
    split_to_two_value, sub_two_shared_secrets,
}};
use crate::create_error_response;
use crate::server_utils::{AnonymousAddParams, AnonymousCompareParams, AnonymousCompareValue1AndValue2Params, AnonymousEncodeValueInternalParams, AnonymousMinusParams, AnonymousMultiplyParams, Args, JsonRpcError, JsonRpcRequest, JsonRpcResponse};
use anyhow::anyhow;
use warp::Rejection;

impl warp::reject::Reject for RpcError {}
pub async fn handle_rpc_request_internal_v1(request: JsonRpcRequest) -> Result<impl warp::Reply, Rejection> {

    info!(
        "Received RPC request: method={}, id={:?}",
        request.method, request.id
    );

    let response = match request.method.as_str() {
        //only internal can use this APIs, for fullnode to do mpc calculation, Can not expose to outside
        "bfcx_getAnonymousAdd" => handle_anonymous_add_v1(request).await,
        "bfcx_getAnonymousMinus" => handle_anonymous_minus_v1(request).await,
        "bfcx_getAnonymousMultiply" => handle_anonymous_multiply_v1(request).await,
        "bfcx_getAnonymousCompare" => handle_anonymous_compare_v1(request).await,
        "bfcx_getAnonymousCompareValue1AndValue2" => handle_anonymous_compare_value1_and_value2_v1(request).await,
        "bfcx_getAnonymousEncodeData" => handle_anonymous_encode_data_v1(request).await,
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
async fn handle_anonymous_add_v1(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousAddParams>(params) {
            Ok(add_params) => {
                println!("add_params.value1:{}", add_params.value1);
                println!("add_params.value2:{}", add_params.value2);
                println!("add_params.value3:{}", add_params.value3);
                println!("add_params.value4:{}", add_params.value4);

                let args_result = Args::try_parse();
                let mut config_path : Option<String> = None;
                if args_result.is_ok() {
                    config_path = Some(args_result.unwrap().config);
                }

                let mask_secret = match get_mask_secret_and_coord_seed_from_config(config_path) {
                    Ok(secret) => secret.mask_secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Internal error: Failed to load configuration".to_string(),
                                                     Some(serde_json::json!({"error": e.to_string()})));
                    }
                };

                let value1_share = recover_two_shares(add_params.value1, add_params.value2, mask_secret);
                let value2_share = recover_two_shares(add_params.value3, add_params.value4, mask_secret);
                if value1_share.is_err() || value2_share.is_err() {
                    return create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": "invalid params"})));
                }

                match add_two_shared_secrets(
                    value1_share.unwrap(),
                    value2_share.unwrap(),
                    mask_secret,
                ) {
                    Ok(result) => {
                        let (result1, result2) = split_to_two_value(result,get_user_address_salt(add_params.owner), mask_secret);
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

async fn handle_anonymous_minus_v1(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousMinusParams>(params) {
            Ok(minus_params) => {
                let args_result = Args::try_parse();
                let mut config_path : Option<String> = None;
                if args_result.is_ok() {
                    config_path = Some(args_result.unwrap().config);
                }

                let mask_secret = match get_mask_secret_and_coord_seed_from_config(config_path) {
                    Ok(secret) => secret.mask_secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,-32603 , "Internal error: Failed to load configuration".to_string(), Some(serde_json::json!({"error": e.to_string()})))
                    }
                };

                let value1_share = recover_two_shares(minus_params.value1, minus_params.value2, mask_secret);
                let value2_share = recover_two_shares(minus_params.value3, minus_params.value4, mask_secret);
                if value1_share.is_err() || value2_share.is_err() {
                    return create_error_response(request.id, -32602, "Invalid params".to_string(),
                                                 Some(serde_json::json!({"error": "invalid params"})));
                }

                match sub_two_shared_secrets(
                    value1_share.unwrap(),
                    value2_share.unwrap(),
                    mask_secret,
                ) {
                    Ok(result) => {
                        let (result1, result2) = split_to_two_value(result, get_user_address_salt(minus_params.owner), mask_secret);
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

async fn handle_anonymous_multiply_v1(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousMultiplyParams>(params) {
            Ok(multiply_params) => {
                let args_result = Args::try_parse();
                let mut config_path : Option<String> = None;
                if args_result.is_ok() {
                    config_path = Some(args_result.unwrap().config);
                }
                let mask_secret = match get_mask_secret_and_coord_seed_from_config(config_path) {
                    Ok(secret) => secret.mask_secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Internal error: Failed to load configuration".to_string(),
                                                     Some(serde_json::json!({"error": e.to_string()})));
                    }
                };

                let value1_share =
                    recover_two_shares(multiply_params.value1, multiply_params.value2, mask_secret);
                let value2_share =
                    recover_two_shares(multiply_params.value3, multiply_params.value4, mask_secret);
                if value1_share.is_err() || value2_share.is_err() {
                    return create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": "invalid params"})));

                }

                match mul_two_shared_secrets(
                    value1_share.unwrap(),
                    value2_share.unwrap(),
                    mask_secret,
                ) {
                    Ok(result) => {
                        let (result1, result2) = split_to_two_value(result, get_user_address_salt(multiply_params.owner), mask_secret);
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


async fn handle_anonymous_encode_data_v1(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousEncodeValueInternalParams>(params) {
            Ok(encode_to_two_value_params) => {

                let args_result = Args::try_parse();
                let mut config_path : Option<String> = None;
                if args_result.is_ok() {
                    config_path = Some(args_result.unwrap().config);
                }
                let mask_secret = match get_mask_secret_and_coord_seed_from_config(config_path) {
                    Ok(secret) => secret.mask_secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Internal error: Failed to load configuration".to_string(),
                                                     Some(serde_json::json!({"error": e.to_string()})));
                    }
                };

                let value = encode_to_two_value_params.value;
                let (result1, result2) = split_to_two_value(value, get_user_address_salt(encode_to_two_value_params.owner), mask_secret);
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(serde_json::json!({
                        "result1": result1,
                        "result2": result2,
                        "operation": "anonymous_encode_to_two_value",
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

async fn handle_anonymous_compare_v1(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousCompareParams>(params) {
            Ok(compare_params) => {
                let args_result = Args::try_parse();
                let mut config_path : Option<String> = None;
                if args_result.is_ok() {
                    config_path = Some(args_result.unwrap().config);
                }
                let mask_secret = match get_mask_secret_and_coord_seed_from_config(config_path) {
                    Ok(secret) => secret.mask_secret,
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

async fn handle_anonymous_compare_value1_and_value2_v1(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousCompareValue1AndValue2Params>(params) {
            Ok(compare_params) => {
                let args_result = Args::try_parse();
                let mut config_path : Option<String> = None;
                if args_result.is_ok() {
                    config_path = Some(args_result.unwrap().config);
                }
                let mask_secret = match get_mask_secret_and_coord_seed_from_config(config_path) {
                    Ok(secret) => secret.mask_secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Internal error: Failed to load configuration".to_string(),
                                                     Some(serde_json::json!({"error": e.to_string()})));
                    }
                };

                let new_value1 = recover_value(compare_params.value1, compare_params.value2, mask_secret);
                let new_value2 = recover_value(compare_params.value3, compare_params.value4, mask_secret);
                if new_value1.is_err() || new_value2.is_err() {
                    warn!("Invalid parameters for bfcx_getAnonymousCompareValue1AndValue2");
                    return create_error_response(request.id,
                                                 -32602,
                                                 "Invalid params".to_string(),
                                                 Some(serde_json::json!("error: recover failed")));
                }
                let value_a = new_value1.unwrap();
                let value_b = new_value2.unwrap();
                let comparison = if value_a > value_b {
                    "1"
                } else if value_a < value_b {
                    "2"
                } else {
                    "0"
                };
                return JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(serde_json::json!({
                                "result1": comparison.to_string(),
                                "result2": "0".to_string(),
                                "operation": "anonymous_compare",
                                "timestamp": chrono::Utc::now().timestamp()
                            })),
                    error: None,
                };
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



#[derive(Debug)]
#[allow(dead_code)]
pub struct RpcError(anyhow::Error);