// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use tracing::info;
use sui_json_rpc_types::SuiTransactionBlockResponseQuery;
use sui_json_rpc_types::TransactionFilter;
use sui_json_rpc_types::{
    SuiObjectDataOptions, SuiObjectResponseQuery, SuiTransactionBlockResponse,
    SuiTransactionBlockResponseOptions, TransactionBlockBytes,
};
use sui_macros::sim_test;
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_types::transaction::SenderSignedData;
use test_cluster::TestClusterBuilder;

use sui_keys::keystore::AccountKeystore;
use sui_json_rpc::api::{IndexerApiClient, TransactionBuilderClient, WriteApiClient};
use sui_swarm_config::genesis_config::AccountConfig;
use sui_types::base_types::{ObjectID, SequenceNumber, SuiAddress};
use sui_types::crypto::{AccountKeyPair, deterministic_random_account_key, SuiKeyPair};
use sui_types::object::Object;
use sui_json_rpc_types::SuiTransactionBlockEffectsAPI;

#[sim_test]
async fn test_get_transaction_block() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await;
    let http_client = cluster.rpc_client();
    let address = cluster.get_address_0();

    let objects = http_client
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;
    let gas_id = objects.last().unwrap().object().unwrap().object_id;

    // Make some transactions
    let mut tx_responses: Vec<SuiTransactionBlockResponse> = Vec::new();
    for obj in &objects[..objects.len() - 1] {
        let oref = obj.object().unwrap();
        let transaction_bytes: TransactionBlockBytes = http_client
            .transfer_object(
                address,
                oref.object_id,
                Some(gas_id),
                1_000_000.into(),
                address,
            )
            .await?;
        let tx = cluster
            .wallet
            .sign_transaction(&transaction_bytes.to_data()?);

        let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();

        let response = http_client
            .execute_transaction_block(
                tx_bytes,
                signatures,
                Some(SuiTransactionBlockResponseOptions::new()),
                Some(ExecuteTransactionRequestType::WaitForLocalExecution),
            )
            .await?;

        tx_responses.push(response);
    }

    // TODO(chris): re-enable after rewriting get_transactions_in_range_deprecated with query_transactions
    // test get_transaction_batch
    // let batch_responses: Vec<SuiTransactionBlockResponse> = http_client
    //     .multi_get_transaction_blocks(tx, Some(SuiTransactionBlockResponseOptions::new()))
    //     .await?;

    // assert_eq!(5, batch_responses.len());

    // for r in batch_responses.iter().skip(1) {
    //     assert!(tx_responses
    //         .iter()
    //         .any(|resp| matches!(resp, SuiTransactionBlockResponse {digest, ..} if *digest == r.digest)))
    // }

    // // test get_transaction
    // for tx_digest in tx {
    //     let response: SuiTransactionBlockResponse = http_client
    //         .get_transaction_block(
    //             tx_digest,
    //             Some(SuiTransactionBlockResponseOptions::new().with_raw_input()),
    //         )
    //         .await?;
    //     assert!(tx_responses.iter().any(
    //         |resp| matches!(resp, SuiTransactionBlockResponse {digest, ..} if *digest == response.digest)
    //     ));
    //     let sender_signed_data: SenderSignedData =
    //         bcs::from_bytes(&response.raw_transaction).unwrap();
    //     assert_eq!(sender_signed_data.digest(), tx_digest);
    // }

    Ok(())
}

