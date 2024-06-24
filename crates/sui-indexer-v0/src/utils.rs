// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::ops::Neg;

use anyhow::anyhow;
use diesel::migration::MigrationSource;
use diesel::{PgConnection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use jsonrpsee::http_client::HttpClient;
use sui_types::digests::ObjectDigest;
use sui_types::effects::ObjectRemoveKind;
use tracing::info;

use sui_json_rpc::api::ReadApiClient;
use sui_json_rpc::{get_balance_changes, ObjectProvider};
use sui_json_rpc_types::{SuiGasCostSummary, SuiTransactionBlockResponseOptions};
use sui_json_rpc_types::{
    BalanceChange, SuiExecutionStatus, SuiTransactionBlockEffects, SuiTransactionBlockEffectsAPI,
};
use sui_json_rpc_types::{ObjectChange, OwnedObjectRef, SuiObjectRef};
use sui_types::base_types::TransactionDigest;
use sui_types::base_types::{ObjectID, ObjectRef, SequenceNumber, SuiAddress};
use sui_types::gas_coin::GAS;
use sui_types::object::Owner;
use sui_types::storage::WriteKind;

use crate::errors::IndexerError;
use crate::types::CheckpointTransactionBlockResponse;
use crate::PgPoolConnection;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Resets the database by reverting all migrations and reapplying them.
///
/// If `drop_all` is set to `true`, the function will drop all tables in the database before
/// resetting the migrations. This option is destructive and will result in the loss of all
/// data in the tables. Use with caution, especially in production environments.
pub fn reset_database(conn: &mut PgPoolConnection, drop_all: bool) -> Result<(), anyhow::Error> {
    info!("Resetting database ...");
    if drop_all {
        drop_all_tables(conn)
            .map_err(|e| anyhow!("Encountering error when dropping all tables {e}"))?;
    } else {
        conn.revert_all_migrations(MIGRATIONS)
            .map_err(|e| anyhow!("Error reverting all migrations {e}"))?;
    }

    conn.run_migrations(&MIGRATIONS.migrations().unwrap())
        .map_err(|e| anyhow!("Failed to run migrations {e}"))?;
    info!("Reset database complete.");
    Ok(())
}

pub fn run_migrations(conn: &mut PgPoolConnection) -> Result<(), anyhow::Error> {
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow!("Failed to run migrations {e}"))?;
    Ok(())
}

pub fn drop_all_tables(conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
    info!("Dropping all tables in the database");
    let table_names: Vec<String> = diesel::dsl::sql::<diesel::sql_types::Text>(
        "
        SELECT tablename FROM pg_tables WHERE schemaname = 'public'
    ",
    )
    .load(conn)?;

    for table_name in table_names {
        let drop_table_query = format!("DROP TABLE IF EXISTS {} CASCADE", table_name);
        diesel::sql_query(drop_table_query).execute(conn)?;
    }

    // Recreate the __diesel_schema_migrations table
    diesel::sql_query(
        "
        CREATE TABLE __diesel_schema_migrations (
            version VARCHAR(50) PRIMARY KEY,
            run_on TIMESTAMP NOT NULL DEFAULT NOW()
        )
    ",
    )
    .execute(conn)?;
    info!("Dropped all tables in the database");
    Ok(())
}

pub async fn multi_get_full_transactions(
    http_client: HttpClient,
    digests: Vec<TransactionDigest>,
) -> Result<Vec<CheckpointTransactionBlockResponse>, IndexerError> {
    let sui_transactions = http_client
        .multi_get_transaction_blocks(
            digests.clone(),
            // MUSTFIX(gegaowp): avoid double fetching both input and raw_input
            Some(
                SuiTransactionBlockResponseOptions::new()
                    .with_input()
                    .with_effects()
                    .with_events()
                    .with_balance_changes()
                    .with_raw_input(),
            ),
        )
        .await
        .map_err(|e| {
            IndexerError::FullNodeReadingError(format!(
                "Failed to get transactions {:?} with error: {:?}",
                digests.clone(),
                e
            ))
        })?;
    let sui_full_transactions: Vec<CheckpointTransactionBlockResponse> = sui_transactions
        .into_iter()
        .map(CheckpointTransactionBlockResponse::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| {
            IndexerError::UnexpectedFullnodeResponseError(format!(
                "Unexpected None value in SuiTransactionBlockFullResponse with error {:?}",
                e
            ))
        })?;
    Ok(sui_full_transactions)
}

