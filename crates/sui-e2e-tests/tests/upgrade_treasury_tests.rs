// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::http_client::HttpClient;
use serde_json::{json, Value};
use sui_sdk::json::SuiJsonValue;
use std::str::FromStr;
use sui_json_rpc_api::{IndexerApiClient, TransactionBuilderClient, WriteApiClient};
use sui_json_rpc_types::{SuiExecutionStatus, SuiObjectDataFilter, SuiObjectDataOptions, SuiObjectResponse, SuiObjectResponseQuery, SuiTransactionBlockEffects, SuiTransactionBlockResponseOptions, SuiTypeTag, TransactionBlockBytes};
use sui_macros::sim_test;
use sui_types::{BFC_SYSTEM_PACKAGE_ID, parse_sui_struct_tag};
use test_cluster::TestClusterBuilder;
use sui_types::base_types::SuiAddress;
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use sui::validator_commands::{SuiValidatorCommand, SuiValidatorCommandResponse};

#[sim_test]
async fn sim_set_oracle_price_address_by_cli_success() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new()
    .with_epoch_duration_ms(6000)
    .with_num_validators(5)
    .build().await;
    
    setup_auth(&test_cluster).await?;

    let addr = SuiAddress::random_for_testing_only();
    SuiValidatorCommand::SetOraclePriceAddress { 
        address: addr, 
        gas_budget: None,
    }
    .execute(&mut test_cluster.wallet)
    .await?
    .print(true);

    test_cluster.wait_for_epoch(Some(2)).await;

    test_cluster
    .swarm
    .validator_nodes()
    .next()
    .unwrap()
    .get_node_handle()
    .unwrap()
    .with(|node| {
        let _state = node
            .state()
            .get_bfc_system_state_object_for_testing().unwrap();
        let _oracle_address = _state.get_oracle_address();
        // should be some 
        assert!(_oracle_address.is_some());
        println!("addr:{:?} == addr:{:?}", addr.to_string(),_oracle_address.unwrap().to_string());
        assert!(addr.to_vec() == _oracle_address.unwrap().to_vec());
    });

    Ok(())
}

#[sim_test]
async fn sim_add_delete_stable_gas_coin_by_cli_success() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new()
    .with_epoch_duration_ms(6000)
    .with_num_validators(5)
    .build().await;
    
    let coin_type = String::from_utf8(b"000::t:TEST".to_vec()).unwrap();
    let resp: SuiValidatorCommandResponse = SuiValidatorCommand::AddStableGasCoin { 
        coin_type_list: vec![coin_type.clone()],
        gas_budget: None,
    }
    .execute(&mut test_cluster.wallet)
    .await?;
    match resp {
        SuiValidatorCommandResponse::AddStableGasCoin(sui_transaction_block_response) => {
            assert!(!sui_transaction_block_response.status_ok().unwrap());
        }
        _ => panic!("should be success"),
    }

    setup_auth(&test_cluster).await?;

    let coin_type = String::from_utf8(b"000::t:TEST".to_vec()).unwrap();
    let resp: SuiValidatorCommandResponse = SuiValidatorCommand::AddStableGasCoin { 
        coin_type_list: vec![coin_type.clone()],
        gas_budget: None,
    }
    .execute(&mut test_cluster.wallet)
    .await?;
    match resp {
        SuiValidatorCommandResponse::AddStableGasCoin(sui_transaction_block_response) => {
            assert!(sui_transaction_block_response.status_ok().unwrap());
        }
        _ => panic!("should be success"),
    }


    let coin_type = String::from_utf8(b"000::t:TEST".to_vec()).unwrap();
    let resp: SuiValidatorCommandResponse = SuiValidatorCommand::DeleteStableGasCoin { 
        coin_type: coin_type.clone(),
        gas_budget: None,
    }
    .execute(&mut test_cluster.wallet)
    .await?;
    match resp {
        SuiValidatorCommandResponse::DeleteStableGasCoin(sui_transaction_block_response) => {
            assert!(sui_transaction_block_response.status_ok().unwrap());
        }
        _ => panic!("should be success"),
    }

    Ok(())
}