#[sim_test]
async fn test_get_raw_transaction() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await;
    let http_client = cluster.rpc_client();
    let address = cluster.get_address_0();

    let objects = http_client
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new(),
            )),
            None,
            None,
        )
        .await?
        .data;
    let object_to_transfer = objects.first().unwrap().object().unwrap().object_id;

    // Make a transfer transactions
    let transaction_bytes: TransactionBlockBytes = http_client
        .transfer_object(address, object_to_transfer, None, 1_000_000.into(), address)
        .await?;
    let tx = cluster
        .wallet
        .sign_transaction(&transaction_bytes.to_data()?);
    let original_sender_signed_data = tx.data().clone();

    let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();

    let response = http_client
        .execute_transaction_block(
            tx_bytes,
            signatures,
            Some(SuiTransactionBlockResponseOptions::new().with_raw_input()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    let decode_sender_signed_data: SenderSignedData =
        bcs::from_bytes(&response.raw_transaction).unwrap();
    // verify that the raw transaction data returned by the response is the same
    // as the original transaction data
    assert_eq!(decode_sender_signed_data, original_sender_signed_data);

    Ok(())
}

#[sim_test]
async fn test_get_fullnode_transaction() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await;

    let context = &cluster.wallet;

    let mut tx_responses: Vec<SuiTransactionBlockResponse> = Vec::new();

    let client = context.get_client().await.unwrap();

    for address in cluster.get_addresses() {
        let objects = client
            .read_api()
            .get_owned_objects(
                address,
                Some(SuiObjectResponseQuery::new_with_options(
                    SuiObjectDataOptions::new()
                        .with_type()
                        .with_owner()
                        .with_previous_transaction(),
                )),
                None,
                None,
            )
            .await?
            .data;
        let gas_id = objects.last().unwrap().object().unwrap().object_id;

        // Make some transactions
        for obj in &objects[..objects.len() - 1] {
            let oref = obj.object().unwrap();
            let data = client
                .transaction_builder()
                .transfer_object(address, oref.object_id, Some(gas_id), 1_000_000, address)
                .await?;
            let tx = cluster.wallet.sign_transaction(&data);

            let response = client
                .quorum_driver_api()
                .execute_transaction_block(
                    tx,
                    SuiTransactionBlockResponseOptions::new(),
                    Some(ExecuteTransactionRequestType::WaitForLocalExecution),
                )
                .await
                .unwrap();

            tx_responses.push(response);
        }
    }

    // test get_recent_transactions with smaller range
    let query = SuiTransactionBlockResponseQuery {
        options: Some(SuiTransactionBlockResponseOptions {
            show_input: true,
            show_effects: true,
            show_events: true,
            ..Default::default()
        }),
        ..Default::default()
    };

    let tx = client
        .read_api()
        .query_transaction_blocks(query, None, Some(3), true)
        .await
        .unwrap();
    assert_eq!(3, tx.data.len());
    assert!(tx.data[0].transaction.is_some());
    assert!(tx.data[0].effects.is_some());
    assert!(tx.data[0].events.is_some());
    assert!(tx.has_next_page);

    // test get all transactions paged
    let first_page = client
        .read_api()
        .query_transaction_blocks(
            SuiTransactionBlockResponseQuery::default(),
            None,
            Some(5),
            false,
        )
        .await
        .unwrap();
    assert_eq!(5, first_page.data.len());
    assert!(first_page.has_next_page);

    let second_page = client
        .read_api()
        .query_transaction_blocks(
            SuiTransactionBlockResponseQuery::default(),
            first_page.next_cursor,
            None,
            false,
        )
        .await
        .unwrap();
    assert!(second_page.data.len() > 5);
    assert!(!second_page.has_next_page);

    let mut all_txs_rev = first_page.data.clone();
    all_txs_rev.extend(second_page.data);
    all_txs_rev.reverse();

    // test get 10 latest transactions paged
    let latest = client
        .read_api()
        .query_transaction_blocks(
            SuiTransactionBlockResponseQuery::default(),
            None,
            Some(10),
            true,
        )
        .await
        .unwrap();
    assert_eq!(10, latest.data.len());
    assert_eq!(Some(all_txs_rev[9].digest), latest.next_cursor);
    assert_eq!(all_txs_rev[0..10], latest.data);
    assert!(latest.has_next_page);

    // test get from address txs in ascending order
    let address_txs_asc = client
        .read_api()
        .query_transaction_blocks(
            SuiTransactionBlockResponseQuery::new_with_filter(TransactionFilter::FromAddress(
                cluster.get_address_0(),
            )),
            None,
            None,
            false,
        )
        .await
        .unwrap();
    assert_eq!(4, address_txs_asc.data.len());

    // test get from address txs in descending order
    let address_txs_desc = client
        .read_api()
        .query_transaction_blocks(
            SuiTransactionBlockResponseQuery::new_with_filter(TransactionFilter::FromAddress(
                cluster.get_address_0(),
            )),
            None,
            None,
            true,
        )
        .await
        .unwrap();
    assert_eq!(4, address_txs_desc.data.len());

    // test get from address txs in both ordering are the same.
    let mut data_asc = address_txs_asc.data;
    data_asc.reverse();
    assert_eq!(data_asc, address_txs_desc.data);

    // test get_recent_transactions
    let tx = client
        .read_api()
        .query_transaction_blocks(
            SuiTransactionBlockResponseQuery::default(),
            None,
            Some(20),
            true,
        )
        .await
        .unwrap();
    assert_eq!(20, tx.data.len());

    // test get_transaction
    for tx_resp in tx.data {
        let response: SuiTransactionBlockResponse = client
            .read_api()
            .get_transaction_with_options(tx_resp.digest, SuiTransactionBlockResponseOptions::new())
            .await
            .unwrap();
        assert_eq!(tx_resp.digest, response.digest);
    }

    Ok(())
}

