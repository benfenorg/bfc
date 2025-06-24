use reqwest;
use serde_json::{json, Value};
use std::error::Error;

#[derive(Debug)]
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

    pub async fn test_add(&self, value1: u64, value2: u64, value3: u64, value4: u64) -> TestResult {
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "value4": value4,
        });

        match self.send_rpc_request("bfcx_getAnonymousAdd", params, 1).await {
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

    pub async fn test_minus(&self, value1: u64, value2: u64, value3: u64, value4: u64) -> TestResult {
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "value4": value4,
        });

        match self.send_rpc_request("bfcx_getAnonymousMinus", params, 2).await {
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

    pub async fn test_multiply(&self, value1: u64, value2: u64, value3: u64, value4: u64) -> TestResult {
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "value4": value4,
        });

        match self.send_rpc_request("bfcx_getAnonymousMultiply", params, 3).await {
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

    pub async fn test_compare(&self, value1: u64, value2: u64, value3: u64, value4:u64) -> TestResult {
        let params = json!({
            "value1": value1,
            "value2": value2,
            "value3": value3,
            "value4": value4,
        });

        match self.send_rpc_request("bfcx_getAnonymousCompare", params, 4).await {
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


    pub async fn test_split(&self, value:u64) -> TestResult {
        let params = json!({
            "value": value,
        });

        match self.send_rpc_request("bfcx_getAnonymousSplitValue", params, 4).await {
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


}



#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use super::*;
    use clap::Parser;
    use tracing::info;
    use tracing_subscriber::fmt;
    use crate::{AnonymousServer, Args};

    #[tokio::test]
    async fn test_client_creation() {
        let client = crate::client_test::AnonymousClient::new("http://localhost:9010");
        assert_eq!(client.base_url, "http://localhost:9010");
    }


    #[tokio::test]
    async fn test_client_with()-> anyhow::Result<()>{

        let subscriber = fmt::Subscriber::new();
        tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

        //let args = Args::parse();
        let addr: SocketAddr = format!("{}:{}", "127.0.0.1", "9010").parse().unwrap();

        info!("the address is {:?}", addr);
        let server= AnonymousServer::new();
        let server_handle = tokio::spawn(async move {
            if let Err(e) = server.start(addr).await {
                eprintln!("Server error: {:?}", e);
            }
        });

        let client = crate::client_test::AnonymousClient::new("http://localhost:9010");
        let ping_result = client.test_ping().await.response.unwrap();
        let add_result = client.test_add(1, 2, 3, 4).await.response.unwrap();
        let minus_result = client.test_minus(10, 5, 2, 1).await.response.unwrap();
        let multiply_result = client.test_multiply(3, 4, 1,  2).await.response.unwrap();
        let compare_result = client.test_compare(5, 10, 3, 5).await.response.unwrap();
        let split_result = client.test_split(10).await.response.unwrap();

        //todo:split and restore.

        info!("Ping Result: {:?}", ping_result);
        info!("Add Result: {:?}", add_result);
        let result1 = add_result["result"]["result1"].as_u64().unwrap();
        let result2 = add_result["result"]["result2"].as_u64().unwrap();
        info!("Add Result Values: result1 = {}, result2 = {}", result1, result2);


        info!("Minus Result: {:?}", minus_result);
        let result1 = minus_result["result"]["result1"].as_u64().unwrap();
        let result2 = minus_result["result"]["result2"].as_u64().unwrap();
        info!("Minus Result Values: result1 = {}, result2 = {}", result1, result2);


        info!("Multiply Result: {:?}", multiply_result);
        let result1 = multiply_result["result"]["result1"].as_u64().unwrap();
        let result2 = multiply_result["result"]["result2"].as_u64().unwrap();
        info!("Multiply Result Values: result1 = {}, result2 = {}", result1, result2);

        info!("Compare Result: {:?}", compare_result);
        let result = compare_result["result"]["result"].as_u64().unwrap();
        info!("Compare Result Values: result{}", result);


        info!("Split Result: {:?}", split_result);
        let result1 = split_result["result"]["result1"].as_u64().unwrap();
        let result2 = split_result["result"]["result2"].as_u64().unwrap();
        info!("Split Result Values: result1 = {}, result2 = {}", result1, result2);




        Ok(())

    }



}