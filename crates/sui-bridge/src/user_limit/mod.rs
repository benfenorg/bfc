
pub mod schema;
pub mod postgres_manager;

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;
use sui_types::bridge::BridgeChainId;
use ethers::types::Address as EthAddress;
use crate::fast_path::FastPathSelector;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use diesel::dsl::sql;
use diesel::sql_types::BigInt;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use crate::user_limit::postgres_manager::{get_connection_pool, PgPool};
use crate::user_limit::schema::{bridge_record, limit_config};
use chrono::{Duration, Utc};
use fastcrypto::encoding::{Hex, Encoding};
use once_cell::sync::Lazy;
use tracing::{error, info};

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug)]
#[diesel(table_name = bridge_record, primary_key(chain_id, address, path))]
pub struct BridgeRecord {
    pub chain_id: i32,
    pub address: Vec<u8>,
    pub path: i32,
    pub amount: i64,
    pub tx_hash: Vec<u8>,
    pub timestamp_ms: i64,
}

#[derive(Queryable, Selectable, Insertable, Identifiable, Debug)]
#[diesel(table_name = limit_config, primary_key(chain_id, path))]
pub struct LimitConfig {
    pub chain_id: i32,
    pub path: i32,
    pub limits: i64,
}

#[allow(unused)]
struct RecordCache {
    total: i64,
    created_at: Instant,
}

#[allow(unused)]
// Cache for user limit records
static RECORD_CACHE: Lazy<Mutex<HashMap<String, RecordCache>>> = Lazy::new(|| Mutex::new(HashMap::new()));
// Cache for limit configuration
static LIMIT_CONF_CACHE: Lazy<Mutex<HashMap<String, i64>>> = Lazy::new(|| Mutex::new(HashMap::new()));
#[allow(unused)]
const RECORD_VALIDITY: std::time::Duration = std::time::Duration::from_secs(24 * 60 * 60);


pub struct UserLimitHandle {
    conn: PgPool,
}

impl UserLimitHandle {
    pub async fn new(database_url: String) -> Self {
        UserLimitHandle {
            conn : get_connection_pool(database_url).await
        }
    }

    pub async fn check_user_limit(
        &self,
        chain_id: BridgeChainId,
        eth_address: EthAddress,
        path: FastPathSelector,
        amount: u64,
    ) -> bool {
        // Check if the limit configuration is loaded
        if let Err(e) = self.load_limit_config().await {
            error!("Failed to load limit config: {:?}", e);
            return false; // If there's an error, we assume the limit is not exceeded
        }
        let bridge_amount = self.get_bridge_amounts(chain_id as i32, eth_address.as_bytes(), path as i32).await;
        if let Err(e) = bridge_amount {
            error!("Failed to get user limit amounts: {:?}", e);
            return false; // If there's an error, we assume the limit is not exceeded
        }
        let bridge_amount = bridge_amount.unwrap_or(0);
        let limit_config = self.get_limit_config(chain_id as i32, path as i32).await;
        if let Err(e) = limit_config {
            error!("Failed to get limit config: {:?}", e);
            return false; // If there's an error, we assume the limit is not exceeded
        }
        let limit_config = limit_config.unwrap_or(0);
        info!(
            "Checking user limit: chain_id: {:?}, eth_address: {:?}, path: {:?}, amount: {}, bridge_amount: {}, limit_config: {}",
            chain_id, eth_address, path, amount, bridge_amount, limit_config
        );
        // Check if the total amount (bridge_amount + amount) exceeds the limit
        if bridge_amount + amount as i64 > limit_config {
            info!(
                "User limit exceeded: chain_id: {:?}, eth_address: {:?}, path: {:?}, amount: {}, bridge_amount: {}, limit_config: {}",
                chain_id, eth_address, path, amount, bridge_amount, limit_config
            );
            return true; // Limit exceeded
        }
        false
    }

