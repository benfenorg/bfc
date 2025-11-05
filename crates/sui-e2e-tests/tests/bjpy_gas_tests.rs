
mod upgrade_treasury_tests;
mod auth;
mod stable;
mod publish_coin;

use std::str::FromStr;
use std::time::Duration;
use std::vec;
use anyhow::{anyhow, Error};
use chrono::Utc;
use fastcrypto::encoding::Base64;
use jsonrpsee::http_client::HttpClient;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::TypeTag;
use move_core_types::parser::parse_struct_tag;
use sui_json_rpc_types::{ObjectChange, SuiExecutionStatus, SuiMoveStruct, SuiMoveValue, SuiObjectData, SuiObjectDataFilter, SuiObjectDataOptions, SuiObjectResponse, SuiObjectResponseQuery, SuiParsedData, SuiTransactionBlockEffects};
use sui_json_rpc_types::{SuiTransactionBlockResponseOptions, SuiTypeTag, TransactionBlockBytes};
use sui_macros::sim_test;
use sui_sdk::json::{type_args, SuiJsonValue};
use sui_test_transaction_builder::TestTransactionBuilder;
use sui_types::base_types::{ObjectID, ObjectRef, SuiAddress};
use sui_types::stable_coin::stable::checked::get_allow_stable_gas_coins_rate_map;
use sui_types::sui_serde::BigInt;
use sui_types::transaction::{CallArg, ObjectArg, TransactionKind, TransactionData, GasData, TEST_ONLY_GAS_UNIT_FOR_TRANSFER, TEST_ONLY_GAS_UNIT_FOR_OBJECT_BASICS, Command, ProgrammableMoveCall};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use test_cluster::{TestCluster, TestClusterBuilder};
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_types::{parse_sui_struct_tag, BFC_SYSTEM_PACKAGE_ID, BFC_SYSTEM_STATE_OBJECT_ID, SUI_CLOCK_OBJECT_ID};
use sui_types::utils::to_sender_signed_transaction_with_multi_signers;
use serde_json::json;
use sui_json_rpc_api::{CoinReadApiClient, IndexerApiClient, WriteApiClient};
use sui_json_rpc_api::TransactionBuilderClient;
use tokio::time::sleep;
use tracing::error;
use sui_keys::keystore::AccountKeystore;

#[sim_test]
async fn sim_test_with_other_coin_gas() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(6000)
        .with_num_validators(5)
        .with_all_vault_init()
        .build()
        .await;
    let mut http_client = test_cluster.rpc_client().clone();
    let address = test_cluster.get_address_0();
    let (package, _) = publish_coin::do_publish(&mut test_cluster,"tests/test_coin_code").await?;
    publish_coin::do_mint(&mut test_cluster, package).await;
    auth::auth_setup(&mut test_cluster, &mut http_client, address, "MINT-OTHER-STABLECOIN-POLLY").await?;
    sleep(Duration::from_secs(10)).await;
    let filter=format!("{}{}{}","0x2::coin::Coin<",package,"::test_coin::TEST_COIN>");

    let objects = get_owned_objects(filter.as_str(), &mut http_client, address).await?;
    println!("objects is {:?}",objects);
    assert_eq!(objects.len(), 1);
    let response = stable::mint_stable_coin_with_gas(100000000000, &test_cluster, &mut http_client, address, "0xc8::bjpy::BJPY", filter.as_str()).await;
    assert!(response.is_err());
    Ok(())
}

#[sim_test]
async fn sim_test_with_new_stable_coin_gas() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(6000)
        .with_num_validators(5)
        // .with_all_vault_init()
        .build()
        .await;
    let mut http_client = test_cluster.rpc_client().clone();
    let address = test_cluster.get_address_0();
    let (package, change_objs) = publish_coin::do_publish(&mut test_cluster,"tests/test_coin_code").await?;

    let mut coin_type: String = "".to_string();
    for ele in change_objs {
        if let ObjectChange::Created { object_type,..} = ele {
            if object_type.module.as_str() == "coin" {
                coin_type = object_type.type_params.get(0).unwrap().to_string();
                println!("object_type is {:?}", coin_type);
                break;
            }
        }
    }
    assert!(!coin_type.is_empty());

    publish_coin::do_mint(&mut test_cluster, package).await;
    publish_coin::do_mint(&mut test_cluster, package).await;
    auth::auth_setup(&mut test_cluster, &mut http_client, address, "MINT-BUSD-right_key").await?;
    stable::mint_stable_coin(100000000000, &test_cluster, &http_client, address, "0xc8::busd::BUSD").await?;

    test_cluster.wait_for_epoch(Some(2)).await;
    let filter=format!("{}{}{}","0x2::coin::Coin<",package,"::test_coin::TEST_COIN>");

    let objects = get_owned_objects(filter.as_str(), &mut http_client, address).await?;
    println!("objects is {:?}",objects);
    assert_eq!(objects.len(), 2);


    println!("coin_type is {:?}",coin_type);
    let response = test_move_call_add_external_stable_gas_coin(&mut test_cluster, coin_type.replace("0x", "")).await;
    assert!(response.is_ok());
    test_cluster.wait_for_epoch(Some(3)).await;


    let (package, _) = publish_coin::do_publish(&mut test_cluster,"tests/test_oracle_price").await?;

    test_cluster.wait_for_epoch(Some(4)).await;
    init_oracele_with_new_test_coin(&mut test_cluster, coin_type.replace("0x", ""), package).await;
    // wait to get oracle price and call bfc_round_v2
    test_cluster.wait_for_epoch(Some(5)).await;

    let data = get_allow_stable_gas_coins_rate_map();
    for ele in &data {
        println!("allow stable is {:?}",ele);
    }
    assert!(data.contains_key(&coin_type.replace("0x", "")));

    // // case 1 : transfer_sui
    // let objects = get_owned_objects(filter.as_str(), &mut http_client.clone(), address).await?;
    // let gas_object = objects.first().unwrap().object().unwrap();
    // println!("transfer_sui gas_object: {:?}", gas_object.to_string());
    // let context = &test_cluster.wallet;
    // let tx = test_cluster.wallet.sign_transaction(
    //     &TestTransactionBuilder::new(address, gas_object.object_ref(), context.get_reference_gas_price().await.unwrap())
    //         .transfer_sui(None, address)
    //         .build(),
    // );
    // let resp = test_cluster.execute_transaction(tx).await;
    // println!("transfer_sui resp: {:?}", resp);
    // assert!(resp.status_ok().unwrap());

    let response = test_move_call_new_test_coin_pool(&mut test_cluster, package, vec![TypeTag::from_str(&*coin_type)?]).await;
    assert!(response.is_ok());
    let pool_id = response.unwrap();

    // case 2 : call move function
    let response = test_move_call_use_new_test_coin(&mut test_cluster, package, vec![TypeTag::from_str(&*coin_type)?], pool_id).await;
    assert!(response.is_ok());

    let data = get_allow_stable_gas_coins_rate_map();
    for ele in &data {
        println!("allow stable is {:?}",ele);
    }
    assert!(data.contains_key(&coin_type.replace("0x", "")));

    // // delete allowed stable coin
    // let response = test_move_call_delete_external_stable_gas_coin(&mut test_cluster, coin_type.replace("0x", "")).await;
    // assert!(response.is_ok());
    //
    // test_cluster.wait_for_epoch(Some(12)).await;
    // let data = get_allow_stable_gas_coins_rate_map();
    // for ele in &data {
    //     println!("allow stable is {:?}",ele);
    // }
    // assert!(!data.contains_key(&coin_type.replace("0x", "")));
    // // sholud fail
    // let response = test_move_call_use_new_test_coin(&mut test_cluster, package, vec![TypeTag::from_str(&*coin_type)?]).await;
    // assert!(response.is_err());

    Ok(())
}


