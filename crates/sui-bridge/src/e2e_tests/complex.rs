// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::abi::{eth_sui_bridge, EthSuiBridge};
use crate::client::bridge_authority_aggregator::BridgeAuthorityAggregator;
use crate::e2e_tests::{auth, stable};
use crate::e2e_tests::test_utils::{
    initiate_bridge_eth_to_sui, initiate_bridge_sui_to_eth, initiate_defi_bridge_unstake_sui_to_eth, BridgeTestCluster, BridgeTestClusterBuilder
};
use crate::events::TokenTransferApproved;
use crate::sui_transaction_builder::build_sui_transaction;
use crate::types::{BridgeAction, EmergencyAction};
use crate::types::{BridgeActionStatus, EmergencyActionType};
use ethers::types::Address as EthAddress;
use sui_types::BRIDGE_PACKAGE_ID;
use tap::TapFallible;
use std::sync::Arc;
use std::collections::HashSet;
use sui_json_rpc_types::{SuiExecutionStatus, TransactionBlockBytes};
use sui_json_rpc_types::SuiTransactionBlockEffectsAPI;
use move_core_types::ident_str;
use move_core_types::language_storage::TypeTag;
use sui_types::bridge::{BridgeChainId, TOKEN_ID_ETH};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{CallArg, ObjectArg, TransactionData, TransactionKind};
use sui_types::{SUI_CLOCK_OBJECT_ID, SUI_CLOCK_OBJECT_SHARED_VERSION};
use sui_types::{BFC_SYSTEM_STATE_OBJECT_ID, BFC_SYSTEM_STATE_OBJECT_SHARED_VERSION};
use tracing::info;
use std::str::FromStr;
use std::time::Duration;
use sui_types::base_types::{ObjectID};
use test_cluster::{TestClusterBuilder};
use tracing::{error};
use sui_sdk::json::{SuiJsonValue};
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use serde_json::json;
use sui_json_rpc_api::WriteApiClient;
use sui_json_rpc_api::TransactionBuilderClient;
use sui_json_rpc_types::{SuiTransactionBlockResponseOptions, SuiTypeTag};


/// 辅助方法：在以太坊上投资已桥接的代币
#[allow(dead_code)]
async fn invest_bridged_tokens_on_eth(
    bridge_test_cluster: &BridgeTestCluster,
    event_seq_num: u64,
) -> Result<ethers::types::TransactionReceipt, anyhow::Error> {
    // 获取解析后的消息
    let parsed_message = bridge_test_cluster.bridge_client()
        .get_parsed_defi_transfer_out_message(bridge_test_cluster.sui_chain_id() as u8, event_seq_num)
        .await;
    assert!(parsed_message.is_ok(), "Parsed message should be found, but got error: {:?}", parsed_message.err());
    let parsed_message = parsed_message.unwrap();
    assert!(parsed_message.is_some(), "Parsed message should be found,bug got none");
    let parsed_message = parsed_message.unwrap();
    println!("Parsed message: {:?}", parsed_message);
    
    // 获取签名
    let sigs = bridge_test_cluster.bridge_client()
        .get_defi_transfer_out_action_onchain_signatures_until_success(bridge_test_cluster.sui_chain_id() as u8, event_seq_num)
        .await;
    assert!(sigs.is_some(), "Signatures should be found");
    let sigs = sigs.unwrap();
    println!("Signatures: {:?}", sigs);
    
    // 转换消息和签名格式
    let message = eth_sui_bridge::Message::from(parsed_message);
    let signatures = sigs
        .into_iter()
        .map(|sig: Vec<u8>| ethers::types::Bytes::from(sig))
        .collect::<Vec<_>>();
    
    // 获取以太坊签名者并创建桥接合约实例
    let (eth_signer, _) = bridge_test_cluster
        .get_eth_signer_and_address()
        .await
        .unwrap();
    let eth_sui_bridge = EthSuiBridge::new(
        bridge_test_cluster.eth_env().contracts().sui_bridge,
        Arc::new(eth_signer),
    );
    
    // 发送交易
    let tx = eth_sui_bridge.invest_bridged_tokens_with_signatures(signatures, message);
    let tx_response = tx.send().await.unwrap().await.unwrap().unwrap();
    println!("Tx response: {:?}", tx_response);
    
    Ok(tx_response)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 16)]
