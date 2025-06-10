
pub mod schema;
pub mod postgres_manager;

use sui_types::bridge::BridgeChainId;
use ethers::types::Address as EthAddress;
use crate::fast_path::FastPathSelector;
use diesel::data_types::PgTimestamp;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use diesel::dsl::now;
use diesel::dsl::sum;
use diesel::dsl::sql;
use diesel::sql_types::BigInt;
use diesel::upsert::excluded;
use diesel::{ExpressionMethods, QueryDsl, TextExpressionMethods};
use diesel::{OptionalExtension, SelectableHelper};
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::AsyncConnection;
use diesel_async::RunQueryDsl;

use sui_types::base_types::TransactionDigest;
use crate::user_limit::postgres_manager::{get_connection_pool, PgPool};
use crate::user_limit::schema::{bridge_record, limit_config};
use chrono::{Duration, Utc};
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


pub struct UserLimitHandle {
    conn: PgPool,
}

impl UserLimitHandle {
    pub async fn new(database_url: String) -> Self {
        UserLimitHandle { conn : get_connection_pool(database_url).await }
    }

    pub async fn check_user_limit(
        &self,
        chain_id: BridgeChainId,
        eth_address: EthAddress,
        path: FastPathSelector,
        amount: u64,
    ) -> bool {
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
            return false; // Limit exceeded
        }
        true
    }

    pub async fn record_user_limit(
        &self,
        chain_id: BridgeChainId,
        eth_address: EthAddress,
        path: FastPathSelector,
        tx_hash: Vec<u8>,
        amount: u64,
    ) -> Result<(), anyhow::Error> {
        //todo
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

        let conn = &mut self.conn.get().await?;
        let result = bridge_record
            .filter(chain_id.eq(chain_id_))
            .filter(address.eq(eth_address))
            .filter(path.eq(path_))
            .filter(timestamp_ms.ge(since_ms))
            // .select(sum(amount))
            .select(sql::<BigInt>("SUM(amount)::BIGINT"))
            .first::<i64>(conn)
            .await?;
         Ok(result)
        // match result {
        //     Ok(amount) => Ok(amount),
        //     Err(_) => {
        //         error!("No records found for chain_id: {:?}, address: {:?}, path: {:?}", chain_id_, eth_address, path_);
        //         Ok(0)
        //     }, // If no record found, return 0
        // }
    }

    async fn get_limit_config(
        &self,
        chain_id_: i32,
        path_: i32,
    ) -> Result<i64, anyhow::Error> {
        use crate::user_limit::schema::limit_config::dsl::*;
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
}