#[sim_test]
async fn sim_test_with_new_stable_coin_gas_check_gas_deposit() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(6000)
        .with_num_validators(5)
        // .with_all_vault_init()
        .build()
        .await;
    let mut http_client = test_cluster.rpc_client().clone();
    let address = test_cluster.get_address_0();
    let (package, change_objs) =
        publish_coin::do_publish(&mut test_cluster, "tests/test_coin_code").await?;

    let mut coin_type: String = "".to_string();
    for ele in change_objs {
        if let ObjectChange::Created { object_type, .. } = ele {
            if object_type.module.as_str() == "coin" {
                coin_type = object_type.type_params.get(0).unwrap().to_string();
                println!("object_type is {:?}", coin_type);
                break;
            }
        }
    }
    assert!(!coin_type.is_empty());

    publish_coin::do_mint(&mut test_cluster, package).await;
    publish_coin::do_mint(&mut test_cluster, package).await;
    auth::auth_setup(
        &mut test_cluster,
        &mut http_client,
        address,
        "MINT-BUSD-right_key",
    )
        .await?;
    stable::mint_stable_coin(100000000000, &test_cluster, &http_client, address, "0xc8::busd::BUSD").await?;

    test_cluster.wait_for_epoch(Some(2)).await;
    let filter = format!(
        "{}{}{}",
        "0x2::coin::Coin<", package, "::test_coin::TEST_COIN>"
    );

    let objects = get_owned_objects(filter.as_str(), &mut http_client, address).await?;
    println!("objects is {:?}", objects);
    assert_eq!(objects.len(), 2);

    println!("coin_type is {:?}", coin_type);
    let response =
        test_move_call_add_external_stable_gas_coin(&mut test_cluster, coin_type.replace("0x", ""))
            .await;
    assert!(response.is_ok());
    test_cluster.wait_for_epoch(Some(3)).await;

    let (package, _) =
        publish_coin::do_publish(&mut test_cluster, "tests/test_oracle_price").await?;

    test_cluster.wait_for_epoch(Some(4)).await;
    init_oracele_with_new_test_coin(&mut test_cluster, coin_type.replace("0x", ""), package).await;
    // wait to get oracle price and call bfc_round_v2
    test_cluster.wait_for_epoch(Some(5)).await;

    let data = get_allow_stable_gas_coins_rate_map();
    for ele in &data {
        println!("allow stable is {:?}", ele);
    }
    assert!(data.contains_key(&coin_type.replace("0x", "")));

    // Query extra_fields in bfc system
    let mut old_extra_fields_size = 0;
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
                .get_bfc_system_state_object_for_testing()
                .unwrap();
            let _extra_fields = _state.get_extra_fields();
            if let Some(extra_fields) = _extra_fields {
                println!("Extra fields size: {}", extra_fields.size);
                old_extra_fields_size = extra_fields.size;
            } else {
                println!("Extra fields not available (BFC system state V1)");
            }
    });

    // // case 1 : transfer_sui
    let objects = get_owned_objects(filter.as_str(), &mut http_client.clone(), address).await?;
    let gas_object = objects.first().unwrap().object().unwrap();
    println!("transfer_sui gas_object: {:?}", gas_object.to_string());
    let context = &test_cluster.wallet;
    let tx = test_cluster.wallet.sign_transaction(
        &TestTransactionBuilder::new(address, gas_object.object_ref(), context.get_reference_gas_price().await.unwrap())
            .transfer_sui(None, address)
            .build(),
    );
    let resp = test_cluster.execute_transaction(tx).await;
    println!("transfer_sui resp: {:?}", resp);
    assert!(resp.status_ok().unwrap());

    let response = test_move_call_new_test_coin_pool(
        &mut test_cluster,
        package,
        vec![TypeTag::from_str(&*coin_type)?],
    )
        .await;
    assert!(response.is_ok());

    let pool_id = response.unwrap();

    // case 2 : call move function
    let response = test_move_call_use_new_test_coin(
        &mut test_cluster,
        package,
        vec![TypeTag::from_str(&*coin_type)?],
        pool_id,
    )
    .await;
    assert!(response.is_ok());

    Ok(())
}

