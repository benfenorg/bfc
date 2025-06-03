use std::collections::BTreeMap;
use sui_types::bridge::{BridgeChainId, TOKEN_ID_BUSD};

use crate::config::BridgeClientConfig;

#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    PartialOrd,
    Ord,
)]
#[repr(u8)]
pub enum FastPathSelector {
    Latest = 0,
    Safe = 1,
    Finalized = 2,
}

impl FastPathSelector {
    pub fn select(token_id:u64,amount:u64,config:&FastPathConfigItem) -> FastPathSelector {
        if TOKEN_ID_BUSD==token_id && amount<= config.threshold_latest {
            FastPathSelector::Latest
        }else if TOKEN_ID_BUSD==token_id && amount<= config.threshold_safe  {
            FastPathSelector::Safe
        }else {
            FastPathSelector::Finalized
        }
    }
}
pub struct FastPathConfig {
    pub items:BTreeMap<BridgeChainId,FastPathConfigItem>,
}

pub struct FastPathConfigItem {
    pub chain_id:BridgeChainId,
    pub enable:bool,
    pub threshold_safe:u64,
    pub threshold_latest:u64,
}
impl Default for FastPathConfigItem {
    fn default() -> Self {
        Self {
            chain_id: BridgeChainId::EthMainnet,
            enable: false,
            threshold_safe: 0,
            threshold_latest: 0,
        }
    }
}
impl Default for &FastPathConfigItem {
    fn default() -> Self {
        &FastPathConfigItem {
            chain_id: BridgeChainId::EthMainnet,
            enable: false,
            threshold_safe: 0,
            threshold_latest: 0,
        }
    }
}




impl FastPathConfig{
    pub fn init_for_testing()-> Self{
        let mut items = BTreeMap::new();
         let busd_decimal = 1_000_000_000;
        // 初始化BSC链的配置
        items.insert(
            BridgeChainId::BscCustom,
            FastPathConfigItem {
                chain_id: BridgeChainId::BscCustom,
                enable: true,
                threshold_safe: 1000 * busd_decimal,
                threshold_latest: 100 * busd_decimal,
            },
        );

        // 初始化ETH链的配置
        items.insert(
            BridgeChainId::EthCustom,
            FastPathConfigItem {
                chain_id: BridgeChainId::EthCustom,
                enable: true,
                threshold_safe: 1000 * busd_decimal,
                threshold_latest: 100 * busd_decimal,
            },
        );

        FastPathConfig { items }
    }

    pub fn init_from_client_config(client_config: &BridgeClientConfig) -> Self {
        let mut items = BTreeMap::new();
        // 初始化 ETH链的配置
        // 从配置中读取快速通道的阈值设置
        items.insert(
            BridgeChainId::EthMainnet,
            FastPathConfigItem {
                chain_id: BridgeChainId::EthMainnet,
                enable: client_config.eth_enable_quick_settle,
                threshold_safe: client_config.eth_safe_quick_settle_threshold.unwrap_or(0),
                threshold_latest: client_config.eth_latest_quick_settle_threshold.unwrap_or(0),
            },
        );
        items.insert(
            BridgeChainId::EthSepolia,
            FastPathConfigItem {
                chain_id: BridgeChainId::EthSepolia,
                enable: client_config.eth_enable_quick_settle,
                threshold_safe: client_config.eth_safe_quick_settle_threshold.unwrap_or(0),
                threshold_latest: client_config.eth_latest_quick_settle_threshold.unwrap_or(0),
            },
        );
        items.insert(
            BridgeChainId::EthCustom,
            FastPathConfigItem {
                chain_id: BridgeChainId::EthCustom,
                enable: client_config.eth_enable_quick_settle,
                threshold_safe: client_config.eth_safe_quick_settle_threshold.unwrap_or(0),
                threshold_latest: client_config.eth_latest_quick_settle_threshold.unwrap_or(0),
            },
        );
        // 遍历配置中的所有链
        for chain_config in &client_config.evm_client_configs {
            // 从配置中读取快速通道的阈值设置
            items.insert(
                chain_config.0.clone(),
                FastPathConfigItem {
                    chain_id: chain_config.0.clone(),
                    enable: chain_config.1.enable_quick_settle,
                    threshold_safe: chain_config.1.safe_quick_settle_threshold.unwrap_or(0),
                    threshold_latest: chain_config.1.latest_quick_settle_threshold.unwrap_or(0),
                },
            );
        }

        FastPathConfig { items }
    }
}