    pub async fn record_user_limit(
        &self,
        chain_id: BridgeChainId,
        eth_address: EthAddress,
        path: FastPathSelector,
        tx_hash: Vec<u8>,
        amount: u64,
    ) -> Result<(), anyhow::Error> {
        use crate::user_limit::schema::bridge_record;
        use diesel::insert_into;
        let now_ms = Utc::now().timestamp_millis();

        let new_record = BridgeRecord {
            chain_id: chain_id as i32,
            address: eth_address.as_bytes().to_vec(),
            path: path as i32,
            amount: amount as i64,
            tx_hash,
            timestamp_ms: now_ms,
        };
        let conn = &mut self.conn.get().await?;
        let result = insert_into(bridge_record::table)
            .values(&new_record)
            .execute(conn)
            .await?;
        if result == 0 {
            return Err(anyhow::anyhow!("Failed to insert user limit record, no rows affected"));
        }
        // Update the cache
        {
            let mut cache = RECORD_CACHE.lock().unwrap();
            let key = format!("{}_{:x}_{}", chain_id as i32, eth_address, path as i32);
            let entry = cache.entry(key).or_insert(RecordCache {
                total: 0,
                created_at: Instant::now(),
            });
            entry.total += amount as i64;
            // entry.created_at = Instant::now();
            info!("Cache updated for chain_id: {}, address: {:x}, path: {}, new total: {}",
                chain_id as i32, eth_address, path as i32, entry.total
            );
        }
        info!("User limit record inserted: chain_id: {:?}, eth_address: {:?}, path: {:?}, amount: {}",
            chain_id, eth_address, path, amount
        );
        Ok(())
    }

     async fn get_bridge_amounts(
        &self,
        chain_id_: i32,
        eth_address: &[u8],
        path_: i32,
    ) -> Result<i64, anyhow::Error> {
         use crate::user_limit::schema::bridge_record::dsl::*;
         let now_ms = Utc::now().timestamp_millis();
         let since_ms = now_ms - Duration::hours(24).num_milliseconds();

         {
             let address_ = Hex::encode(eth_address);
             let mut cache_map = RECORD_CACHE.lock().unwrap();
             let key = format!("{}_{}_{}", chain_id_, address_, path_);
             let mut is_expired = false;
             cache_map.get(&key)
                 .and_then(|cache_data| {
                     if cache_data.created_at.elapsed() < RECORD_VALIDITY {
                         Some(cache_data.total)
                     } else {
                         is_expired = true;
                         None
                     }
                 })
                 .map(|total| {
                     info!("Using cached record for chain_id: {}, address: {}, path: {}", chain_id_, address_, path_);
                     return Ok::<i64, anyhow::Error>(total);
                 });
                if is_expired {
                    cache_map.remove(&key);
                    info!("Cache expired and deleted for chain_id: {}, address: {}, path: {}", chain_id_, address_, path_);
                }
         }
        let conn = &mut self.conn.get().await?;
        let result = bridge_record
            .filter(chain_id.eq(chain_id_))
            .filter(address.eq(eth_address))
            .filter(path.eq(path_))
            .filter(timestamp_ms.ge(since_ms))
            .select(sql::<BigInt>("COALESCE(SUM(amount), 0)::BIGINT"))
            .first::<i64>(conn)
            .await?;
         Ok(result)
    }

    async fn get_limit_config(
        &self,
        chain_id_: i32,
        path_: i32,
    ) -> Result<i64, anyhow::Error> {
        use crate::user_limit::schema::limit_config::dsl::*;
        {
            let cache_map = LIMIT_CONF_CACHE.lock().unwrap();
            let key = format!("{}_{}", chain_id_, path_);
            if let Some(&limit) = cache_map.get(&key) {
                info!("Using cached limit config for chain_id: {}, path: {}", chain_id_, path_);
                return Ok(limit);
            }
        }

        let conn = &mut self.conn.get().await?;
        let result = limit_config
            .filter(chain_id.eq(chain_id_))
            .filter(path.eq(path_))
            .select(limits)
            .first::<i64>(conn)
            .await;

        match result {
            Ok(limit) => Ok(limit),
            Err(_) => Err(anyhow::anyhow!("Limit config not found for chain_id: {:?}, path: {:?}", chain_id_, path_)),
        }
    }

