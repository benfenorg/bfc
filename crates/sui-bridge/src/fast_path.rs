use std::collections::BTreeMap;
use sui_types::bridge::{BridgeChainId, TOKEN_ID_BUSD, TOKEN_ID_USDC, TOKEN_ID_USDT};
use strum_macros::Display;
use tracing::info;
use crate::{config::BridgeClientConfig, types::BridgeAction};

#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    PartialOrd,
    Ord,
    Display,
    serde::Serialize,
    serde::Deserialize,
)]
#[repr(u8)]
pub enum FastPathSelector {
    Latest = 0,
    Safe = 1,
    Finalized = 2,
}

impl FastPathSelector {
    pub fn select(token_id:u64,amount:u64,config:&FastPathConfigItem) -> FastPathSelector {
        let amount=Self::get_sui_adjusted_amount(token_id,amount,config.chain_id);
        let is_stable_coin = Self::is_stable_coin(token_id);
        if is_stable_coin && amount<= config.threshold_latest && config.enable_latest {
            FastPathSelector::Latest
        }else if is_stable_coin && amount<= config.threshold_safe && config.enable_safe {
            FastPathSelector::Safe
        }else {
            FastPathSelector::Finalized
        }
    }

    pub fn select_by_action(bridge_action:BridgeAction,config:&FastPathConfig) -> FastPathSelector {
        match bridge_action {
            BridgeAction::EthToSuiBridgeAction(action) => {
                let config_item = config.items.get(&action.eth_bridge_event.eth_chain_id);
                info!("bbking config_item: {:?} config: {:?}", config_item,config.items);
                if config_item.is_none() {
                    return FastPathSelector::Finalized;
                }
                let config_item = config_item.unwrap();
                let is_stable_coin = Self::is_stable_coin(action.eth_bridge_event.token_id);
                let sui_adjusted_amount=Self::get_sui_adjusted_amount(action.eth_bridge_event.token_id,action.eth_bridge_event.sui_adjusted_amount,action.eth_bridge_event.eth_chain_id);
                if is_stable_coin && sui_adjusted_amount<= config_item.threshold_latest && config_item.enable_latest {
                    FastPathSelector::Latest
                }else if is_stable_coin && sui_adjusted_amount<= config_item.threshold_safe && config_item.enable_safe {
                    FastPathSelector::Safe
                }else { 
                    FastPathSelector::Finalized
                }
            }
            BridgeAction::EthSendBackBridgeAction(action) => {
                let config_item = config.items.get(&action.sui_bridge_event.eth_chain_id);
                if config_item.is_none() {
                    return FastPathSelector::Finalized;
                }
                let config_item = config_item.unwrap();
                let is_stable_coin = Self::is_stable_coin(action.sui_bridge_event.token_id);
                let amount_sui_adjusted=Self::get_sui_adjusted_amount(action.sui_bridge_event.token_id,action.sui_bridge_event.amount_sui_adjusted,action.sui_bridge_event.eth_chain_id);
                if is_stable_coin && amount_sui_adjusted<= config_item.threshold_latest && config_item.enable_latest {
                    FastPathSelector::Latest
                }else if is_stable_coin && amount_sui_adjusted<= config_item.threshold_safe && config_item.enable_safe {
                    FastPathSelector::Safe
                }else { 
                    FastPathSelector::Finalized
                }
            }
            _ => FastPathSelector::Finalized,
        }
    }

    pub fn is_stable_coin(token_id:u64) -> bool {
        TOKEN_ID_USDT==token_id || TOKEN_ID_USDC==token_id || TOKEN_ID_BUSD==token_id
    }

    pub fn get_sui_adjusted_amount(token_id:u64,amount:u64,chain_id:BridgeChainId) -> u64 {
        let need_adjust = (token_id == TOKEN_ID_USDC || token_id == TOKEN_ID_USDT) && chain_id.is_eth_chain();
        if need_adjust {
            amount.checked_mul(1000).unwrap_or(amount)
        } else {
            amount
        }
    }

    pub fn is_finalized(self) -> bool {
        match self {
            FastPathSelector::Latest | FastPathSelector::Safe => false,
            FastPathSelector::Finalized => true,
        }
    }
}
#[derive(Clone)]
pub struct FastPathConfig {
    pub items:BTreeMap<BridgeChainId,FastPathConfigItem>,
}

impl Default for FastPathConfig {
    fn default() -> Self {
        Self {
            items: BTreeMap::new()
        }
    }
}


#[derive(Clone,Debug)]
pub struct FastPathConfigItem {
    pub chain_id:BridgeChainId,
    pub enable_latest:bool,
    pub enable_safe:bool,
    pub threshold_safe:u64,
    pub threshold_latest:u64,
}
impl Default for FastPathConfigItem {
    fn default() -> Self {
        Self {
            chain_id: BridgeChainId::EthMainnet,
            enable_latest: false,
            enable_safe: false,
            threshold_safe: 0,
            threshold_latest: 0,
        }
    }
}
impl Default for &FastPathConfigItem {
    fn default() -> Self {
        &FastPathConfigItem {
            chain_id: BridgeChainId::EthMainnet,
            enable_latest: false,
            enable_safe: false,
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
                enable_latest: true,
                enable_safe: true,
                threshold_safe: 1000 * busd_decimal,
                threshold_latest: 100 * busd_decimal,
            },
        );

        // 初始化ETH链的配置
        items.insert(
            BridgeChainId::EthCustom,
            FastPathConfigItem {
                chain_id: BridgeChainId::EthCustom,
                enable_latest: true,
                enable_safe: true,
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
                enable_latest: client_config.eth_enable_fast_path_latest,
                enable_safe: client_config.eth_enable_fast_path_safe,
                threshold_safe: client_config.eth_safe_fast_path_threshold.unwrap_or(0),
                threshold_latest: client_config.eth_latest_fast_path_threshold.unwrap_or(0),
            },
        );
        items.insert(
            BridgeChainId::EthSepolia,
            FastPathConfigItem {
                chain_id: BridgeChainId::EthSepolia,
                enable_latest: client_config.eth_enable_fast_path_latest,
                enable_safe: client_config.eth_enable_fast_path_safe,
                threshold_safe: client_config.eth_safe_fast_path_threshold.unwrap_or(0),
                threshold_latest: client_config.eth_latest_fast_path_threshold.unwrap_or(0),
            },
        );
        items.insert(
            BridgeChainId::EthCustom,
            FastPathConfigItem {
                chain_id: BridgeChainId::EthCustom,
                enable_latest: client_config.eth_enable_fast_path_latest,
                enable_safe: client_config.eth_enable_fast_path_safe,
                threshold_safe: client_config.eth_safe_fast_path_threshold.unwrap_or(0),
                threshold_latest: client_config.eth_latest_fast_path_threshold.unwrap_or(0),
            },
        );
        // 遍历配置中的所有链
        for chain_config in &client_config.evm_client_configs {
            // 从配置中读取快速通道的阈值设置
            items.insert(
                chain_config.0.clone(),
                FastPathConfigItem {
                    chain_id: chain_config.0.clone(),
                    enable_latest: chain_config.1.enable_fast_path_latest,
                    enable_safe: chain_config.1.enable_fast_path_safe,
                    threshold_safe: chain_config.1.safe_fast_path_threshold.unwrap_or(0),
                    threshold_latest: chain_config.1.latest_fast_path_threshold.unwrap_or(0),
                },
            );
        }

        FastPathConfig { items }
    }
}