#[sim_test]
async fn sim_test_with_new_stable_coin_gas_check_gas_deposit_sponsored() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(6000)
        .with_num_validators(5)
        // .with_all_vault_init()
        .build()
        .await;
    let mut http_client = test_cluster.rpc_client().clone();
    let sender = test_cluster.get_address_0();
    let sponsor = test_cluster.get_address_1();
    let (package, change_objs) =
        publish_coin::do_publish(&mut test_cluster, "tests/test_coin_code").await?;

    let mut coin_type: String = "".to_string();
    for ele in change_objs {
        if let ObjectChange::Created { object_type, .. } = ele {
            if object_type.module.as_str() == "coin" {
                coin_type = object_type.type_params.get(0).unwrap().to_string();
                println!("object_type is {:?}", coin_type);
                break;
            }
        }
    }
    assert!(!coin_type.is_empty());

    publish_coin::do_mint(&mut test_cluster, package).await;
    publish_coin::do_mint(&mut test_cluster, package).await;
    auth::auth_setup(
        &mut test_cluster,
        &mut http_client,
        sender,
        "MINT-BUSD-right_key",
    )
        .await?;

    stable::mint_stable_coin(100000000000, &test_cluster, &http_client, sender, "0xc8::busd::BUSD").await?;

    test_cluster.wait_for_epoch(Some(2)).await;
    let filter = format!(
        "{}{}{}",
        "0x2::coin::Coin<", package, "::test_coin::TEST_COIN>"
    );

    let objects = get_owned_objects(filter.as_str(), &mut http_client, sender).await?;
    println!("objects is {:?}", objects);
    assert_eq!(objects.len(), 2);

    println!("coin_type is {:?}", coin_type);
    let response =
        test_move_call_add_external_stable_gas_coin(&mut test_cluster, coin_type.replace("0x", ""))
            .await;
    assert!(response.is_ok());
    test_cluster.wait_for_epoch(Some(3)).await;

    let (package, _) =
        publish_coin::do_publish(&mut test_cluster, "tests/test_oracle_price").await?;

    test_cluster.wait_for_epoch(Some(4)).await;
    init_oracele_with_new_test_coin(&mut test_cluster, coin_type.replace("0x", ""), package).await;
    // wait to get oracle price and call bfc_round_v2
    test_cluster.wait_for_epoch(Some(5)).await;

    let data = get_allow_stable_gas_coins_rate_map();
    for ele in &data {
        println!("allow stable is {:?}", ele);
    }
    assert!(data.contains_key(&coin_type.replace("0x", "")));

    // Query extra_fields in bfc system
    let mut old_extra_fields_size = 0;
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
                .get_bfc_system_state_object_for_testing()
                .unwrap();
            let _extra_fields = _state.get_extra_fields();
            if let Some(extra_fields) = _extra_fields {
                println!("Extra fields size: {}", extra_fields.size);
                old_extra_fields_size = extra_fields.size;
            } else {
                println!("Extra fields not available (BFC system state V1)");
            }
    });

    // Create sponsored transaction for transfer_sui
    let objects = get_owned_objects(filter.as_str(), &mut http_client.clone(), sender).await?;
    let sender_object = objects.first().unwrap().object().unwrap();
    
    // Get BFC coins from sponsor for gas payment (not test coins)
    let sponsor_bfc_objects = do_get_owned_objects_with_filter(
        "0x2::coin::Coin<0x2::bfc::BFC>",
        &mut http_client,
        sponsor,
    ).await?;
    
    if sponsor_bfc_objects.is_empty() {
        return Err(anyhow::anyhow!("Sponsor has no BFC coins for gas payment"));
    }
    
    let sponsor_gas_object = sponsor_bfc_objects.first().unwrap().object().unwrap();
    
    println!("Sponsored transfer sender_object: {:?}", sender_object.to_string());
    println!("Sponsored transfer sponsor_gas_object: {:?}", sponsor_gas_object.to_string());
    
    let context = &test_cluster.wallet;
    let rgp = context.get_reference_gas_price().await.unwrap();
    
    // Create sponsored transfer transaction
    let pt = {
        let mut builder = ProgrammableTransactionBuilder::new();
        builder.transfer_object(sender, sender_object.object_ref()).unwrap();
        builder.finish()
    };
    let kind = TransactionKind::programmable(pt);
    
    let tx_data = TransactionData::new_with_gas_data(
        kind,
        sender, // transaction sender
        GasData {
            payment: vec![sponsor_gas_object.object_ref()], // sponsor pays gas with BFC
            owner: sponsor, // sponsor owns the gas
            price: rgp,
            budget: rgp * TEST_ONLY_GAS_UNIT_FOR_TRANSFER,
        },
    );
    
    // Sign with both sender and sponsor using the correct multi-signer pattern
    let sponsored_tx = to_sender_signed_transaction_with_multi_signers(
        tx_data,
        vec![
            context.config.keystore.get_key(&sender).unwrap(),
            context.config.keystore.get_key(&sponsor).unwrap(),
        ],
    );
    let resp = test_cluster.execute_transaction(sponsored_tx).await;
    println!("Sponsored transfer_sui resp: {:?}", resp);
    assert!(resp.status_ok().unwrap());

    let response = test_move_call_new_test_coin_pool_sponsored(
        &mut test_cluster,
        package,
        vec![TypeTag::from_str(&*coin_type)?],
        sender,
        sponsor,
    )
        .await;
    assert!(response.is_ok());

    // Test calling the Move contract method to get balance
    let balance_result = test_move_call_get_deposited_balance(&mut test_cluster, coin_type.clone()).await;
    match &balance_result {
        Ok(balance) => {
            println!("Got balance from Move contract: {}", balance);
            assert!(*balance == 0);
        } 
        Err(e) => println!("Failed to get balance from Move contract: {:?}", e),
    }
    let old_gas_balance = balance_result.unwrap();
    assert!(old_gas_balance == 0);

    let pool_id = response.unwrap();

    // Sponsored move function call
    let response = test_move_call_use_new_test_coin_sponsored(
        &mut test_cluster,
        package,
        vec![TypeTag::from_str(&*coin_type)?],
        pool_id,
        sender,
        sponsor,
    )
    .await;
    assert!(response.is_ok());

    test_cluster.wait_for_epoch(Some(6)).await;
    // Query extra_fields in bfc system
    let mut new_extra_fields_size = 0;
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
                .get_bfc_system_state_object_for_testing()
                .unwrap();

            // Query extra_fields from BFC system state
            let _extra_fields = _state.get_extra_fields();
            println!("=============extra_fields: {:?}", &_extra_fields);

            // Check if extra_fields exists (only available in V2)
            if let Some(extra_fields) = _extra_fields {
                println!("Extra fields size: {}", extra_fields.size);
                new_extra_fields_size = extra_fields.size;
            } else {
                println!("Extra fields not available (BFC system state V1)");
            }
        });

        println!("old_extra_fields_size is {:?}, new_extra_fields_size is {:?}", old_extra_fields_size, new_extra_fields_size);
        // assert!(new_extra_fields_size > old_extra_fields_size,
        //     "Extra fields size should increase after adding a new stable gas coin"
        // );

    // Test calling the Move contract method to get balance
    let balance_result = test_move_call_get_deposited_balance(&mut test_cluster, coin_type).await;
    match &balance_result {
        Ok(balance) => {
            println!("Got balance from Move contract: {}", balance);
            assert!(*balance == 0);
        } 
        Err(e) => println!("Failed to get balance from Move contract: {:?}", e),
    }

    let new_gas_balance = balance_result.unwrap();
    assert!(new_gas_balance == 0);

    println!("old_gas_balance is {:?}, new_gas_balance is {:?}", old_gas_balance, new_gas_balance);
    assert!(new_gas_balance == old_gas_balance);

    Ok(())
}