#[sim_test]
async fn sim_test_mint_stable_with_unauthorized() -> Result<(), anyhow::Error> {
    // init
    let test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(6000)
        .with_num_validators(5)
        .build()
        .await;
    let http_client = test_cluster.rpc_client();
    let address = test_cluster.get_address_0();
    let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let args0 = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::new(Value::Array(vec![json!(address.to_string())]))?,
    ];
    let transaction_bytes0: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "init_admin_capability".to_string(),
            vec![],
            args0,
            None,
            10_000_00000.into(),
            None,
        )
        .await?;
    let tx0 = test_cluster
        .wallet
        .sign_transaction(&transaction_bytes0.to_data()?);
    let (tx_bytes0, signatures0) = tx0.to_tx_bytes_and_signatures();
    http_client
        .execute_transaction_block(
            tx_bytes0,
            signatures0,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    let admin_cap_vec = get_owned_objects("0xc8::bfc_system_state_inner::BfcSystemAdminCap", http_client, address).await.unwrap();
    let admin_cap = admin_cap_vec.first().unwrap().object().unwrap();
    let args1 = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::from_str(&admin_cap.object_id.to_string())?,
        SuiJsonValue::new(json!(b"MINT-BUSD-right_key"))?,
        SuiJsonValue::new(json!(address.to_string()))?,
    ];
    let transaction_bytes1: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "set_single_operation_capability".to_string(),
            vec![],
            args1,
            None,
            10_000_00000.into(),
            None,
        )
        .await?;
    let tx1 = test_cluster
        .wallet
        .sign_transaction(&transaction_bytes1.to_data()?);
    let (tx_bytes1, signatures1) = tx1.to_tx_bytes_and_signatures();
    http_client
        .execute_transaction_block(
            tx_bytes1,
            signatures1,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    let args2 = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::from_str(&admin_cap.object_id.to_string())?,
        SuiJsonValue::new(json!(b"MINT-BUSD-right_key"))?,
        SuiJsonValue::new(json!(address.to_string()))?,
    ];
    let transaction_bytes2: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "remove_operation_capability".to_string(),
            vec![],
            args2,
            None,
            10_000_00000.into(),
            None,
        )
        .await?;
    let tx2 = test_cluster
        .wallet
        .sign_transaction(&transaction_bytes2.to_data()?);
    let (tx_bytes2, signatures2) = tx2.to_tx_bytes_and_signatures();
    http_client
        .execute_transaction_block(
            tx_bytes2,
            signatures2,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    let modify_cap_vec = get_owned_objects("0xc8::bfc_system_state_inner::BfcSystemModifyCap", http_client, address).await.unwrap();
    let modify_cap = modify_cap_vec.first().unwrap().object().unwrap();
    let args = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::new(json!(100u64.to_string()))?,
        SuiJsonValue::from_str(&modify_cap.object_id.to_string())?,
    ];
    let transaction_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "mint_stable_entry".to_string(),
            vec![SuiTypeTag::new("0xc8::busd::BUSD".to_string())],
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
    effect_fail(effects, "1004");
    Ok(())
}

#[sim_test]
async fn sim_test_mint_busd_with_operation_unauthorized() -> Result<(), anyhow::Error> {
    // init
    let test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(6000)
        .with_num_validators(5)
        .build()
        .await;
    let http_client = test_cluster.rpc_client();
    let address = test_cluster.get_address_0();
    let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let args0 = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::new(Value::Array(vec![json!(address.to_string())]))?,
    ];
    let transaction_bytes0: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "init_admin_capability".to_string(),
            vec![],
            args0,
            None,
            10_000_00000.into(),
            None,
        )
        .await?;
    let tx0 = test_cluster
        .wallet
        .sign_transaction(&transaction_bytes0.to_data()?);
    let (tx_bytes0, signatures0) = tx0.to_tx_bytes_and_signatures();
    http_client
        .execute_transaction_block(
            tx_bytes0,
            signatures0,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    let admin_cap_vec = get_owned_objects("0xc8::bfc_system_state_inner::BfcSystemAdminCap", http_client, address).await.unwrap();
    let admin_cap = admin_cap_vec.first().unwrap().object().unwrap();
    let args1 = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::from_str(&admin_cap.object_id.to_string())?,
        SuiJsonValue::new(json!(b"MINT-OTHER-right_key"))?,
        SuiJsonValue::new(json!(address.to_string()))?,
    ];
    let transaction_bytes1: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "set_single_operation_capability".to_string(),
            vec![],
            args1,
            None,
            10_000_00000.into(),
            None,
        )
        .await?;
    let tx1 = test_cluster
        .wallet
        .sign_transaction(&transaction_bytes1.to_data()?);
    let (tx_bytes1, signatures1) = tx1.to_tx_bytes_and_signatures();
    http_client
        .execute_transaction_block(
            tx_bytes1,
            signatures1,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    let modify_cap_vec = get_owned_objects("0xc8::bfc_system_state_inner::BfcSystemModifyCap", http_client, address).await.unwrap();
    let modify_cap = modify_cap_vec.first().unwrap().object().unwrap();
    let args = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::new(json!(100u64.to_string()))?,
        SuiJsonValue::from_str(&modify_cap.object_id.to_string())?,
    ];
    let transaction_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "mint_stable_entry".to_string(),
            vec![SuiTypeTag::new("0xc8::busd::BUSD".to_string())],
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
    effect_fail(effects, "1008");
    Ok(())
}

