// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde_json::json;
use tracing::error;
use sui_json_rpc_api::{TransactionBuilderClient, WriteApiClient};
use sui_json_rpc_types::{SuiExecutionStatus, SuiTransactionBlockEffects, SuiTransactionBlockResponseOptions, SuiTypeTag, TransactionBlockBytes};
use sui_macros::sim_test;
use sui_types::{BFC_SYSTEM_PACKAGE_ID, BFC_SYSTEM_STATE_OBJECT_ID, SUI_CLOCK_OBJECT_ID};
use test_cluster::TestClusterBuilder;
use sui_sdk::json::{SuiJsonValue};
use sui_types::base_types::SuiAddress;
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;

#[sim_test]
async fn sim_test0() -> Result<(), anyhow::Error> {
    // init
    let test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(6000)
        .with_num_validators(5)
        .build()
        .await;
    let http_client = test_cluster.rpc_client();
    let address = test_cluster.get_address_0();
    // call
    let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let args = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::new(json!(100u64.to_string()))?,
        // SuiJsonValue::from_str(&"wrong_key".to_string())?,
        SuiJsonValue::new(json!("wrong_key".to_string()))?,
    ];
    let transaction_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "mint_stable".to_string(),
            vec![SuiTypeTag::new("0xc8::usdc::USDC".to_string())],
            args,
            None,
            10_000_00000.into(),
            None,
        )
        .await?;
    let tx = test_cluster
        .wallet
        .sign_transaction(&transaction_bytes.to_data()?);
    let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();
    let tx_response = http_client
        .execute_transaction_block(
            tx_bytes,
            signatures,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    let effects = tx_response.effects.unwrap().clone();
    match effects {
        SuiTransactionBlockEffects::V1(_effects) => {
            if _effects.status.is_err() {
                error!("effects is {:?}",_effects);
            }
            assert!(_effects.status.is_ok());
        }
    };
    Ok(())
}

#[sim_test]
async fn sim_test1() -> Result<(), anyhow::Error> {
    // init
    let test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(6000)
        .with_num_validators(5)
        .build()
        .await;
    let http_client = test_cluster.rpc_client();
    let address = test_cluster.get_address_0();
    // call
    let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let args = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::new(json!(100u64.to_string()))?,
        SuiJsonValue::new(json!("wrong_key"))?,
    ];
    let transaction_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "mint_stable_v1".to_string(),
            vec![SuiTypeTag::new("0xc8::usdc::USDC".to_string())],
            args,
            None,
            10_000_00000.into(),
            None,
        )
        .await?;
    let tx = test_cluster
        .wallet
        .sign_transaction(&transaction_bytes.to_data()?);
    let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();
    let tx_response = http_client
        .execute_transaction_block(
            tx_bytes,
            signatures,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    let effects = tx_response.effects.unwrap().clone();
    match effects {
        SuiTransactionBlockEffects::V1(_effects) => {
            match _effects.status {
                SuiExecutionStatus::Success => {
                    assert!(false);
                }
                SuiExecutionStatus::Failure { error } => {
                    assert!(error.contains("1004"));
                }
            }
        }
    };
    Ok(())
}