pub async fn get_balance_changes_from_effect<P: ObjectProvider<Error = E>, E>(
    object_provider: &P,
    effects: &SuiTransactionBlockEffects,
) -> Result<Vec<BalanceChange>, E> {
    let gas_owner = effects.gas_object().owner;
    // Only charge gas when tx fails, skip all object parsing
    let gas_cost_summary: SuiGasCostSummary = effects.gas_cost_summary().clone();
    if effects.status() != &SuiExecutionStatus::Success {
        return Ok(vec![BalanceChange {
            owner: gas_owner,
            coin_type: GAS::type_tag(),
            amount: gas_cost_summary.net_gas_usage().neg() as i128,
        }]);
    }

    let all_mutated: Vec<(ObjectID, SequenceNumber, Option<ObjectDigest>)> = effects
        .all_changed_objects()
        .into_iter()
        .map(|(owner_obj_ref, _)| {
            (
                owner_obj_ref.reference.object_id,
                owner_obj_ref.reference.version,
                Some(owner_obj_ref.reference.digest),
            )
        })
        .collect();
    // TODO: thread through input object digests here instead of passing None
    let modified_at_versions: Vec<(ObjectID, SequenceNumber, Option<ObjectDigest>)> = effects
        .modified_at_versions()
        .into_iter()
        .map(|(id, version)| (id, version, None))
        .collect();
    get_balance_changes(object_provider, &modified_at_versions, &all_mutated).await
}

pub async fn get_object_changes<P: ObjectProvider<Error = E>, E>(
    object_provider: &P,
    sender: SuiAddress,
    modified_at_versions: Vec<(ObjectID, SequenceNumber)>,
    all_changed_objects: Vec<(&OwnedObjectRef, WriteKind)>,
    all_deleted: Vec<(&SuiObjectRef, ObjectRemoveKind)>,
) -> Result<Vec<ObjectChange>, E> {
    let all_changed: Vec<(ObjectRef, Owner, WriteKind)> = all_changed_objects
        .into_iter()
        .map(|(obj_owner_ref, write_kind)| {
            (
                (
                    obj_owner_ref.reference.object_id,
                    obj_owner_ref.reference.version,
                    obj_owner_ref.reference.digest,
                ),
                obj_owner_ref.owner,
                write_kind,
            )
        })
        .collect();
    let all_changed_objects = all_changed
        .into_iter()
        .map(|(obj_ref, owner, write_kind)| (obj_ref, owner, write_kind))
        .collect();

    let all_deleted_objects: Vec<(ObjectRef, ObjectRemoveKind)> = all_deleted
        .into_iter()
        .map(|(obj_ref, delete_kind)| {
            (
                (obj_ref.object_id, obj_ref.version, obj_ref.digest),
                delete_kind,
            )
        })
        .collect();

    sui_json_rpc::get_object_changes(
        object_provider,
        sender,
        modified_at_versions,
        all_changed_objects,
        all_deleted_objects,
    )
    .await
}

pub mod validator_stake {
    use std::collections::HashMap;

    use jsonrpsee::http_client::HttpClient;
    use sui_types::{
        base_types::SuiAddress,
        collection_types::VecMap,
        sui_system_state::{
            sui_system_state_inner_v1::{StakingPoolV1, ValidatorV1},
            PoolTokenExchangeRate, SuiSystemState,
        },
        TypeTag,
    };
    use tracing::warn;

    use crate::errors::IndexerError;

    use super::stable_pool::{self, parse_pool_key, StablePoolSummary};

