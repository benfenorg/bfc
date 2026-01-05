use clap::Parser;
use fastcrypto::encoding::{Base64, Encoding};
use log::{info, warn};
use move_core_types::account_address::AccountAddress;
use sui_types::base_types_bfc::bfc_address_util::convert_to_evm_address;
use crate::create_error_response;
use crate::server_utils::{GetAnonymousObjectVersionParams, AnonymousAddParams, AnonymousCompareParams, AnonymousCompareValue1AndValue2Params, AnonymousEncodeValueArrayForZkloginAddressParams, AnonymousEncodeValueArrayParams, AnonymousEncodeValueInternalParams, AnonymousMinusParams, AnonymousMultiplyParams, AnonymousRestoreArrayParams, AnonymousRestoreArrayParamsZKLoginParams, Args, JsonRpcError, JsonRpcRequest, JsonRpcResponse};
use crate::signature::verify_signature;
use crate::utils::{convert_value_array_to_string, create_sign_message, get_object_owner_address, get_object_value1_and_value2, public_key_bytes_to_sui_address, verify_zklogin_signature};
use anyhow::anyhow;
use warp::Rejection;
use mpc_transmission_v2::{is_v1_transmission_shares_format, mul_two_shared_secrets_v2, process_shares_data_convert, recover_value_from_shares_v2};
use mpc_transmission::get_mask_secret_and_coord_seed_from_config;
use mpc_transmission::get_user_address_salt;
use mpc_transmission_v2::two_party_share::{add_two_shared_secrets_v2, sub_two_shared_secrets_v2, split_to_two_value_v2};
use mpc_transmission::get_zklogin_rpc_address_from_config;
//use sui_types::balance;

impl warp::reject::Reject for RpcError {}

pub async fn handle_rpc_request_internal_v2(request: JsonRpcRequest) -> Result<impl warp::Reply, Rejection> {

    info!(
        "Received RPC request: method={}, id={:?}",
        request.method, request.id
    );

    let response = match request.method.as_str() {
        //only internal can use this APIs, for fullnode to do mpc calculation, Can not expose to outside
        "bfcx_getAnonymousAdd" => handle_anonymous_add_v2(request).await,
        "bfcx_getAnonymousMinus" => handle_anonymous_minus_v2(request).await,
        "bfcx_getAnonymousMultiply" => handle_anonymous_multiply_v2(request).await,
        "bfcx_getAnonymousCompare" => handle_anonymous_compare_v2(request).await,
        "bfcx_getAnonymousCompareValue1AndValue2" => handle_anonymous_compare_value1_and_value2_v2(request).await,
        "bfcx_getAnonymousEncodeData" => handle_anonymous_encode_data_v2(request).await,
        "bfcx_getV2Working" => handle_v2_working_status(request).await,
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

async fn handle_anonymous_add_v2(request: JsonRpcRequest) -> JsonRpcResponse {
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

                let mask_secret_and_coord_seed = match get_mask_secret_and_coord_seed_from_config(config_path) {
                    Ok(mask_secret_and_coord_seed) => mask_secret_and_coord_seed,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Internal error: Failed to load configuration".to_string(),
                                                     Some(serde_json::json!({"error": e.to_string()})));
                    }
                };

                let (value1, value2, coord_seed_a) = match process_shares_data_convert(&add_params.value1, &add_params.value2, mask_secret_and_coord_seed.mask_secret, mask_secret_and_coord_seed.coord_seed, get_user_address_salt(add_params.owner)) {
                    Ok(data_convert) => (data_convert.value1, data_convert.value2, data_convert.coord_seed),
                    Err(e) => {
                        warn!("Failed to convert value1 and value2 transmission shares to core shares: {}", e);
                        return create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})));
                    }
                };

                let (value3, value4, coord_seed_b) = match process_shares_data_convert(&add_params.value3, &add_params.value4, mask_secret_and_coord_seed.mask_secret, mask_secret_and_coord_seed.coord_seed, get_user_address_salt(add_params.owner)) {
                    Ok(data_convert) => (data_convert.value1, data_convert.value2, data_convert.coord_seed),
                    Err(e) => {
                        warn!("Failed to convert value3 and value4 transmission shares to core shares: {}", e);
                        return create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})));
                    }
                };

                let result1 = match add_two_shared_secrets_v2(value1, value3, mask_secret_and_coord_seed.mask_secret, 0, get_user_address_salt(add_params.owner), coord_seed_a, coord_seed_b) {
                    Ok(result) => {
                        result
                    }
                    Err(e) => {
                        warn!("Invalid parameters for bfcx_getAnonymousAdd: {}", e);
                        return create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
                    }
                };
                let result2 = match add_two_shared_secrets_v2(value2, value4, mask_secret_and_coord_seed.mask_secret, 1, get_user_address_salt(add_params.owner), coord_seed_a, coord_seed_b) {
                    Ok(result) => {
                        result
                    }
                    Err(e) => {
                        warn!("Invalid parameters for bfcx_getAnonymousAdd: {}", e);
                        return create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
                    }
                };

                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(serde_json::json!({
                        "result1": hex::encode(result1),
                        "result2": hex::encode(result2),
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
        },
        None => {
            create_error_response(request.id, -32602, "Missing params".to_string(), None)
        }
    }
}