#[sim_test]
async fn sim_test_mint_busd_with_zero() -> Result<(), anyhow::Error> {
    // init
    let test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(6000)
        .with_num_validators(5)
        .build()
        .await;
    let http_client = test_cluster.rpc_client();
    let address = test_cluster.get_address_0();
    let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let args0 = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::new(Value::Array(vec![json!(address.to_string())]))?,
    ];
    let transaction_bytes0: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "init_admin_capability".to_string(),
            vec![],
            args0,
            None,
            10_000_00000.into(),
            None,
        )
        .await?;
    let tx0 = test_cluster
        .wallet
        .sign_transaction(&transaction_bytes0.to_data()?);
    let (tx_bytes0, signatures0) = tx0.to_tx_bytes_and_signatures();
    http_client
        .execute_transaction_block(
            tx_bytes0,
            signatures0,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    let admin_cap_vec = get_owned_objects("0xc8::bfc_system_state_inner::BfcSystemAdminCap", http_client, address).await.unwrap();
    let admin_cap = admin_cap_vec.first().unwrap().object().unwrap();
    let args1 = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::from_str(&admin_cap.object_id.to_string())?,
        SuiJsonValue::new(json!(b"MINT-BUSD-right_key"))?,
        SuiJsonValue::new(json!(address.to_string()))?,
    ];
    let transaction_bytes1: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "set_single_operation_capability".to_string(),
            vec![],
            args1,
            None,
            10_000_00000.into(),
            None,
        )
        .await?;
    let tx1 = test_cluster
        .wallet
        .sign_transaction(&transaction_bytes1.to_data()?);
    let (tx_bytes1, signatures1) = tx1.to_tx_bytes_and_signatures();
    http_client
        .execute_transaction_block(
            tx_bytes1,
            signatures1,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    let modify_cap_vec = get_owned_objects("0xc8::bfc_system_state_inner::BfcSystemModifyCap", http_client, address).await.unwrap();
    let modify_cap = modify_cap_vec.first().unwrap().object().unwrap();
    let args = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::new(json!(0u64.to_string()))?,
        SuiJsonValue::from_str(&modify_cap.object_id.to_string())?,
    ];
    let transaction_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "mint_stable_entry".to_string(),
            vec![SuiTypeTag::new("0xc8::busd::BUSD".to_string())],
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
    effect_fail(effects, "1007");
    Ok(())
}