    #[derive(Debug, Clone)]
    pub struct ValidatorStake {
        pub address: SuiAddress,
        pub staking_pool: StakingPoolV1,
        pub current_exchange_rates: PoolTokenExchangeRate,
        pub stable_pool: Option<stable_pool::StablePool>,
    }

    pub struct ValidatorSet {
        epoch: u64,
        active_validators: Vec<ValidatorV1>,
        last_epoch_stable_rate: VecMap<String, u64>,
    }

    impl ValidatorSet {
        pub fn from_system_state(state: &SuiSystemState) -> Self {
            let (epoch, validators) = match state {
                SuiSystemState::V1(v1) => (v1.epoch, &v1.validators),
                SuiSystemState::V2(v2) => (v2.epoch, &v2.validators),
                #[cfg(msim)]
                _ => {
                    return Self {
                        epoch: 0u64,
                        active_validators: vec![],
                        last_epoch_stable_rate: VecMap { contents: vec![] },
                    };
                }
            };
            ValidatorSet {
                epoch,
                active_validators: validators.active_validators.clone(),
                last_epoch_stable_rate: validators.last_epoch_stable_rate.clone(),
            }
        }

        pub async fn parse_stake(
            &self,
            http_client: HttpClient,
        ) -> Result<Vec<ValidatorStake>, IndexerError> {
            let mut reader = stable_pool::StablePoolBulkReader::new();
            self.active_validators.iter().for_each(|x| {
                let pool_id = x.stable_pools.id.clone();
                let pool_keys = get_stable_pool_keys();
                let summary = x.clone().into_sui_validator_summary();
                reader.add(summary.sui_address, pool_id, pool_keys);
            });
            let mut stable_pools = reader.bulk_read(http_client.clone(), self.epoch).await?;
            let mut results = vec![];
            for x in self.active_validators.iter() {
                let summary = x.clone().into_sui_validator_summary();
                let current_exchange_rates = match stable_pool::get_pool_exchange_rate(
                    http_client.clone(),
                    x.staking_pool.exchange_rates.id,
                    self.epoch,
                )
                .await
                {
                    Ok(rates) => rates,
                    Err(err) => {
                        warn!(
                            "Failed to read exchange rates of staking pool {:?} at epoch {} with error {:?}",
                            x.staking_pool.exchange_rates.id, self.epoch, err);
                        PoolTokenExchangeRate::default()
                    }
                };
                results.push(ValidatorStake {
                    stable_pool: stable_pools.remove(&summary.sui_address),
                    address: summary.sui_address,
                    staking_pool: x.staking_pool.clone(),
                    current_exchange_rates,
                })
            }
            Ok(results)
        }

        pub fn get_stable_rates(&self) -> HashMap<TypeTag, u64> {
            self.last_epoch_stable_rate
                .contents
                .iter()
                .map(|x| (parse_pool_key(&x.key).unwrap_or(TypeTag::Bool), x.value))
                .collect()
        }
    }

    pub fn get_stable_pool_keys() -> Vec<String> {
        // https://github.com/hellokittyboy-code/obc/blob/88585bcd2406962ba7cefe46944b0a785cd32a67/crates/sui-framework/packages/sui-system/sources/validator.move#L18-L34
        let stable_coins = vec![
            "bfc_system::bars::BARS",
            "bfc_system::baud::BAUD",
            "bfc_system::bbrl::BBRL",
            "bfc_system::bcad::BCAD",
            "bfc_system::beur::BEUR",
            "bfc_system::bgbp::BGBP",
            "bfc_system::bidr::BIDR",
            "bfc_system::binr::BINR",
            "bfc_system::bjpy::BJPY",
            "bfc_system::bkrw::BKRW",
            "bfc_system::bmxn::BMXN",
            "bfc_system::brub::BRUB",
            "bfc_system::bsar::BSAR",
            "bfc_system::btry::BTRY",
            "bfc_system::busd::BUSD",
            "bfc_system::bzar::BZAR",
            "bfc_system::mgg::MGG",
        ];
        stable_coins
            .iter()
            .map(|x| {
                let mut part = x.split("::");
                let _package = part.next().unwrap();
                let module_id = part.next().unwrap();
                let name = part.next().unwrap();
                format!(
                    "00000000000000000000000000000000000000000000000000000000000000c8::{}::{}",
                    module_id, name
                )
            })
            .collect()
    }

