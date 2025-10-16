use reqwest;
use serde_json::{json, Value};
use std::error::Error;
use std::str::FromStr;
use log::info;
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
            .post(&format!("{}/rpc", self.base_url))
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
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "value4": value4,
        });

        match self
            .send_rpc_request("bfcx_getAnonymousAdd", params, 1)
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
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "value4": value4,
        });

        match self
            .send_rpc_request("bfcx_getAnonymousMinus", params, 2)
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
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "value4": value4,
        });

        match self
            .send_rpc_request("bfcx_getAnonymousMultiply", params, 3)
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
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
        });

        match self
            .send_rpc_request("bfcx_getAnonymousCompare", params, 4)
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

    pub async fn test_restore_value_array(
        &self,
        value1: Vec<u8>,
        value2: Vec<u8>,
        signature: Vec<u8>,
        objectid: String,
        publickey: Vec<u8>,
    ) -> TestResult {

        let params = Value::Object(
            serde_json::Map::from_iter([
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
            .send_rpc_request("bfcx_getAnonymousRestoreValueArray", params, 4)
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
        let params = Value::Object(
            serde_json::Map::from_iter([
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
            .send_rpc_request("bfcx_getAnonymousRestoreValueArrayForZKloginAddress", params, 4)
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
        let params = json!({
            "value1": value1,
            "value2": value2,
            "signature": signature,
            "objectid": objectid,
        });

        match self
            .send_rpc_request("bfcx_getAnonymousRestoreValueForZKloginAddress", params, 4)
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

    pub async fn test_restore_value(
        &self,
        value1: Vec<u8>,
        value2: Vec<u8>,
        signature: Vec<u8>,
        objectid: String,
        publickey: Vec<u8>,
    ) -> TestResult {
        let params = json!({
            "value1": value1,
            "value2": value2,
            "signature": signature,
            "objectid": objectid,
            "publickey": publickey,
        });

        match self
            .send_rpc_request("bfcx_getAnonymousRestoreValue", params, 4)
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
        let params = json!({
            "value": value,
        });

        match self
            .send_rpc_request("bfcx_getAnonymousEncodeData", params, 4)
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
        match self.send_rpc_request("bfcx_ping", json!({}), 5).await {
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
        let signature = "BQNNMTAwNjc1MzY4Nzc3NjU0NDM2MTcxMTYzMDYyNDU3MTgwNzQ3NzY2MDMxMDc5MTc4NjE4ODQ5NTAyNDA4MDI5NjE5MzE0OTAzNjYwMjFNMjA3NTY3NDc2MDI4NzM5MDkxNDAwOTg4NDMwNzMyOTQ4NTUyNjUyNTg2MzM5OTI2NTIyNjA4MTY2ODU3MDgzMzE3NTU0Mjc4MTE1OTABMQMCTTIxNzEyOTU0MDI2Nzk5ODI2NDEwMDY2MTY5MzcyNzU3MTE1OTY3MDU1ODUzNjY4NjYyOTY5ODc1MTI4NDA5MDY3NzgwNTUyNDYwNzYxTDk1MTc2MzI3MDY3ODYwMzAxMDA1ODYzMTc4MDg5OTI1MTcyODYyNzk5MDYzNDExNTAyOTUyODQ3NDQ3OTcyMTM4MjMzNDY5NDEzNTICSzUzMTQ2NTU4NDMwNDAyMDk3NzAyNzQwNjgyMTA3NzYzNDYwNjQ0NTE1MzU2NjI1MjAwODY1NDM2ODY2Mzk3ODkyMzEzMTM4MTU0MkwzNjQ3NzUxMjgzOTMxNDUzNjcxNDIxNDkzMzI5NzkxOTk2MzE4MzI3NzU4MTg1NDk1ODIyNzE5NTg0MDU1OTI5NzU4NDYwNDUxMjc1AgExATADTTE5MTYzOTQwNDc4MTY2MzU3NzA5Nzg4NTQyMzAzNzk2NTY4OTE2Njg2MDE4OTcxNjE5NzU3MjQ5MTIxNTUzMTM2NTYyMTk5NTM3NjQxTTE1NTM4Mzk1NDEwNzA0NjA0NjkyNzg5OTE4NzE5NzAwNDM2NjM0Mjc2OTM2Nzk2MDc1MDE2MTQ1MzU5MzU0Nzc1NjAzNzkyODg0MzY1ATExeUpwYzNNaU9pSm9kSFJ3Y3pvdkwyRmpZMjkxYm5SekxtZHZiMmRzWlM1amIyMGlMQwFmZXlKaGJHY2lPaUpTVXpJMU5pSXNJbXRwWkNJNkltTTRZV0kzTVRVek1EazNNbUppWVRJd1lqUTVaamM0WVRBNVl6azROVEpqTkRObVpqa3hNVGdpTENKMGVYQWlPaUpLVjFRaWZRTTE2MjcxMjcyODMwMTA3NjU0NDczNDYxNzQ5Nzc4NjM0NjIzMDg0NTAzNzM0MDM3ODIzNTc1MTYwMDA5Mzg1MjU2NTQyNzYzNzY5Nzg3jwEAAAAAAABhAGuV3qp3Eg57tOYbo66khJbEmgKIwUzI/dOtJPx9VSafpW2moEtI4MoJUh9XmizMvqeTaQVKVg9F/CezODR7Dg4F1NcBhOWa2gnuk4QCZSIUzY7bgZesXwl6+Cb1ahmmYQ==";
        let byte = "QkZDYTRlN2QzODMyZDZjY2YyMGY3MmY4MzFiYzE2NGM1YWZjNmFkMmQwMzA4MTEwMWIwOTgyOTdiNGM0Y2Y2ZDM3NDNhYzhCRkNhNmVlY2RkYWFiYjExYmVmMzRjOTllOTgyZTZmNzRkOWMzODIwZDNiZjg3MjIwZWMzZjIwOGEzNjBiNzcyMjNmNWZkMA==".to_string();
        let signature_bytes= ZkVerifyRequest {
            signature: signature.to_string(),
            bytes: byte, // abs token objectid
            intent_scope: 3,
            cur_epoch: None,
            cur_rpc_url : Some("https://testrpc.benfen.org/".to_string()),
            author: "BFC8c92533545c7f97e7491ea0049c9efdd25f43bf18fe56e92b4ce4b40d05165be80f1".to_string(),
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
        let signature = "80361ef8ca66108d1fb68ac81970cc9f7315ca6b1dea0bc493059603faffc8bcdcc7644b2ec55b8e48fe613b30e9b534000ef9e1b626f9f5bdf9519e7b7cef04";
        let publickey = "8496d3d932986b43bb64b5d5c7548d5c97a73aebf4301447f3746680b2114ae1";

        let object_id = SuiAddress::from_str(
            "0xa6eecddaabb11bef34c99e982e6f74d9c3820d3bf87220ec3f208a360b77223f",
        )
            .unwrap()
            .to_string();

        let publickey_bytes = hex_to_bytes(publickey);
        let signature = "BQNMMjgzMzYwMjYzNTk1NTYyMjM3NjczOTA3OTk2NzgwMjgzMzI2NjA1NDU2NDkxMzk2NTQ5MDQ5NTE2NDIwNzc0ODY2MTIxNDQ0MTk1NkwyNDc1OTEyOTUxNTAxNTUwOTExMTk0MjIzNTA1ODMxMDMyNzQzNDUzNDkyNDc1NDQ1OTEyODYwNzMyMTE1NjM4MTIxNTYzNTQzNzAwATEDAk0xNTAxNDc1MzAzNjkyODg4NzY2MTg1MTA3NzgzNDYwNDM5MjU2NTQxODk0OTU0MzE0OTY4OTY3NjE4MDIwMDQzOTI1OTU2MjIyMTk0Mk0xMTU5NDA2NzI0ODU4NzAyNjQxMzc4NjY4Mzk0NTc5MDg4MzY3OTg4Nzg1ODc1MDgyMzkwMTExNDkyMjg1MDYzMzg4MjU3ODUzNDA4OAJMNDQ2NzM4NjUwODU1NTQxMjU5NjUyODAxMzc5NjMyMjI3MTA2NDUwNzUyNDM1NjM0ODI3Nzg1NjA5MjM1MjU3MTk3MzMyOTg1MTc5M00xNDUzOTc2Mzk2MDg5Mzc4MDM2Nzk5Njg5NTAwMzI2NDg4MzkxMDM4NjgyNzE3NjE4OTk5MTQwMDIyNTM4MTQ5MzUxMTIwMTY3NDEyMQIBMQEwA0szMjk3MjM2MDkyNDcxMTkxNjc2MDU3MzM2MTE2MjQ3MzE0MDgxMDkyMTg1MDU2NjgzNzU4ODgzMTA4NzA3ODk4MDk1OTQ3OTE2NDlNMTYwMjA5OTAyNjQ5MTMxMzg4NDk3ODU2MTI2Mjk4NjQ3NDU1OTEzNDAwODE2ODE0OTk5NjAxNzYxNTgzNzIzMDQzNjEzMTE1MjA4NDMBMTF5SnBjM01pT2lKb2RIUndjem92TDJGalkyOTFiblJ6TG1kdmIyZHNaUzVqYjIwaUxDAWZleUpoYkdjaU9pSlNVekkxTmlJc0ltdHBaQ0k2SW1NNFlXSTNNVFV6TURrM01tSmlZVEl3WWpRNVpqYzRZVEE1WXprNE5USmpORE5tWmpreE1UZ2lMQ0owZVhBaU9pSktWMVFpZlFNMTYyNzEyNzI4MzAxMDc2NTQ0NzM0NjE3NDk3Nzg2MzQ2MjMwODQ1MDM3MzQwMzc4MjM1NzUxNjAwMDkzODUyNTY1NDI3NjM3Njk3ODeMAQAAAAAAAGEAjtxMFS4GFnahbYxmI8teVbo2iLHRlMiveOdoh0rfKW4mTjA2YlJ3DOA3sSMVfGWnQPg/H1KgwDOl/t/GUiFhB56JP/IwBKtUjNOnocmfTmLDPvOKHyeHN00Rv0xh8XIF".to_string();
        let byte = "MHhhNmVlY2RkYWFiYjExYmVmMzRjOTllOTgyZTZmNzRkOWMzODIwZDNiZjg3MjIwZWMzZjIwOGEzNjBiNzcyMjNm".to_string();
        let signature_bytes= ZkVerifyRequest {
            signature: signature,
            bytes: byte, // abfc token objectid
            intent_scope: 3,
            cur_epoch: None,
            cur_rpc_url : Some("https://testrpc.benfen.org/".to_string()),
            author: "BFC8c92533545c7f97e7491ea0049c9efdd25f43bf18fe56e92b4ce4b40d05165be80f1".to_string(),
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
    use std::net::SocketAddr;
    use tracing::info;
    use tracing_subscriber::fmt;

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
    async fn test_client_without_server() {
        let client = crate::client_test::AnonymousClient::new("http://localhost:9010");
        assert_eq!(client.base_url, "http://localhost:9010");
    }

    #[tokio::test]
    async fn test_client_restore_value_array(){
        let subscriber = fmt::Subscriber::new();
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set tracing subscriber");

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
        let subscriber = fmt::Subscriber::new();
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set tracing subscriber");

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
        let subscriber = fmt::Subscriber::new();
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set tracing subscriber");

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
        let subscriber = fmt::Subscriber::new();
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set tracing subscriber");

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
}