#[sim_test]
async fn sim_test_with_new_stable_coin_gas_check_gas_deposit_sponsored_test_coin() -> Result<(), anyhow::Error> {
    let mut test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(6000)
        .with_num_validators(5)
        // .with_all_vault_init()
        .build()
        .await;
    let mut http_client = test_cluster.rpc_client().clone();
    let sender = test_cluster.get_address_0();
    let sponsor = test_cluster.get_address_1();
    let (test_coin_package, change_objs) =
        publish_coin::do_publish(&mut test_cluster, "tests/test_coin_code").await?;

    let mut coin_type: String = "".to_string();
    for ele in change_objs {
        if let ObjectChange::Created { object_type, .. } = ele {
            if object_type.module.as_str() == "coin" {
                coin_type = object_type.type_params.get(0).unwrap().to_string();
                println!("object_type is {:?}", coin_type);
                break;
            }
        }
    }
    assert!(!coin_type.is_empty());

    publish_coin::do_mint(&mut test_cluster, test_coin_package).await;
    publish_coin::do_mint(&mut test_cluster, test_coin_package).await;
    auth::auth_setup(
        &mut test_cluster,
        &mut http_client,
        sender,
        "MINT-BUSD-right_key",
    )
        .await?;

    stable::mint_stable_coin(100000000000, &test_cluster, &http_client, sender, "0xc8::busd::BUSD").await?;

    test_cluster.wait_for_epoch(Some(2)).await;
    let filter = format!(
        "{}{}{}",
        "0x2::coin::Coin<", test_coin_package, "::test_coin::TEST_COIN>"
    );

    let objects = get_owned_objects(filter.as_str(), &mut http_client, sender).await?;
    println!("objects is {:?}", objects);
    assert_eq!(objects.len(), 2);

    println!("coin_type is {:?}", coin_type);
    let response =
        test_move_call_add_external_stable_gas_coin(&mut test_cluster, coin_type.replace("0x", ""))
            .await;
    assert!(response.is_ok());
    test_cluster.wait_for_epoch(Some(3)).await;

    let (oracle_package, _) =
        publish_coin::do_publish(&mut test_cluster, "tests/test_oracle_price").await?;

    test_cluster.wait_for_epoch(Some(4)).await;
    init_oracele_with_new_test_coin(&mut test_cluster, coin_type.replace("0x", ""), oracle_package).await;
    // wait to get oracle price and call bfc_round_v2
    test_cluster.wait_for_epoch(Some(5)).await;

    let data = get_allow_stable_gas_coins_rate_map();
    for ele in &data {
        println!("allow stable is {:?}", ele);
    }
    assert!(data.contains_key(&coin_type.replace("0x", "")));

    // Query extra_fields in bfc system
    let mut old_extra_fields_size = 0;
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
                .get_bfc_system_state_object_for_testing()
                .unwrap();
            let _extra_fields = _state.get_extra_fields();
            if let Some(extra_fields) = _extra_fields {
                println!("Extra fields size: {}", extra_fields.size);
                old_extra_fields_size = extra_fields.size;
            } else {
                println!("Extra fields not available (BFC system state V1)");
            }
    });

    // Mint test coins for sender
    publish_coin::do_mint(&mut test_cluster, test_coin_package).await;
    publish_coin::do_mint(&mut test_cluster, test_coin_package).await;
    
    // For this test, we need the sponsor to have test coins for gas payment
    // Since do_mint only works for address_0, we'll transfer some test coins from sender to sponsor
    let sender_test_objects = get_owned_objects(filter.as_str(), &mut http_client.clone(), sender).await?;
    println!("Sender has {} test coin objects", sender_test_objects.len());
    assert!(sender_test_objects.len() >= 2, "Need at least 2 test coins to transfer to sponsor");
    
    // Transfer TWO test coins from sender to sponsor for gas payment (we need multiple for multiple transactions)
    let test_coin_to_transfer1 = sender_test_objects.get(0).unwrap().object().unwrap();
    let test_coin_to_transfer2 = sender_test_objects.get(1).unwrap().object().unwrap();
    let context = &test_cluster.wallet;
    
    // Transfer first test coin
    let transfer_gas1 = context
        .get_one_gas_object_owned_by_address(sender)
        .await
        .unwrap()
        .unwrap();
    
    let rgp = context.get_reference_gas_price().await.unwrap();
    let pt1 = {
        let mut builder = ProgrammableTransactionBuilder::new();
        builder.transfer_object(sponsor, test_coin_to_transfer1.object_ref()).unwrap();
        builder.finish()
    };
    let kind1 = TransactionKind::programmable(pt1);
    
    let tx_data1 = TransactionData::new_with_gas_data(
        kind1,
        sender,
        GasData {
            payment: vec![transfer_gas1],
            owner: sender,
            price: rgp,
            budget: rgp * TEST_ONLY_GAS_UNIT_FOR_TRANSFER,
        },
    );
    
    let transfer_tx1 = context.sign_transaction(&tx_data1);
    let transfer_resp1 = test_cluster.execute_transaction(transfer_tx1).await;
    println!("Transfer first test coin to sponsor resp: {:?}", transfer_resp1.status_ok());
    assert!(transfer_resp1.status_ok().unwrap());
    
    // Transfer second test coin
    let transfer_gas2 = context
        .get_one_gas_object_owned_by_address(sender)
        .await
        .unwrap()
        .unwrap();
    
    let pt2 = {
        let mut builder = ProgrammableTransactionBuilder::new();
        builder.transfer_object(sponsor, test_coin_to_transfer2.object_ref()).unwrap();
        builder.finish()
    };
    let kind2 = TransactionKind::programmable(pt2);
    
    let tx_data2 = TransactionData::new_with_gas_data(
        kind2,
        sender,
        GasData {
            payment: vec![transfer_gas2],
            owner: sender,
            price: rgp,
            budget: rgp * TEST_ONLY_GAS_UNIT_FOR_TRANSFER,
        },
    );
    
    let transfer_tx2 = context.sign_transaction(&tx_data2);
    let transfer_resp2 = test_cluster.execute_transaction(transfer_tx2).await;
    println!("Transfer second test coin to sponsor resp: {:?}", transfer_resp2.status_ok());
    assert!(transfer_resp2.status_ok().unwrap());
    
    let sponsor_http_client = test_cluster.rpc_client().clone();
    stable::mint_stable_coin_to_address(100000000000, &test_cluster, &sponsor_http_client, sender, "0xc8::busd::BUSD", sponsor).await?;

    // Create sponsored transaction using Test Coin as gas payment (not BFC)
    let objects = get_owned_objects(filter.as_str(), &mut http_client.clone(), sender).await?;
    let sender_object = objects.first().unwrap().object().unwrap();
    
    // Get Test coins from sponsor for gas payment (this is the key difference)
    let sponsor_test_objects = get_owned_objects(filter.as_str(), &mut http_client, sponsor).await?;
    
    if sponsor_test_objects.is_empty() {
        return Err(anyhow::anyhow!("Sponsor has no Test coins for gas payment"));
    }
    
    let sponsor_gas_object = sponsor_test_objects.first().unwrap().object().unwrap();
    
    println!("Sponsored transfer sender_object: {:?}", sender_object.to_string());
    println!("Sponsored transfer sponsor_gas_object (TEST_COIN): {:?}", sponsor_gas_object.to_string());
    
    // Create sponsored transfer transaction using Test Coin as gas
    let pt = {
        let mut builder = ProgrammableTransactionBuilder::new();
        builder.transfer_object(sender, sender_object.object_ref()).unwrap();
        builder.finish()
    };
    let kind = TransactionKind::programmable(pt);
    
    let tx_data = TransactionData::new_with_gas_data(
        kind,
        sender, // transaction sender
        GasData {
            payment: vec![sponsor_gas_object.object_ref()], // sponsor pays gas with Test Coins
            owner: sponsor, // sponsor owns the gas
            price: rgp,
            budget: rgp * TEST_ONLY_GAS_UNIT_FOR_TRANSFER,
        },
    );
    
    // Sign with both sender and sponsor using the correct multi-signer pattern
    let sponsored_tx = to_sender_signed_transaction_with_multi_signers(
        tx_data,
        vec![
            context.config.keystore.get_key(&sender).unwrap(),
            context.config.keystore.get_key(&sponsor).unwrap(),
        ],
    );
    let resp = test_cluster.execute_transaction(sponsored_tx).await;
    println!("Sponsored transfer with Test Coin gas resp: {:?}", resp);
    assert!(resp.status_ok().unwrap());

    let response = test_move_call_new_test_coin_pool_sponsored(
        &mut test_cluster,
        oracle_package,
        vec![TypeTag::from_str(&*coin_type)?],
        sender,
        sponsor,
    )
        .await;
    assert!(response.is_ok());

    let pool_id = response.unwrap();

    // Sponsored move function call using Test Coin as gas (the key difference)
    let response = test_move_call_use_new_test_coin_sponsored_with_test_coin_gas(
        &mut test_cluster,
        oracle_package, // Use oracle_package for Move contract call
        vec![TypeTag::from_str(&*coin_type)?],
        pool_id,
        sender,
        sponsor,
    )
    .await;
    println!("Response from use_new_test_coin_sponsored_with_test_coin_gas: {:?}", response);
    assert!(response.is_ok());

    test_cluster.wait_for_epoch(Some(6)).await;
    // Query extra_fields in bfc system
    let mut new_extra_fields_size = 0;
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
                .get_bfc_system_state_object_for_testing()
                .unwrap();

            // Query extra_fields from BFC system state
            let _extra_fields = _state.get_extra_fields();
            println!("=============extra_fields: {:?}", &_extra_fields);

            // Check if extra_fields exists (only available in V2)
            if let Some(extra_fields) = _extra_fields {
                println!("Extra fields size: {}", extra_fields.size);
                new_extra_fields_size = extra_fields.size;
            } else {
                println!("Extra fields not available (BFC system state V1)");
            }
        });

        println!("old_extra_fields_size is {:?}, new_extra_fields_size is {:?}", old_extra_fields_size, new_extra_fields_size);
        // Test coin gas deposit functionality in sponsored transactions

    println!("coin_type is {:?}", coin_type);

    Ok(())
}

