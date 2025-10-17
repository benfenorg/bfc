use crate::e2e_tests::{auth, stable};
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use crate::e2e_tests::test_utils::BridgeTestClusterBuilder;
use crate::types::BridgeActionStatus;
use sui_json_rpc_api::WriteApiClient;
use sui_types::BRIDGE_PACKAGE_ID;
use std::collections::HashSet;
use sui_json_rpc_types::{SuiExecutionStatus, SuiTransactionBlockEffectsAPI};
use move_core_types::ident_str;
use move_core_types::language_storage::TypeTag;
use sui_types::bridge::{BridgeChainId};
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
use sui_types::transaction::{ObjectArg, TransactionData};
use sui_types::{BFC_SYSTEM_STATE_OBJECT_ID, BFC_SYSTEM_STATE_OBJECT_SHARED_VERSION};
use std::str::FromStr;
use std::time::Duration;
use tracing::{error, info};
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_json_rpc_types::{SuiTransactionBlockResponseOptions};
use crate::e2e_tests::test_utils::{
    initiate_defi_bridge_unstake_sui_to_eth,mock_bridge_unstake_eth_to_sui,
};
use crate::events::TokenTransferApproved;


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