async fn test_sui_bridge_paused() {
    telemetry_subscribers::init_for_testing();

    // approve pause action in bridge nodes
    let pause_action = BridgeAction::EmergencyAction(EmergencyAction {
        nonce: 0,
        chain_id: BridgeChainId::SuiCustom,
        action_type: EmergencyActionType::Pause,
    });

    let unpause_action = BridgeAction::EmergencyAction(EmergencyAction {
        nonce: 1,
        chain_id: BridgeChainId::SuiCustom,
        action_type: EmergencyActionType::Unpause,
    });

    // Setup bridge test env
    let bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(true)
        .with_num_validators(4)
        .with_approved_governance_actions(vec![
            vec![pause_action.clone(), unpause_action.clone()],
            vec![unpause_action.clone()],
            vec![unpause_action.clone()],
            vec![],
        ])
        .build()
        .await;

    let bridge_client = bridge_test_cluster.bridge_client();
    let sui_address = bridge_test_cluster.sui_user_address();
    let sui_token_type_tags = bridge_client.get_token_id_map().await.unwrap();

    // verify bridge are not paused
    assert!(!bridge_client.get_bridge_summary().await.unwrap().is_frozen);

    // try bridge from eth and verify it works on sui
    initiate_bridge_eth_to_sui(&bridge_test_cluster, 10, 0,false)
        .await
        .unwrap();
    // verify Eth was transferred to Sui address
    let eth_coin_type = sui_token_type_tags.get(&TOKEN_ID_ETH).unwrap();
    let eth_coin = bridge_client
        .sui_client()
        .coin_read_api()
        .get_coins(sui_address, Some(eth_coin_type.to_string()), None, None)
        .await
        .unwrap()
        .data;
    assert_eq!(1, eth_coin.len());

    // get pause bridge signatures from committee
    let bridge_committee = Arc::new(bridge_client.get_bridge_committee().await.unwrap());
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_action = agg
        .request_committee_signatures(pause_action)
        .await
        .unwrap();

    // execute pause bridge on sui
    let gas = bridge_test_cluster
        .wallet()
        .get_one_gas_object_owned_by_address(sui_address)
        .await
        .unwrap()
        .unwrap();

    let tx = build_sui_transaction(
        sui_address,
        &gas,
        certified_action,
        bridge_client
            .get_mutable_bridge_object_arg_must_succeed()
            .await,
        None,
        &sui_token_type_tags,
        1000,
    )
    .unwrap();

    let response = bridge_test_cluster.sign_and_execute_transaction(&tx).await;
    assert_eq!(
        response.effects.unwrap().status(),
        &SuiExecutionStatus::Success
    );
    info!("Bridge paused");

    // verify bridge paused
    assert!(bridge_client.get_bridge_summary().await.unwrap().is_frozen);

    // Transfer from eth to sui should fail on Sui
    let eth_to_sui_bridge_action = initiate_bridge_eth_to_sui(&bridge_test_cluster, 10, 1,false).await;
    assert!(eth_to_sui_bridge_action.is_err());
    // message should not be recorded on Sui when the bridge is paused
    let res = bridge_test_cluster
        .bridge_client()
        .get_token_transfer_action_onchain_status_until_success(
            bridge_test_cluster.eth_chain_id() as u8,
            1,
        )
        .await;
    assert_eq!(BridgeActionStatus::NotFound, res);
    // Transfer from Sui to eth should fail
    let sui_to_eth_bridge_action = initiate_bridge_sui_to_eth(
        &bridge_test_cluster,
        EthAddress::random(),
        eth_coin.first().unwrap().object_ref(),
        0,
        10,
        TOKEN_ID_ETH,
    )
    .await;
    assert!(sui_to_eth_bridge_action.is_err())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 16)]