    pub fn get_avg_exchange_rate(stakes: &[ValidatorStake]) -> f64 {
        if stakes.len() > 0 {
            stakes
                .iter()
                .flat_map(|x| {
                    if let Some(_) = &x.stable_pool {
                        // let mut rates: Vec<f64> = pool
                        //     .coins
                        //     .iter()
                        //     .filter(|(_, v)| v.balance > 0)
                        //     .map(|(_, v)| v.current_exchange_rates.rate())
                        //     .collect();
                        // rates.push(x.current_exchange_rates.rate());
                        // rates
                        vec![0f64]
                    } else {
                        vec![x.current_exchange_rates.rate()]
                    }
                })
                .sum::<f64>()
                / stakes.len() as f64
        } else {
            1_f64
        }
    }

    pub fn extract_stable_stakes(validator_stakes: &[ValidatorStake]) -> Vec<StablePoolSummary> {
        let mut results: HashMap<String, StablePoolSummary> = HashMap::new();
        for v in validator_stakes.iter() {
            if let Some(stable_pool) = &v.stable_pool {
                for item in stable_pool.coins.iter() {
                    let entry =
                        results
                            .entry(item.1.coin_type.to_string())
                            .or_insert(StablePoolSummary {
                                coin_type: item.1.coin_type.clone(),
                                balance: item.1.balance,
                                bfc_value: item.1.bfc_value,
                            });
                    entry.balance += item.1.balance;
                    entry.bfc_value += item.1.bfc_value;
                }
            }
        }
        results.into_iter().map(|(_, v)| v).collect()
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn test_stable_keys() {
            let stable_keys = super::get_stable_pool_keys();
            for x in stable_keys.iter() {
                println!("{}", x.to_string());
            }
        }
    }
}

pub mod stable_pool {
    use std::collections::HashMap;
    use std::hash::Hash;

    use jsonrpsee::http_client::HttpClient;
    use move_core_types::parser::parse_type_tag;
    use sui_types::base_types::ObjectID;
    use sui_types::collection_types::Table;
    use sui_types::sui_system_state::PoolTokenExchangeRate;
    use sui_types::{id::UID, sui_system_state::sui_system_state_inner_v1::StablePoolV1, TypeTag};
    use tracing::warn;

    use crate::benfen;
    use crate::errors::IndexerError;

    struct StablePoolRead<K> {
        validator: K,
        pool_id: UID,
        pool_keys: Vec<String>,
    }

    #[derive(Debug, Clone)]
    pub struct StablePool {
        pub pool_id: UID,
        pub coins: HashMap<TypeTag, StablePoolCoin>,
    }

    #[derive(Debug, Clone)]
    pub struct StablePoolCoin {
        pub coin_type: TypeTag,
        pub balance: u64,
        pub bfc_value: u64,
        pub exchange_rates: Table,
        pub current_exchange_rates: PoolTokenExchangeRate,
    }

    pub struct StablePoolBulkReader<K> {
        reads: Vec<StablePoolRead<K>>,
    }

    impl<K> StablePoolRead<K> {
        async fn read(
            &self,
            http_client: HttpClient,
        ) -> Result<HashMap<String, StablePoolV1>, IndexerError> {
            let mut results = HashMap::new();
            for key in self.pool_keys.iter() {
                let pool = get_stable_pool_from_dynamic_fields(
                    http_client.clone(),
                    self.pool_id.object_id().to_owned(),
                    key.to_owned(),
                )
                .await?;
                results.insert(key.to_owned(), pool);
            }
            Ok(results)
        }
    }