async fn handle_anonymous_minus_v2(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousMinusParams>(params) {
            Ok(minus_params) => {
                let args_result = Args::try_parse();
                let mut config_path : Option<String> = None;
                if args_result.is_ok() {
                    config_path = Some(args_result.unwrap().config);
                }

                let mask_secret_and_coord_seed = match get_mask_secret_and_coord_seed_from_config(config_path) {
                    Ok(secret) => secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,-32603 , "Internal error: Failed to load configuration".to_string(), Some(serde_json::json!({"error": e.to_string()})))
                    }
                };

                let (value1, value2, coord_seed_a) = match process_shares_data_convert(&minus_params.value1, &minus_params.value2, mask_secret_and_coord_seed.mask_secret, mask_secret_and_coord_seed.coord_seed, get_user_address_salt(minus_params.owner)) {
                    Ok(data_convert) => (data_convert.value1, data_convert.value2, data_convert.coord_seed),
                    Err(e) => {
                        warn!("Failed to convert value1 and value2 transmission shares to core shares: {}", e);
                        return create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})));
                    }
                };

                let (value3, value4, coord_seed_b) = match process_shares_data_convert(&minus_params.value3, &minus_params.value4, mask_secret_and_coord_seed.mask_secret, mask_secret_and_coord_seed.coord_seed, get_user_address_salt(minus_params.owner)){
                    Ok(data_convert) => (data_convert.value1, data_convert.value2, data_convert.coord_seed),
                    Err(e) => {
                        warn!("Failed to convert value1 and value2 transmission shares to core shares: {}", e);
                        return create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})));
                    }
                };

                let result1 = match sub_two_shared_secrets_v2(value1, value3, mask_secret_and_coord_seed.mask_secret, 0, get_user_address_salt(minus_params.owner),coord_seed_a, coord_seed_b) {
                    Ok(result) => {
                        result
                    }
                    Err(e) => {
                        warn!("Invalid parameters for bfcx_getAnonymousMinus: {}", e);
                        return create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
                    }
                };
                let result2 = match sub_two_shared_secrets_v2(value2, value4, mask_secret_and_coord_seed.mask_secret, 1,  get_user_address_salt(minus_params.owner), coord_seed_a, coord_seed_b) {
                    Ok(result) => {
                        result
                    }
                    Err(e) => {
                        warn!("Invalid parameters for bfcx_getAnonymousMinus: {}", e);
                        return create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
                    }
                };

                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(serde_json::json!({
                        "result1": hex::encode(result1),
                        "result2": hex::encode(result2),
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
        },
        None => {
            create_error_response(request.id, -32602, "Missing params".to_string(), None)
        }
    }
}

