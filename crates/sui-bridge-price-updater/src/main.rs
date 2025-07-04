use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use clap::Parser;
use sui_bridge::metrics::BridgeMetrics;
use sui_bridge::sui_client::SuiBridgeClient;
use tracing::{info, error};
use crate::price_client::PriceClient;
use std::process::Command;
use sui_bridge::types::BridgeActionType;
use sui_types::bridge::BridgeSummary;

mod price_client;

#[derive(Parser, Clone, Debug)]
struct Args {
    #[clap(long, short)]
    cli_path: Option<PathBuf>,
    /// Path to a cli config
    #[clap(long, short)]
    cli_config_path: Option<PathBuf>,
    /// SUI RPC URL
    sui_rpc_url: String,
    /// Chain Type, contains: Main , Testnet, Custom
    #[clap(long, short, default_value = "Mainnet")]
    chain_type: String,
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

    // Get bridge summary
    let bridge_summary = sui_bridge_client.get_bridge_summary().await;
    if bridge_summary.is_err() {
        panic!("get price summary error!");
    }
    let summary = bridge_summary.clone().unwrap();
    // Get last price
    let prices = get_notional_values(&summary);
    let mut nonce = get_nonce(&summary).unwrap_or(0);
    info!("Current update price nonce: {}", nonce);

