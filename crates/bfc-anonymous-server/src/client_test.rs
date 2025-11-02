use reqwest;
use serde_json::{json, Value};
use std::error::Error;
use std::str::FromStr;
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

        let response_text = response.text().await?;
        let response_json: Value = serde_json::from_str(&response_text)?;

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

    pub async fn test_encode_data_for_client(
        &self,
        value: u64,
        signature: Vec<u8>,
        owner: String,
        publickey: Vec<u8>,
    ) -> TestResult {
        let params = json!({
            "value": value,
            "signature": signature,
            "publickey": publickey,
            "owner": owner,
        });

        match self
            .send_rpc_request("bfcx_getAnonymousEncodeDataForClient", params, 4, "rpc")
            .await
        {
            Ok(response) => TestResult {
                method: "bfcx_getAnonymousEncodeDataForClient".to_string(),
                success: true,
                response: Some(response),
                error: None,
            },
            Err(e) => TestResult {
                method: "bfcx_getAnonymousEncodeDataForClient".to_string(),
                success: false,
                response: None,
                error: Some(e.to_string()),
            },
        }
    }

    pub async fn test_restore_value(
        &self,
        value1: Vec<u8>,
        value2: Vec<u8>,
        signature: Vec<u8>,
        objectid: String,
        publickey: Vec<u8>,
    ) -> TestResult {
        let owner = AccountAddress::from_hex_literal("0x1").unwrap();
        let params = json!({
            "value1": value1,
            "value2": value2,
            "signature": signature,
            "objectid": objectid,
            "publickey": publickey,
            "owner": owner,
        });

        match self
            .send_rpc_request("bfcx_getAnonymousRestoreValue", params, 4, "rpc")
            .await
        {
            Ok(response) => TestResult {
                method: "bfcx_getAnonymousRestoreValue".to_string(),
                success: true,
                response: Some(response),
                error: None,
            },
            Err(e) => TestResult {
                method: "bfcx_getAnonymousRestoreValue".to_string(),
                success: false,
                response: None,
                error: Some(e.to_string()),
            },
        }
    }

    pub async fn test_split(&self, value: u64) -> TestResult {
        let user_address = AccountAddress::from_hex_literal("0x1").unwrap();
        let params = json!({
            "value": value,
            "owner": user_address,
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

    async fn test_recover_array_with_signature_for_zklogin(&self, share1: String, share2: String) -> Result<Vec<u64>, serde_json::error::Error> {
        // test restore 20
        let publickey = "8496d3d932986b43bb64b5d5c7548d5c97a73aebf4301447f3746680b2114ae1";
        let object_id1 = "BFCa4e7d3832d6ccf20f72f831bc164c5afc6ad2d03081101b098297b4c4cf6d3743ac8";
        let object_id2 = "BFCa6eecddaabb11bef34c99e982e6f74d9c3820d3bf87220ec3f208a360b77223f5fd0";

        let object_id_list = "BFCa4e7d3832d6ccf20f72f831bc164c5afc6ad2d03081101b098297b4c4cf6d3743ac8BFCa6eecddaabb11bef34c99e982e6f74d9c3820d3bf87220ec3f208a360b77223f5fd0";
        let publickey_bytes = hex_to_bytes(publickey);
        let signature = "BQNNMTg0MTk0NTMzNDY1NTIzMjA3MTI1MTU4OTI4NzE2NDU0MTE0MTQxMTA4MTYxNjE2MzcwMTcyMzE4NzYzMzI5ODY1NTE4NDM3MjgxMThNMTc0Mjg1NDc2MTkwMTQyODAxMTMzMTcxODA2Njc2NjU5ODk4NDYxNTkxOTYyODQ3MDYxMTY4NTEwNzM0OTkwNzkwMDIzMTkzODkwNzMBMQMCTTE3ODUzNjY4OTA0NjA4NzUwMDEzMDIzOTAzODU5MTg5MjUwNzA5MjMzNjg0NjczNjg3MzA4NjcyMDA5MTAyNTkxNDM4NTMzNzg1OTk4TTE0MTA2MzI0Njk1OTI0NTgyMDU0NzY1NDcyODExMTE3NzgzNjk4ODM3MjQxMDkyNTg0NjUwMDI1NTI0NDEwOTk1NzU0Njc5MTAxNTUxAkw2NjI1Mjc3OTA2NzcwOTQxMDgwMjMwODEyMTExMDUxOTExMjU2MzUzMzYzOTE2NTk4ODYzOTI5MjMwMjIxNTgxMDI0NzMyNjA2NzczTTE1MDgyMzcxOTQzNDIwNTAxNzI3MzY2NTQ1NzgxMDQwNzk0MTk3NjQ5NDU1MjIxNDYwNzczNTc3ODU3NTg5NzkxMDcwMzcwMjEzNzE0AgExATADSzIzNzQ4NTUzNzMxNjgxNTk3NDI2MDMwOTM1MDc3NTU1NzY2Nzc3OTQ3OTc3OTgyMjgyOTMyNzIwNTIxMzQwNzY3NzgyNTA3MTA0N00xNTg3MDcwMDQzMDk2NzAzMDA0NjI1MjQ0NTYwOTYzMTI3ODQ1NDk3ODg1NDE1OTAxNDgxMjExNDM0NzM4MTExNzc4NzI3MTQ3MTY5NwExMXlKcGMzTWlPaUpvZEhSd2N6b3ZMMkZqWTI5MWJuUnpMbWR2YjJkc1pTNWpiMjBpTEMBZmV5SmhiR2NpT2lKU1V6STFOaUlzSW10cFpDSTZJamc0TkRnNU1qRXlNbVV5T1RNNVptUXhaak14TXpjMVlqSmlNell6WldNNE1UVTNNak5pWW1JaUxDSjBlWEFpT2lKS1YxUWlmUUwzMDA0NDUyNDMzOTUzNDc3NDY4Mjk3MTI4MDIyMjYwMjEyODIyNjk0MjM5MzMwNTU4OTE5ODc3OTM5ODA2MjAyOTgzMDk5ODkyODU1rwEAAAAAAABhANpFQhyM8aC5Kpwc36oVLQMufXEtbzHyHZHExy73Fki5JmSORe63A2L4ZAMaHHZmE+/6TCoqnWEbJI+VwAEIqQwg/J1PkdfE2/sruj/vZ5unh5gbotMAgFOdpb3P1+CniA==";
        let byte = "QkZDOGMyOTM4ZjFmMzJjODBhY2M1NDhiM2FhNjI2ZDcxNzE2M2Y4MDU4OTdiNTg2ZTBkODE5YTg4NzU0ZTNmYWMyYzAwZWM6ODdMWVRja1BqQTE5WTdqNHpDaHVSamFuY3hVdEFSdXdsUEtQVHMwLVhPdlg5RW1RSUVGdkNaa2V6OWIxenJUYw==".to_string();
        let signature_bytes= ZkVerifyRequest {
            signature: signature.to_string(),
            bytes: byte, // abs token objectid
            intent_scope: 3,
            cur_epoch: None,
            cur_rpc_url : Some("https://testrpc.benfen.org/".to_string()),
            author: "BFC8c2938f1f32c80acc548b3aa626d717163f805897b586e0d819a88754e3fac2c00ec".to_string(),
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


    async fn test_recover_with_signature(&self, share1: String, share2: String) -> u64 {
        // test restore 20
        let signature = "80361ef8ca66108d1fb68ac81970cc9f7315ca6b1dea0bc493059603faffc8bcdcc7644b2ec55b8e48fe613b30e9b534000ef9e1b626f9f5bdf9519e7b7cef04";
        let publickey = "8496d3d932986b43bb64b5d5c7548d5c97a73aebf4301447f3746680b2114ae1";

        let object_id = SuiAddress::from_str(
            "0xd4c2360f11b1608f3be0b8d89bc97ff3047378dbc34d0b3976f40e6392496fd5",
        )
        .unwrap()
        .to_string();

        let signature_bytes = hex_to_bytes(signature);
        let publickey_bytes = hex_to_bytes(publickey);
        
        let restore_result = self
            .test_restore_value(share1.into_bytes(), share2.into_bytes(), signature_bytes, object_id, publickey_bytes)
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
        let split_result_0 = client.test_split(20).await.response.unwrap();
        println!("Split 20 Result: {:?}", split_result_0);

    }

    #[tokio::test]
    async fn test_client_encode_data_for_client() {

        let addr: SocketAddr = format!("{}:{}", "127.0.0.1", "9010").parse().unwrap();

        let server = AnonymousServer::new(None);
        let _server_handle = tokio::spawn(async move {
            if let Err(e) = server.start(addr).await {
                eprintln!("Server error: {:?}", e);
            }
        });

        let signature = "4385a68699cc6a3fad8aaa0660f557773767f054c9d451b2cf3bdd956c54e15bc75d766851c0dafd523fe0b8086910717b5c48fba3a342a3387bc43411581c04";
        let signature_bytes = hex_to_bytes(signature);
        let owner = "0xfc171f86c07b0311a347d7e71b261c684848becbececec78802f1bf8a599f729";
        let publickey = "8496d3d932986b43bb64b5d5c7548d5c97a73aebf4301447f3746680b2114ae1";
        let publickey_bytes = hex_to_bytes(publickey);

        let client = crate::client_test::AnonymousClient::new("http://localhost:9010");
        let encode_value = client.test_encode_data_for_client(10000000, signature_bytes, owner.to_string(), publickey_bytes).await.response.unwrap();
        println!("encode value: {:?}", encode_value);
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
        let split_result_0 = client.test_split(20).await.response.unwrap();
        info!("Split 20 Result: {:?}", split_result_0);

        let split_result_1 = client.test_split(10).await.response.unwrap();
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
        let split_result_0 = client.test_split(20).await.response.unwrap();
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
    async fn test_client_restore_value_for_zklogin(){
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
        let split_result_0 = client.test_split(20).await.response.unwrap();
        info!("Split 20 Result: {:?}", split_result_0);

        let add_result = client
            .test_recover_with_signature_for_zklogin(
                split_result_0["result"]["result1"].as_str().unwrap().to_owned(),
                split_result_0["result"]["result2"].as_str().unwrap().to_owned(),
            )
            .await;
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
        let split_result_0 = client.test_split(20).await.response.unwrap();
        info!("Split 20 Result: {:?}", split_result_0);

        let add_result = client
            .test_recover_array_with_signature_for_zklogin(
                split_result_0["result"]["result1"].as_str().unwrap().to_owned(),
                split_result_0["result"]["result2"].as_str().unwrap().to_owned(),
            )
            .await;
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
        let split_result_0 = client.test_split(20).await.response.unwrap();
        info!("Split 20 Result: {:?}", split_result_0);
        let split_result_1 = client.test_split(10).await.response.unwrap();
        info!("Split 10 Result: {:?}", split_result_1);

        let split_result_0_repeat = client.test_split(20).await.response.unwrap();
        info!("Split 20 Result repeat: {:?}", split_result_0);
        let split_result_1_repeat = client.test_split(10).await.response.unwrap();
        info!("Split 10 Result repeat: {:?}", split_result_1);

        assert_eq!(split_result_0["result"]["result1"], split_result_0_repeat["result"]["result1"]);
        assert_eq!(split_result_0["result"]["result2"], split_result_0_repeat["result"]["result2"]);

        assert_eq!(split_result_1["result"]["result1"], split_result_1_repeat["result"]["result1"]);
        assert_eq!(split_result_1["result"]["result2"], split_result_1_repeat["result"]["result2"]);



        //test 20 + 10
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
                split_result_1["result"]["result1"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                split_result_1["result"]["result2"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
            )
            .await
            .response
            .unwrap();
        info!("Add Result: {:?}", add_result);
        let add_result = client
            .test_recover_with_signature(
                add_result["result"]["result1"].as_str().unwrap().to_owned(),
                add_result["result"]["result2"].as_str().unwrap().to_owned(),
            )
            .await;
        info!("Add Result recover to u64:{}", add_result);
        assert_eq!(add_result, 30);

        //test 20 - 10
        let minus_result = client
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
            .await
            .response
            .unwrap();
        info!("Minus Result: {:?}", minus_result);
        let minus_result = client
            .test_recover_with_signature(
                minus_result["result"]["result1"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                minus_result["result"]["result2"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
            )
            .await;
        info!("Minus Result recover to u64:{}", minus_result);
        assert_eq!(minus_result, 10);

        //test 20 * 10
        let multiply_result = client
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
            .await
            .response
            .unwrap();
        info!("Multiply Result: {:?}", multiply_result);

        let multiply_result = client
            .test_recover_with_signature(
                multiply_result["result"]["result1"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
                multiply_result["result"]["result2"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
            )
            .await;
        info!("Multiply Result recover to u64:{}", multiply_result);
        assert_eq!(multiply_result, 200);

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
