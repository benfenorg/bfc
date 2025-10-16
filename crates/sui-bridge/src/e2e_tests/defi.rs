// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;

use sui_types::bridge::BridgeChainId;
use tracing::info;
use crate::abi::{eth_sui_bridge, EthSuiBridge};
use crate::client::bridge_authority_aggregator::BridgeAuthorityAggregator;
use crate::e2e_tests::{auth, stable};
use crate::e2e_tests::test_utils::{
    initiate_bridge_eth_to_sui, initiate_bridge_sui_to_eth, BridgeTestCluster, BridgeTestClusterBuilder,
};
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
use sui_types::bridge::{TOKEN_ID_ETH};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{CallArg, ObjectArg, TransactionData, TransactionKind};
use sui_types::{SUI_CLOCK_OBJECT_ID, SUI_CLOCK_OBJECT_SHARED_VERSION};
use sui_types::{BFC_SYSTEM_STATE_OBJECT_ID, BFC_SYSTEM_STATE_OBJECT_SHARED_VERSION};
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
use crate::e2e_tests::test_utils::{
    get_signatures, initiate_bridge_erc20_to_sui, initiate_bridge_eth_to_sui,
    initiate_bridge_sui_to_eth, send_eth_tx_and_get_tx_receipt,
    initiate_defi_bridge_unstake_sui_to_eth,mock_bridge_unstake_eth_to_sui,
};
use crate::events::{TokenTransferApproved, TokenTransferClaimed};

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_eth_test_cluster_builder() {
    telemetry_subscribers::init_for_testing();
    BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_eth_chain_id(BridgeChainId::BscCustom)
        .with_bridge_cluster(true)
        .with_num_validators(3)
        .build_eth_env()
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_unstake_from_eth_to_sui() {
    telemetry_subscribers::init_for_testing();

    let eth_chain_id = BridgeChainId::EthCustom as u8;
    let sui_chain_id = BridgeChainId::SuiCustom as u8;
    let timer = std::time::Instant::now();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(true)
        .with_num_validators(3)
        .build()
        .await;
    info!(
        "[Timer] Bridge test cluster started in {:?}",
        timer.elapsed()
    );
    let timer = std::time::Instant::now();
    let (eth_signer, _) = bridge_test_cluster
        .get_eth_signer_and_address()
        .await
        .unwrap();

    mock_bridge_unstake_eth_to_sui(&bridge_test_cluster, false).await.unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_defi_unstake_sui_to_eth() {
    telemetry_subscribers::init_for_testing();

    // let eth_chain_id = BridgeChainId::EthCustom as u8;
    // let sui_chain_id = BridgeChainId::SuiCustom as u8;
    let timer = std::time::Instant::now();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(true)
        .with_num_validators(3)
        .build()
        .await;
    info!(
        "[Timer] Bridge test cluster started in {:?}",
        timer.elapsed()
    );
    let timer = std::time::Instant::now();
    let (eth_signer, _) = bridge_test_cluster
        .get_eth_signer_and_address()
        .await
        .unwrap();

    // let sui_address = bridge_test_cluster.sui_user_address();
    let amount = 42;
    let sui_amount = amount * 100_000_000;

    initiate_defi_bridge_unstake_sui_to_eth(&bridge_test_cluster, 1, 3, 3, amount)
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
    // There are exactly 1 approved and 1 claimed event
    assert_eq!(events.len(), 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 16)]
async fn test_bridge_defi_stake_mock_evm_event_e2e() -> Result<(), anyhow::Error> {
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
    let protocol_type = 2u64; // Compound protocol type
    let protocol_version = 1u64; // Version 1
    let protocol_token_id = 4u64; // USDT token ID for DeFi protocol
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

    mock_bridge_unstake_eth_to_sui(&bridge_test_cluster, true).await.unwrap();

    Ok(())
}
