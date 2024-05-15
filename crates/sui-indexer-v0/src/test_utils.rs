// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use prometheus::Registry;
use tokio::task::JoinHandle;

use sui_json_rpc_types::SuiTransactionBlockResponse;

use crate::errors::IndexerError;
use crate::store::PgIndexerStore;
use crate::utils::reset_database;
use crate::IndexerMetrics;
use crate::{new_pg_connection_pool, Indexer, IndexerConfig};

/// Spawns an indexer thread with provided Postgres DB url
pub async fn start_test_indexer(
    config: IndexerConfig,
) -> Result<(PgIndexerStore, JoinHandle<Result<(), IndexerError>>), anyhow::Error> {
    let parsed_url = config.base_connection_url()?;
    let blocking_pool = new_pg_connection_pool(&parsed_url)
        .map_err(|e| anyhow!("unable to connect to Postgres, is it running? {e}"))?;
    if config.reset_db {
        reset_database(
            &mut blocking_pool
                .get()
                .map_err(|e| anyhow!("Fail to get pg_connection_pool {e}"))?,
            true,
        )?;
    }

    let registry = Registry::default();
    let indexer_metrics = IndexerMetrics::new(&registry);

    let store = PgIndexerStore::new(blocking_pool, indexer_metrics.clone());
    let store_clone = store.clone();
    let handle = tokio::spawn(async move {
        Indexer::start(&config, &registry, store_clone, indexer_metrics, None).await
    });
    Ok((store, handle))
}

#[derive(Clone)]
pub struct SuiTransactionBlockResponseBuilder<'a> {
    response: SuiTransactionBlockResponse,
    full_response: &'a SuiTransactionBlockResponse,
}

impl<'a> SuiTransactionBlockResponseBuilder<'a> {
    pub fn new(full_response: &'a SuiTransactionBlockResponse) -> Self {
        Self {
            response: SuiTransactionBlockResponse::default(),
            full_response,
        }
    }

    pub fn with_input(mut self) -> Self {
        self.response = SuiTransactionBlockResponse {
            transaction: self.full_response.transaction.clone(),
            ..self.response
        };
        self
    }

    pub fn with_raw_input(mut self) -> Self {
        self.response = SuiTransactionBlockResponse {
            raw_transaction: self.full_response.raw_transaction.clone(),
            ..self.response
        };
        self
    }

    pub fn with_effects(mut self) -> Self {
        self.response = SuiTransactionBlockResponse {
            effects: self.full_response.effects.clone(),
            ..self.response
        };
        self
    }

    pub fn with_events(mut self) -> Self {
        self.response = SuiTransactionBlockResponse {
            events: self.full_response.events.clone(),
            ..self.response
        };
        self
    }

    pub fn with_balance_changes(mut self) -> Self {
        self.response = SuiTransactionBlockResponse {
            balance_changes: self.full_response.balance_changes.clone(),
            ..self.response
        };
        self
    }

    pub fn with_object_changes(mut self) -> Self {
        self.response = SuiTransactionBlockResponse {
            object_changes: self.full_response.object_changes.clone(),
            ..self.response
        };
        self
    }

    pub fn with_input_and_changes(mut self) -> Self {
        self.response = SuiTransactionBlockResponse {
            transaction: self.full_response.transaction.clone(),
            balance_changes: self.full_response.balance_changes.clone(),
            object_changes: self.full_response.object_changes.clone(),
            ..self.response
        };
        self
    }

    pub fn build(self) -> SuiTransactionBlockResponse {
        SuiTransactionBlockResponse {
            transaction: self.response.transaction,
            raw_transaction: self.response.raw_transaction,
            effects: self.response.effects,
            events: self.response.events,
            balance_changes: self.response.balance_changes,
            object_changes: self.response.object_changes,
            // Use full response for any fields that aren't showable
            ..self.full_response.clone()
        }
    }
}

#[cfg(test)]
mod test_stable_pool {
    use crate::utils::stable_pool::{get_pool_exchange_rate, get_stable_pool_from_dynamic_fields};
    use jsonrpsee::http_client::{HeaderMap, HeaderValue, HttpClient, HttpClientBuilder};
    use move_core_types::account_address::AccountAddress;
    use sui_json_rpc::CLIENT_SDK_TYPE_HEADER;
    use sui_types::base_types::ObjectID;

    fn create_http_client() -> HttpClient {
        let rpc_client_url = "https://testrpc.benfen.org:443/";
        let mut headers = HeaderMap::new();
        headers.insert(CLIENT_SDK_TYPE_HEADER, HeaderValue::from_static("indexer"));
        HttpClientBuilder::default()
            .max_request_body_size(2 << 30)
            .max_concurrent_requests(usize::MAX)
            .set_headers(headers.clone())
            .build(rpc_client_url)
            .unwrap()
    }

    #[ignore]
    #[tokio::test]
    async fn test_stable_pool() {
        let http_client = create_http_client();
        // https://explorer.benfen.org/#/object/BFCf3923fc05269d085b120171094893759b655dae115a85beaac028d52d703d53356e6
        let pool_id = ObjectID::from_address(
            AccountAddress::from_hex_literal(
                "0xb09616a1f82ffe6d9d6c048a0e52b2b98fa74779566d901c9e6b044f7b192105",
            )
            .unwrap(),
        );
        let pool = get_stable_pool_from_dynamic_fields(
            http_client,
            pool_id,
            "00000000000000000000000000000000000000000000000000000000000000c8::busd::BUSD"
                .to_owned(),
        )
        .await
        .unwrap();
        println!("{:?}", pool);
    }

    #[ignore]
    #[tokio::test]
    async fn test_exchange_rate() {
        let http_client = create_http_client();
        // https://explorer.benfen.org/#/object/BFCfd8dc21acd65cdf6ed263c16bc552342e1a0aca38f4db4e36284acc48eed36688f96
        let table_id = ObjectID::from_hex_literal(
            "0xaa38424a4ac5ef3514e17715ff51368fd36420c43232fff5857aa991b331c44b",
        )
        .unwrap();
        let exchange_rate = get_pool_exchange_rate(http_client, table_id, 31)
            .await
            .unwrap();
        println!("{:?}", exchange_rate)
    }
}
