use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use clap::Parser;
use sui_bridge::metrics::BridgeMetrics;
use sui_bridge::sui_client::SuiBridgeClient;
use tracing::{info, error};
use crate::price_client::PriceClient;
use std::process::Command;

mod price_client;

#[derive(Parser, Clone, Debug)]
struct Args {
    #[clap(long, short)]
    cli_path: Option<PathBuf>,
    /// Path to a cli config
    #[clap(long, short)]
    cli_config_path: Option<PathBuf>,
    /// SUI RPC URL
    pub sui_rpc_url: String,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 解析配置
    let args = Args::parse();
    // load config
    let cli_path = if let Some(path) = args.cli_path {
        path.join("sui-bridge-cli")
    } else {
        env::current_dir()
            .expect("Couldn't get current path")
            .join("sui-bridge-cli")
    };
    let cli_config_path = if let Some(path) = args.cli_config_path {
        path
    } else {
        env::current_dir()
            .expect("Couldn't get current directory")
            .join("config.yaml")
    };
    
    let bridge_metrics = Arc::new(BridgeMetrics::new_for_testing());
    let sui_bridge_client =
        SuiBridgeClient::new(&args.sui_rpc_url, bridge_metrics.clone()).await?;
    // Get last price
    let prices = sui_bridge_client.get_notional_values().await;
    if prices.is_err() {
        error!("get price error!");
    }

    let price_client = PriceClient::new();
    
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1800));
    loop {
        interval.tick().await;
        // 4. 获取价格
        for  (token_id, price) in prices.clone().unwrap() {
            let token = token_id_to_symbol(token_id);
            // 获取最新价格
            let latest_price = price_client.get_price(token).await;
            if let Ok(new_value) = latest_price {
                if price > 0 {
                    let diff = if new_value > (price as u128) {
                        new_value - (price as u128)
                    } else {
                        (price as u128) - new_value
                    };
                    // 比较差异是否大于30%
                    if diff * 100 > (price as u128) * 30 {
                        // 差异大于30%，更新价格
                        info!("Token {} price larger than 30%: old={}, new={}",token, price, new_value);
                        update_price(
                            cli_path.to_str().unwrap(),
                            cli_config_path.to_str().unwrap(),
                            0, 
                            token_id, 
                            new_value
                        ).await.unwrap_or_else(|e| {
                            error!("update price: {}", e);
                        });
                    }
                }
            } else {
                error!("get {} new price error!", token_id);
            }
        }
        
    }
    
    fn token_id_to_symbol(token_id: u64) -> &'static str {
        match token_id {
            0 => "SUI",
            1 => "BTC",
            2 => "ETH",
            3 => "USDC",
            4 => "USDT",
            5 => "BUSD",
            6 => "BNB",
            7 => "OP",
            8 => "POL",
            _ => {
                error!("unknown token_id: {}", token_id);
                "UNKNOWN"
            },
        }
    }
}


// ./bin/sui-bridge-cli governance --config-path ./conf/bridge-cli-config.yaml --chain-id 2 update-asset-price --nonce 4  --token-id 43 --new-usd-price 10000000000
async fn update_price(
    cli_path: &str,
    cli_config_path: &str,
    nonce: u64,
    token_id: u64,
    new_usd_price: u128,
) -> anyhow::Result<()> {
    let status = Command::new(cli_path)
        .arg("governance")
        .arg("--config-path")
        .arg(cli_config_path)
        .arg("--chain-id")
        .arg("2")
        .arg("update-asset-price")
        .arg("--nonce")
        .arg(nonce.to_string())
        .arg("--token-id")
        .arg(token_id.to_string())
        .arg("--new-usd-price")
        .arg(new_usd_price.to_string())
        .status()?;

    if !status.success() {
        anyhow::bail!("update price execute error, code: {:?}", status.code());
    }
    Ok(())
}