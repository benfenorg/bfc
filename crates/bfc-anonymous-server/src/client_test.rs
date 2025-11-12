use reqwest;
use serde_json::{json, Value};
use std::error::Error;
use std::str::FromStr;
use std::string::String;
use log::info;
use move_core_types::account_address::AccountAddress;
use sui_types::base_types::SuiAddress;
use crate::utils::ZkVerifyRequest;

#[derive(Debug)]
#[allow(unused)]
pub(crate) struct TestResult {
    method: String,
    success: bool,
    response: Option<Value>,
    error: Option<String>,
}

pub(crate) struct AnonymousClient {
    pub(crate) base_url: String,
    client: reqwest::Client,
}
#[allow(unused)]
impl AnonymousClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: reqwest::Client::new(),
        }
    }

    async fn send_rpc_request(
        &self,
        method: &str,
        params: Value,
        id: u64,
        path: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let request_body = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": id
        });

        info!("the request body is {:?}", request_body);

        let response = self
            .client
            .post(&format!("{}/{}", self.base_url, path))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;
        
        if !status.is_success() {
            return Err(format!("HTTP error {}: {}", status, response_text).into());
        }
        
        let response_json: Value = match serde_json::from_str(&response_text) {
            Ok(json) => json,
            Err(e) => {
                return Err(format!("Failed to parse JSON response: {}. Response text: {}", e, response_text).into());
            }
        };

        Ok(response_json)
    }

    pub async fn test_add(
        &self,
        value1: String,
        value2: String,
        value3: String,
        value4: String,
    ) -> TestResult {
        let owner = AccountAddress::from_hex_literal("0x1").unwrap();
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "value4": value4,
            "owner": owner,
            "user_id": 1u64,
        });

        match self
            .send_rpc_request("bfcx_getAnonymousAdd", params, 1, "rpc_internal")
            .await
        {
            Ok(response) => TestResult {
                method: "bfcx_getAnonymousAdd".to_string(),
                success: true,
                response: Some(response),
                error: None,
            },
            Err(e) => TestResult {
                method: "bfcx_getAnonymousAdd".to_string(),
                success: false,
                response: None,
                error: Some(e.to_string()),
            },
        }
    }

    pub async fn test_minus(
        &self,
        value1: String,
        value2: String,
        value3: String,
        value4: String,
    ) -> TestResult {
        let owner = AccountAddress::from_hex_literal("0x1").unwrap();
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "value4": value4,
            "owner": owner,
            "user_id": 1u64,
        });

        match self
            .send_rpc_request("bfcx_getAnonymousMinus", params, 2, "rpc_internal")
            .await
        {
            Ok(response) => TestResult {
                method: "bfcx_getAnonymousMinus".to_string(),
                success: true,
                response: Some(response),
                error: None,
            },
            Err(e) => TestResult {
                method: "bfcx_getAnonymousMinus".to_string(),
                success: false,
                response: None,
                error: Some(e.to_string()),
            },
        }
    }

    pub async fn test_multiply(
        &self,
        value1: String,
        value2: String,
        value3: String,
        value4: String,
    ) -> TestResult {
        let owner = AccountAddress::from_hex_literal("0x1").unwrap();
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "value4": value4,
            "owner": owner,
            "user_id": 1u64,
        });

        match self
            .send_rpc_request("bfcx_getAnonymousMultiply", params, 3, "rpc_internal")
            .await
        {
            Ok(response) => TestResult {
                method: "bfcx_getAnonymousMultiply".to_string(),
                success: true,
                response: Some(response),
                error: None,
            },
            Err(e) => TestResult {
                method: "bfcx_getAnonymousMultiply".to_string(),
                success: false,
                response: None,
                error: Some(e.to_string()),
            },
        }
    }

    pub async fn test_compare(&self, value1: String, value2: String, value3: u64) -> TestResult {
        let owner = AccountAddress::from_hex_literal("0x1").unwrap();
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "owner": owner,
        });

        match self
            .send_rpc_request("bfcx_getAnonymousCompare", params, 4, "rpc_internal")
            .await
        {
            Ok(response) => TestResult {
                method: "bfcx_getAnonymousCompare".to_string(),
                success: true,
                response: Some(response),
                error: None,
            },
            Err(e) => TestResult {
                method: "bfcx_getAnonymousCompare".to_string(),
                success: false,
                response: None,
                error: Some(e.to_string()),
            },
        }
    }


    pub async fn test_compare_value1_value2(&self, value1: String, value2: String, value3: String, value4: String) -> TestResult {
        let owner = AccountAddress::from_hex_literal("0x1").unwrap();
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "value4": value4,
            "owner": owner,
        });

        match self
            .send_rpc_request("bfcx_getAnonymousCompareValue1AndValue2", params, 4, "rpc_internal")
            .await
        {
            Ok(response) => TestResult {
                method: "bfcx_getAnonymousCompareValue1AndValue2".to_string(),
                success: true,
                response: Some(response),
                error: None,
            },
            Err(e) => TestResult {
                method: "bfcx_getAnonymousCompareValue1AndValue2".to_string(),
                success: false,
                response: None,
                error: Some(e.to_string()),
            },
        }
    }


    pub async fn test_restore_value_array(
        &self,
        value1: Vec<u8>,
        value2: Vec<u8>,
        signature: Vec<u8>,
        objectid: String,
        publickey: Vec<u8>,
    ) -> TestResult {
        let owner = AccountAddress::from_hex_literal("0x1").unwrap();
        let params = Value::Object(
            serde_json::Map::from_iter([
                ("owner".to_string(), json!(owner)),
                ("publickey".to_string(), json!(publickey)),
                ("signature".to_string(), json!(signature)),
                ("anonymous_restore_array".to_string(), Value::Array(vec![
                    Value::Object(serde_json::Map::from_iter([
                        ("value1".to_string(), json!(value1)),
                        ("value2".to_string(), json!(value2)),
                        ("objectid".to_string(), serde_json::Value::String(objectid.clone())),
                    ])),
                    Value::Object(serde_json::Map::from_iter([
                        ("value1".to_string(), json!(value1)),
                        ("value2".to_string(), json!(value2)),
                        ("objectid".to_string(), serde_json::Value::String(objectid.clone())),
                    ])),
                ])),
            ])
        );

        match self
            .send_rpc_request("bfcx_getAnonymousRestoreValueArray", params, 4, "rpc")
            .await
        {
            Ok(response) => TestResult {
                method: "bfcx_getAnonymousRestoreValueArray".to_string(),
                success: true,
                response: Some(response),
                error: None,
            },
            Err(e) => TestResult {
                method: "bfcx_getAnonymousRestoreValueArray".to_string(),
                success: false,
                response: None,
                error: Some(e.to_string()),
            },
        }
    }

    pub async fn test_restore_value_array_for_zklogin(
        &self,
        value1: Vec<u8>,
        value2: Vec<u8>,
        signature: ZkVerifyRequest,
        objectid1: String,
        objectid2: String,
        object_id_list: String,
    ) -> TestResult {

        let owner = AccountAddress::from_hex_literal("0x1").unwrap();
        let params = Value::Object(
            serde_json::Map::from_iter([
                ("owner".to_string(), json!(owner)),
                ("object_ids".to_string(), Value::String(object_id_list.clone())),
                ("signature".to_string(), json!(signature)),
                ("anonymous_restore_array".to_string(), Value::Array(vec![
                    Value::Object(serde_json::Map::from_iter([
                        ("value1".to_string(), json!(value1)),
                        ("value2".to_string(), json!(value2)),
                        ("objectid".to_string(), serde_json::Value::String(objectid1.clone())),
                    ])),
                    Value::Object(serde_json::Map::from_iter([
                        ("value1".to_string(), json!(value1)),
                        ("value2".to_string(), json!(value2)),
                        ("objectid".to_string(), serde_json::Value::String(objectid2.clone())),
                    ])),
                ])),
            ])
        );

        match self
            .send_rpc_request("bfcx_getAnonymousRestoreValueArrayForZKloginAddress", params, 4, "rpc")
            .await
        {
            Ok(response) => TestResult {
                method: "bfcx_getAnonymousRestoreValueArrayForZKloginAddress".to_string(),
                success: true,
                response: Some(response),
                error: None,
            },
            Err(e) => TestResult {
                method: "bfcx_getAnonymousRestoreValueArrayForZKloginAddress".to_string(),
                success: false,
                response: None,
                error: Some(e.to_string()),
            },
        }
    }

    pub async fn test_restore_value_for_zklogin(
        &self,
        value1: Vec<u8>,
        value2: Vec<u8>,
        signature: ZkVerifyRequest,
        objectid: String,
    ) -> TestResult {
        let owner = AccountAddress::from_hex_literal("0x1").unwrap();
        let params = json!({
            "value1": value1,
            "value2": value2,
            "signature": signature,
            "objectid": objectid,
            "owner": owner,
        });

        match self
            .send_rpc_request("bfcx_getAnonymousRestoreValueForZKloginAddress", params, 4, "rpc")
            .await
        {
            Ok(response) => TestResult {
                method: "bfcx_getAnonymousRestoreValueForZKloginAddress".to_string(),
                success: true,
                response: Some(response),
                error: None,
            },
            Err(e) => TestResult {
                method: "bfcx_getAnonymousRestoreValueForZKloginAddress".to_string(),
                success: false,
                response: None,
                error: Some(e.to_string()),
            },
        }
    }

    pub async fn test_encode_data_array(&self,
                                        value: Vec<String>,
                                        owner: AccountAddress,
                                        signature: Vec<u8>,
                                        objectid: String,
                                        publickey: Vec<u8>) -> TestResult {
        let params = Value::Object(
            serde_json::Map::from_iter([
                ("owner".to_string(), json!(owner)),
                ("publickey".to_string(), json!(publickey)),
                ("signature".to_string(), json!(signature)),
                ("value_array".to_string(), json!(value)),

            ])
        );

        match self
            .send_rpc_request("bfcx_getAnonymousEncodeDataArrayForClient", params, 4, "rpc")
            .await
        {
            Ok(response) => TestResult {
                method: "bfcx_getAnonymousEncodeDataArrayForClient".to_string(),
                success: true,
                response: Some(response),
                error: None,
            },
            Err(e) => TestResult {
                method: "bfcx_getAnonymousEncodeDataArrayForClient".to_string(),
                success: false,
                response: None,
                error: Some(e.to_string()),
            },
        }
    }

    pub async fn test_encode_data_array_for_zklogin(&self,
                                        value: Vec<String>,
                                        owner: AccountAddress,
                                        signature: ZkVerifyRequest) -> TestResult {

        let json_array: Value = Value::Array(
            value
                .into_iter()
                .map(Value::String)
                .collect()
        );


        let params = Value::Object(
            serde_json::Map::from_iter([
                ("owner".to_string(), json!(owner)),
                ("signature".to_string(), json!(signature)),
                ("value_array".to_string(), json!(json_array)),
            ])
        );

        match self
            .send_rpc_request("bfcx_getAnonymousEncodeDataArrayForZKloginAddress", params, 4, "rpc")
            .await
        {
            Ok(response) => TestResult {
                method: "bfcx_getAnonymousEncodeDataArrayForZKloginAddress".to_string(),
                success: true,
                response: Some(response),
                error: None,
            },
            Err(e) => TestResult {
                method: "bfcx_getAnonymousEncodeDataArrayForZKloginAddress".to_string(),
                success: false,
                response: None,
                error: Some(e.to_string()),
            },
        }
    }

    pub async fn test_split(&self, value: u64, user_id: u64) -> TestResult {
        let user_address = AccountAddress::from_hex_literal("0x1").unwrap();
        let params = json!({
            "value": value,
            "owner": user_address,
            "user_id": user_id,
            "publickey": Vec::<u8>::new(),
            "signature": Vec::<u8>::new(),
        });

        match self
            .send_rpc_request("bfcx_getAnonymousEncodeData", params, 4, "rpc_internal")
            .await
        {
            Ok(response) => TestResult {
                method: "bfcx_getAnonymousEncodeData".to_string(),
                success: true,
                response: Some(response),
                error: None,
            },
            Err(e) => TestResult {
                method: "bfcx_getAnonymousEncodeData".to_string(),
                success: false,
                response: None,
                error: Some(e.to_string()),
            },
        }
    }

    pub async fn test_ping(&self) -> TestResult {
        match self.send_rpc_request("bfcx_ping", json!({}), 5, "rpc").await {
            Ok(response) => TestResult {
                method: "bfcx_ping".to_string(),
                success: true,
                response: Some(response),
                error: None,
            },
            Err(e) => TestResult {
                method: "bfcx_ping".to_string(),
                success: false,
                response: None,
                error: Some(e.to_string()),
            },
        }
    }

    async fn test_encode_data_array_with_signature_for_zklogin(&self, value : Vec<String>) -> TestResult {
        let signature = "BQNNMTUyNzczMDQzNTY4ODY3ODExMDI3MTIzMTM5NDU3ODYwMzAzNjI1MjEyODU4Mzg5OTQxNzE4Nzc3MjA3MzY3NjgwNzQ2NTI5MzA0MjFNMjEyNzU0MDk1NDEzMTA2OTYxNDQyNDA5NDQ0NzU3ODM3NTUxNjA2NTM3NTEwMTg1Nzc1MzI1MjM4NDA4Njk0Mjg0MTM4ODcxOTU5MTEBMQMCTTEyNDI1NDAxOTAyMjI3NzYwODczMjY4NTgzNTczNjQ3ODY0NzQzNDM2NzQ4MTY3MDA3NDk1NDA3NTU5MjI3Nzc2NjE1MzM1MzI1MjMzTDUyMjM4NTQyMDY0NjU2OTgwNTI5MjgxMjM5OTg4MjQ0ODcxMDgwNTE3MTUyNzA0MDM4NjEzNTY2MDEwNjUxMDk3MTQ5NzM3NTQ3MDECTDcyMDgxNTMzNTk4MzQ5NzYyMjg3MzI2MzA0ODM5NjM2NzEyNjM2OTIxNzI0NzQ0NTg0NDMyMDEzMDAxMDczOTg1OTk0MjkxNDg5ODlNMTMwNTI3MDQ4MDQ3NDgxNDU1ODkwMDI1OTU5NTcwODU1MzMwMzE1NTc0MTcxMTUzMDgwODMzNzExMTA2OTcyNzIwMDY2MjkxOTQxNTkCATEBMANNMTk2NDA3MjU4NjI4ODY0OTkzMTE1NDIyMzQ3MDI0MTU1MDc2MDc1MjQ0NTMzNDE4NTYxODkwODA1MzQ1MzQ5NjM4Mjk3OTU0MjkxNDNMNDU3OTI3MTAyMTg5NzQwMjcyMDE0MTgxMjI4MTA0OTE3Mjk2MjAyNTQyNzkyNjU3MTcwNjA2ODcyMTk4Mzc5NzE3MzU4NTQxMDIyNAExMXlKcGMzTWlPaUpvZEhSd2N6b3ZMMkZqWTI5MWJuUnpMbWR2YjJkc1pTNWpiMjBpTEMBZmV5SmhiR2NpT2lKU1V6STFOaUlzSW10cFpDSTZJalJtWldJME5HWXdaamRoTjJVeU4yTTNZelF3TXpNM09XRm1aakl3WVdZMVl6aGpaalV5WkdNaUxDSjBlWEFpT2lKS1YxUWlmUU0xNjI3MTI3MjgzMDEwNzY1NDQ3MzQ2MTc0OTc3ODYzNDYyMzA4NDUwMzczNDAzNzgyMzU3NTE2MDAwOTM4NTI1NjU0Mjc2Mzc2OTc4N4kBAAAAAAAAYQD6rQDMXyHOgTZriGFhoo2kZTJm3wMdOhhJ6grSEJEEUnkKOWJjJ/32jOQZn30zLYsDk9t7qQRkIaXFpLNeVrUK1AOWWqwvXa6U4LDD6ZhU979QOp2XBQDyAA/aUkas+oQ=".to_string();
        let byte = "MTAwMDAwMDAwMA==".to_string();
        let signature_bytes= ZkVerifyRequest {
            signature: signature.to_string(),
            bytes: byte, // abs token objectid
            intent_scope: 3,
            cur_epoch: None,
            cur_rpc_url : Some("https://testrpc.benfen.org/".to_string()),
            author: "0x8c92533545c7f97e7491ea0049c9efdd25f43bf18fe56e92b4ce4b40d05165be".to_string(),
        };

        let owner = AccountAddress::from_hex_literal("0x8c92533545c7f97e7491ea0049c9efdd25f43bf18fe56e92b4ce4b40d05165be").unwrap();

        let restore_result = self
            .test_encode_data_array_for_zklogin(value, owner, signature_bytes)
            .await;
        restore_result
    }

    async fn test_recover_array_with_signature_for_zklogin(&self, share1: String, share2: String) -> Result<Vec<u64>, serde_json::error::Error> {
        // test restore 20
        let publickey = "8496d3d932986b43bb64b5d5c7548d5c97a73aebf4301447f3746680b2114ae1";
        let object_id1 = "BFC47c715b758d549e531baf6ef516b1fa716f766e312a209123bdc9acd7cb5810374dc";
        let object_id2 = "BFCa861988889cf31134ee643746cc2b768ef7409a4c579f3b7ff4ce4ce6ccfcc7b9e63";

        let object_id_list = "BFCa4e7d3832d6ccf20f72f831bc164c5afc6ad2d03081101b098297b4c4cf6d3743ac8BFCa6eecddaabb11bef34c99e982e6f74d9c3820d3bf87220ec3f208a360b77223f5fd0";
        let publickey_bytes = hex_to_bytes(publickey);
        let signature = "BQNNMTA4ODc1MDgzMTU3Njk5NDk2NDUzMDAzODM4MjkyMDc2OTYxNDU0OTkzMjQ2MjAxMzgzODg3MzIxMTk5MDg1MzA3NTY2MTQ2OTkxNDFNMjA2NDU4MjM4NDA1NDEzMDQ5Nzc1ODk0MTc0ODM4NjYwMDAwOTczMjIwNzMwODIxNTUzODczMDY4OTQ4ODg4NDgyMDMzMDYyMTY4NDQBMQMCTTE1MjM0NTAxMjExODc2MDExNDMzMDcxNDQyNjM3MjI5MjYzMzk1NzQ2OTI3MzkxMjU1NDcyMDk4NDc2MzI0MTg3NzQwMDg0OTI2NDkyTTIwMzY2MjA1MTMwNTU2NzA1MzI1MDAwNzQ4NzUwNzQyNzcyODEzNDAwMzc2MDUxMTQwMzE5NjAzNTU2NTQ2MDg2MDMyNDUzNTc1MTUxAkwzOTAzODA3NDEyNjQ3MjY2NjIxNzU2NTc1NDc0NzQzMjc2OTkxMjY2NTEyODI1NTYyNTExNjE0OTE1MzQyMDMwMDg0NTczMjYzMTA0TDMzNTU1NjI0NDMxMzAzODA5MTcyMzkyNzkyMzAzMjQ5OTcwMzA3NDQ0NTAyODAzODA1NTMzNzYwMzYxNjc0OTYwMzczODgwMTU2OTkCATEBMANNMTEwOTc3NjU2NzE3MTk0ODE3NjIzODg0MTgyNTIwMTY0MDM2NjE1NjY5MTk3NDkxOTQyMTA3NDA0NDEyNTQzMDE3MzE2NDM1MDA5NjJNMTc5MDQ5NDQ2ODQ2MjkwNjAzNzk5OTkxOTk0NzYwMDk5MzY5ODE1Njg1MzAzNzkwOTYyMzg3ODczODg0NTUzMTE0NzQ5MjY0NjI4NDABMTF5SnBjM01pT2lKb2RIUndjem92TDJGalkyOTFiblJ6TG1kdmIyZHNaUzVqYjIwaUxDAWZleUpoYkdjaU9pSlNVekkxTmlJc0ltdHBaQ0k2SWpSbVpXSTBOR1l3WmpkaE4yVXlOMk0zWXpRd016TTNPV0ZtWmpJd1lXWTFZemhqWmpVeVpHTWlMQ0owZVhBaU9pSktWMVFpZlFNMTYyNzEyNzI4MzAxMDc2NTQ0NzM0NjE3NDk3Nzg2MzQ2MjMwODQ1MDM3MzQwMzc4MjM1NzUxNjAwMDkzODUyNTY1NDI3NjM3Njk3ODeKAQAAAAAAAGEArPANz4Ki4LFjpA5qYIfSw4nRwumL3A+4a/5xBtrZsVlu70sQMBD+cQzkd6gEjTJRyOHNK0F7xxtg+PYdeZ7zCaN106ftoGFcqhnmRz0MY+XjNnWO05bdY5vs01T4bgoz".to_string();
        let byte = "QkZDNDdjNzE1Yjc1OGQ1NDllNTMxYmFmNmVmNTE2YjFmYTcxNmY3NjZlMzEyYTIwOTEyM2JkYzlhY2Q3Y2I1ODEwMzc0ZGNCRkNhODYxOTg4ODg5Y2YzMTEzNGVlNjQzNzQ2Y2MyYjc2OGVmNzQwOWE0YzU3OWYzYjdmZjRjZTRjZTZjY2ZjYzdiOWU2Mw==".to_string();
        let signature_bytes= ZkVerifyRequest {
            signature: signature.to_string(),
            bytes: byte, // abs token objectid
            intent_scope: 3,
            cur_epoch: None,
            cur_rpc_url : Some("https://testrpc.benfen.org/".to_string()),
            author: "0x8c92533545c7f97e7491ea0049c9efdd25f43bf18fe56e92b4ce4b40d05165be".to_string(),
        };

        let restore_result = self
            .test_restore_value_array_for_zklogin(share1.into_bytes(), share2.into_bytes(), signature_bytes, object_id1.to_string(), object_id2.to_string(), object_id_list.to_string())
            .await
            .response
            .unwrap();

        serde_json::from_value(restore_result["result"]["result1"].clone())
    }

    async fn test_recover_with_signature_for_zklogin(&self, share1: String, share2: String) -> u64 {
        // test restore 20
        let publickey = "8496d3d932986b43bb64b5d5c7548d5c97a73aebf4301447f3746680b2114ae1";

        let object_id = SuiAddress::from_str(
            "0x9637bea020496db9fa40583c2ce3a52fcefdf71bd7a0edcf89861677da15af6b",
        )
            .unwrap()
            .to_string();

        let publickey_bytes = hex_to_bytes(publickey);
        let signature = "BQNNMTg0MTk0NTMzNDY1NTIzMjA3MTI1MTU4OTI4NzE2NDU0MTE0MTQxMTA4MTYxNjE2MzcwMTcyMzE4NzYzMzI5ODY1NTE4NDM3MjgxMThNMTc0Mjg1NDc2MTkwMTQyODAxMTMzMTcxODA2Njc2NjU5ODk4NDYxNTkxOTYyODQ3MDYxMTY4NTEwNzM0OTkwNzkwMDIzMTkzODkwNzMBMQMCTTE3ODUzNjY4OTA0NjA4NzUwMDEzMDIzOTAzODU5MTg5MjUwNzA5MjMzNjg0NjczNjg3MzA4NjcyMDA5MTAyNTkxNDM4NTMzNzg1OTk4TTE0MTA2MzI0Njk1OTI0NTgyMDU0NzY1NDcyODExMTE3NzgzNjk4ODM3MjQxMDkyNTg0NjUwMDI1NTI0NDEwOTk1NzU0Njc5MTAxNTUxAkw2NjI1Mjc3OTA2NzcwOTQxMDgwMjMwODEyMTExMDUxOTExMjU2MzUzMzYzOTE2NTk4ODYzOTI5MjMwMjIxNTgxMDI0NzMyNjA2NzczTTE1MDgyMzcxOTQzNDIwNTAxNzI3MzY2NTQ1NzgxMDQwNzk0MTk3NjQ5NDU1MjIxNDYwNzczNTc3ODU3NTg5NzkxMDcwMzcwMjEzNzE0AgExATADSzIzNzQ4NTUzNzMxNjgxNTk3NDI2MDMwOTM1MDc3NTU1NzY2Nzc3OTQ3OTc3OTgyMjgyOTMyNzIwNTIxMzQwNzY3NzgyNTA3MTA0N00xNTg3MDcwMDQzMDk2NzAzMDA0NjI1MjQ0NTYwOTYzMTI3ODQ1NDk3ODg1NDE1OTAxNDgxMjExNDM0NzM4MTExNzc4NzI3MTQ3MTY5NwExMXlKcGMzTWlPaUpvZEhSd2N6b3ZMMkZqWTI5MWJuUnpMbWR2YjJkc1pTNWpiMjBpTEMBZmV5SmhiR2NpT2lKU1V6STFOaUlzSW10cFpDSTZJamc0TkRnNU1qRXlNbVV5T1RNNVptUXhaak14TXpjMVlqSmlNell6WldNNE1UVTNNak5pWW1JaUxDSjBlWEFpT2lKS1YxUWlmUUwzMDA0NDUyNDMzOTUzNDc3NDY4Mjk3MTI4MDIyMjYwMjEyODIyNjk0MjM5MzMwNTU4OTE5ODc3OTM5ODA2MjAyOTgzMDk5ODkyODU1rwEAAAAAAABhANpFQhyM8aC5Kpwc36oVLQMufXEtbzHyHZHExy73Fki5JmSORe63A2L4ZAMaHHZmE+/6TCoqnWEbJI+VwAEIqQwg/J1PkdfE2/sruj/vZ5unh5gbotMAgFOdpb3P1+CniA==".to_string();
        let byte = "QkZDOGMyOTM4ZjFmMzJjODBhY2M1NDhiM2FhNjI2ZDcxNzE2M2Y4MDU4OTdiNTg2ZTBkODE5YTg4NzU0ZTNmYWMyYzAwZWM6ODdMWVRja1BqQTE5WTdqNHpDaHVSamFuY3hVdEFSdXdsUEtQVHMwLVhPdlg5RW1RSUVGdkNaa2V6OWIxenJUYw==".to_string();
        let signature_bytes= ZkVerifyRequest {
            signature: signature,
            bytes: byte, // abfc token objectid
            intent_scope: 3,
            cur_epoch: None,
            cur_rpc_url : Some("https://testrpc.benfen.org/".to_string()),
            author: "BFC8c2938f1f32c80acc548b3aa626d717163f805897b586e0d819a88754e3fac2c00ec".to_string(),
        };

        let restore_result = self
            .test_restore_value_for_zklogin(share1.into_bytes(), share2.into_bytes(), signature_bytes, object_id)
            .await
            .response
            .unwrap();

        restore_result["result"]["result1"].as_u64().unwrap()
    }



    async fn test_recover_array_with_signature(&self, share1: String, share2: String) -> Result<Vec<u64>, serde_json::error::Error> {
        // test restore 20
        let signature = "be916feb774aa5ca80e72db8d9952003608adec00604108c18fbcb205095ab2e9ede1b032c6c6ac1bdc3110d425e64f977e313e4b3b57d81f6066bfe227e6f06";
        let publickey = "8496d3d932986b43bb64b5d5c7548d5c97a73aebf4301447f3746680b2114ae1";

        let object_id = SuiAddress::from_str(
            "0xd4c2360f11b1608f3be0b8d89bc97ff3047378dbc34d0b3976f40e6392496fd5",
        )
            .unwrap()
            .to_string();

        let signature_bytes = hex_to_bytes(signature);
        let publickey_bytes = hex_to_bytes(publickey);

        let restore_result = self
            .test_restore_value_array(share1.into_bytes(), share2.into_bytes(), signature_bytes, object_id, publickey_bytes)
            .await
            .response
            .unwrap();
        serde_json::from_value(restore_result["result"]["result1"].clone())
    }

}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{AnonymousServer};
    use crate::utils::write_unsigned_leb128;
    use std::net::SocketAddr;
    use move_core_types::account_address::AccountAddress;
    use serde_json::Value::String;
    use tracing::info;
    use tracing_subscriber::fmt;
    use crate::client_test::hex_to_bytes;

    #[tokio::test]
    async fn test_blake2b_hash() {
        use fastcrypto::hash::HashFunction;
        let data1 = b"300"; // Personal message prefix 300
        let data2 = b"hello world";
        let databytes = [data1.as_slice(), data2.as_slice()].concat();
        let hash = fastcrypto::hash::Blake2b256::digest(databytes);
        let expected_hex = "c9e8caa4700fd1c98a7568ba636f47d35e43f8d2ba08f379bad7baf7d113e1f9";
        assert_eq!(hex::encode(hash), expected_hex);
    }

    #[tokio::test]
    async fn test_client_encode_data() {

        let addr: SocketAddr = format!("{}:{}", "127.0.0.1", "9010").parse().unwrap();

        let server = AnonymousServer::new(None);
        let _server_handle = tokio::spawn(async move {
            if let Err(e) = server.start(addr).await {
                eprintln!("Server error: {:?}", e);
            }
        });

        let client = crate::client_test::AnonymousClient::new("http://localhost:9010");
        
        // Wait for server to be ready by pinging it first
        let ping_result = client.test_ping().await;
        assert!(ping_result.success, "Server ping failed: {:?}", ping_result.error);
        info!("Ping Result: {:?}", ping_result.response);
        
        let split_result = client.test_split(20, 1).await;
        assert!(split_result.success, "test_split failed: {:?}", split_result.error);
        let split_result_0 = split_result.response.expect("test_split returned None response");
        println!("Split 20 Result: {:?}", split_result_0);

    }

    #[tokio::test]
    async fn test_client_encode_data_array_for_client() {
        let addr: SocketAddr = format!("{}:{}", "127.0.0.1", "9010").parse().unwrap();
        let server = AnonymousServer::new(None);
        let _server_handle = tokio::spawn(async move {
            if let Err(e) = server.start(addr).await {
                eprintln!("Server error: {:?}", e);
            }
        });

        let client = crate::client_test::AnonymousClient::new("http://localhost:9010");

        // Wait for server to be ready by pinging it first
        let ping_result = client.test_ping().await;
        assert!(ping_result.success, "Server ping failed: {:?}", ping_result.error);
        info!("Ping Result: {:?}", ping_result.response);

        let vec = vec![String::from("1000000000")];

        let signature = "993473e0c9f7f7a1487e25a8fc3ddf4adc495d0d4a8aab97e4279d4dcac4dfebcd1e170627ac36aebd57de362019a89c4c2305ef957848a80c19d30f150c8f03";
        let signature_bytes = hex_to_bytes(signature);
        let object_id_list = "BFCa4e7d3832d6ccf20f72f831bc164c5afc6ad2d03081101b098297b4c4cf6d3743ac8BFCa6eecddaabb11bef34c99e982e6f74d9c3820d3bf87220ec3f208a360b77223f5fd0";
        let publickey = "e674db6a6027824dd9ed453808ee034bc75b5fe661ae6483dafffe1b062f7073";
        let publickey_bytes = hex_to_bytes(publickey);
        let owner = AccountAddress::from_hex_literal("0x6f0f9a9a72f7d48b8fcbfa09ebb61123d847aaad5760297d68c64795bad514b1").unwrap();

        let split_result = client.test_encode_data_array(vec, owner, signature_bytes, object_id_list.to_string(), publickey_bytes).await;
        assert!(split_result.success, "test_split failed: {:?}", split_result.error);
        let split_result_0 = split_result.response.expect("test_split returned None response");
        println!("Split 20 Result: {:?}", split_result_0);
    }

    #[tokio::test]
    async fn test_client_compare_value1_and_value2() {
        let addr: SocketAddr = format!("{}:{}", "127.0.0.1", "9010").parse().unwrap();

        let server = AnonymousServer::new(None);
        let _server_handle = tokio::spawn(async move {
            if let Err(e) = server.start(addr).await {
                eprintln!("Server error: {:?}", e);
            }
        });

        let client = crate::client_test::AnonymousClient::new("http://localhost:9010");
        let split_result_0 = client.test_split(20, 1).await.response.unwrap();
        info!("Split 20 Result: {:?}", split_result_0);

        let split_result_1 = client.test_split(10, 1).await.response.unwrap();
        info!("Split 20 Result: {:?}", split_result_1);


        //test 20 > 10
        let compare_result = client
            .test_compare_value1_value2(
                split_result_0["result"]["result1"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                split_result_0["result"]["result2"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                split_result_1["result"]["result1"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                split_result_1["result"]["result2"]
                    .as_str()
                    .unwrap()
                    .to_owned()
            )
            .await
            .response
            .unwrap();
        println!("Compare Result: {:?}", compare_result);


    }

    #[tokio::test]
    async fn test_client_without_server() {
        let client = crate::client_test::AnonymousClient::new("http://localhost:9010");
        assert_eq!(client.base_url, "http://localhost:9010");
    }

    #[tokio::test]
    async fn test_client_restore_value_array(){
        // let subscriber = fmt::Subscriber::new();
        // tracing::subscriber::set_global_default(subscriber)
        //     .expect("Failed to set tracing subscriber");

        //let args = Args::parse();
        let addr: SocketAddr = format!("{}:{}", "127.0.0.1", "9010").parse().unwrap();

        info!("the address is {:?}", addr);
        let server = AnonymousServer::new(None);
        let _server_handle = tokio::spawn(async move {
            if let Err(e) = server.start(addr).await {
                eprintln!("Server error: {:?}", e);
            }
        });

        let client = crate::client_test::AnonymousClient::new("http://localhost:9010");
        let ping_result = client.test_ping().await.response.unwrap();
        info!("Ping Result: {:?}", ping_result);


        // test split first
        let split_result_0 = client.test_split(20, 1).await.response.unwrap();
        info!("Split 20 Result: {:?}", split_result_0);


        //test 20 + 20
        let add_result = client
            .test_add(
                split_result_0["result"]["result1"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                split_result_0["result"]["result2"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                split_result_0["result"]["result1"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                split_result_0["result"]["result2"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
            )
            .await
            .response
            .unwrap();
        info!("Add Result: {:?}", add_result);
        let data1 = add_result["result"]["result1"].as_str().unwrap().to_owned();
        let data2 = add_result["result"]["result2"].as_str().unwrap().to_owned();
        info!("data1 = {}, data2 = {}", data1, data2);
        let add_result = client
            .test_recover_array_with_signature(
                data1,
                data2,
            )
            .await;
        assert!(add_result.is_ok());

        let value = add_result .expect("failed to get restore value array");
        assert_eq!(value[0], 40);
        assert_eq!(value[1], 40);

    }

    #[tokio::test]
    async fn test_client_restore_value_array_for_zklogin(){
        // let subscriber = fmt::Subscriber::new();
        // tracing::subscriber::set_global_default(subscriber)
        //     .expect("Failed to set tracing subscriber");

        //let args = Args::parse();
        let addr: SocketAddr = format!("{}:{}", "127.0.0.1", "9010").parse().unwrap();

        info!("the address is {:?}", addr);
        let server = AnonymousServer::new(None);
        let _server_handle = tokio::spawn(async move {
            if let Err(e) = server.start(addr).await {
                eprintln!("Server error: {:?}", e);
            }
        });

        let client = crate::client_test::AnonymousClient::new("http://localhost:9010");
        let split_result_0 = client.test_split(20, 1).await.response.unwrap();
        info!("Split 20 Result: {:?}", split_result_0);

        let add_result = client
            .test_recover_array_with_signature_for_zklogin(
                split_result_0["result"]["result1"].as_str().unwrap().to_owned(),
                split_result_0["result"]["result2"].as_str().unwrap().to_owned(),
            )
            .await;
    }

    #[tokio::test]
    async fn test_encode_data_for_zklogin(){
        // let subscriber = fmt::Subscriber::new();
        // tracing::subscriber::set_global_default(subscriber)
        //     .expect("Failed to set tracing subscriber");

        //let args = Args::parse();
        let addr: SocketAddr = format!("{}:{}", "127.0.0.1", "9010").parse().unwrap();

        info!("the address is {:?}", addr);
        let server = AnonymousServer::new(None);
        let _server_handle = tokio::spawn(async move {
            if let Err(e) = server.start(addr).await {
                eprintln!("Server error: {:?}", e);
            }
        });

        let client = crate::client_test::AnonymousClient::new("http://localhost:9010");
        let split_result_0 = client.test_encode_data_array_with_signature_for_zklogin(vec![String::from("1000000000")]).await.response.unwrap();
        info!("Split 20 Result: {:?}", split_result_0);

    }

    #[tokio::test]
    async fn test_client_with_server() -> anyhow::Result<()> {
        // let subscriber = fmt::Subscriber::new();
        // tracing::subscriber::set_global_default(subscriber)
        //     .expect("Failed to set tracing subscriber");

        //let args = Args::parse();
        let addr: SocketAddr = format!("{}:{}", "127.0.0.1", "9010").parse().unwrap();

        info!("the address is {:?}", addr);
        let server = AnonymousServer::new(None);
        let _server_handle = tokio::spawn(async move {
            if let Err(e) = server.start(addr).await {
                eprintln!("Server error: {:?}", e);
            }
        });

        let client = crate::client_test::AnonymousClient::new("http://localhost:9010");
        let ping_result = client.test_ping().await.response.unwrap();
        info!("Ping Result: {:?}", ping_result);

        // test split first
        let split_result_0 = client.test_split(20, 1).await.response.unwrap();
        info!("Split 20 Result: {:?}", split_result_0);
        let split_result_1 = client.test_split(10, 1).await.response.unwrap();
        info!("Split 10 Result: {:?}", split_result_1);

        let split_result_0_repeat = client.test_split(20, 1).await.response.unwrap();
        info!("Split 20 Result repeat: {:?}", split_result_0);
        let split_result_1_repeat = client.test_split(10, 1).await.response.unwrap();
        info!("Split 10 Result repeat: {:?}", split_result_1);

        assert_eq!(split_result_0["result"]["result1"], split_result_0_repeat["result"]["result1"]);
        assert_eq!(split_result_0["result"]["result2"], split_result_0_repeat["result"]["result2"]);

        assert_eq!(split_result_1["result"]["result1"], split_result_1_repeat["result"]["result1"]);
        assert_eq!(split_result_1["result"]["result2"], split_result_1_repeat["result"]["result2"]);



        //test 20 + 10
        let add_result_response = client
            .test_add(
                split_result_0["result"]["result1"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                split_result_0["result"]["result2"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                split_result_1["result"]["result1"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                split_result_1["result"]["result2"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
            )
            .await;
        assert!(add_result_response.success, "test_add failed: {:?}", add_result_response.error);
        let add_result = add_result_response.response.expect("test_add returned None response");
        info!("Add Result: {:?}", add_result);
        let add_result = client
            .test_compare(
                add_result["result"]["result1"].as_str().unwrap().to_owned(),
                add_result["result"]["result2"].as_str().unwrap().to_owned(),
                30,
            )
            .await;


        //test 20 - 10
        let minus_result_response = client
            .test_minus(
                split_result_0["result"]["result1"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                split_result_0["result"]["result2"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                split_result_1["result"]["result1"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                split_result_1["result"]["result2"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
            )
            .await;
        assert!(minus_result_response.success, "test_minus failed: {:?}", minus_result_response.error);
        let minus_result = minus_result_response.response.expect("test_minus returned None response");
        info!("Minus Result: {:?}", minus_result);
        let minus_result = client
            .test_compare(
                minus_result["result"]["result1"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                minus_result["result"]["result2"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                10,
            )
            .await;


        //test 20 * 10
        let multiply_result_response = client
            .test_multiply(
                split_result_0["result"]["result1"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                split_result_0["result"]["result2"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                split_result_1["result"]["result1"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                split_result_1["result"]["result2"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
            )
            .await;
        assert!(multiply_result_response.success, "test_multiply failed: {:?}", multiply_result_response.error);
        let multiply_result = multiply_result_response.response.expect("test_multiply returned None response");
        info!("Multiply Result: {:?}", multiply_result);

        let multiply_result = client
            .test_compare(
                multiply_result["result"]["result1"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                multiply_result["result"]["result2"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                200,
            )
            .await;
        
        //test 20 > 5
        let compare_result = client
            .test_compare(
                split_result_0["result"]["result1"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                split_result_0["result"]["result2"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                5,
            )
            .await
            .response
            .unwrap();
        info!("Compare Result: {:?}", compare_result);
        let result = compare_result["result"]["result1"]
            .as_str()
            .unwrap()
            .to_owned();
        info!("Compare Result Values: result = {}", result);

        Ok(())
    }

    #[tokio::test]
    async fn test_write_unsigned_leb128(){
        // test zero
        let mut buffer = [0u8; 10];
        let bytes_written = write_unsigned_leb128(&mut buffer, 0);
        assert_eq!(bytes_written, 1);
        assert_eq!(buffer[0], 0);

        // test 1
        let bytes_written = write_unsigned_leb128(&mut buffer, 1);
        assert_eq!(bytes_written, 1);
        assert_eq!(buffer[0], 1);

        // test 127 (0x7F)
        let bytes_written = write_unsigned_leb128(&mut buffer, 127);
        assert_eq!(bytes_written, 1);
        assert_eq!(buffer[0], 127);

        // Test boundary value
        let bytes_written = write_unsigned_leb128(&mut buffer, 0x7F);
        assert_eq!(bytes_written, 1);
        assert_eq!(buffer[0], 0x7F);
        let bytes_written = write_unsigned_leb128(&mut buffer, 0x80);
        assert_eq!(bytes_written, 2);
        assert_eq!(buffer[0], 0x80);
        assert_eq!(buffer[1], 0x01);
    }
}
