use reqwest;
use serde_json::{json, Value};
use std::error::Error;
use tokio;

#[derive(Debug)]
struct TestResult {
    method: String,
    success: bool,
    response: Option<Value>,
    error: Option<String>,
}

struct AnonymousClient {
    base_url: String,
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

    pub async fn test_add(&self, value1: i32, value2: i32, value3: i32, value4: i32) -> TestResult {
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

    pub async fn test_minus(&self, value1: i32, value2: i32, value3: i32, value4: i32) -> TestResult {
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

    pub async fn test_multiply(&self, value1: i32, value2: i32) -> TestResult {
        let params = json!({
            "value1": value1,
            "value2": value2
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

    pub async fn test_compare(&self, value1: i32, value2: i32) -> TestResult {
        let params = json!({
            "value1": value1,
            "value2": value2
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
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_client_creation() {
        let client = AnonymousClient::new("http://localhost:9010");
        assert_eq!(client.base_url, "http://localhost:9010");
    }

    #[tokio::test]
    #[ignore]
    async fn test_all_operations() {
        let client = AnonymousClient::new("http://localhost:9010");
        let add_result = client.test_add(2, 3, 4, 5).await;
        assert!(add_result.success);
        let response = add_result.response.unwrap();
        let result = response["result"]["result1"].as_u64().unwrap();
        assert_eq!(result, 5); // 5 + 3 = 8

        
        let minus_result = client.test_minus(10, 4, 3, 2).await;
        assert!(minus_result.success);
        let response = minus_result.response.unwrap();
        let result = response["result"]["result1"].as_u64().unwrap();
        assert_eq!(result, 6); // 10 - 4 = 6
        println!("Minus result: {}", result);

        let multiply_result = client.test_multiply(3, 4).await;
        assert!(multiply_result.success);
        let response = multiply_result.response.unwrap();
        let result = response["result"]["result1"].as_u64().unwrap();
        assert_eq!(result, 12); // 3 * 4 = 12
        println!("Multiply result: {}", result);

        let compare_result = client.test_compare(7, 5).await;
        assert!(compare_result.success);
        let response = compare_result.response.unwrap();
        let result = response["result"]["result"].as_u64().unwrap();
        assert_eq!(result, 1);
        println!("Compare result: {}", result);

        let ping_result = client.test_ping().await;
        assert!(ping_result.success);
        let response = ping_result.response.unwrap();
        let message = response["result"]["message"].as_str().unwrap();
        assert_eq!(message, "pong");
        println!("Ping result: {}", message);
    }
}