async fn handle_anonymous_multiply_v2(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousMultiplyParams>(params) {
            Ok(multiply_params) => {
                let args_result = Args::try_parse();
                let mut config_path : Option<String> = None;
                if args_result.is_ok() {
                    config_path = Some(args_result.unwrap().config);
                }
                let mask_secret_and_coord_seed = match get_mask_secret_and_coord_seed_from_config(config_path) {
                    Ok(secret) => secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Internal error: Failed to load configuration".to_string(),
                                                     Some(serde_json::json!({"error": e.to_string()})));
                    }
                };

                let user_id = get_user_address_salt(multiply_params.owner);

                let (value1, value2, coord_seed_a) = match process_shares_data_convert(&multiply_params.value1, &multiply_params.value2, mask_secret_and_coord_seed.mask_secret, mask_secret_and_coord_seed.coord_seed, get_user_address_salt(multiply_params.owner)){
                    Ok(data_convert) => (data_convert.value1, data_convert.value2, data_convert.coord_seed),
                    Err(e) => {
                        warn!("Failed to convert value1 and value2 transmission shares to core shares: {}", e);
                        return create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})));
                    }
                };

                let (value3, value4, coord_seed_b) = match process_shares_data_convert(&multiply_params.value3, &multiply_params.value4, mask_secret_and_coord_seed.mask_secret, mask_secret_and_coord_seed.coord_seed, get_user_address_salt(multiply_params.owner)){
                    Ok(data_convert) => (data_convert.value1, data_convert.value2, data_convert.coord_seed),
                    Err(e) => {
                        warn!("Failed to convert value1 and value2 transmission shares to core shares: {}", e);
                        return create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})));
                    }
                };

                // Ensure coordinate seeds match for multiplication
                if coord_seed_a != coord_seed_b {
                    warn!("Coordinate seeds don't match (a={}, b={}), using coord_seed_a", coord_seed_a, coord_seed_b);
                }

                // Use the high-level multiplication function
                let (encoded_result_0, encoded_result_1) = match mul_two_shared_secrets_v2(
                    value1,
                    value2,
                    value3,
                    value4,
                    mask_secret_and_coord_seed.mask_secret,
                    user_id,
                    coord_seed_a,
                    coord_seed_b,
                ) {
                    Ok((result1, result2)) => (result1, result2),
                    Err(e) => {
                        warn!("Failed to multiply shared secrets: {}", e);
                        return create_error_response(request.id, -32603, "Internal error".to_string(), Some(serde_json::json!({"error": e.to_string()})));
                    }
                };

                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(serde_json::json!({
                        "result1": hex::encode(encoded_result_0),
                        "result2": hex::encode(encoded_result_1),
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
        },
        None => {
            create_error_response(request.id, -32602, "Missing params".to_string(), None)
        }
    }
}

async fn handle_anonymous_compare_v2(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousCompareParams>(params) {
            Ok(compare_params) => {
                let args_result = Args::try_parse();
                let mut config_path : Option<String> = None;
                if args_result.is_ok() {
                    config_path = Some(args_result.unwrap().config);
                }
                let mask_secret_and_coord_seed = match get_mask_secret_and_coord_seed_from_config(config_path) {
                    Ok(secret) => secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Internal error: Failed to load configuration".to_string(),
                                                     Some(serde_json::json!({"error": e.to_string()})));
                    }
                };

                let value_a = match recover_value_from_shares_v2(compare_params.value1, compare_params.value2, mask_secret_and_coord_seed.mask_secret) {
                    Ok(value) => value,
                    Err(e) => {
                        return create_error_response(request.id,
                                                     -32602,
                                                     "Invalid params".to_string(),
                                                     Some(serde_json::json!({"error": e})));
                    }
                };

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
                create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
            }
        },
        None => {
            create_error_response(request.id, -32602, "Missing params".to_string(), None)
        }
    }
}