async fn do_get_owned_objects_with_filter(filter_tag: &str, http_client: &HttpClient, address: SuiAddress) -> Result<Vec<SuiObjectResponse>, anyhow::Error> {
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
                Option::Some(filter),
                Option::Some(data_option),
            )),
            None,
            None,
        )
        .await?
        .data;
    Ok(objects)
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

#[allow(unused)]
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

async fn set_oracle_address(test_cluster: &mut TestCluster, oracle_address: String) -> Result<(), Error> {
    let module = "bfc_system".to_string();
    let package_id = BFC_SYSTEM_PACKAGE_ID;
    let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let context = &test_cluster.wallet;

    let http_client = test_cluster.rpc_client();
    let address = test_cluster.get_address_0();
    println!("address: {:?}", address);

    let bfc_objects = do_get_owned_objects_with_filter("0x2::coin::Coin<0x2::bfc::BFC>", http_client, address).await?;
    let gas = bfc_objects.first().unwrap().object().unwrap();

    let tx = context.sign_transaction(
        &http_client.move_call(
            address,
            package_id,
            module,
            "set_oracle_address".to_string(),
            type_args![]?,
            vec![
                SuiJsonValue::from_str(&bfc_status_address.to_string())?,
                SuiJsonValue::from_str(&oracle_address)?,
            ],
            Some(gas.object_id),
            10_000_00000.into(),
            None,
        ).await?.to_data()?,
    );
    let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();
    let tx_response = http_client
        .execute_transaction_block(
            tx_bytes,
            signatures,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    // println!("set_oracle_address tx_response: {:#?}", tx_response);

    assert!(tx_response.status_ok().unwrap());

    Ok(())
}

async fn init_oracele_with_new_test_coin(test_cluster: &mut TestCluster, test_coin: String, package: ObjectID) {
    let context = &test_cluster.wallet;
    let address = test_cluster.get_address_0();
    println!("address: {:?}", address);
    let mut gases = context
        .get_all_gas_objects_owned_by_address(address)
        .await
        .unwrap();
    let gas = gases.pop().unwrap();

    let tx = context.sign_transaction(
        &TestTransactionBuilder::new(address, gas, context.get_reference_gas_price().await.unwrap())
            .move_call(
                package,
                "test_oracle",
                "oracle_with_test_coin",
                vec![
                    CallArg::Pure(bcs::to_bytes(&test_coin).unwrap()),
                ],
            )
            .build(),
    );
    let resp = test_cluster.execute_transaction(tx).await;

    let oracle_id = resp.object_changes.unwrap().iter()
        .find(|change| match change {
            ObjectChange::Created {
                object_type, ..
            } => {
                object_type.to_string().contains("dynamic_field")
            }
            _ => false,
        }).unwrap().object_id();

    set_oracle_address(test_cluster, oracle_id.clone().to_bfc_address()).await.unwrap();

    let state = test_cluster.fullnode_handle.sui_node.state().clone();
    let bfc_sys_state = state.get_bfc_system_state_object_for_testing().unwrap();

    println!("bfc_sys_state.get_oracle_address(): {:#?}", &bfc_sys_state.get_oracle_address().unwrap());
    let price = state.get_oracle_price_by_id(ObjectID::from(bfc_sys_state.get_oracle_address().unwrap())).unwrap();
    println!("oracle price: {:?}", price.to_exchange_rate_against_busd());
    assert!(price.value.contents.len() > 0);
}

async fn test_move_call_use_new_test_coin(test_cluster: &mut TestCluster, package: ObjectID, mut type_args: Vec<TypeTag>, pool_id: (ObjectRef,ObjectRef)) -> Result<(), Error> {
    let context = &test_cluster.wallet;
    let address = test_cluster.get_address_0();
    let mut gases = test_cluster.rpc_client().clone().get_all_coins(address, None, None)
    .await
        .unwrap();

    let mut gas: Option<ObjectRef> = None;
    gases.data.retain(|e| {
        if e.coin_type.contains("test_coin::TEST_COIN") {
            gas = Some(e.object_ref());
            false
        } else {
            true
        }
    });

    assert!(gas.is_some());

    // //get busd
    // let busd_response_vec = do_get_owned_objects_with_filter(
    //     "0x2::coin::Coin<0xc8::busd::BUSD>",
    //     test_cluster.rpc_client(),
    //     address,
    // ).await?;
    // let busd_coin_id = busd_response_vec.first().unwrap().data.as_ref().unwrap().object_id;
    // println!("busd_coin_id is {:?}",busd_coin_id);

    type_args.insert(0,TypeTag::from_str("0xc8::busd::BUSD")?);

    let txn_data = TestTransactionBuilder::new(address, gas.unwrap(), context.get_reference_gas_price().await.unwrap())
        .move_call_with_split_gas_coins(
            package,
            "test_oracle",
            "stable_coin_test_swap",
            vec![
                CallArg::Object(ObjectArg::SharedObject{
                    id:pool_id.0.0,
                    initial_shared_version:pool_id.0.1,
                    mutable:true}
                ),
                CallArg::Object(ObjectArg::ImmOrOwnedObject(pool_id.1))
            ],
            type_args,
        )
        .build();
    tracing::error!("txn_data is {:?}",txn_data);
    let tx = context.sign_transaction(&txn_data);
    let resp = test_cluster.wallet.execute_transaction_may_fail(tx).await;
    tracing::error!("test_move_call_use_new_test_coin resp: {:#?}", resp);

    if  resp.is_err() {
        println!("test_move_call_use_new_test_coin resp: {:#?}", resp);
        return Err(resp.unwrap_err());
    }

    Ok(())
}


async fn test_move_call_new_test_coin_pool(test_cluster: &mut TestCluster, package: ObjectID, mut type_args: Vec<TypeTag>,) -> Result<(ObjectRef,ObjectRef), Error> {
    let context = &test_cluster.wallet;
    let address = test_cluster.get_address_0();
    let  gases = test_cluster.rpc_client().clone().get_all_coins(address, None, None)
        .await
        .unwrap();

    let mut gas: Option<ObjectRef> = None;
    gases.data.iter().for_each(|e| {
        if e.coin_type.contains("BFC") {
            gas = Some(e.object_ref());
        }
    });

    assert!(gas.is_some());

    let mut coin: Option<ObjectRef> = None;
    gases.data.iter().for_each(|e| {
        if e.coin_type.contains("BUSD") {
            coin = Some(e.object_ref());
        }
    });

    assert!(coin.is_some());

    type_args.insert(0,TypeTag::from_str("0xc8::busd::BUSD")?);

    let txn_data = TestTransactionBuilder::new(address, gas.unwrap(), context.get_reference_gas_price().await.unwrap())
        .move_call_with_tag(
            package,
            "test_oracle",
            "new_pool",
            type_args,
            vec![
                CallArg::Object(ObjectArg::ImmOrOwnedObject(coin.unwrap())),
            ],
        )
        .build();
    tracing::info!("txn_data is {:?}",txn_data);
    let tx = context.sign_transaction(&txn_data);
    let resp = test_cluster.wallet.execute_transaction_may_fail(tx).await;
    tracing::info!("test_move_call_use_new_test_coin resp: {:#?}", resp);

    if  resp.is_err() {
        println!("test_move_call_use_new_test_coin resp: {:#?}", resp);
        return Err(resp.unwrap_err());
    }

    let mut oracle_price: Option<ObjectRef> = None;
    let mut global: Option<ObjectRef> = None;
    for ele in  resp?.object_changes.unwrap() {
        if let ObjectChange::Created { object_id, version,digest,object_type,.. } = ele {
            if object_type.name == Identifier::from_str("TestOraclePrice").unwrap(){
                oracle_price = Some((object_id,version,digest));
            }
            if object_type.name == Identifier::from_str("Global").unwrap(){
                global = Some((object_id,version,digest));
            }
        }
    }
    if oracle_price.is_none() || global.is_none() {
        Err(anyhow!("not found"))
    } else {
        Ok((oracle_price.unwrap(), global.unwrap()))
    }

}


async fn test_move_call_add_external_stable_gas_coin(test_cluster: &mut TestCluster,  coin_type: String) -> Result<(), Error> {
    let context = &test_cluster.wallet;
    let address = test_cluster.get_address_0();
    let mut gases = test_cluster.rpc_client().clone().get_all_coins(address, None, None)
    .await
        .unwrap();

    let mut gas: Option<ObjectRef> = None;
    gases.data.retain(|e| {
        if e.coin_type.contains("BFC") {
            gas = Some(e.object_ref());
            false
        } else {
            true
        }
    });

    assert!(gas.is_some());

    let mut stable_coin_type: Vec<String> = vec![];
    stable_coin_type.push(coin_type);

    let tx = context.sign_transaction(
        &TestTransactionBuilder::new(address, gas.unwrap(), context.get_reference_gas_price().await.unwrap())
            .move_call(
                BFC_SYSTEM_PACKAGE_ID,
                "bfc_system",
                "add_external_stable_gas_coin",
                vec![
                    CallArg::BFC_SYSTEM_MUT,
                    CallArg::Pure(bcs::to_bytes(&stable_coin_type).unwrap()),
                ],
            )
            .build(),
    );
    let resp = test_cluster.execute_transaction(tx).await;
    // println!("test_move_call_add_external_stable_gas_coin resp: {:#?}", resp);

    assert!(resp.status_ok().unwrap());
    Ok(())
}
#[allow(dead_code)]
async fn test_move_call_delete_external_stable_gas_coin(test_cluster: &mut TestCluster,  coin_type: String) -> Result<(), Error> {
    let context = &test_cluster.wallet;
    let address = test_cluster.get_address_0();
    let mut gases = test_cluster.rpc_client().clone().get_all_coins(address, None, None)
    .await
        .unwrap();

    let mut gas: Option<ObjectRef> = None;
    gases.data.retain(|e| {
        if e.coin_type.contains("BFC") {
            gas = Some(e.object_ref());
            false
        } else {
            true
        }
    });

    assert!(gas.is_some());

    let tx = context.sign_transaction(
        &TestTransactionBuilder::new(address, gas.unwrap(), context.get_reference_gas_price().await.unwrap())
            .move_call(
                BFC_SYSTEM_PACKAGE_ID,
                "bfc_system",
                "delete_external_stable_gas_coin",
                vec![
                    CallArg::BFC_SYSTEM_MUT,
                    CallArg::Pure(bcs::to_bytes(&coin_type).unwrap()),
                ],
            )
            .build(),
    );
    let resp = test_cluster.execute_transaction(tx).await;
    println!("test_move_call_delete_external_stable_gas_coin resp: {:#?}", resp);

    assert!(resp.status_ok().unwrap());
    Ok(())
}

async fn test_move_call_get_deposited_balance(
    test_cluster: &mut TestCluster,
    coin_type: String
) -> Result<u64, Error> {
    let address = test_cluster.get_address_0();
    let mut gases = test_cluster.rpc_client().clone().get_all_coins(address, None, None)
        .await
        .unwrap();

    let mut gas: Option<ObjectRef> = None;
    gases.data.retain(|e| {
        if e.coin_type.contains("BFC") {
            gas = Some(e.object_ref());
            false
        } else {
            true
        }
    });
    assert!(gas.is_some());

    let pt = {
        let mut builder = ProgrammableTransactionBuilder::new();
        builder.move_call(
            BFC_SYSTEM_PACKAGE_ID,
            Identifier::new("bfc_system").unwrap(),
            Identifier::new("get_deposited_stable_gas_coin_balance").unwrap(),
            vec![TypeTag::from_str(&coin_type)?],
            vec![CallArg::BFC_SYSTEM_MUT],
        )?;
        builder.finish()
    };
    
    let txn = TransactionKind::programmable(pt);
    let response = test_cluster
        .rpc_client()
        .dev_inspect_transaction_block(
            address,
            Base64::from_bytes(&bcs::to_bytes(&txn).unwrap()),
            None,
            None,
            None,
        )
        .await
        .unwrap();

    let results = response.results.unwrap();
    let return_value = &results.first().unwrap().return_values.first().unwrap().0;
    let balance: u64 = bcs::from_bytes(return_value).unwrap();
    Ok(balance)
}

async fn test_move_call_new_test_coin_pool_sponsored(
    test_cluster: &mut TestCluster, 
    package: ObjectID, 
    mut type_args: Vec<TypeTag>,
    sender: SuiAddress,
    sponsor: SuiAddress,
) -> Result<(ObjectRef,ObjectRef), Error> {
    let context = &test_cluster.wallet;
    let gases = test_cluster.rpc_client().clone().get_all_coins(sender, None, None)
        .await
        .unwrap();

    let mut gas: Option<ObjectRef> = None;
    gases.data.iter().for_each(|e| {
        if e.coin_type.contains("BFC") {
            gas = Some(e.object_ref());
        }
    });

    assert!(gas.is_some());

    let mut coin: Option<ObjectRef> = None;
    gases.data.iter().for_each(|e| {
        if e.coin_type.contains("BUSD") {
            coin = Some(e.object_ref());
        }
    });

    assert!(coin.is_some());

    // Get sponsor gas - use BFC coins for gas payment
    let sponsor_bfc_objects = do_get_owned_objects_with_filter(
        "0x2::coin::Coin<0x2::bfc::BFC>",
        test_cluster.rpc_client(),
        sponsor,
    ).await?;

    if sponsor_bfc_objects.is_empty() {
        return Err(anyhow::anyhow!("Sponsor has no BFC coins for gas payment"));
    }

    let sponsor_gas = sponsor_bfc_objects.first().unwrap().object().unwrap().object_ref();

    type_args.insert(0,TypeTag::from_str("0xc8::busd::BUSD")?);

    let rgp = context.get_reference_gas_price().await.unwrap();
    
    // Build the programmable transaction directly
    let pt = {
        let mut builder = ProgrammableTransactionBuilder::new();
        
        // Add the BUSD coin as input - builder.input returns Argument
        let coin_input = builder.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(coin.unwrap()))).unwrap();
        
        // Call the move function with the coin directly
        builder.command(Command::MoveCall(Box::new(
            ProgrammableMoveCall {
                package,
                module: Identifier::new("test_oracle").unwrap().to_string(),
                function: Identifier::new("new_pool").unwrap().to_string(),
                type_arguments: type_args.into_iter().map(|t| t.into()).collect(),
                arguments: vec![
                    coin_input,
                ],
            },
        )));
        
        builder.finish()
    };

    let kind = TransactionKind::programmable(pt);

    // Create sponsored transaction
    let tx_data = TransactionData::new_with_gas_data(
        kind,
        sender,
        GasData {
            payment: vec![sponsor_gas],
            owner: sponsor,
            price: rgp,
            budget: rgp * TEST_ONLY_GAS_UNIT_FOR_OBJECT_BASICS,
        },
    );

    // Sign with both sender and sponsor using the correct multi-signer pattern
    let sponsored_tx = to_sender_signed_transaction_with_multi_signers(
        tx_data,
        vec![
            context.config.keystore.get_key(&sender).unwrap(),
            context.config.keystore.get_key(&sponsor).unwrap(),
        ],
    );
    let resp = test_cluster.execute_transaction(sponsored_tx).await;
    tracing::info!("test_move_call_new_test_coin_pool_sponsored resp: {:#?}", resp);

    if  resp.status_ok().is_none() || !resp.status_ok().unwrap() {
        println!("test_move_call_new_test_coin_pool_sponsored resp: {:#?}", resp);
        return Err(anyhow::anyhow!("Transaction failed"));
    }

    let mut oracle_price: Option<ObjectRef> = None;
    let mut global: Option<ObjectRef> = None;
    for ele in  resp.object_changes.unwrap() {
        if let ObjectChange::Created { object_id, version,digest,object_type,.. } = ele {
            if object_type.name == Identifier::from_str("TestOraclePrice").unwrap(){
                oracle_price = Some((object_id,version,digest));
            }
            if object_type.name == Identifier::from_str("Global").unwrap(){
                global = Some((object_id,version,digest));
            }
        }
    }
    if oracle_price.is_none() || global.is_none() {
        Err(anyhow!("not found"))
    } else {
        Ok((oracle_price.unwrap(), global.unwrap()))
    }
}

