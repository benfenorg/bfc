// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// integration test with standalone postgresql database
#[cfg(feature = "pg_integration")]
pub mod pg_integration_test {
    use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
    use tokio::task::JoinHandle;
    use sui_types::sui_serde::BigInt;
    use sui_indexer_v0::errors::IndexerError;
    use ntest::timeout;
    use sui_indexer_v0::store::{PgIndexerStore};
    use sui_indexer_v0::test_utils::{start_test_indexer};
    use sui_indexer_v0::{IndexerConfig};
    use sui_json_rpc_types::{CheckpointId, SuiGetPastObjectRequest, SuiObjectDataOptions, SuiTransactionBlockResponseOptions, SuiTransactionBlockResponseQuery, TransactionFilter};
    use sui_types::base_types::{ObjectID, SequenceNumber};
    use sui_types::digests::{TransactionDigest};
    use test_cluster::{TestCluster, TestClusterBuilder};

    #[tokio::test]
    #[timeout(80000)]
    async fn test_read_api() {
        let (test_cluster, _, _, handle) = start_test_cluster(None).await;

        // CoinReadApi 6 method
        let coin_read_api = test_cluster.sui_client().coin_read_api();
        let coins = coin_read_api.get_coins(test_cluster.get_address_0(), None, None, None).await.unwrap().data;
        assert!(coins.len() != 0);
        let all_coins = coin_read_api.get_all_coins(test_cluster.get_address_0(), None, None).await.unwrap().data;
        assert!(all_coins.len() != 0);
        let balance = coin_read_api.get_balance(test_cluster.get_address_0(), None).await.unwrap();
        assert!(balance.total_balance > 0);
        let all_balance = coin_read_api.get_all_balances(test_cluster.get_address_0()).await.unwrap();
        assert!(all_balance[0].total_balance > 0);
        let coin_metadata = coin_read_api.get_coin_metadata("0x2::bfc::BFC".to_string()).await.unwrap().unwrap();
        assert!(coin_metadata.symbol == "BFC");
        let total_supply = coin_read_api.get_total_supply("0x2::bfc::BFC".to_string()).await.unwrap();
        assert!(total_supply.value > 0);

        // ExtendedApi select db

        // GovernanceReadApi 4 method
        let governance_read_api = test_cluster.sui_client().governance_api();
        let stakes = governance_read_api.get_stakes(test_cluster.get_address_0()).await.unwrap();
        assert!(stakes.len() == 0);
        let committee_info = governance_read_api.get_committee_info(Some(BigInt::from(0))).await.unwrap();
        assert!(committee_info.epoch == 0);
        let latest_sui_system_state = governance_read_api.get_latest_sui_system_state().await.unwrap();
        assert!(latest_sui_system_state.epoch == 0);
        let reference_gas_price = governance_read_api.get_reference_gas_price().await.unwrap();
        assert!(reference_gas_price > 0);

        // IndexerApi select db

        // ReadApi 18 method
        let read_api = test_cluster.sui_client().read_api();
        let object_from_full_node = read_api.get_owned_objects(test_cluster.get_address_0(), None, None, None).await.unwrap().data;
        assert!(object_from_full_node.len() != 0);
        let dynamic_fields = read_api.get_dynamic_fields(ObjectID::from(test_cluster.get_address_0()), None, None).await.unwrap().data;
        assert!(dynamic_fields.len() >= 0);
        let _parsed_past_object = read_api.try_get_parsed_past_object(ObjectID::from(test_cluster.get_address_0()), SequenceNumber::from_u64(0), SuiObjectDataOptions::full_content()).await.unwrap();
        let _parsed_past_objects = read_api.try_multi_get_parsed_past_object(vec![SuiGetPastObjectRequest { object_id: ObjectID::from(test_cluster.get_address_0()), version: SequenceNumber::from_u64(0) }], SuiObjectDataOptions::full_content()).await.unwrap();
        let _object_with_option = read_api.get_object_with_options(ObjectID::from(test_cluster.get_address_0()), SuiObjectDataOptions::full_content()).await.unwrap();
        let _object_with_options = read_api.multi_get_object_with_options(vec![ObjectID::from(test_cluster.get_address_0())], SuiObjectDataOptions::full_content()).await.unwrap();
        let _total_transaction_blocks = read_api.get_total_transaction_blocks().await.unwrap();
        let _transaction_with_option = read_api.get_transaction_with_options(TransactionDigest::ZERO, SuiTransactionBlockResponseOptions::full_content()).await;
        let _committee_info = read_api.get_committee_info(Some(BigInt::from(0))).await.unwrap();
        let checkpoint_seq_query = SuiTransactionBlockResponseQuery::new_with_filter(TransactionFilter::Checkpoint(2u64));
        let _transaction_blocks = read_api.query_transaction_blocks(checkpoint_seq_query, None, None, true).await.unwrap();
        let _chain_identifier = read_api.get_chain_identifier().await.unwrap();
        let _checkpoint = read_api.get_checkpoint(CheckpointId::SequenceNumber(0)).await.unwrap();
        let _checkpoints = read_api.get_checkpoints(None, None, true).await.unwrap();
        let _latest_checkpoint_sequence_number = read_api.get_latest_checkpoint_sequence_number().await.unwrap();
        let _modules_by_package = read_api.get_normalized_move_modules_by_package(ObjectID::from(test_cluster.get_address_0())).await;
        let _reference_gas_price = read_api.get_reference_gas_price().await.unwrap();
        let _loaded_child_objects = read_api.get_loaded_child_objects(TransactionDigest::ZERO).await.unwrap();
        let _loaded_child_objects = read_api.get_protocol_config(Some(BigInt::from(0))).await;

        drop(handle);
    }

    async fn start_test_cluster(
        epoch_duration_ms: Option<u64>,
    ) -> (
        TestCluster,
        HttpClient,
        PgIndexerStore,
        JoinHandle<Result<(), IndexerError>>,
    ) {
        let db_url = format!("postgres://dbuser1:@localhost:5432");

        let test_cluster = if let Some(epoch) = epoch_duration_ms {
            TestClusterBuilder::new()
                .with_epoch_duration_ms(epoch)
                .build()
                .await
        } else {
            TestClusterBuilder::new().build().await
        };

        let config = IndexerConfig {
            db_url: Some(db_url),
            rpc_client_url: test_cluster.rpc_url().to_string(),
            migrated_methods: IndexerConfig::all_implemented_methods(),
            reset_db: true,
            ..Default::default()
        };

        let http_addr_port = format!(
            "http://{}:{}",
            config.rpc_server_url, config.rpc_server_port
        );
        let http_client = HttpClientBuilder::default().build(http_addr_port).unwrap();

        let (store, handle) = start_test_indexer(config).await.unwrap();

        (test_cluster, http_client, store, handle)
    }
}