    let price_client = PriceClient::new();
    let sui_chain_id = get_sui_chain_id(&args.chain_type).await?;

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1800));
    loop {
        interval.tick().await;
        // 4. 获取价格
        for  (token_id, price) in prices.clone() {
            //Skip stable coins
            if token_id == 3 || token_id == 4 || token_id == 5 {
                continue;
            }
            let chain_id = get_chain_id(token_id, &args.chain_type).await?;
            let token = token_id_to_symbol(token_id);
            // 获取最新价格
            let latest_price = price_client.get_price(token).await;
            if let Ok(new_value) = latest_price {
                let diff = if new_value > (price as u128) {
                    new_value - (price as u128)
                } else {
                    (price as u128) - new_value
                };
                // 比较差异是否大于30%
                if diff * 100 > (price as u128) * 30 {
                    // 差异大于30%，更新价格
                    info!("Token {} price larger than 30%: old={}, new={}",token, price, new_value);
                    //update price of sui chain
                    update_price(
                        cli_path.to_str().unwrap(),
                        cli_config_path.to_str().unwrap(),
                        sui_chain_id,
                        0,
                        token_id,
                        new_value
                    ).await.unwrap_or_else(|e| {
                        error!("update sui price: {}", e);
                    });
                    //update price of evm chain
                    if token_id == 2 { // ETH
                        let evm_chain_ids = get_eth_chain_id(&args.chain_type).await?;
                        for evm_chain_id in evm_chain_ids {
                            let result = update_price(
                                cli_path.to_str().unwrap(),
                                cli_config_path.to_str().unwrap(),
                                evm_chain_id,
                                nonce,
                                token_id,
                                new_value
                            ).await;
                            if let Err(e) = result {
                                error!("update eth price error: {}", e);
                            } else {
                                info!("update eth price success, token_id: {}, new_value: {}, nonce: {}", token_id, new_value, nonce);
                                nonce = nonce + 1;
                            }
                        }
                        info!("Token {} is ETH, update price for all EVM chains", token);
                    } else {
                        // For other tokens, update price for single chain
                        if token_id == 1 {
                            info!("Token {} is BTC, no need update price for EVM chains", token);
                            continue; // BTC is not updated on EVM chains
                        }
                        let result = update_price(
                            cli_path.to_str().unwrap(),
                            cli_config_path.to_str().unwrap(),
                            chain_id,
                            nonce,
                            token_id,
                            new_value
                        ).await;
                        if let Err(e) = result {
                            error!("update evm price: {}", e);
                        } else {
                            info!("update evm price success, token_id: {}, new_value: {}, nonce: {}", token_id, new_value, nonce);
                            nonce = nonce + 1;
                        }
                    }
                }else {
                    info!("Token {} price not changed significantly: old={}, new={}", token, price, new_value);
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
    fn get_notional_values(bridge_summary: &BridgeSummary) -> HashMap<u64, u64> {
        bridge_summary
            .treasury
            .id_token_type_map
            .iter()
            .map(|(id, type_name)| {
                bridge_summary
                    .treasury
                    .supported_tokens
                    .iter()
                    .find_map(|(tn, metadata)| {
                        if type_name == tn {
                            Some((*id, metadata.notional_value))
                        } else {
                            None
                        }
                    }).expect("Token type not found in supported tokens")
            })
            .collect()
    }
    fn get_nonce(
        bridge_summary: &BridgeSummary
    ) -> anyhow::Result<u64> {
        for (type_, nonce) in bridge_summary.sequence_nums.clone() {
            if BridgeActionType::try_from(type_).unwrap() == BridgeActionType::AssetPriceUpdate {
                return Ok(nonce);
            }
        }
        anyhow::bail!("AssetPriceUpdate nonce not found");
    }
}

async fn get_chain_id(
    token_id: u64,
    chain_type: &String
) -> anyhow::Result<u64> {
    let chain_id = match chain_type.to_uppercase().as_str() {
        "MAINNET" => match token_id {
            0 => 0, // SUI
            1 => 20, // BTC
            6 => 30, // BNB
            7 => 33, // OP
            8 => 39, // POL
            _ => {
                error!("no need handle token_id: {}", token_id);
                return Err(anyhow::anyhow!("no need handle token_id"));
            },
        },
        "TESTNET" => match token_id {
            0 => 1, // SUI
            1 => 21, // BTC
            6 => 31, // BNB
            7 => 34, // OP
            8 => 40, // POL
            _ => {
                error!("no need handle token_id: {}", token_id);
                return Err(anyhow::anyhow!("no need handle token_id"));
            },
        },
        "CUSTOM" => match token_id {
            0 => 2, // SUI
            6 => 32, // BNB
            7 => 35, // OP
            8 => 41, // POL
            _ => {
                error!("no need handle token_id: {}", token_id);
                return Err(anyhow::anyhow!("no need handle token_id"));
            },
        },
        _ => {
            error!("unknown chain type: {}", chain_type);
            return Err(anyhow::anyhow!("Unknown chain type"));
        }
    };
    Ok(chain_id)
}

async fn get_sui_chain_id(
    chain_type: &String
) -> anyhow::Result<u64> {
    let chain_id = match chain_type.to_uppercase().as_str() {
        "MAINNET" => 0,
        "TESTNET" => 1,
        "CUSTOM" => 2,
        _ => {
            error!("unknown chain type: {}", chain_type);
            return Err(anyhow::anyhow!("Unknown chain type"));
        }
    };
    Ok(chain_id)
}

async fn get_eth_chain_id(
    chain_type: &String
) -> anyhow::Result<Vec<u64>> {
    let chain_ids = match chain_type.to_uppercase().as_str() {
        "MAINNET" => vec![10, 33, 36, 42],
        "TESTNET" => vec![11, 34, 37, 43],
        "CUSTOM" => vec![12, 35, 38, 44],
        _ => {
            error!("unknown chain type: {}", chain_type);
            return Err(anyhow::anyhow!("Unknown chain type"));
        }
    };
    Ok(chain_ids)
}

// ./bin/sui-bridge-cli governance --config-path ./conf/bridge-cli-config.yaml --chain-id 2 update-asset-price --nonce 4  --token-id 43 --new-usd-price 10000000000
async fn update_price(
    cli_path: &str,
    cli_config_path: &str,
    chain_id: u64,
    nonce: u64,
    token_id: u64,
    new_usd_price: u128,
) -> anyhow::Result<()> {
    let status = Command::new(cli_path)
        .arg("governance")
        .arg("--config-path")
        .arg(cli_config_path)
        .arg("--chain-id")
        .arg(chain_id.to_string())
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