    async fn load_limit_config(&self) -> Result<(), anyhow::Error> {
        use crate::user_limit::schema::limit_config::dsl::*;
        {
            let cache_map = LIMIT_CONF_CACHE.lock().unwrap();
            if !cache_map.is_empty() {
                info!("Limit config cache is already loaded, skipping database query");
                return Ok(()); // Cache already loaded
            }
        }
        let conn = &mut self.conn.get().await?;
        let limit_configs = limit_config
            .load::<LimitConfig>(conn)
            .await?;
        {
            let mut cache_map = LIMIT_CONF_CACHE.lock().unwrap();
            for config in limit_configs {
                let key = format!("{}_{}", config.chain_id, config.path);
                cache_map.insert(key, config.limits);
                info!("Loaded limit config: chain_id: {}, path: {}, limits: {}", config.chain_id, config.path, config.limits);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use crate::fast_path::FastPathSelector;

    #[tokio::test]
    async fn test_user_limit() {
        let user_limit_handle = UserLimitHandle::new("postgres://user_limit:limit@localhost:5432/user_limit".to_string()).await;

        let chain_id = BridgeChainId::BtcTestnet;
        let eth_address = EthAddress::from_str("0x2e6547f8a54d261a4a3e508c4b321b84c0aee44e").unwrap();
        let path = FastPathSelector::Latest;
        let amount = 99;

        // Check not exist user limit
        let is_within_limit = user_limit_handle.check_user_limit(chain_id, eth_address, path, amount).await;
        assert!(!is_within_limit, "User limit check should ok");

        // Record user limit
        let tx_hash = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let record_result = user_limit_handle.record_user_limit(chain_id, eth_address, path, tx_hash.clone(), amount).await;
        assert!(record_result.is_ok(), "Failed to record user limit");

        // Check again after recording
        let is_within_limit_after_recording = user_limit_handle.check_user_limit(chain_id, eth_address, path, amount).await;
        assert!(is_within_limit_after_recording, "User limit check failed after recording");
    }

    #[tokio::test]
    async fn test_load_limit_config() {
        let user_limit_handle = UserLimitHandle::new("postgres://user_limit:limit@localhost:5432/user_limit".to_string()).await;

        // Load limit config
        let load_result = user_limit_handle.load_limit_config().await;
        assert!(load_result.is_ok(), "Failed to load limit config");

        // Check if the cache is populated
        let cache_map = LIMIT_CONF_CACHE.lock().unwrap();
        assert!(!cache_map.is_empty(), "Limit config cache should not be empty");

        // Check a specific chain_id and path
        let key = format!("{}_{}", BridgeChainId::BtcTestnet as i32, FastPathSelector::Latest as i32);
        assert!(cache_map.contains_key(&key), "Cache should contain key: {}", key);
    }

    #[tokio::test]
    async fn test_limit_by_cached() {
        let user_limit_handle = UserLimitHandle::new("postgres://user_limit:limit@localhost:5432/user_limit".to_string()).await;

        let chain_id = BridgeChainId::BtcTestnet;
        let eth_address = EthAddress::from_str("0x2e6547f8a54d261a4a3e508c4b321b84c0aee45f").unwrap();
        let path = FastPathSelector::Latest;
        let amount = 10;

        // Check not exist user limit
        let is_within_limit = user_limit_handle.check_user_limit(chain_id, eth_address, path, amount).await;
        assert!(!is_within_limit, "User limit check should ok");

        // Record user limit
        let tx_hash = vec![11, 22, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let record_result = user_limit_handle.record_user_limit(chain_id, eth_address.clone(), path, tx_hash.clone(), amount).await;
        assert!(record_result.is_ok(), "Failed to record user limit 1");

        let tx_hash = vec![12, 22, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let record_result = user_limit_handle.record_user_limit(chain_id, eth_address.clone(), path, tx_hash.clone(), amount).await;
        assert!(record_result.is_ok(), "Failed to record user limit 2");

        let tx_hash = vec![13, 22, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let record_result = user_limit_handle.record_user_limit(chain_id, eth_address.clone(), path, tx_hash.clone(), amount).await;
        assert!(record_result.is_ok(), "Failed to record user limit 3");

        // Check cached record
        let bridge_amount = user_limit_handle.get_bridge_amounts(chain_id as i32, eth_address.as_bytes(), path as i32).await;
        assert!(bridge_amount.is_ok(), "Failed to get bridge amounts");
        assert_eq!(bridge_amount.unwrap(), (amount *3 ) as i64);

        // Check again after recording
        let is_within_limit_after_recording = user_limit_handle.check_user_limit(chain_id, eth_address.clone(), path, amount).await;
        assert!(!is_within_limit_after_recording, "User limit check failed after recording");

    }
}