#[sim_test]
async fn test_get_transaction_block_with_stable_gascoin() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await;
    let http_client = cluster.rpc_client();
    let address = cluster.get_address_0();

    let objects = http_client
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new()
                    .with_type()
                    .with_owner()
                    .with_previous_transaction(),
            )),
            None,
            None,
        )
        .await?
        .data;
    let gas_id = objects.last().unwrap().object().unwrap().object_id;

    // Make some transactions
    let mut tx_responses: Vec<SuiTransactionBlockResponse> = Vec::new();
    for obj in &objects[..objects.len() - 1] {
        let oref = obj.object().unwrap();
        let transaction_bytes: TransactionBlockBytes = http_client
            .transfer_object(
                address,
                oref.object_id,
                Some(gas_id),
                1_000_000.into(),
                address,
            )
            .await?;
        let tx = cluster
            .wallet
            .sign_transaction(&transaction_bytes.to_data()?);

        let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();

        let response = http_client
            .execute_transaction_block(
                tx_bytes,
                signatures,
                Some(SuiTransactionBlockResponseOptions::new()),
                Some(ExecuteTransactionRequestType::WaitForLocalExecution),
            )
            .await?;

        tx_responses.push(response);
    }

    // TODO(chris): re-enable after rewriting get_transactions_in_range_deprecated with query_transactions
    // test get_transaction_batch
    // let batch_responses: Vec<SuiTransactionBlockResponse> = http_client
    //     .multi_get_transaction_blocks(tx, Some(SuiTransactionBlockResponseOptions::new()))
    //     .await?;

    // assert_eq!(5, batch_responses.len());

    // for r in batch_responses.iter().skip(1) {
    //     assert!(tx_responses
    //         .iter()
    //         .any(|resp| matches!(resp, SuiTransactionBlockResponse {digest, ..} if *digest == r.digest)))
    // }

    // // test get_transaction
    // for tx_digest in tx {
    //     let response: SuiTransactionBlockResponse = http_client
    //         .get_transaction_block(
    //             tx_digest,
    //             Some(SuiTransactionBlockResponseOptions::new().with_raw_input()),
    //         )
    //         .await?;
    //     assert!(tx_responses.iter().any(
    //         |resp| matches!(resp, SuiTransactionBlockResponse {digest, ..} if *digest == response.digest)
    //     ));
    //     let sender_signed_data: SenderSignedData =
    //         bcs::from_bytes(&response.raw_transaction).unwrap();
    //     assert_eq!(sender_signed_data.digest(), tx_digest);
    // }

    Ok(())
}

#[sim_test]
async fn test_get_raw_transaction_with_stable_gascoin() -> Result<(), anyhow::Error> {
    let obj_id = ObjectID::random();
    let (address, keypair): (SuiAddress, AccountKeyPair) =
        deterministic_random_account_key();
    let gas_object = Object::with_stable_id_owner_version_for_testing(
        obj_id,
        SequenceNumber::from_u64(1),
        address,
    );
    let bfc_object = Object::with_id_owner_gas_for_testing(ObjectID::random(), address, 100000000000);
    let exchange_id = ObjectID::random();
    let bfc_exchange = Object::with_id_owner_gas_for_testing(exchange_id, address, 200000000000);

    let mut test_cluster = TestClusterBuilder::new()
        .with_accounts(vec![AccountConfig {
            gas_amounts: vec![30_000_000_000_000_000],
            address: Some(address),
        }])
        .with_objects([
            gas_object.clone(), bfc_object, bfc_exchange
        ])
        .build()
        .await;

    let context = &mut test_cluster.wallet;
    context
        .config
        .keystore
        .add_key(SuiKeyPair::Ed25519(keypair))?;
    let http_client = test_cluster.rpc_client();
    // let address = cluster.get_address_0();

    let objects = http_client
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new(),
            )),
            None,
            None,
        )
        .await?
        .data;
    let object_to_transfer = objects.first().unwrap().object().unwrap().object_id;

    // Make a transfer transactions
    let transaction_bytes: TransactionBlockBytes = http_client
        .transfer_object(address, object_to_transfer, Some(gas_object.id()), 100_000.into(), address)
        .await?;
    let tx = test_cluster
        .wallet
        .sign_transaction(&transaction_bytes.to_data()?);

    let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();


    let dry_response = http_client
        .dry_run_transaction_block(tx_bytes.clone()).await;

    assert_eq!(dry_response.is_ok(), true);
    let dry_gas = dry_response.unwrap().effects.gas_cost_summary().clone();
    let response = http_client
        .execute_transaction_block(
            tx_bytes,
            signatures,
            Some(SuiTransactionBlockResponseOptions::new()
                .with_raw_input()
                .with_effects()
                .with_balance_changes()
            ),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await.unwrap();

    let gas_usage = response.effects.unwrap().gas_cost_summary().clone();
    let stable_gas_used= gas_usage.net_gas_usage();
    info!("gas_summary: {:?}, {}", &gas_usage, stable_gas_used);

    Ok(())
}