async fn test_move_call_use_new_test_coin_sponsored(
    test_cluster: &mut TestCluster, 
    package: ObjectID, 
    mut type_args: Vec<TypeTag>, 
    pool_id: (ObjectRef,ObjectRef),
    sender: SuiAddress,
    sponsor: SuiAddress,
) -> Result<(), Error> {
    let context = &test_cluster.wallet;
    let mut gases = test_cluster.rpc_client().clone().get_all_coins(sender, None, None)
        .await
        .unwrap();

    let mut gas: Option<ObjectRef> = None;
    gases.data.retain(|e| {
        if e.coin_type.contains("test_coin::TEST_COIN") {
            gas = Some(e.object_ref());
            false
        } else {
            true
        }
    });

    assert!(gas.is_some());

    // Get sponsor gas - use BFC coins for gas payment
    let sponsor_bfc_objects = do_get_owned_objects_with_filter(
        "0x2::coin::Coin<0x2::bfc::BFC>",
        test_cluster.rpc_client(),
        sponsor,
    ).await?;

    if sponsor_bfc_objects.is_empty() {
        return Err(anyhow::anyhow!("Sponsor has no BFC coins for gas payment"));
    }

    let sponsor_gas = sponsor_bfc_objects.first().unwrap().object().unwrap().object_ref();

    type_args.insert(0,TypeTag::from_str("0xc8::busd::BUSD")?);

    let rgp = context.get_reference_gas_price().await.unwrap();
    
    // Build the programmable transaction directly instead of using TestTransactionBuilder
    let pt = {
        let mut builder = ProgrammableTransactionBuilder::new();
        
        // Add the test coin as input (not split from gas coin)
        let test_coin_input = builder.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(gas.unwrap()))).unwrap();
        
        // Add shared object input
        let shared_object_input = builder.input(CallArg::Object(ObjectArg::SharedObject{
            id: pool_id.0.0,
            initial_shared_version: pool_id.0.1,
            mutable: true,
        })).unwrap();
        
        // Add owned object input
        let owned_object_input = builder.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(pool_id.1))).unwrap();
        
        // Call the move function with the test coin directly (no splitting)
        builder.command(Command::MoveCall(Box::new(
            ProgrammableMoveCall {
                package,
                module: Identifier::new("test_oracle").unwrap().to_string(),
                function: Identifier::new("stable_coin_test_swap").unwrap().to_string(),
                type_arguments: type_args.into_iter().map(|t| t.into()).collect(),
                arguments: vec![
                    test_coin_input,
                    shared_object_input,
                    owned_object_input,
                ],
            },
        )));
        
        builder.finish()
    };

    let kind = TransactionKind::programmable(pt);

    // Create sponsored transaction
    let tx_data = TransactionData::new_with_gas_data(
        kind,
        sender,
        GasData {
            payment: vec![sponsor_gas],
            owner: sponsor,
            price: rgp,
            budget: rgp * TEST_ONLY_GAS_UNIT_FOR_OBJECT_BASICS,
        },
    );

    // Sign with both sender and sponsor using the correct multi-signer pattern
    let sponsored_tx = to_sender_signed_transaction_with_multi_signers(
        tx_data,
        vec![
            context.config.keystore.get_key(&sender).unwrap(),
            context.config.keystore.get_key(&sponsor).unwrap(),
        ],
    );
    let resp = test_cluster.execute_transaction(sponsored_tx).await;
    tracing::error!("test_move_call_use_new_test_coin_sponsored resp: {:#?}", resp);

    if  resp.status_ok().is_none() || !resp.status_ok().unwrap() {
        println!("test_move_call_use_new_test_coin_sponsored resp: {:#?}", resp);
        return Err(anyhow::anyhow!("Transaction failed"));
    }

    Ok(())
}