async fn test_bridge_defi_stake_e2e() -> Result<(), anyhow::Error> {
    telemetry_subscribers::init_for_testing();
    // Setup bridge test env
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
    // .with_eth_env(true)
    .with_bridge_cluster(true)
    .with_num_validators(4)
    .build()
    .await;

    let address = bridge_test_cluster.sui_user_address();
    let http_client = bridge_test_cluster.test_cluster.inner.rpc_client().clone();
    let minter_address = bridge_test_cluster.minter_address.unwrap();
    let minter_key_pair = bridge_test_cluster.minter_key_pair.as_ref().unwrap();

    // Step 2: Mint BUSD tokens for DeFi staking
    let busd_amount = 50_000_000_000u64; // 500 BUSD for testing
    println!("Minting {} BUSD tokens for DeFi staking...", busd_amount);
    stable::mint_stable_coin_to_address(
        busd_amount,
        &http_client,
        minter_address,
        minter_key_pair,
        "0xc8::busd::BUSD",
        address,
    )
    .await?;

    // Step 3: Verify BUSD tokens were minted successfully
    let busd_objects = auth::do_get_owned_objects_with_filter(
        "0x2::coin::Coin<0xc8::busd::BUSD>",
        &http_client,
        address
    ).await?;
    assert!(busd_objects.len() >= 1, "Should have at least one BUSD coin object");

    let busd_coin = busd_objects.first().unwrap().object().unwrap();
    let busd_balance = auth::get_balance(busd_coin);
    assert!(busd_balance > 0, "BUSD balance should be greater than 0");

    // Step 4: Get bridge object for DeFi staking
    let bridge_object_arg = bridge_test_cluster
        .bridge_client()
        .get_mutable_bridge_object_arg_must_succeed()
        .await;
    let bridge_object_id = match bridge_object_arg {
        sui_types::transaction::ObjectArg::SharedObject { id, .. } => id,
        _ => panic!("Bridge object is not a shared object"),
    };

    // Step 5: Setup DeFi staking parameters
    let target_chain = 12u8; // ETH Custom chain (matches defi_protocols::initial_defi_protocol)
    let protocol_type = 1u64; // AAVE protocol type
    let protocol_version = 3u64; // Version 3
    let protocol_token_id = 3u64; // USDC token ID for DeFi protocol
    let stake_amount = busd_amount / 2; // Use half of the BUSD for staking

    // Step 6: Call defi_stake Move function
    info!("Calling defi_stake Move function with parameters:");
    info!("  - Target chain: {}", target_chain);
    info!("  - Protocol type: {}", protocol_type);
    info!("  - Protocol version: {}", protocol_version);
    info!("  - Protocol token ID: {}", protocol_token_id);
    info!("  - Stake amount: {}", stake_amount);

    let mut builder = ProgrammableTransactionBuilder::new();

    let bridge_arg = builder.obj(bridge_object_arg).unwrap();
    let bfc_system_state_arg = builder.obj(ObjectArg::SharedObject {
        id: BFC_SYSTEM_STATE_OBJECT_ID,
        initial_shared_version: BFC_SYSTEM_STATE_OBJECT_SHARED_VERSION,
        mutable: true,
    }).unwrap();
    let busd_arg = builder.obj(ObjectArg::ImmOrOwnedObject(busd_coin.object_ref())).unwrap();

    let target_chain_arg = builder.pure(target_chain).unwrap();
    let protocol_type_arg = builder.pure(protocol_type).unwrap();
    let protocol_version_arg = builder.pure(protocol_version).unwrap();
    let protocol_token_id_arg = builder.pure(protocol_token_id).unwrap();

    builder.programmable_move_call(
        BRIDGE_PACKAGE_ID,
        ident_str!("bridge").to_owned(),
        ident_str!("defi_stake").to_owned(),
        vec![TypeTag::from_str("0xc8::busd::BUSD").unwrap()],
        vec![
            bridge_arg,
            bfc_system_state_arg,
            target_chain_arg,
            busd_arg,
            protocol_type_arg,
            protocol_version_arg,
            protocol_token_id_arg,
        ],
    );
    let pt = builder.finish();
    let gas = bridge_test_cluster.test_cluster.inner
        .wallet
        .get_one_gas_object_owned_by_address(address)
        .await
        .unwrap()
        .unwrap();

    let tx_data = TransactionData::new_programmable(
        address,
        vec![gas],
        pt,
        50_000_000_000,
        bridge_test_cluster.test_cluster.inner.get_reference_gas_price().await,
    );

    // Step 7: Sign and execute the DeFi stake transaction
    let tx = bridge_test_cluster.test_cluster.inner.wallet.sign_transaction(&tx_data);
    let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();
    let tx_response = http_client
        .execute_transaction_block(
            tx_bytes,
            signatures,
            Some(SuiTransactionBlockResponseOptions::new().with_effects().with_events()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;

    // Step 8: Verify transaction execution
    let effects = tx_response.effects.as_ref().unwrap();
    let defi_stake_succeeded = match effects.status() {
        SuiExecutionStatus::Success => {
            true
        },
        SuiExecutionStatus::Failure { error } => {
            error!("❌ DeFi stake transaction failed: {}", error);
            info!("Note: This is expected if DeFi protocol configuration is not initialized in test environment");
            info!("The test will skip the approve_defi_transfer_out verification");
            false
        }
    };
    assert!(defi_stake_succeeded, "DeFi stake transaction should succeed");

    //  Check for DeFi bridge events (if transaction was successful)
    if let Some(events) = &tx_response.events {
        for (idx, event) in events.data.iter().enumerate() {
            // Look for DeFi-related events
            if event.type_.address.to_string().contains("bridge") ||
               event.type_.module.as_str().contains("bridge") ||
               event.type_.name.as_str().contains("defi") {
                info!("🌉 Found bridge/DeFi related event: {:?}", event);
            }
        }
    }


    println!("=== Step 4: Extract DefiTransferOutEvent from transaction ===");
    let events = tx_response.events.as_ref().expect("Should have events");
    let mut defi_event = None;
    let mut event_seq_num = 0u64;

    let mut amount_after_fee=0;

    for (idx, event) in events.data.iter().enumerate() {
        if event.type_.name.as_str() == "DefiTransferOutEvent" {
            defi_event = Some(event.clone());

            // Parse event to get seq_num
            if let Ok(parsed_event) = bcs::from_bytes::<crate::events::MoveDefiTransferOutEvent>(&event.bcs.bytes()) {
                event_seq_num = parsed_event.seq_num;
                amount_after_fee=parsed_event.amount_after_fee;
            }
            break;
        }
    }

    assert!(defi_event.is_some(), "DefiTransferOutEvent should be emitted");

    // Give bridge cluster enough time to process the action
    tokio::time::sleep(Duration::from_secs(15)).await;

    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                crate::events::TokenTransferApproved.get().unwrap().clone(),
            ]),
            true,
        )
        .await;

    assert!(!events.is_empty(), "Should have TokenTransferApproved event");
    // Verify the record has verified signatures now
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Check that the action status is approved (not pending anymore)
    // For DeFi messages, we need to use a different query function
    let status = bridge_test_cluster
        .bridge_client()
        .get_defi_transfer_action_status_until_success(
            bridge_test_cluster.sui_chain_id() as u8,
            event_seq_num,
        )
        .await;

    println!("DeFi transfer action status: {:?}", status);
    // The status should be Approved (signatures verified) but not Claimed
    // because this is a Sui->Eth transfer that will be claimed on the Eth side
    assert_eq!(status, BridgeActionStatus::Approved, "Action should be approved");
    
    let tx_response = invest_bridged_tokens_on_eth(&bridge_test_cluster, event_seq_num).await?;
    if let Some(log) = tx_response.logs.last() {
        let event = decode_tokens_staked_event(log)?;
        assert_eq!(event.0,BridgeChainId::EthCustom as u8);
        assert_eq!(event.1,0); //evm
        assert_eq!(event.2,BridgeChainId::SuiCustom as u8);
        assert_eq!(event.3,event_seq_num); // from benfen
        assert_eq!(event.5,bridge_test_cluster.eth_env().contracts().arrow);
        assert_eq!(event.6,0);
        assert_eq!(event.7,amount_after_fee);
        assert_eq!(event.8,protocol_type);
        assert_eq!(event.9,protocol_version);
        assert_eq!(event.10,protocol_token_id);
        assert_eq!(event.11,0);    
    }

    // Poll until the DeFi action becomes Claimed (with a timeout)
    let mut attempts = 0;
    let max_attempts = 60; // ~2 minutes if sleeping 2s between polls
    let mut status = BridgeActionStatus::NotFound;
    loop {
        status = bridge_test_cluster
            .bridge_client()
            .get_defi_transfer_action_status_until_success(
                bridge_test_cluster.eth_chain_id() as u8,
                event_seq_num,
            )
            .await;
        if status == BridgeActionStatus::Claimed {
            break;
        }
        attempts += 1;
        if attempts >= max_attempts {
            panic!(
                "Timed out waiting for DeFi transfer action to become Claimed (last status: {:?})",
                status
            );
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    // The status should be Claimed now
    assert_eq!(status, BridgeActionStatus::Claimed, "Action should be claimed");

    // Query and print defi_holders information
    let defi_amount = bridge_test_cluster
        .bridge_client()
        .get_defi_holders_amount(
            address,
            protocol_type,
            protocol_version,
            protocol_token_id,
            target_chain,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get defi holders amount: {:?}", e))?;

    let defi_lp_token_amount = bridge_test_cluster
        .bridge_client()
        .get_defi_holders_lp_token_amount(
            address,
            protocol_type,
            protocol_version,
            protocol_token_id,
            target_chain,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get defi holders lp token amount: {:?}", e))?;

    println!("  Amount: {}", defi_amount);
    println!("  LP Token Amount: {}", defi_lp_token_amount);
    
    assert!(defi_amount > 0, "Defi amount should be greater than 0");
    assert!(defi_lp_token_amount > 0, "Defi LP token amount should be greater than 0");

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 16)]
async fn test_bridge_defi_stake_and_unstake_e2e() -> Result<(), anyhow::Error> {
    telemetry_subscribers::init_for_testing();
    // Setup bridge test env
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
    // .with_eth_env(true)
    .with_bridge_cluster(true)
    .with_num_validators(4)
    .build()
    .await;

    let address = bridge_test_cluster.sui_user_address();
    let http_client = bridge_test_cluster.test_cluster.inner.rpc_client().clone();

    // auth::auth_setup(&mut test_cluster, &mut http_client, address, "MINT-BUSD-right_key").await?;
    // info!("Setting up authentication for BUSD minting...");

    let minter_address = bridge_test_cluster.minter_address.unwrap();
    let minter_key_pair = bridge_test_cluster.minter_key_pair.as_ref().unwrap();
    println!("Using minter address: {}", minter_address);

    // Step 2: Mint BUSD tokens for DeFi staking
    let busd_amount = 50_000_000_000u64; // 500 BUSD for testing
    println!("Minting {} BUSD tokens for DeFi staking...", busd_amount);
    stable::mint_stable_coin_to_address(
        busd_amount,
        &http_client,
        minter_address,
        minter_key_pair,
        "0xc8::busd::BUSD",
        address,
    )
    .await?;
    println!("Minted {} BUSD tokens for DeFi staking to {}", busd_amount, address);

    // Step 3: Verify BUSD tokens were minted successfully
    let busd_objects = auth::do_get_owned_objects_with_filter(
        "0x2::coin::Coin<0xc8::busd::BUSD>",
        &http_client,
        address
    ).await?;
    assert!(busd_objects.len() >= 1, "Should have at least one BUSD coin object");

    let busd_coin = busd_objects.first().unwrap().object().unwrap();
    let busd_balance = auth::get_balance(busd_coin);
    println!("BUSD balance before DeFi stake: {}", busd_balance);
    assert!(busd_balance > 0, "BUSD balance should be greater than 0");

    // Step 4: Get bridge object for DeFi staking
    info!("Getting bridge object for DeFi operations...");
    let bridge_object_arg = bridge_test_cluster
        .bridge_client()
        .get_mutable_bridge_object_arg_must_succeed()
        .await;
    let bridge_object_id = match bridge_object_arg {
        sui_types::transaction::ObjectArg::SharedObject { id, .. } => id,
        _ => panic!("Bridge object is not a shared object"),
    };

    // Step 5: Setup DeFi staking parameters
    let target_chain = 12u8; // ETH Custom chain (matches defi_protocols::initial_defi_protocol)
    let protocol_type = 1u64; // AAVE protocol type
    let protocol_version = 3u64; // Version 3
    let protocol_token_id = 3u64; // USDC token ID for DeFi protocol
    let stake_amount = busd_amount / 2; // Use half of the BUSD for staking

    // Step 6: Call defi_stake Move function
    info!("Calling defi_stake Move function with parameters:");
    info!("  - Target chain: {}", target_chain);
    info!("  - Protocol type: {}", protocol_type);
    info!("  - Protocol version: {}", protocol_version);
    info!("  - Protocol token ID: {}", protocol_token_id);
    info!("  - Stake amount: {}", stake_amount);

    let mut builder = ProgrammableTransactionBuilder::new();

    let bridge_arg = builder.obj(bridge_object_arg).unwrap();
    let bfc_system_state_arg = builder.obj(ObjectArg::SharedObject {
        id: BFC_SYSTEM_STATE_OBJECT_ID,
        initial_shared_version: BFC_SYSTEM_STATE_OBJECT_SHARED_VERSION,
        mutable: true,
    }).unwrap();
    let busd_arg = builder.obj(ObjectArg::ImmOrOwnedObject(busd_coin.object_ref())).unwrap();

    let target_chain_arg = builder.pure(target_chain).unwrap();
    let protocol_type_arg = builder.pure(protocol_type).unwrap();
    let protocol_version_arg = builder.pure(protocol_version).unwrap();
    let protocol_token_id_arg = builder.pure(protocol_token_id).unwrap();

    builder.programmable_move_call(
        BRIDGE_PACKAGE_ID,
        ident_str!("bridge").to_owned(),
        ident_str!("defi_stake").to_owned(),
        vec![TypeTag::from_str("0xc8::busd::BUSD").unwrap()],
        vec![
            bridge_arg,
            bfc_system_state_arg,
            target_chain_arg,
            busd_arg,
            protocol_type_arg,
            protocol_version_arg,
            protocol_token_id_arg,
        ],
    );
    let pt = builder.finish();
    let gas = bridge_test_cluster.test_cluster.inner
        .wallet
        .get_one_gas_object_owned_by_address(address)
        .await
        .unwrap()
        .unwrap();

    let tx_data = TransactionData::new_programmable(
        address,
        vec![gas],
        pt,
        50_000_000_000,
        bridge_test_cluster.test_cluster.inner.get_reference_gas_price().await,
    );

    // Step 7: Sign and execute the DeFi stake transaction
    let tx = bridge_test_cluster.test_cluster.inner.wallet.sign_transaction(&tx_data);
    let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();

    println!("Executing DeFi stake transaction...");
    let tx_response = http_client
        .execute_transaction_block(
            tx_bytes,
            signatures,
            Some(SuiTransactionBlockResponseOptions::new().with_effects().with_events()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    println!("DeFi stake transaction executed: {:?}", tx_response);

    // Step 8: Verify transaction execution
    let effects = tx_response.effects.as_ref().unwrap();
    let defi_stake_succeeded = match effects.status() {
        SuiExecutionStatus::Success => {
            true
        },
        SuiExecutionStatus::Failure { error } => {
            error!("❌ DeFi stake transaction failed: {}", error);
            info!("Note: This is expected if DeFi protocol configuration is not initialized in test environment");
            info!("The test will skip the approve_defi_transfer_out verification");
            false
        }
    };

    assert!(defi_stake_succeeded, "DeFi stake transaction should succeed");
    println!("✅ DeFi stake transaction executed successfully!");

    //  Check for DeFi bridge events (if transaction was successful)
    if let Some(events) = &tx_response.events {
        println!("Transaction emitted {} events", events.data.len());

        for (idx, event) in events.data.iter().enumerate() {
            println!("Event {}: type={}, sender={:?}", idx, event.type_, event.sender);

            // Look for DeFi-related events
            if event.type_.address.to_string().contains("bridge") ||
               event.type_.module.as_str().contains("bridge") ||
               event.type_.name.as_str().contains("defi") {
                info!("🌉 Found bridge/DeFi related event: {:?}", event);
            }
        }
    }


    println!("=== Step 4: Extract DefiTransferOutEvent from transaction ===");
    let events = tx_response.events.as_ref().expect("Should have events");
    let mut defi_event = None;
    let mut event_seq_num = 0u64;

    let mut amount_after_fee=0;

    for (idx, event) in events.data.iter().enumerate() {
        println!("Event {}: type={}", idx, event.type_);

        if event.type_.name.as_str() == "DefiTransferOutEvent" {
            println!("🌉 Found DefiTransferOutEvent!");
            defi_event = Some(event.clone());

            // Parse event to get seq_num
            if let Ok(parsed_event) = bcs::from_bytes::<crate::events::MoveDefiTransferOutEvent>(&event.bcs.bytes()) {
                event_seq_num = parsed_event.seq_num;
                info!("  - Sequence number: {}", event_seq_num);
                info!("  - Source chain: {}", parsed_event.source_chain);
                info!("  - Target chain: {}", parsed_event.target_chain);
                info!("  - Protocol type: {}", parsed_event.protocol_type);
                info!("  - Protocol version: {}", parsed_event.protocol_version);
                info!("  - Protocol token ID: {}", parsed_event.protocol_token_id);
                info!("  - Amount after fee: {}", parsed_event.amount_after_fee);
                amount_after_fee=parsed_event.amount_after_fee;
                info!("  - Action type: {}", parsed_event.action_type);
            }
            break;
        }
    }

    assert!(defi_event.is_some(), "DefiTransferOutEvent should be emitted");
    println!("✅ DefiTransferOutEvent confirmed with seq_num: {}", event_seq_num);

    println!("=== Step 5: Wait for bridge cluster to automatically process DeFi action ===");
    // Wait for bridge cluster to detect the event, get signatures, and approve
    println!("Waiting for TokenTransferApproved event...");

    // Give bridge cluster enough time to process the action
    tokio::time::sleep(Duration::from_secs(15)).await;

    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                crate::events::TokenTransferApproved.get().unwrap().clone(),
            ]),
            true,
        )
        .await;

    assert!(!events.is_empty(), "Should have TokenTransferApproved event");
    println!("✅ Bridge cluster automatically approved the DeFi transfer");

    println!("=== Step 6: Verify the bridge record was approved ===");
    // Verify the record has verified signatures now
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Check that the action status is approved (not pending anymore)
    // For DeFi messages, we need to use a different query function
    let status = bridge_test_cluster
        .bridge_client()
        .get_defi_transfer_action_status_until_success(
            bridge_test_cluster.sui_chain_id() as u8,
            event_seq_num,
        )
        .await;

    println!("DeFi transfer action status: {:?}", status);
    // The status should be Approved (signatures verified) but not Claimed
    // because this is a Sui->Eth transfer that will be claimed on the Eth side
    assert_eq!(status, BridgeActionStatus::Approved, "Action should be approved");

    println!("=== Test Summary ===");
    println!("✅ Successfully completed full DeFi stake integration test:");
    println!("  1. Minted BUSD tokens");
    println!("  2. Called defi_stake Move function");
    println!("  3. Detected DefiTransferOutEvent");
    println!("  4. Bridge cluster automatically approved the DeFi transfer");
    println!("  5. Verified action status is Approved");
    println!("📊 BUSD minted: {}, Staked amount: {}", busd_amount, stake_amount);
    println!("🎯 DeFi protocol: type={}, version={}, token_id={}", protocol_type, protocol_version, protocol_token_id);

    // 在以太坊上投资已桥接的代币
    let tx_response = invest_bridged_tokens_on_eth(&bridge_test_cluster, event_seq_num).await?;
    if let Some(log) = tx_response.logs.last() {
        let event = decode_tokens_staked_event(log)?;
        assert_eq!(event.0,BridgeChainId::EthCustom as u8);
        assert_eq!(event.1,0); //evm
        assert_eq!(event.2,BridgeChainId::SuiCustom as u8);
        assert_eq!(event.3,event_seq_num); // from benfen
        assert_eq!(event.5,bridge_test_cluster.eth_env().contracts().arrow);
        assert_eq!(event.6,0);
        assert_eq!(event.7,amount_after_fee);
        assert_eq!(event.8,protocol_type);
        assert_eq!(event.9,protocol_version);
        assert_eq!(event.10,protocol_token_id);
        assert_eq!(event.11,0);    
    }

    // check claim on sui side
    println!("=== Step 7: Wait for Sui side to process DeFi claim ===");
    // Poll until the DeFi action becomes Claimed (with a timeout)
    let mut attempts = 0;
    let max_attempts = 60; // ~2 minutes if sleeping 2s between polls
    let mut status = BridgeActionStatus::NotFound;
    loop {
        status = bridge_test_cluster
            .bridge_client()
            .get_defi_transfer_action_status_until_success(
                bridge_test_cluster.eth_chain_id() as u8,
                event_seq_num,
            )
            .await;
        println!("DeFi transfer action status: {:?}", status);
        if status == BridgeActionStatus::Claimed {
            break;
        }
        attempts += 1;
        if attempts >= max_attempts {
            panic!(
                "Timed out waiting for DeFi transfer action to become Claimed (last status: {:?})",
                status
            );
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    // The status should be Claimed now
    assert_eq!(status, BridgeActionStatus::Claimed, "Action should be claimed");
    info!("unstake test started");
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                TokenTransferApproved.get().unwrap().clone(),
            ]),
            true,
        )
        .await;
    assert!(!events.is_empty(), "Should have TokenTransferApproved event");
    assert_eq!(events.len(), 1);
    initiate_defi_bridge_unstake_sui_to_eth(&bridge_test_cluster, protocol_type, protocol_version, protocol_token_id, amount_after_fee)
        .await
        .unwrap();
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                TokenTransferApproved.get().unwrap().clone(),
            ]),
            true,
        )
        .await;
    // There are exactly 2 approved events,one for unstake and one for stake
    assert_eq!(events.len(), 1);
    info!("unstake benfen approved");
    // 在以太坊上投资已桥接的代币
    let tx_response = invest_bridged_tokens_on_eth(&bridge_test_cluster, event_seq_num+1).await?;
    info!("unstake tx response: {:?}", tx_response);

    if let Some(log) = tx_response.logs.last() {
        let event = decode_tokens_unstaked_event(log)?;
        info!("unstaked event: {:?}", event);
        assert_eq!(event.0,BridgeChainId::EthCustom as u8);
        assert_eq!(event.1,event_seq_num+1); //evm
        assert_eq!(event.2,BridgeChainId::SuiCustom as u8);
        assert_eq!(event.3,event_seq_num+1); // from benfen
        assert_eq!(event.5,bridge_test_cluster.eth_env().contracts().arrow);
        assert_eq!(event.6,amount_after_fee);
        assert_eq!(event.7,amount_after_fee);
        assert_eq!(event.8,protocol_type);
        assert_eq!(event.9,protocol_version);
        assert_eq!(event.10,protocol_token_id);
        assert_eq!(event.11,1);
    }
    info!("unstake event decoded");
    let mut attempts = 0;
    let max_attempts = 60; // ~2 minutes if sleeping 2s between polls
    let mut status = BridgeActionStatus::NotFound;
    loop {
        status = bridge_test_cluster
            .bridge_client()
            .get_defi_transfer_action_status_until_success(
                bridge_test_cluster.eth_chain_id() as u8,
                event_seq_num+1,
            )
            .await;
        info!("unstake claim action status: {:?}", status);
        if status == BridgeActionStatus::Claimed {
            break;
        }
        attempts += 1;
        if attempts >= max_attempts {
            panic!(
                "Timed out waiting for unstake claim action to become Claimed (last status: {:?})",
                status
            );
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    // The status should be Claimed now
    assert_eq!(status, BridgeActionStatus::Claimed, "Action should be claimed");
    info!("unstake test completed");

    Ok(())
}


fn decode_tokens_staked_event(
    log: &ethers::types::Log,
) -> anyhow::Result<(
    u8,                        // sourceChainID (indexed)
    u64,                       // nonce (indexed)
    u8,                        // destinationChainID (indexed)
    u64,                       // originNonce
    Vec<u8>,                   // senderAddress
    ethers::types::Address,    // recipientAddress
    u64,                       // suiAdjustedAmount
    u64,                       // suiLpTokenAmount
    u64,                       // protocolType
    u64,                       // protocolVersion
    u64,                       // protocolTokenId
    u8,                        // actionType
)> {
    use anyhow::{anyhow, Result};
    use ethers::abi::{decode, ParamType};
    use ethers::types::{H256, U256};

    // 校验事件签名
    let expected_sig = ethers::utils::keccak256(
        b"TokensStaked(uint8,uint64,uint8,uint64,bytes,address,uint64,uint64,uint64,uint64,uint64,uint8)",
    );
    if log.topics.len() < 4 || log.topics[0] != H256::from(expected_sig) {
        return Err(anyhow!("not a TokensStaked event (signature mismatch)"));
    }

    // 解析 indexed 参数（topics）
    let source_chain_id: u8 = {
        let mut buf = [0u8; 32];
        buf.copy_from_slice(log.topics[1].as_bytes());
        U256::from_big_endian(&buf).as_u64() as u8
    };
    let nonce: u64 = {
        let mut buf = [0u8; 32];
        buf.copy_from_slice(log.topics[2].as_bytes());
        U256::from_big_endian(&buf).as_u64()
    };
    let destination_chain_id: u8 = {
        let mut buf = [0u8; 32];
        buf.copy_from_slice(log.topics[3].as_bytes());
        U256::from_big_endian(&buf).as_u64() as u8
    };

    // 解析非 indexed 参数（data）
    let tokens = decode(
        &[
            ParamType::Uint(64),  // originNonce
            ParamType::Bytes,     // senderAddress
            ParamType::Address,   // recipientAddress
            ParamType::Uint(64),  // suiAdjustedAmount
            ParamType::Uint(64),  // suiLpTokenAmount
            ParamType::Uint(64),  // protocolType
            ParamType::Uint(64),  // protocolVersion
            ParamType::Uint(64),  // protocolTokenId
            ParamType::Uint(8),   // actionType
        ],
        &log.data.0,
    )?;

    let origin_nonce = tokens[0]
        .clone()
        .into_uint()
        .ok_or_else(|| anyhow!("originNonce decode error"))?
        .as_u64();

    let sender_address = tokens[1]
        .clone()
        .into_bytes()
        .ok_or_else(|| anyhow!("senderAddress decode error"))?;

    let recipient_address = tokens[2]
        .clone()
        .into_address()
        .ok_or_else(|| anyhow!("recipientAddress decode error"))?;

    let sui_adjusted_amount = tokens[3]
        .clone()
        .into_uint()
        .ok_or_else(|| anyhow!("suiAdjustedAmount decode error"))?
        .as_u64();

    let sui_lp_token_amount = tokens[4]
        .clone()
        .into_uint()
        .ok_or_else(|| anyhow!("suiLpTokenAmount decode error"))?
        .as_u64();

    let protocol_type = tokens[5]
        .clone()
        .into_uint()
        .ok_or_else(|| anyhow!("protocolType decode error"))?
        .as_u64();

    let protocol_version = tokens[6]
        .clone()
        .into_uint()
        .ok_or_else(|| anyhow!("protocolVersion decode error"))?
        .as_u64();

    let protocol_token_id = tokens[7]
        .clone()
        .into_uint()
        .ok_or_else(|| anyhow!("protocolTokenId decode error"))?
        .as_u64();

    let action_type = tokens[8]
        .clone()
        .into_uint()
        .ok_or_else(|| anyhow!("actionType decode error"))?
        .as_u64() as u8;

    Ok((
        source_chain_id,
        nonce,
        destination_chain_id,
        origin_nonce,
        sender_address,
        recipient_address,
        sui_adjusted_amount,
        sui_lp_token_amount,
        protocol_type,
        protocol_version,
        protocol_token_id,
        action_type,
    ))
}

fn decode_tokens_unstaked_event(
    log: &ethers::types::Log,
) -> anyhow::Result<(
    u8,                        // sourceChainID (indexed)
    u64,                       // nonce (indexed)
    u8,                        // destinationChainID (indexed)
    u64,                       // originNonce
    Vec<u8>,                   // recipientAddress
    ethers::types::Address,    // senderAddress
    u64,                       // suiAdjustedAmount
    u64,                       // suiLpTokenAmount
    u64,                       // protocolType
    u64,                       // protocolVersion
    u64,                       // protocolTokenId
    u8,                        // actionType
)> {
    use anyhow::{anyhow, Result};
    use ethers::abi::{decode, ParamType};
    use ethers::types::{H256, U256};

    // 校验事件签名
    let expected_sig = ethers::utils::keccak256(
        b"TokensUnStaked(uint8,uint64,uint8,uint64,bytes,address,uint64,uint64,uint64,uint64,uint64,uint8)",
    );
    if log.topics.len() < 4 || log.topics[0] != H256::from(expected_sig) {
        return Err(anyhow!("not a TokensUnStaked event (signature mismatch)"));
    }

    // 解析 indexed 参数（topics）
    let source_chain_id: u8 = {
        let mut buf = [0u8; 32];
        buf.copy_from_slice(log.topics[1].as_bytes());
        U256::from_big_endian(&buf).as_u64() as u8
    };
    let nonce: u64 = {
        let mut buf = [0u8; 32];
        buf.copy_from_slice(log.topics[2].as_bytes());
        U256::from_big_endian(&buf).as_u64()
    };
    let destination_chain_id: u8 = {
        let mut buf = [0u8; 32];
        buf.copy_from_slice(log.topics[3].as_bytes());
        U256::from_big_endian(&buf).as_u64() as u8
    };

    // 解析非 indexed 参数（data）
    // 注意：TokensUnStaked 的参数顺序与 TokensStaked 不同
    // TokensUnStaked: originNonce, recipientAddress(bytes), senderAddress(address), ...
    // TokensStaked:   originNonce, senderAddress(bytes), recipientAddress(address), ...
    let tokens = decode(
        &[
            ParamType::Uint(64),  // originNonce
            ParamType::Bytes,     // recipientAddress (bytes)
            ParamType::Address,   // senderAddress (address)
            ParamType::Uint(64),  // suiAdjustedAmount
            ParamType::Uint(64),  // suiLpTokenAmount
            ParamType::Uint(64),  // protocolType
            ParamType::Uint(64),  // protocolVersion
            ParamType::Uint(64),  // protocolTokenId
            ParamType::Uint(8),   // actionType
        ],
        &log.data.0,
    )?;

    let origin_nonce = tokens[0]
        .clone()
        .into_uint()
        .ok_or_else(|| anyhow!("originNonce decode error"))?
        .as_u64();

    let recipient_address = tokens[1]
        .clone()
        .into_bytes()
        .ok_or_else(|| anyhow!("recipientAddress decode error"))?;

    let sender_address = tokens[2]
        .clone()
        .into_address()
        .ok_or_else(|| anyhow!("senderAddress decode error"))?;

    let sui_adjusted_amount = tokens[3]
        .clone()
        .into_uint()
        .ok_or_else(|| anyhow!("suiAdjustedAmount decode error"))?
        .as_u64();

    let sui_lp_token_amount = tokens[4]
        .clone()
        .into_uint()
        .ok_or_else(|| anyhow!("suiLpTokenAmount decode error"))?
        .as_u64();

    let protocol_type = tokens[5]
        .clone()
        .into_uint()
        .ok_or_else(|| anyhow!("protocolType decode error"))?
        .as_u64();

    let protocol_version = tokens[6]
        .clone()
        .into_uint()
        .ok_or_else(|| anyhow!("protocolVersion decode error"))?
        .as_u64();

    let protocol_token_id = tokens[7]
        .clone()
        .into_uint()
        .ok_or_else(|| anyhow!("protocolTokenId decode error"))?
        .as_u64();

    let action_type = tokens[8]
        .clone()
        .into_uint()
        .ok_or_else(|| anyhow!("actionType decode error"))?
        .as_u64() as u8;

    Ok((
        source_chain_id,
        nonce,
        destination_chain_id,
        origin_nonce,
        recipient_address,
        sender_address,
        sui_adjusted_amount,
        sui_lp_token_amount,
        protocol_type,
        protocol_version,
        protocol_token_id,
        action_type,
    ))
}
