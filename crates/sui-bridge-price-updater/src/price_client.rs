use std::time::Duration;
use reqwest::Client;
use tracing::error;

pub struct PriceClient {
    client: Client,
}

impl PriceClient { 
    pub fn new() -> Self {
        Self {
            client: 
            Client::builder()
                .connect_timeout(Duration::from_secs(2))
                .timeout(Duration::from_secs(2))
                .build().expect("failed to create http client"),
        }
    }

    pub async fn get_price(&self, symbol: &str) -> anyhow::Result<u128> {
        // 币安API的symbol格式如BTCUSDT
        let symbol = format!("{}USDT", symbol.to_uppercase());
        let url = format!(
            "https://api.binance.com/api/v3/ticker/price?symbol={}",
            symbol
        );
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let price = resp["price"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("价格字段不存在"))?
            .parse::<f64>() // 先解析为f64
            .map(|p| (p * 100_000_000.0).round() as u128) // 放大8位并转为u128
            .map_err(|_| anyhow::anyhow!("价格解析失败"))?;
        error!("price: {}", price);
        Ok(price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_price_btc_usdt() {
        let client = PriceClient::new();
        let price = client.get_price("BTC").await.unwrap();
        assert!(price > 0);
    }
}