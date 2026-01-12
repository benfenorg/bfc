// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::client::bridge_authority_aggregator::BridgeAuthorityAggregator;
use crate::e2e_tests::test_utils::{
    initiate_bridge_eth_to_sui, initiate_bridge_sui_to_eth, BridgeTestCluster, BridgeTestClusterBuilder
};
use crate::sui_transaction_builder::build_sui_transaction;
use crate::types::{BridgeAction, EmergencyAction};
use crate::types::{BridgeActionStatus, EmergencyActionType};
use ethers::types::Address as EthAddress;
use solana_sdk::signer::Signer;
use std::sync::Arc;
use sui_json_rpc_types::SuiExecutionStatus;
use sui_json_rpc_types::SuiTransactionBlockEffectsAPI;
use crate::e2e_tests::test_utils::solana_cross_token_to_bridge;
use solana_sdk::pubkey::Pubkey;
use sui_types::bridge::{BridgeChainId, TOKEN_ID_ETH};
use tracing::info;
use spl_associated_token_account::get_associated_token_address;

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


#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async  fn test_solana_bridge_call() {
    telemetry_subscribers::init_for_testing();
    let bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_solana_env(true)
        .with_eth_env(true)
        .with_solana_chain_id(BridgeChainId::SolanaTestnet)
        .with_bridge_cluster(true)
        .with_num_validators(3)
        .build()
        .await;

    let sol_env = &bridge_test_cluster.sol_environment;
    let solana_signer = sol_env.get_signer().await.unwrap();
    let rpc_url = sol_env.rpc_url.clone();
    let ws_url = rpc_url.replace("http", "ws"); 

    // The following part is hard, because I can't easily create a token.
    // I will use placeholders. The user will see the test fails and can provide more info.
    // let token_mint = Pubkey::new_unique();
    //let source_token_account = Pubkey::new_unique();
    let token_mint = sol_env.usdc();
    let source_token_account_owner = solana_signer.pubkey();
    let source_token_account = get_associated_token_address(&source_token_account_owner, &token_mint);;
    let amount = 100;
    let benfen_address = vec![1; 32];
    let target_chain_id = bridge_test_cluster.sui_chain_id();
    let token_id = 3u64;

    solana_cross_token_to_bridge(
        &rpc_url,
        &ws_url,
        solana_signer,
        target_chain_id,
        amount,
        benfen_address,
        token_id,
        token_mint,
        source_token_account,
    ).await.unwrap();

}