
use crate::parse_response;
use serde_json::json;
use anyhow::anyhow;

pub fn verify_signature(){

}


pub async fn get_object_owneraddress(object_id: String) ->  Result<String, Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();

    let response = client
        .post("https://rpc-mainnet.benfen.org")
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "bfc_getObject",
            "params": [
                object_id,
                    {
                        "showType": true,
                        "showOwner": true,
                        "showContent": true,
                        "showDisplay": true,
                        "showBcs": false,
                        "showStorageRebate": false
                    }
            ]
        }))
        .send()
        .await?;

    let result = response.text().await?;

    let object_id = parse_response(&result.clone());

    match object_id {
        Some(val) => return Ok(val),
        None => return Err(anyhow!("object owner not exit").into())
    }
}