#[sim_test]
async fn sim_test_mint_stable_with_success() -> Result<(), anyhow::Error> {
    // init
    let test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(6000)
        .with_num_validators(5)
        .build()
        .await;
    let http_client = test_cluster.rpc_client();
    let address = test_cluster.get_address_0();
    let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();

    let args0 = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::new(Value::Array(vec![json!(address.to_string())]))?,
    ];
    let transaction_bytes0: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "init_admin_capability".to_string(),
            vec![],
            args0,
            None,
            10_000_00000.into(),
            None,
        )
        .await?;
    let tx0 = test_cluster
        .wallet
        .sign_transaction(&transaction_bytes0.to_data()?);
    let (tx_bytes0, signatures0) = tx0.to_tx_bytes_and_signatures();
    http_client
        .execute_transaction_block(
            tx_bytes0,
            signatures0,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    let admin_cap_vec = get_owned_objects("0xc8::bfc_system_state_inner::BfcSystemAdminCap", http_client, address).await.unwrap();
    let admin_cap = admin_cap_vec.first().unwrap().object().unwrap();
    let args1 = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::from_str(&admin_cap.object_id.to_string())?,
        SuiJsonValue::new(json!(b"MINT-BUSD-right_key"))?,
        SuiJsonValue::new(json!(address.to_string()))?,
    ];
    let transaction_bytes1: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "set_single_operation_capability".to_string(),
            vec![],
            args1,
            None,
            10_000_00000.into(),
            None,
        )
        .await?;
    let tx1 = test_cluster
        .wallet
        .sign_transaction(&transaction_bytes1.to_data()?);
    let (tx_bytes1, signatures1) = tx1.to_tx_bytes_and_signatures();
    http_client
        .execute_transaction_block(
            tx_bytes1,
            signatures1,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    let modify_cap_vec = get_owned_objects("0xc8::bfc_system_state_inner::BfcSystemModifyCap", http_client, address).await.unwrap();
    let modify_cap = modify_cap_vec.first().unwrap().object().unwrap();
    let args = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::new(json!(100u64.to_string()))?,
        SuiJsonValue::from_str(&modify_cap.object_id.to_string())?,
    ];
    let transaction_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "mint_stable_entry".to_string(),
            vec![SuiTypeTag::new("0xc8::busd::BUSD".to_string())],
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
    effect_success(effects);
    let objects = get_owned_objects("0x2::coin::Coin<0xc8::busd::BUSD>", http_client, address).await?;
    assert!(objects.len() >= 1, "Expected at least 1 BUSD coin object, but found {}", objects.len());
    Ok(())
}

pub async fn setup_auth(test_cluster: &test_cluster::TestCluster) -> Result<(), anyhow::Error> {
    let http_client = test_cluster.rpc_client();
    let address = test_cluster.get_address_0();
    let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let args0 = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::new(Value::Array(vec![json!(address.to_string())]))?,
    ];
    let transaction_bytes0: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "init_admin_capability".to_string(),
            vec![],
            args0,
            None,
            10_000_00000.into(),
            None,
        )
        .await?;
    let tx0 = test_cluster
        .wallet
        .sign_transaction(&transaction_bytes0.to_data()?);
    let (tx_bytes0, signatures0) = tx0.to_tx_bytes_and_signatures();
    http_client
        .execute_transaction_block(
            tx_bytes0,
            signatures0,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    let admin_cap_vec = get_owned_objects("0xc8::bfc_system_state_inner::BfcSystemAdminCap", http_client, address).await.unwrap();
    let admin_cap = admin_cap_vec.first().unwrap().object().unwrap();
    let args1 = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::from_str(&admin_cap.object_id.to_string())?,
        SuiJsonValue::new(json!(b"MINT-BUSD-right_key"))?,
        SuiJsonValue::new(json!(address.to_string()))?,
    ];
    let transaction_bytes1: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "set_single_operation_capability".to_string(),
            vec![],
            args1,
            None,
            10_000_00000.into(),
            None,
        )
        .await?;
    let tx1 = test_cluster
        .wallet
        .sign_transaction(&transaction_bytes1.to_data()?);
    let (tx_bytes1, signatures1) = tx1.to_tx_bytes_and_signatures();
    http_client
        .execute_transaction_block(
            tx_bytes1,
            signatures1,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    Ok(())
}

async fn get_owned_objects(filter_tag: &str, http_client: &HttpClient, address: SuiAddress) -> Result<Vec<SuiObjectResponse>, anyhow::Error> {
    let filter = SuiObjectDataFilter::StructType(parse_sui_struct_tag(filter_tag).unwrap());
    let data_option = SuiObjectDataOptions::new()
        .with_type()
        .with_owner()
        .with_previous_transaction()
        .with_content();
    let objects = http_client
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new(
                Some(filter),
                Some(data_option),
            )),
            None,
            None,
        )
        .await?
        .data;
    Ok(objects)
}

fn effect_fail(effects: SuiTransactionBlockEffects, s: &str) {
    match effects {
        SuiTransactionBlockEffects::V1(_effects) => {
            match _effects.status {
                SuiExecutionStatus::Success => {
                    assert!(false);
                }
                SuiExecutionStatus::Failure { error } => {
                    assert!(error.contains(s));
                }
            }
        }
    };
}

fn effect_success(effects: SuiTransactionBlockEffects) {
    match effects {
        SuiTransactionBlockEffects::V1(_effects) => {
            match _effects.status {
                SuiExecutionStatus::Success => {
                    assert!(true);
                }
                SuiExecutionStatus::Failure { error } => {
                    assert!(false, "{}", error);
                }
            }
        }
    };
}