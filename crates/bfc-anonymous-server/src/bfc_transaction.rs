use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransactionResponse {
    jsonrpc: String,
    id: u64,
    result: Option<SuiTransactionResponse>,
    error: Option<TransactionError>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuiTransactionResponse {
    pub digest: String,
    pub transaction: Transaction,
    pub effects: Effect,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    pub created: Vec<EffectInfo>,
    pub mutated: Vec<EffectInfo>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectInfo {
    pub owner: EffectOwner,
    pub reference: EffectReference,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectReference {
    pub object_id: String,
    pub version: u64,
    pub digest: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct EffectOwner {
    pub AddressOwner: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub data: TransactionData,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionData {
    pub sender: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionError {
    code: String,
    message: String,
}


pub fn parse_transaction_response(response: &str) -> Option<SuiTransactionResponse> {
    let rpc_response: RpcTransactionResponse = match serde_json::from_str(response) {
        Ok(parsed) => parsed,
        Err(_) =>  return None,
    };
    match rpc_response.error {
        Some(_) => None,
        _ => rpc_response.result
    }
}