pub async fn handle_rpc_request_for_client(request: JsonRpcRequest) -> Result<impl warp::Reply, Rejection> {

    info!(
        "Received RPC request: method={}, id={:?}",
        request.method, request.id
    );

    let response = match request.method.as_str() {
        //outside can use this APIs
        "bfcx_getAnonymousEncodeDataArrayForClient" => handle_anonymous_encode_data_array_for_client(request).await,
        "bfcx_getAnonymousEncodeDataArrayForZKloginAddress" => handle_anonymous_encode_data_array_for_zklogin_address(request).await,
        "bfcx_getAnonymousRestoreValueArray" => handle_anonymous_restore_value_array(request).await,
        "bfcx_getAnonymousRestoreValueArrayForZKloginAddress" => handle_anonymous_restore_value_array_for_zklogin_address(request).await,
        "bfcx_ping" => handle_ping(request).await,
        "bfcx_getAnonymouseObjectVersion" => handle_get_anonymouse_object_version(request).await,
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

async fn handle_anonymous_encode_data_array_for_client(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousEncodeValueArrayParams>(params) {
            Ok(encode_to_two_value_params) => {
                let signature = encode_to_two_value_params.signature;

                let value_array_result: Result<Vec<u64>, _> = encode_to_two_value_params
                    .value_array.iter().map(|s| s.parse()).collect();
                if value_array_result.is_err() {
                    return create_error_response(request.id,
                                                 -32603,
                                                 "Invalid input".to_string(),
                                                 Some(serde_json::json!({"error": "Invalid input, failed to parse to u64"})));
                }
                let value_array_u64 = value_array_result.unwrap();

                let message = create_sign_message(
                    convert_value_array_to_string(&encode_to_two_value_params
                        .value_array));

                let mut pass_verify_signature = verify_signature(
                    &encode_to_two_value_params.publickey,
                    &*signature,
                    message.as_slice(),
                ) .is_ok();

                info!("temporary skip check, important todo need object ownership check to continue restore value!!!!!");
                if pass_verify_signature == true {
                    info!("handle_anonymous_restore_value pass verify signature");
                    let publickey_from_send = public_key_bytes_to_sui_address(
                        encode_to_two_value_params.publickey.clone(),
                    );
                    if publickey_from_send.is_err() {
                        info!("failed public key to sui address: {:?}", publickey_from_send.err());
                        pass_verify_signature = false;
                    } else {
                        let sui_address_from_send = publickey_from_send.unwrap();
                        let sui_account_address_from_send =
                            AccountAddress::from(sui_address_from_send);
                        pass_verify_signature = encode_to_two_value_params.owner
                            == sui_account_address_from_send;
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
                let mask_secret_and_coord_seed = match get_mask_secret_and_coord_seed_from_config(config_path) {
                    Ok(secret) => secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Internal error: Failed to load configuration".to_string(),
                                                     Some(serde_json::json!({"error": e.to_string()})));
                    }
                };

                let mut result_array = Vec::new();
                for value in value_array_u64 {
                    let (result1, result2, _) =
                        split_to_two_value_v2(value,
                                              get_user_address_salt(encode_to_two_value_params.owner),
                                              mask_secret_and_coord_seed.mask_secret,
                                              mask_secret_and_coord_seed.coord_seed);
                    result_array.push(serde_json::json!({
                        "result1": result1,
                        "result2": result2
                    }));
                }
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(serde_json::json!({
                        "result1": result_array,
                        "result2": 0,
                        "operation": "anonymous_encode_to_two_value",
                        "timestamp": chrono::Utc::now().timestamp()
                    })),
                    error: None,
                }
            }
            Err(e) => {
                warn!("Invalid parameters for bfcx_getAnonymousEncodeDataArrayForClient: {}", e);
                create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
            }
        },
        None => {
            create_error_response(request.id, -32602, "Missing params".to_string(), None)
        }
    }
}

