use clap::Parser;
use fastcrypto::encoding::{Base64, Encoding};
use log::{info, warn};
use move_core_types::account_address::AccountAddress;
use mpc_transmission::{get_mask_secret_and_coord_seed_from_config, get_sui_config_directory, get_user_address_salt, get_zklogin_rpc_address_from_config, two_party_share::{
    add_two_shared_secrets, mul_two_shared_secrets, recover_two_shares, recover_value,
    split_to_two_value, sub_two_shared_secrets,
}};
use sui_types::base_types_bfc::bfc_address_util::convert_to_evm_address;
use crate::create_error_response;
use crate::server_utils::{AnonymousAddParams, AnonymousCompareParams, AnonymousCompareValue1AndValue2Params, AnonymousEncodeValueArrayForZkloginAddressParams, AnonymousEncodeValueArrayParams, AnonymousEncodeValueInternalParams, AnonymousMinusParams, AnonymousMultiplyParams, AnonymousRestoreArrayParams, AnonymousRestoreArrayParamsZKLoginParams, Args, JsonRpcError, JsonRpcRequest, JsonRpcResponse};
use crate::signature::verify_signature;
use crate::utils::{convert_value_array_to_string, create_sign_message, get_object_owneraddress, public_key_bytes_to_sui_address, verify_zklogin_signature};

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

async fn handle_anonymous_encode_data_array_for_zklogin_address_v1(request: JsonRpcRequest) -> JsonRpcResponse {
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


                let mut result_array = Vec::new();
                for value in value_array_u64 {
                    let (result1, result2) =
                        split_to_two_value(value,
                                           get_user_address_salt(encode_to_two_value_params.owner),
                                           mask_secret);
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
async fn handle_anonymous_restore_value_array_for_zklogin_address_v1(request: JsonRpcRequest) -> JsonRpcResponse {
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
async fn handle_anonymous_restore_value_array_v1(request: JsonRpcRequest) -> JsonRpcResponse {
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

                let mask_secret = match get_mask_secret_and_coord_seed_from_config(config_path) {
                    Ok(secret) => secret.mask_secret,
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

async fn handle_anonymous_encode_data_array_for_client_v1(request: JsonRpcRequest) -> JsonRpcResponse {
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

                let mut result_array = Vec::new();
                for value in value_array_u64 {
                    let (result1, result2) =
                        split_to_two_value(value,
                                           get_user_address_salt(encode_to_two_value_params.owner),
                                           mask_secret);
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