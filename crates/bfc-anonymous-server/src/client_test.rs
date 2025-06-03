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

/// HTTP客户端测试工具
struct AnonymousClient {
    base_url: String,
    client: reqwest::Client,
}

impl AnonymousClient {
    /// 创建新的客户端实例
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// 发送JSON-RPC请求
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

    /// 测试加法操作
    pub async fn test_add(&self, value1: i32, value2: i32) -> TestResult {
        let params = json!({
            "value1": value1,
            "value2": value2
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

    pub async fn test_minus(&self, value1: i32, value2: i32) -> TestResult {
        let params = json!({
            "value1": value1,
            "value2": value2
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

    /// 运行所有测试
    pub async fn run_all_tests(&self) -> Vec<TestResult> {
        let mut results = Vec::new();

        let ping_result = self.test_ping().await;
        if let Some(ref response) = ping_result.response {
            if let Some(result) = response.get("result") {
                if let Some(message) = result.get("message") {
                    println!("Ping result: {}", message.as_str().unwrap_or("unknown"));
                }
            }
            println!("Full response: {}", serde_json::to_string_pretty(response).unwrap_or_default());
        }
        results.push(ping_result);
        
        // 测试加法
        println!("test : 10 + 5");
        let add_result = self.test_add(10, 5).await;
        if let Some(ref response) = add_result.response {
            if let Some(result) = response.get("result") {
                if let Some(calc_result) = result.get("result") {
                    println!("Add result: {}", calc_result.as_u64().unwrap_or(0));
                }
            }
            println!("Full response: {}", serde_json::to_string_pretty(response).unwrap_or_default());
        }
        results.push(add_result);
        println!();

        // 测试减法
        println!(" test: 10 - 3");
        let minus_result = self.test_minus(10, 3).await;
        if let Some(ref response) = minus_result.response {
            if let Some(result) = response.get("result") {
                if let Some(calc_result) = result.get("result") {
                    println!("Minus result: {}", calc_result.as_u64().unwrap_or(0));
                }
            }
            println!("Full response: {}", serde_json::to_string_pretty(response).unwrap_or_default());
        }
        results.push(minus_result);
        println!();

        // 测试乘法
        println!("test: 6 * 7");
        let multiply_result = self.test_multiply(6, 7).await;
        if let Some(ref response) = multiply_result.response {
            if let Some(result) = response.get("result") {
                if let Some(calc_result) = result.get("result") {
                    println!("Multiply result: {}", calc_result.as_u64().unwrap_or(0));
                }
            }
            println!("Full response: {}", serde_json::to_string_pretty(response).unwrap_or_default());
        }
        results.push(multiply_result);
        println!();

        // 测试比较
        println!("test: 8 vs 5");
        let compare_result = self.test_compare(8, 5).await;
        if let Some(ref response) = compare_result.response {
            if let Some(result) = response.get("result") {
                if let Some(calc_result) = result.get("result") {
                    let comparison = calc_result.as_u64().unwrap_or(0);
                    let comparison_text = match comparison {
                        0 => "equal",
                        1 => "first > second",
                        2 => "first < second",
                        _ => "unknown"
                    };
                    println!("Compare result: {} ({})", comparison, comparison_text);
                }
            }
            println!("Full response: {}", serde_json::to_string_pretty(response).unwrap_or_default());
        }
        results.push(compare_result);
        println!();

        // 统计结果
        let success_count = results.iter().filter(|r| r.success).count();
        let total_count = results.len();
        
        println!("{}", "=".repeat(50));
        
    
        results
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 创建客户端实例
    let client = AnonymousClient::new("http://localhost:3030");
    
    // 运行所有测试
    let _results = client.run_all_tests().await;
    
    Ok(())
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
    #[ignore] // 需要服务器运行才能测试
    async fn test_all_operations() {
        let client = AnonymousClient::new("http://localhost:9010");
        
        // 测试加法
        let add_result = client.test_add(5, 3).await;
        assert!(add_result.success);
        let response = add_result.response.unwrap();
        let result = response["result"]["result"].as_u64().unwrap();
        assert_eq!(result, 8); // 5 + 3 = 8
        println!("Add result: {}", result);

        
        // 测试减法
        let minus_result = client.test_minus(10, 4).await;
        assert!(minus_result.success);
        let response = minus_result.response.unwrap();
        let result = response["result"]["result"].as_u64().unwrap();
        assert_eq!(result, 6); // 10 - 4 = 6
        println!("Minus result: {}", result);


        // 测试乘法
        let multiply_result = client.test_multiply(3, 4).await;
        assert!(multiply_result.success);
        let response = multiply_result.response.unwrap();
        let result = response["result"]["result"].as_u64().unwrap();
        assert_eq!(result, 12); // 3 * 4 = 12
        println!("Multiply result: {}", result);


        // 测试比较
        let compare_result = client.test_compare(7, 5).await;
        assert!(compare_result.success);
        let response = compare_result.response.unwrap();
        let result = response["result"]["result"].as_u64().unwrap();
        assert_eq!(result, 1); // 7 > 5, 所以返回1
        println!("Compare result: {}", result);


        // 测试ping
        let ping_result = client.test_ping().await;
        assert!(ping_result.success);
        let response = ping_result.response.unwrap();
        let message = response["result"]["message"].as_str().unwrap();
        assert_eq!(message, "pong");
        println!("Ping result: {}", message);
    }
}