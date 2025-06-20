use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{info, warn};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RpcResponse {
    jsonrpc: String,
    id: u64,
    result: Option<SuiObjectResponse>,
    error: Option<RpcError>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SuiObjectResponse {
    data: Option<SuiObjectData>,
    error: Option<ObjectError>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SuiObjectData {
    object_id: String,
    version: String,
    digest: String,
    #[serde(rename = "type")]
    object_type: String,
    #[serde(rename = "owner")]
    owner: Owner,
    content: Content,
    display: Option<Value>, // Dynamic display metadata
    storage_rebate: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Owner {
    #[serde(rename = "AddressOwner")]
    pub address_owner: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Shared {
    initial_shared_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Content {
    data_type: String,
    #[serde(rename = "type")]
    content_type: String,
    has_public_transfer: bool,
    fields: Value, // Dynamic content fields
}

#[derive(Debug, Serialize, Deserialize)]
struct ObjectError {
    code: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RpcError {
    code: i32,
    message: String,
    data: Option<Value>,
}

pub fn parse_response(response: &str) -> Option<String> {
    let rpc_response: RpcResponse = serde_json::from_str(response).unwrap();

    match (rpc_response.error, rpc_response.result) {
        // Top-level RPC error (e.g., invalid request)
        (Some(err), _) => {
            info!("RPC Error ({}): {}", err.code, err.message);
            return None;
        }

        // Successful response with object data
        (None, Some(SuiObjectResponse { data: Some(obj), error: None })) => {
            info!("Object ID: {}", obj.object_id);
            info!("Version: {}", obj.version);
            info!("Type: {}", obj.object_type);
            info!("Owner: {:#?}", obj.owner);
            info!("Content: {:#?}", obj.content);
            return obj.owner.address_owner;
        }

        // Object-specific error (e.g., not found)
        (None, Some(SuiObjectResponse { data: None, error: Some(obj_err) })) => {
            info!("Object Error ({}): {}", obj_err.code, obj_err.message);
            return None;
        }

        // Invalid response cases
        _ => {
            info!("Unexpected response format");
            return None;
        }
    }
}