async fn handle_anonymous_compare_value1_and_value2_v2(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousCompareValue1AndValue2Params>(params) {
            Ok(compare_params) => {
                let args_result = Args::try_parse();
                let mut config_path : Option<String> = None;
                if args_result.is_ok() {
                    config_path = Some(args_result.unwrap().config);
                }
                let mask_secret_and_coord_seed = match get_mask_secret_and_coord_seed_from_config(config_path) {
                    Ok(secret) => secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Internal error: Failed to load configuration".to_string(),
                                                     Some(serde_json::json!({"error": e.to_string()})));
                    }
                };


                let value_a = match recover_value_from_shares_v2(compare_params.value1, compare_params.value2, mask_secret_and_coord_seed.mask_secret) {
                    Ok(value) => value,
                    Err(e) => {
                        return create_error_response(request.id,
                                                     -32602,
                                                     "Invalid params".to_string(),
                                                     Some(serde_json::json!({"error": e})));
                    }
                };

                let value_b = match recover_value_from_shares_v2(compare_params.value3, compare_params.value4, mask_secret_and_coord_seed.mask_secret) {
                    Ok(value) => value,
                    Err(e) => {
                        return create_error_response(request.id,
                                                     -32602,
                                                     "Invalid params".to_string(),
                                                     Some(serde_json::json!({"error": e})));
                    }
                };

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

async fn handle_anonymous_encode_data_array_for_zklogin_address(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousEncodeValueArrayForZkloginAddressParams>(params) {
            Ok(encode_to_two_value_params) => {
                let signature = encode_to_two_value_params.signature;
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

                let value_array_result: Result<Vec<u64>, _> = encode_to_two_value_params
                    .value_array.iter().map(|s| s.parse()).collect();
                if value_array_result.is_err() {
                    return create_error_response(request.id,
                                                 -32603,
                                                 "Invalid input".to_string(),
                                                 Some(serde_json::json!({"error": "Invalid input, failed to parse to u64"})));
                }
                let value_array_u64 = value_array_result.unwrap();

                let value_array = convert_value_array_to_string(&encode_to_two_value_params.value_array);
                let value = Base64::encode(value_array);
                if  !signature.bytes.eq(&value) {
                    warn!(
                        "authentication failed for bfcx_getAnonymousEncodeDataArrayForZKloginAddress"
                        );
                    return create_error_response(request.id, -32602, "authentication failed".to_string(), None);
                }

                let pass_verify_signature = verify_zklogin_signature(
                    signature,
                    zklogin_address
                ).await.is_ok();


                if pass_verify_signature == false {
                    return create_error_response(request.id,
                                                 -32603,
                                                 "Verify signature or get owner address failed".to_string(),
                                                 Some(serde_json::json!({"error": "verify signature or get owner address failed"})));
                }

                let mask_secret_and_coord_seed = match get_mask_secret_and_coord_seed_from_config(config_path) {
                    Ok(secret) => secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Internal error: Failed to load configuration".to_string(),
                                                     Some(serde_json::json!({"error": e.to_string()})));
                    }
                };


                let mut result_array = Vec::new();
                for value in value_array_u64 {
                    let (result1, result2, _) =
                        split_to_two_value_v2(value,
                                              get_user_address_salt(encode_to_two_value_params.owner),
                                              mask_secret_and_coord_seed.mask_secret,
                                              mask_secret_and_coord_seed.coord_seed);
                    result_array.push(serde_json::json!({
                        "result1": result1,
                        "result2": result2
                    }));
                }
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(serde_json::json!({
                        "result1": result_array,
                        "result2": 0,
                        "operation": "anonymous_encode_to_two_value",
                        "timestamp": chrono::Utc::now().timestamp()
                    })),
                    error: None,
                }
            }
            Err(e) => {
                warn!("Invalid parameters for bfcx_getAnonymousEncodeDataArrayForClient: {}", e);
                create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
            }
        },
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
                    let signature_bytes = signature.bytes.clone();
                    let args_result = Args::try_parse();
                    let mut config_path: Option<String> = None;
                    if args_result.is_ok() {
                        config_path = Some(args_result.unwrap().config);
                    }

                    let zklogin_rpc_address = match get_zklogin_rpc_address_from_config(config_path.clone()) {
                        Ok(address) => address,
                        Err(e) => {
                            warn!("Failed to get zklogin rpc address from config: {}", e);
                            return create_error_response(request.id, -32603, "Internal error: Failed to load configuration".to_string(), Some(serde_json::json!({"error": e.to_string()})))
                        }
                    };

                    let pass_verify_signature = verify_zklogin_signature(
                        signature,
                        zklogin_rpc_address
                    ).await.is_ok();
                    info!("temporary skip check, important todo need object ownership check to continue restore value!!!!!");

                    if pass_verify_signature == false {
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Verify signature or get owner address failed".to_string(),
                                                     Some(serde_json::json!({"error": "verify signature or get owner address failed"})));
                    }

                    let mask_secret_and_coord_seed = match get_mask_secret_and_coord_seed_from_config(config_path) {
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

                    let mut pass_authentication: bool;
                    for anonymous_restore_value in restore_value_params.anonymous_restore_array {
                        let anonymous_restore_value1 = anonymous_restore_value.value1;
                        let anonymous_restore_value2 = anonymous_restore_value.value2;

                        match get_object_owner_address(anonymous_restore_value.objectid.to_string(), anonymous_restore_value1.clone(), anonymous_restore_value2.clone()).await {
                            Ok(owner_address_value) => {
                                let owner_address_from_send = restore_value_params.owner;
                                let evm_addr_from_system =
                                    convert_to_evm_address(owner_address_value);
                                pass_authentication = evm_addr_from_system
                                    == owner_address_from_send.to_hex_with_hex_head();
                            }
                            Err(error) => {
                                info!("failed get owner address: {}", error);
                                pass_authentication = false;
                            }
                        }

                        if pass_authentication == false {
                            return create_error_response(request.id, -32602, "verify owner address failed".to_string(), None);
                        }

                        object_ids = format!("{}{}", object_ids, anonymous_restore_value.objectid);
                        info!("data1 len: {}, data2 len: {}", anonymous_restore_value1.len(), anonymous_restore_value2.len());

                        let data_str1 = String::from_utf8(anonymous_restore_value1).unwrap_or_default();
                        let data_str2 = String::from_utf8(anonymous_restore_value2).unwrap_or_default();
                        info!("=== data_str1: {}, data_str2: {} ===", data_str1, data_str2);

                        match recover_value_from_shares_v2(data_str1, data_str2, mask_secret_and_coord_seed.mask_secret) {
                            Ok(value) => {
                                restore_result_array.push(value);
                            },
                            Err(e) => {
                                restore_result_array.push(0);
                                warn!("process recover_value error, caused by: {}",e);
                            }
                        }
                    }

                    if !signature_bytes.eq(&Base64::encode(object_ids)) {
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
                        "operation": "restore_value_array_for_zklogin",
                        "timestamp": chrono::Utc::now().timestamp()
                    })),
                        error: None,
                    }
                }

                Err(e) => {
                    warn!(
                        "Invalid parameters for bfcx_getAnonymousRestoreArrayParamsZKLoginAddress: {}",
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

            let message = create_sign_message(object_ids.clone());

            let mut pass_authentication = verify_signature(
                &anonymous_restore_value_array.publickey,
                &*anonymous_restore_value_array.signature,
                message.as_slice(),
            ).is_ok();
            if pass_authentication == false {
                warn!(
                        "Invalid authentication for bfcx_getAnonymousRestoreValueArray",
                    );
                return create_error_response(request.id, -32602, "Invalid authentication".to_string(), None)
            }

            info!("temporary skip check, important todo need object ownership check to continue restore value!!!!!");
            let mut restore_result_array = Vec::new();

            for anonymous_restore_value in anonymous_restore_value_array.anonymous_restore_array {
                let objectid = anonymous_restore_value.objectid.clone();
                match get_object_owner_address(objectid.clone(), anonymous_restore_value.value1.clone(), anonymous_restore_value.value2.clone()).await {
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

                let mask_secret_and_coord_seed = match get_mask_secret_and_coord_seed_from_config(config_path) {
                    Ok(config) => config,
                    Err(e) => {
                        info!("get_mask_secret_and_coord_seed_from_config failed, caused by: {}", e);
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

                match recover_value_from_shares_v2(data_str1, data_str2, mask_secret_and_coord_seed.mask_secret) {
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
                                "operation": "anonymous_restore_value_array",
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



async fn handle_anonymous_encode_data_v2(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<AnonymousEncodeValueInternalParams>(params) {
            Ok(encode_to_two_value_params) => {

                let args_result = Args::try_parse();
                let mut config_path : Option<String> = None;
                if args_result.is_ok() {
                    config_path = Some(args_result.unwrap().config);
                }
                let mask_secret_and_coord_seed = match get_mask_secret_and_coord_seed_from_config(config_path) {
                    Ok(secret) => secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Internal error: Failed to load configuration".to_string(),
                                                     Some(serde_json::json!({"error": e.to_string()})));
                    }
                };

                let value = encode_to_two_value_params.value;
                let (result1, result2, _) = split_to_two_value_v2(value, get_user_address_salt(encode_to_two_value_params.owner), mask_secret_and_coord_seed.mask_secret, mask_secret_and_coord_seed.coord_seed);
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



async fn handle_get_anonymouse_object_version(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.params {
        Some(params) => match serde_json::from_value::<GetAnonymousObjectVersionParams>(params) {
            Ok(get_version_params) => {
                let args_result = Args::try_parse();
                let mut config_path : Option<String> = None;
                if args_result.is_ok() {
                    config_path = Some(args_result.unwrap().config);
                }
                let object_id = get_version_params.object_id;
                //let data1 = get_version_params.value1.clone();
                //let data2 = get_version_params.value2.clone();


                let object_balance = get_object_value1_and_value2(object_id.clone()).await;
                match object_balance {
                    Ok(_) => {}
                    Err(_error) =>{
                        return create_error_response(request.id, -32602, "Object not found or has no anonymous data".to_string(), None);
                    }
                }

                let data = object_balance.unwrap().clone();
                let data1 = data.clone().0;
                let data2 = data.clone().1;

                info!("data1 len: {}, data2 len: {}", data1.len(), data2.len());

                let data_str1 = String::from_utf8(data1).unwrap_or_default();
                let data_str2 = String::from_utf8(data2).unwrap_or_default();
                let mask_secret_and_coord_seed = match get_mask_secret_and_coord_seed_from_config(config_path) {
                    Ok(secret) => secret,
                    Err(e) => {
                        warn!("Failed to get mask secret from config: {}", e);
                        return create_error_response(request.id,
                                                     -32603,
                                                     "Internal error: Failed to load configuration".to_string(),
                                                     Some(serde_json::json!({"error": e.to_string()})));
                    }
                };

                let is_version_1 = is_v1_transmission_shares_format(&data_str1, &data_str2, mask_secret_and_coord_seed.mask_secret);
                let version = if is_version_1 {
                    1
                } else {
                    2
                };

                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(serde_json::json!({
                                "objectid": object_id,
                                "version": version,
                                "operation": "get_anonymouse_object_version",
                                "timestamp": chrono::Utc::now().timestamp()
                            })),
                    error: None,
                }
            }
            Err(e) => {
                warn!("Invalid parameters for bfcx_getAnonymouseObjectVersion: {}", e);
                create_error_response(request.id, -32602, "Invalid params".to_string(), Some(serde_json::json!({"error": e.to_string()})))
            }
        },
        None => {
            create_error_response(request.id, -32602, "Missing params".to_string(), None)
        }
    }
}
async fn handle_v2_working_status(request: JsonRpcRequest) -> JsonRpcResponse{
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: request.id,
        result: Some(serde_json::json!({
            "status": "v2 working"
        })),
        error: None,
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct RpcError(anyhow::Error);