    pub async fn get_stable_pool_from_dynamic_fields(
        http_client: HttpClient,
        pool_id: ObjectID,
        pool_key: String,
    ) -> Result<StablePoolV1, IndexerError> {
        let type_ = parse_type_tag("0x1::ascii::String").unwrap();
        let object =
            benfen::get_dynamic_field_object(http_client, pool_id.clone(), type_, pool_key.clone())
                .await
                .map_err(|err| {
                    IndexerError::FullNodeReadingError(format!(
                        "Failed to fetch stable pool object {:?}.{} with error {:?}",
                        pool_id, pool_key, err
                    ))
                })?;
        let pool: StablePoolV1 =
            benfen::dynamic_field_from_object::<String, StablePoolV1>(&object)?;
        Ok(pool)
    }

    pub async fn get_pool_exchange_rate(
        http_client: HttpClient,
        exchange_rates_id: ObjectID,
        epoch: u64,
    ) -> Result<PoolTokenExchangeRate, IndexerError> {
        let object = benfen::get_dynamic_field_object(
            http_client,
            exchange_rates_id.clone(),
            TypeTag::U64,
            format!("{}", epoch),
        )
        .await?;
        let exchange_rates: PoolTokenExchangeRate =
            benfen::dynamic_field_from_object::<u64, PoolTokenExchangeRate>(&object)?;
        Ok(exchange_rates)
    }

    impl<K> StablePoolBulkReader<K>
    where
        K: Eq + PartialEq + Hash + Clone,
    {
        pub fn new() -> Self {
            Self { reads: vec![] }
        }

        pub fn add(&mut self, validator: K, pool_id: UID, pool_keys: Vec<String>) {
            self.reads.push(StablePoolRead {
                validator,
                pool_id,
                pool_keys,
            });
        }

        pub async fn bulk_read(
            self,
            http_client: HttpClient,
            epoch: u64,
        ) -> Result<HashMap<K, StablePool>, IndexerError> {
            let mut results: HashMap<K, StablePool> = HashMap::new();
            for read in self.reads.iter() {
                let pools = read.read(http_client.clone()).await?;
                for key in read.pool_keys.iter() {
                    let pool = pools.get(key).ok_or_else(|| {
                        IndexerError::FullNodeReadingError(format!(
                            "Failed to read stable pool {:?} from object {:?}",
                            key, read.pool_id
                        ))
                    })?;
                    let coin_type = parse_pool_key(key)?;
                    let current_exchange_rates = match get_pool_exchange_rate(
                        http_client.clone(),
                        pool.exchange_rates.id,
                        epoch,
                    )
                    .await
                    {
                        Ok(rates) => rates,
                        Err(err) => {
                            warn!("Failed to read exchange rates of stable pool {:?} at epoch {} with error {:?}",
                                  pool, epoch, err);
                            PoolTokenExchangeRate::default()
                        }
                    };
                    results
                        .entry(read.validator.clone())
                        .or_insert(StablePool {
                            pool_id: read.pool_id.clone(),
                            coins: HashMap::new(),
                        })
                        .coins
                        .insert(
                            coin_type.clone(),
                            StablePoolCoin {
                                coin_type: coin_type.clone(),
                                balance: pool.stable_balance,
                                bfc_value: benfen::get_bfc_value_of_stable_coin(
                                    coin_type,
                                    pool.stable_balance,
                                    http_client.clone(),
                                )
                                .await?,
                                exchange_rates: pool.exchange_rates.clone(),
                                current_exchange_rates,
                            },
                        );
                }
            }
            Ok(results)
        }
    }

    #[derive(Debug, Clone)]
    pub struct StablePoolSummary {
        pub coin_type: TypeTag,
        pub balance: u64,
        pub bfc_value: u64,
    }

    pub fn parse_pool_key(s: &str) -> Result<TypeTag, IndexerError> {
        let tt = if s.starts_with("0x") || s.starts_with("BFC") || s.starts_with("bfc") {
            parse_type_tag(s)
        } else {
            parse_type_tag(&format!("0x{}", s))
        }?;
        Ok(tt)
    }
}
