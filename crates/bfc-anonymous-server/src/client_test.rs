use reqwest;
use serde_json::{json, Value};
use std::error::Error;
use std::str::FromStr;
use sui_types::base_types::SuiAddress;

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

    pub async fn test_restore_value(
        &self,
        value1: String,
        value2: String,
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
            .send_rpc_request("bfcx_getAnonymousSplitValue", params, 4)
            .await
        {
            Ok(response) => TestResult {
                method: "bfcx_getAnonymousSplitValue".to_string(),
                success: true,
                response: Some(response),
                error: None,
            },
            Err(e) => TestResult {
                method: "bfcx_getAnonymousSplitValue".to_string(),
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

    async fn test_recover_with_signature(&self, share1: String, share2: String) -> u64 {
        // test restore 20
        let signature = "0f31177f8ece16b2cfb8c1ba0b71f73252acaa6cfbbe13d36c3320617f05bc7f9a860f16c8b10c787455a01ca7bcca3469858aae4e369bc994ab64967f1fd20f";
        let publickey = "8496d3d932986b43bb64b5d5c7548d5c97a73aebf4301447f3746680b2114ae1";

        let object_id = SuiAddress::from_str(
            "0xd4c2360f11b1608f3be0b8d89bc97ff3047378dbc34d0b3976f40e6392496fd5",
        )
        .unwrap()
        .to_string();

        let signature_bytes = hex_to_bytes(signature);
        let publickey_bytes = hex_to_bytes(publickey);

        let restore_result = self
            .test_restore_value(share1, share2, signature_bytes, object_id, publickey_bytes)
            .await
            .response
            .unwrap();

        restore_result["result"]["result1"].as_u64().unwrap()
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
    use crate::AnonymousServer;
    use std::net::SocketAddr;
    use tracing::info;
    use tracing_subscriber::fmt;

    #[tokio::test]
    async fn test_client_without_server_auto_start() {
        let client = crate::client_test::AnonymousClient::new("http://localhost:9010");
        assert_eq!(client.base_url, "http://localhost:9010");

        let ping_result = client.test_ping().await.response.unwrap();
        info!("Ping Result: {:?}", ping_result);

        // test split first
        let split_result_0 = client.test_split(20).await.response.unwrap();
        info!("Split 20 Result: {:?}", split_result_0);
        let split_result_1 = client.test_split(10).await.response.unwrap();
        info!("Split 10 Result: {:?}", split_result_1);
    }

    #[tokio::test]
    async fn test_client_with() -> anyhow::Result<()> {
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
