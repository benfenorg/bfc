use std::str::FromStr;
use std::time::Duration;
use jsonrpsee::core::Serialize;
use jsonrpsee::http_client::HttpClient;
use serde::Deserialize;
use tracing::info;
use sui_json_rpc_api::ReadApiClient;
use sui_json_rpc_types::{SuiMoveValue, SuiObjectDataOptions, SuiParsedData, SuiParsedMoveObject};
use sui_json_rpc_types::SuiMoveStruct::{WithFields, WithTypes};
use sui_types::base_types::{ObjectID, SuiAddress};
use crate::errors::IndexerError;
use thiserror::Error;
use chrono::Utc;
use moka::future::Cache;
use crate::benfen;
use crate::models::pending_reward::StakePendingItem;

#[derive(Error, Debug)]
enum MyError {
    #[error("Field not found: {0}")]
    FieldNotFound(String),
    // #[error("Value not struct")]
    // ValueNotStruct,
    #[error("Field not found: {0}")]
    FieldValueNotFound(String),
    #[error("Field not found")]
    LoadConfigFailed,
    // #[error("Field not found")]
    // OtherError,
    #[error("Request error {0}")]
    RequestError(String),
    #[error("response error:{0}")]
    ResponseError(#[from] sui_types::error::SuiObjectResponseError),
    #[error("SDKError :{0}")]
    SDKError(#[from] sui_sdk::error::Error),
}

#[derive(Debug,Clone)]
pub struct PendingReward {
    pub power: u64,
    pub fullnode: HttpClient,
    pub config_cache: Cache<String, MiningConfig>,
    pub mining_contract_address: String,
    pub price_cache:Cache<String, f64>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct MiningConfig {
    pub reward_per_power: u64,
    pub reward_per_second: u64,
    pub decrease_interval: u64,
    pub total_power: u64,
    pub last_reward_ts: u64,
    pub begin_at: u64,
    pub period: u64,
}

impl MiningConfig {
    fn build(reward_per_power: u64, reward_per_second: u64, decrease_interval: u64, total_power: u64, last_reward_ts: u64, begin_at: u64,period:u64) -> Self {
        Self {
            reward_per_power,
            reward_per_second,
            decrease_interval,
            total_power,
            last_reward_ts,
            begin_at,
            period
        }
    }
}

fn read_from_response(name: &str, sui_parsed_data: &SuiParsedMoveObject) -> Result<String, MyError> {
    if let WithFields(btree) = &sui_parsed_data.fields {
        let value = btree.get("value").ok_or(MyError::FieldValueNotFound("value".to_string()))?;
        if let SuiMoveValue::Struct(struct_value) = value {
            if let WithTypes { type_: _, fields } = struct_value {
                let field_value = fields.get(name).ok_or(MyError::FieldNotFound(name.to_string()));
                return Ok(field_value?.to_string());
            }
        }
    };
    Err(MyError::FieldNotFound(name.to_string()).into())
}

impl PendingReward {

    pub async fn build(power: u64, fullnode: HttpClient, config_cache: Cache<String, MiningConfig>,mining_contract_address:String)->Self {
        let price_cache=Cache::builder()
            .max_capacity(3)
            .time_to_live(Duration::from_secs(2*60))
            .build();
         if mining_contract_address.is_empty(){
             panic!("--mining-contract-address is empty");
         }
        let instance = Self{
            power,
            fullnode,
            config_cache,
            mining_contract_address,
            price_cache
        };
        instance.fetch_config_from_full_node().await.expect("TODO: panic message");
        instance
    }
    pub async fn fetch_config_from_full_node(&self) -> Result<MiningConfig, anyhow::Error>{
        let clone= self.config_cache.clone();
        clone.insert("mining_config".to_string(), self.get_config().await?).await;
        Ok(self.config_cache.get("mining_config").await.unwrap())
    }

    pub async fn get_bfc_price(&self) -> f64 {
        let r = self.price_cache.get_with("bfc_price".to_string(), async move {
            benfen::get_bfc_price_in_usd(self.fullnode.clone()).await.unwrap_or(0.08)
        });
        r.await
    }

    pub async fn get_config_from_cache(&self) ->Result<Option<MiningConfig>, anyhow::Error>{
        Ok(self.config_cache.get("mining_config").await)
    }

    async fn get_config(&self) -> Result<MiningConfig, anyhow::Error> {
        if self.mining_contract_address == "" {
            return Ok(MiningConfig::build(0, 3, 0, 1, 0, 0, 2));
        }
        let sui_address = SuiAddress::from_str(&self.mining_contract_address).unwrap();
        let object_id = ObjectID::from(sui_address);
        let response = self.fullnode.get_object(object_id, Some(SuiObjectDataOptions::full_content())).await
            .map_err(|e| {
                MyError::RequestError(format!(
                    "Failed to get object read of name with error {:?}",
                    e
                ))
            })?;
        info!("get_config response: {:?}", response);
        let data = response.object()?;
        if let Some(SuiParsedData::MoveObject(sui_parsed_data)) = &data.content {
            let reward_per_power = read_from_response("reward_per_power", &sui_parsed_data)?.parse::<u64>().unwrap_or_default();
            let reward_per_second = read_from_response("reward_per_second", &sui_parsed_data)?.parse::<u64>().unwrap_or_default();
            let decrease_interval = read_from_response("decrease_interval", &sui_parsed_data)?.parse::<u64>().unwrap_or_default();
            let total_power = read_from_response("total_power", &sui_parsed_data)?.parse::<u64>().unwrap_or_default();
            let last_reward_ts = read_from_response("last_reward_ts", &sui_parsed_data)?.parse::<u64>().unwrap_or_default();
            let begin_at = read_from_response("begin_at", &sui_parsed_data)?.parse::<u64>().unwrap_or_default();
            let period = read_from_response("period", &sui_parsed_data)?.parse::<u64>().unwrap_or_default();
            return Ok(MiningConfig::build(reward_per_power, reward_per_second, decrease_interval, total_power, last_reward_ts, begin_at,period));
        };
        Err(MyError::LoadConfigFailed.into())
    }

    pub async fn pending_reward(&self, stake_item: &StakePendingItem,mining_config: &MiningConfig) -> Result<u64, IndexerError> {
        let reward=self.get_reward(&mining_config).await;
        let reward_per_power = mining_config.reward_per_power+reward/mining_config.total_power;
        Ok(reward_per_power * self.power - stake_item.debt as u64)
    }

    pub async fn get_reward(&self,mining_config: &MiningConfig) -> u64 {
        let end_time=Utc::now().timestamp() as u64;
        let mut  period = mining_config.period;
        if period>=64 {
            return 0;
        }
        let mut last_reward_ts = mining_config.last_reward_ts;
        let mut rewards:u64=0;
        while last_reward_ts<end_time{
            let period_end = mining_config.begin_at + period * mining_config.decrease_interval;
            let time_diff = if end_time < period_end {
                end_time - last_reward_ts
            } else {
                period_end - last_reward_ts
            };

            let decrease_times = 2u64.pow((period - 1) as u32);
            let current_period_reward_per_second = mining_config.reward_per_second / decrease_times;
            if current_period_reward_per_second == 0 {
                break
            };

            rewards = rewards + time_diff * current_period_reward_per_second;
            period = period + 1;
            last_reward_ts = period_end;
        }
        rewards
    }
}
