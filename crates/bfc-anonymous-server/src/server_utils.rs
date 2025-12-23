use clap::Parser;
use move_core_types::account_address::AccountAddress;
use serde::{Deserialize, Serialize};
use crate::utils::ZkVerifyRequest;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 9010)]
    pub port: u16,

    /// Host to bind to
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    #[clap(short, long, default_value = "")]
    pub config: String,
}


pub fn create_error_response(requestid : serde_json::Value, code: i32, message: String, data: Option<serde_json::Value>) -> JsonRpcResponse {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    pub result: Option<serde_json::Value>,
    pub(crate) error: Option<JsonRpcError>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct AnonymousAddParams {
    pub value1: String,
    pub value2: String,
    pub value3: String,
    pub value4: String,
    pub owner: AccountAddress,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousMinusParams {
    pub value1: String,
    pub value2: String,
    pub value3: String,
    pub value4: String,
    pub owner: AccountAddress,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousMultiplyParams {
    pub value1: String,
    pub value2: String,
    pub value3: String,
    pub value4: String,
    pub owner: AccountAddress,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousCompareParams {
    pub value1: String,
    pub value2: String,
    pub value3: u64,
    pub owner: AccountAddress
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousCompareValue1AndValue2Params {
    pub value1: String,
    pub value2: String,
    pub value3: String,
    pub value4: String,
    pub owner: AccountAddress
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousEncodeValueInternalParams {
    pub value: u64,
    pub owner: AccountAddress,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousEncodeValueParams {
    pub value: u64,
    pub owner: AccountAddress,
    pub publickey: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousEncodeValueArrayParams {
    pub value_array: Vec<String>,
    pub owner: AccountAddress,
    pub publickey: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousEncodeValueArrayForZkloginAddressParams {
    pub value_array: Vec<String>,
    pub owner: AccountAddress,
    pub signature: ZkVerifyRequest,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousRestoreValueInternalParams {
    pub value1: Vec<u8>,
    pub value2: Vec<u8>,
    pub owner: AccountAddress
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousRestoreValueParams {
    pub value1: Vec<u8>,
    pub value2: Vec<u8>,
    pub signature: Vec<u8>,
    pub objectid: String,
    pub publickey: Vec<u8>,
    pub owner: AccountAddress
}
#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousRestoreElementParams {
    pub value1: Vec<u8>,
    pub value2: Vec<u8>,
    pub objectid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousRestoreArrayParams {
    pub anonymous_restore_array: Vec<AnonymousRestoreElementParams>,
    pub signature: Vec<u8>,
    pub publickey: Vec<u8>,
    pub owner: AccountAddress
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousRestoreZKLoginParams {
    pub value1: Vec<u8>,
    pub value2: Vec<u8>,
    pub signature: ZkVerifyRequest,
    pub objectid: String,
    pub owner: AccountAddress
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousRestoreArrayParamsZKLoginParams {
    pub anonymous_restore_array: Vec<AnonymousRestoreElementParams>,
    pub signature: ZkVerifyRequest,
    pub object_ids: String,
    pub owner: AccountAddress
}