async fn test_move_call_use_new_test_coin_sponsored_with_test_coin_gas(
    test_cluster: &mut TestCluster, 
    package: ObjectID, 
    mut type_args: Vec<TypeTag>, 
    pool_id: (ObjectRef,ObjectRef),
    sender: SuiAddress,
    sponsor: SuiAddress,
) -> Result<(), Error> {
    let context = &test_cluster.wallet;
    let mut gases = test_cluster.rpc_client().clone().get_all_coins(sender, None, None)
        .await
        .unwrap();

    let mut gas: Option<ObjectRef> = None;
    gases.data.retain(|e| {
        if e.coin_type.contains("test_coin::TEST_COIN") {
            gas = Some(e.object_ref());
            false
        } else {
            true
        }
    });

    assert!(gas.is_some());

    // Get sponsor gas - use Test Coins for gas payment (not BFC)
    let sponsor_gases = test_cluster.rpc_client().clone().get_all_coins(sponsor, None, None)
        .await
        .unwrap();
    let mut sponsor_gas: Option<ObjectRef> = None;
    sponsor_gases.data.iter().for_each(|e| {
        if e.coin_type.contains("test_coin::TEST_COIN") && sponsor_gas.is_none() {
            sponsor_gas = Some(e.object_ref());
        }
    });
    if sponsor_gas.is_none() {
        return Err(anyhow::anyhow!("Sponsor has no Test coins for gas payment"));
    }
    let sponsor_gas = sponsor_gas.unwrap();

    type_args.insert(0,TypeTag::from_str("0xc8::busd::BUSD")?);

    let rgp = context.get_reference_gas_price().await.unwrap();
    
    // Build the programmable transaction directly instead of using TestTransactionBuilder
    let pt = {
        let mut builder = ProgrammableTransactionBuilder::new();
        
        // Add the test coin as input (not split from gas coin)
        let test_coin_input = builder.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(gas.unwrap()))).unwrap();
        
        // Add shared object input
        let shared_object_input = builder.input(CallArg::Object(ObjectArg::SharedObject{
            id: pool_id.0.0,
            initial_shared_version: pool_id.0.1,
            mutable: true,
        })).unwrap();
        
        // Add owned object input
        let owned_object_input = builder.input(CallArg::Object(ObjectArg::ImmOrOwnedObject(pool_id.1))).unwrap();
        
        // Call the move function with the test coin directly (no splitting)
        builder.command(Command::MoveCall(Box::new(
            ProgrammableMoveCall {
                package,
                module: Identifier::new("test_oracle").unwrap().to_string(),
                function: Identifier::new("stable_coin_test_swap").unwrap().to_string(),
                type_arguments: type_args.into_iter().map(|t| t.into()).collect(),
                arguments: vec![
                    test_coin_input,
                    shared_object_input,
                    owned_object_input,
                ],
            },
        )));
        
        builder.finish()
    };

    let kind = TransactionKind::programmable(pt);

    // Create sponsored transaction using Test Coin as gas
    let tx_data = TransactionData::new_with_gas_data(
        kind,
        sender,
        GasData {
            payment: vec![sponsor_gas],
            owner: sponsor,
            price: rgp,
            budget: rgp * TEST_ONLY_GAS_UNIT_FOR_OBJECT_BASICS,
        },
    );

    // Sign with both sender and sponsor using the correct multi-signer pattern
    let sponsored_tx = to_sender_signed_transaction_with_multi_signers(
        tx_data,
        vec![
            context.config.keystore.get_key(&sender).unwrap(),
            context.config.keystore.get_key(&sponsor).unwrap(),
        ],
    );
    let resp = test_cluster.execute_transaction(sponsored_tx).await;
    tracing::error!("test_move_call_use_new_test_coin_sponsored_with_test_coin_gas resp: {:#?}", resp);

    if  resp.status_ok().is_none() || !resp.status_ok().unwrap() {
        println!("test_move_call_use_new_test_coin_sponsored_with_test_coin_gas resp: {:#?}", resp);
        return Err(anyhow::anyhow!("Transaction failed"));
    }

    Ok(())
}
