// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::abi::{eth_sui_bridge, EthSuiBridge};
use crate::client::bridge_authority_aggregator::BridgeAuthorityAggregator;
use crate::crypto::BridgeAuthorityKeyPair;
use crate::e2e_tests::test_utils::TestClusterWrapperBuilder;
use crate::e2e_tests::test_utils::{
    get_signatures, initiate_bridge_erc20_to_sui, initiate_bridge_eth_to_sui,
    initiate_bridge_sui_to_eth, send_eth_tx_and_get_tx_receipt, BridgeTestClusterBuilder,
    add_coin_on_solana,
};
use crate::eth_transaction_builder::build_eth_transaction;
use crate::events::{
    SuiBridgeEvent, SuiToEthTokenBridgeV2,TokenSendBackEvent, TokenTransferApproved,
    TokenTransferClaimed,
};
use crate::sui_client::SuiClientInner;
use crate::sui_transaction_builder::build_add_external_coin_admin_transaction;
use crate::sui_transaction_builder::build_add_external_coin_target_transaction;
use crate::sui_transaction_builder::build_add_external_coin_witness_transaction;
use crate::sui_transaction_builder::build_remove_external_coin_admin_transaction;
use crate::sui_transaction_builder::build_remove_external_coin_target_transaction;
use crate::sui_transaction_builder::build_remove_external_coin_witness_transaction;
use crate::sui_transaction_builder::{
    build_add_token_on_token_list_transaction, build_add_tokens_on_sui_transaction,
    build_refund_admin_operate_transaction, build_remove_token_on_token_list_transaction,
    build_set_cross_in_bridge_fee_transaction, build_set_cross_out_bridge_fee_transaction,
    build_withdraw_fee_cap_transaction,
};

use crate::solana_transaction_builder::build_solana_transaction;
use crate::solana_client::{GetSignaturesConfig, SolanaClient};
use sui_json_rpc_types::SuiObjectDataOptions;
// use ethers::types::Address;
use ethers::types::Address as EthAddress;

use solana_client::rpc_client::RpcClient;

use crate::query_solana_account;

use crate::types::BridgeActionType::{AddTokensOnSolana};
use crate::types::BridgeActionType;


anchor_lang::declare_program!(benfen_bridge);
use benfen_bridge::{
    client::accounts, 
    client::args, 
    accounts::BridgeConfig, 
    accounts::Committee,
    accounts::TokenConfigAccount,
};

use anchor_lang;
use anchor_client::{Program,Client, Cluster};
// use solana_client::rpc_client::RpcClient;
use spl_token;

use crate::types::{
    AddExternalCoinAdminAction, AddExternalCoinTargetAction, AddExternalCoinWitnessAction, AddLpTokenIdAction, AddTokenOnSolanaAction, AddTokenOnTokenListAction, AddTokensOnEvmAction, BridgeAction, RefundAdminAction, RemoveExternalCoinAdminAction, RemoveExternalCoinTargetAction, RemoveExternalCoinWitnessAction, RemoveTokenOnTokenListAction, SingleTransferLimitUpdateAction, UpdateBridgeFeeOnCrossInAction, UpdateBridgeFeeOnCrossOutAction, UpdateInvestAddressAction, WithdrawBridgeFeeAction
};
use crate::utils::publish_and_register_coins_return_add_coins_on_sui_action;
use crate::BRIDGE_ENABLE_PROTOCOL_VERSION;
use ethers::prelude::*;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use sui_json_rpc_api::BridgeReadApiClient;
use sui_types::crypto::get_key_pair;
use test_cluster::TestClusterBuilder;

use std::path::Path;
use std::sync::Arc;
use sui_json_rpc_types::{SuiExecutionStatus, SuiTransactionBlockEffectsAPI};
use sui_types::bridge::{
    get_bridge, BridgeChainId, BridgeTokenMetadata, BridgeTrait, TOKEN_ID_BUSD, TOKEN_ID_ETH,
    TOKEN_ID_USDT,
};
use sui_types::{TypeTag, SUI_BRIDGE_OBJECT_ID};
use tracing::info;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;

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
async  fn test_solana_test_cluster_builder() {
    telemetry_subscribers::init_for_testing();
    BridgeTestClusterBuilder::new()
        .with_solana_env(true)
        .with_solana_chain_id(BridgeChainId::SolanaTestnet)
        .with_bridge_cluster(true)
        .with_num_validators(3)
        .build()
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_sui_test_cluster_builder() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_keys = vec![];
    // let mut bridge_keys_copy = vec![];
    for _ in 0..3 {
        let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
        bridge_keys.push(kp);
        // bridge_keys_copy.push(kp);
    }
    let test_cluster = TestClusterWrapperBuilder::new()
        .with_bridge_authority_keys(bridge_keys)
        .with_deploy_tokens(true)
        .with_eth_chain_id(BridgeChainId::BscCustom)
        .build()
        .await;
    info!("Test cluster built");
    test_cluster
        .trigger_reconfiguration_if_not_yet_and_assert_bridge_committee_initialized()
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_bridge_from_eth_to_sui_to_eth() {
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

    let sui_address = bridge_test_cluster.sui_user_address();
    let amount = 42;
    let sui_amount = amount * 100_000_000;

    initiate_bridge_eth_to_sui(&bridge_test_cluster, amount, 0, false)
        .await
        .unwrap();
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                TokenTransferApproved.get().unwrap().clone(),
                TokenTransferClaimed.get().unwrap().clone(),
            ]),
            true,
        )
        .await;
    // There are exactly 1 approved and 1 claimed event
    assert_eq!(events.len(), 2);

    let eth_coin = bridge_test_cluster
        .sui_client()
        .coin_read_api()
        .get_all_coins(sui_address, None, None)
        .await
        .unwrap()
        .data
        .iter()
        .find(|c| c.coin_type.contains("ETH"))
        .expect("Recipient should have received ETH coin now")
        .clone();
    assert_eq!(eth_coin.balance, sui_amount);
    info!(
        "[Timer] Eth to Sui bridge transfer finished in {:?}",
        timer.elapsed()
    );
    let timer = std::time::Instant::now();

    // Now let the recipient send the coin back to ETH
    let eth_address_1 = EthAddress::random();
    let nonce = 0;

    let sui_to_eth_bridge_action = initiate_bridge_sui_to_eth(
        &bridge_test_cluster,
        eth_address_1,
        eth_coin.object_ref(),
        nonce,
        sui_amount,
        TOKEN_ID_ETH,
    )
    .await
    .unwrap();
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                SuiToEthTokenBridgeV2.get().unwrap().clone(),
                TokenTransferApproved.get().unwrap().clone(),
                TokenTransferClaimed.get().unwrap().clone(),
            ]),
            true,
        )
        .await;
    // There are exactly 1 deposit and 1 approved event
    assert_eq!(events.len(), 2);
    info!(
        "[Timer] Sui to Eth bridge transfer approved in {:?}",
        timer.elapsed()
    );
    let timer = std::time::Instant::now();

    // Test `get_parsed_token_transfer_message`
    let parsed_msg = bridge_test_cluster
        .bridge_client()
        .get_parsed_token_transfer_message(sui_chain_id, nonce)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(parsed_msg.source_chain as u8, sui_chain_id);
    assert_eq!(parsed_msg.seq_num, nonce);
    assert_eq!(
        parsed_msg.parsed_payload.sender_address,
        sui_address.to_vec()
    );
    assert_eq!(
        &parsed_msg.parsed_payload.target_address,
        eth_address_1.as_bytes()
    );
    assert_eq!(parsed_msg.parsed_payload.target_chain, eth_chain_id);
    assert_eq!(parsed_msg.parsed_payload.token_type, TOKEN_ID_ETH);
    //eth no fee
    assert_eq!(parsed_msg.parsed_payload.amount, sui_amount);

    let message = eth_sui_bridge::Message::from(sui_to_eth_bridge_action);
    let signatures = get_signatures(bridge_test_cluster.bridge_client(), nonce, sui_chain_id).await;

    let eth_sui_bridge = EthSuiBridge::new(
        bridge_test_cluster.contracts().sui_bridge,
        eth_signer.clone().into(),
    );
    let call = eth_sui_bridge.transfer_bridged_tokens_with_signatures(signatures, message);
    let eth_claim_tx_receipt = send_eth_tx_and_get_tx_receipt(call).await;
    assert_eq!(eth_claim_tx_receipt.status.unwrap().as_u64(), 1);
    info!(
        "[Timer] Sui to Eth bridge transfer claimed in {:?}",
        timer.elapsed()
    );
    // Assert eth_address_1 has received ETH
    assert_eq!(
        eth_signer.get_balance(eth_address_1, None).await.unwrap(),
        U256::from(amount) * U256::exp10(18)
    );
}

/// 测试从 Solana 跨入 Sui
/// 流程：
/// 1. 用户在 Solana 上调用 cross_token_to_bridge 存入 USDC
/// 2. Bridge Node 监听到 Solana 上的 TokensDeposited 事件
/// 3. Bridge Node 自动在 Sui 上执行 approve 和 claim
/// 4. 验证用户在 Sui 上收到对应的 token
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_bridge_from_solana_to_sui() {
    use spl_associated_token_account::get_associated_token_address;
    use crate::e2e_tests::test_utils::wait_for_transfer_action_status;
    use crate::types::BridgeActionStatus;
    use solana_sdk::signer::Signer;
    use anchor_lang::solana_program::system_program;

    telemetry_subscribers::init_for_testing();

    let timer = std::time::Instant::now();
    
    // 创建测试集群，同时启用 Solana 和 Eth 环境
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_solana_env(true)
        .with_eth_env(false)
        .with_solana_chain_id(BridgeChainId::SolanaTestnet)
        .with_bridge_cluster(true)
        .with_num_validators(3)
        .build()
        .await;

    info!(
        "[Timer] Bridge test cluster started in {:?}",
        timer.elapsed()
    );

    let timer = std::time::Instant::now();

    // 获取 Solana 环境和签名者
    let sol_env = bridge_test_cluster.solana_env();
    let solana_signer = sol_env.get_signer().await.expect("Failed to get solana signer");
    let rpc_url = sol_env.rpc_url.clone();
    let ws_url = sol_env.ws_url.clone();
    let usdc_mint = sol_env.usdc();

    // 获取 Sui 用户地址
    let sui_address = bridge_test_cluster.sui_user_address();
    let solana_chain_id = bridge_test_cluster.solana_chain_id();

    // token_id = 3 是 USDC (在 start_solana_env 中已注册)
    let token_id: u64 = 3;
    let amount: u64 = 1_000_000; // 1 USDC (6 decimals)

    // 获取用户的 USDC token 账户
    let source_token_account = get_associated_token_address(&solana_signer.pubkey(), &usdc_mint);

    info!(
        "Initiating Solana to Sui bridge transfer: amount={}, token_id={}, sui_address={:?}",
        amount, token_id, sui_address
    );

    // 构建并发送 cross_token_to_bridge 交易
    let client = Client::new_with_options(
        Cluster::Custom(rpc_url.clone(), ws_url.clone()),
        solana_signer.clone(),
        CommitmentConfig::confirmed(),
    );

    let program = client.program(benfen_bridge::ID).expect("Failed to get program");
    let init_pdas = crate::query_solana_account::get_init_account(program.id(), bridge_test_cluster.sui_chain_id() as u8);

    let (token_vault, _) = Pubkey::find_program_address(&[b"vault", &token_id.to_be_bytes()], &benfen_bridge::ID);
    let (token_config, _) = Pubkey::find_program_address(&[b"token_config", &token_id.to_be_bytes()], &benfen_bridge::ID);
    let (message_config, _) = Pubkey::find_program_address(
        &[
            b"message_config",
            &[BridgeActionType::TokenTransfer as u8],
            init_pdas.message_verifier.as_ref(),
        ],
        &benfen_bridge::ID,
    );

    let cross_ix = program
        .request()
        .accounts(accounts::CrossTokenToBridge {
            payer: solana_signer.pubkey(),
            token_account: source_token_account,
            token_vault,
            message_config,
            token_config,
            chain_limit: init_pdas.bridge_limiter,
            bridge_config: init_pdas.bridge_config,
            bridge: init_pdas.benfen_bridge,
            verifier: init_pdas.message_verifier,
            token_mint: usdc_mint,
            token_program: spl_token::ID,
            system_program: system_program::ID,
        })
        .args(args::CrossTokenToBridge {
            amount,
            benfen_address: sui_address.to_vec(),
        })
        .instructions()
        .expect("Failed to build cross_token_to_bridge instructions")
        .remove(0);

    let signature = program
        .request()
        .instruction(cross_ix)
        .signer(solana_signer.clone())
        .send()
        .await
        .expect("Failed to send cross_token_to_bridge transaction");

    info!(
        "[Timer] Solana cross_token_to_bridge TX sent in {:?}, signature: {}",
        timer.elapsed(),
        signature
    );

    // 等待 Bridge 处理并在 Sui 上 claim token
    // nonce = 1 (第一笔跨链交易，nonce 从 1 开始)
    let nonce = 1u64;
    wait_for_transfer_action_status(
        bridge_test_cluster.bridge_client(),
        solana_chain_id,
        nonce,
        BridgeActionStatus::Claimed,
    )
    .await
    .expect("Failed to wait for Solana to Sui bridge transfer to be claimed");

    info!(
        "[Timer] Solana to Sui bridge transfer claimed in {:?}",
        timer.elapsed()
    );

    // 验证用户在 Sui 上收到了 token
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                TokenTransferApproved.get().unwrap().clone(),
                TokenTransferClaimed.get().unwrap().clone(),
            ]),
            true,
        )
        .await;

    // 应该有 approved 和 claimed 事件
    assert!(events.len() >= 2, "Expected at least 2 events (approved + claimed), got {}", events.len());

    info!(
        "[Timer] Solana to Sui bridge transfer completed successfully in {:?}",
        timer.elapsed()
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_bridge_from_bsc_to_sui() {
    telemetry_subscribers::init_for_testing();
    let timer = std::time::Instant::now();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_eth_chain_id(BridgeChainId::BscCustom)
        .with_bridge_cluster(true)
        .with_num_validators(3)
        .build()
        .await;
    info!(
        "[Timer] Bridge test cluster started in {:?}",
        timer.elapsed()
    );
    let timer = std::time::Instant::now();
    let (_eth_signer, _) = bridge_test_cluster
        .get_eth_signer_and_address()
        .await
        .unwrap();

    let sui_address = bridge_test_cluster.sui_user_address();
    let amount = 17;
    let sui_amount = amount * 100_000_000;

    initiate_bridge_eth_to_sui(&bridge_test_cluster, amount, 0, false)
        .await
        .unwrap();
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                TokenTransferApproved.get().unwrap().clone(),
                TokenTransferClaimed.get().unwrap().clone(),
            ]),
            true,
        )
        .await;
    // There are exactly 1 approved and 1 claimed event
    assert_eq!(events.len(), 2);

    let eth_coin = bridge_test_cluster
        .sui_client()
        .coin_read_api()
        .get_all_coins(sui_address, None, None)
        .await
        .unwrap()
        .data
        .iter()
        .find(|c| c.coin_type.contains("BNB"))
        .expect("Recipient should have received BNB coin now")
        .clone();
    assert_eq!(eth_coin.balance, sui_amount);
    info!(
        "[Timer] Eth to Sui bridge transfer finished in {:?}",
        timer.elapsed()
    );
    // let timer = std::time::Instant::now();

    // // Now let the recipient send the coin back to ETH
    // let eth_address_1 = EthAddress::random();
    // let nonce = 0;

    // let sui_to_eth_bridge_action = initiate_bridge_sui_to_eth(
    //     &bridge_test_cluster,
    //     eth_address_1,
    //     eth_coin.object_ref(),
    //     nonce,
    //     sui_amount,
    //     TOKEN_ID_ETH,
    // )
    // .await
    // .unwrap();
    // let events = bridge_test_cluster
    //     .new_bridge_events(
    //         HashSet::from_iter([
    //             SuiToEthTokenBridgeV1.get().unwrap().clone(),
    //             TokenTransferApproved.get().unwrap().clone(),
    //             TokenTransferClaimed.get().unwrap().clone(),
    //         ]),
    //         true,
    //     )
    //     .await;
    // // There are exactly 1 deposit and 1 approved event
    // assert_eq!(events.len(), 2);
    // info!(
    //     "[Timer] Sui to Eth bridge transfer approved in {:?}",
    //     timer.elapsed()
    // );
    // let timer = std::time::Instant::now();

    // // Test `get_parsed_token_transfer_message`
    // let parsed_msg = bridge_test_cluster
    //     .bridge_client()
    //     .get_parsed_token_transfer_message(sui_chain_id, nonce)
    //     .await
    //     .unwrap()
    //     .unwrap();
    // assert_eq!(parsed_msg.source_chain as u8, sui_chain_id);
    // assert_eq!(parsed_msg.seq_num, nonce);
    // assert_eq!(
    //     parsed_msg.parsed_payload.sender_address,
    //     sui_address.to_vec()
    // );
    // assert_eq!(
    //     &parsed_msg.parsed_payload.target_address,
    //     eth_address_1.as_bytes()
    // );
    // assert_eq!(parsed_msg.parsed_payload.target_chain, eth_chain_id);
    // assert_eq!(parsed_msg.parsed_payload.token_type, TOKEN_ID_ETH);
    // assert_eq!(parsed_msg.parsed_payload.amount, sui_amount);

    // let message = eth_sui_bridge::Message::from(sui_to_eth_bridge_action);
    // let signatures = get_signatures(bridge_test_cluster.bridge_client(), nonce, sui_chain_id).await;

    // let eth_sui_bridge = EthSuiBridge::new(
    //     bridge_test_cluster.contracts().sui_bridge,
    //     eth_signer.clone().into(),
    // );
    // let call = eth_sui_bridge.transfer_bridged_tokens_with_signatures(signatures, message);
    // let eth_claim_tx_receipt = send_eth_tx_and_get_tx_receipt(call).await;
    // assert_eq!(eth_claim_tx_receipt.status.unwrap().as_u64(), 1);
    // info!(
    //     "[Timer] Sui to Eth bridge transfer claimed in {:?}",
    //     timer.elapsed()
    // );
    // // Assert eth_address_1 has received ETH
    // assert_eq!(
    //     eth_signer.get_balance(eth_address_1, None).await.unwrap(),
    //     U256::from(amount) * U256::exp10(18)
    // );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
#[ignore]
async fn test_bridge_from_eth_to_sui_refund() {
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

    let sui_address = bridge_test_cluster.sui_user_address();
    let amount = 42;
    let sui_amount = amount * 100_000_000;

    initiate_bridge_eth_to_sui(&bridge_test_cluster, amount, 0, true)
        .await
        .unwrap();
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                TokenSendBackEvent.get().unwrap().clone(),
                TokenTransferApproved.get().unwrap().clone(),
            ]),
            true,
        )
        .await;
    // There are exactly 1 refund and 1 approved event
    assert_eq!(events.len(), 2);

    let eth_coin = bridge_test_cluster
        .sui_client()
        .coin_read_api()
        .get_all_coins(sui_address, None, None)
        .await
        .unwrap()
        .data
        .iter()
        .find(|c| c.coin_type.contains("ETH"))
        .is_none();
    assert!(eth_coin == true);
    info!(
        "[Timer] Eth to Sui bridge transfer refunded in {:?}",
        timer.elapsed()
    );
    let timer = std::time::Instant::now();

    // Now let the recipient send the coin back to ETH
    let eth_address_1 = eth_signer.address();
    let nonce = 0;

    // Test `get_parsed_token_transfer_message`
    let parsed_msg = bridge_test_cluster
        .bridge_client()
        .get_parsed_token_transfer_message(sui_chain_id, nonce)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(parsed_msg.source_chain as u8, sui_chain_id);
    assert_eq!(parsed_msg.seq_num, nonce);

    assert_eq!(
        &parsed_msg.parsed_payload.target_address,
        eth_address_1.as_bytes()
    );
    assert_eq!(parsed_msg.parsed_payload.target_chain, eth_chain_id);
    assert_eq!(parsed_msg.parsed_payload.token_type, TOKEN_ID_ETH);
    assert_eq!(parsed_msg.parsed_payload.amount, sui_amount);
    let balance_before =
        eth_signer.get_balance(eth_address_1, None).await.unwrap() / U256::exp10(18);

    let message = eth_sui_bridge::Message::from(parsed_msg);
    let signatures = get_signatures(bridge_test_cluster.bridge_client(), nonce, sui_chain_id).await;

    let eth_sui_bridge = EthSuiBridge::new(
        bridge_test_cluster.contracts().sui_bridge,
        eth_signer.clone().into(),
    );
    let call = eth_sui_bridge.transfer_bridged_tokens_with_signatures(signatures, message);
    let eth_claim_tx_receipt = send_eth_tx_and_get_tx_receipt(call).await;
    assert_eq!(eth_claim_tx_receipt.status.unwrap().as_u64(), 1);
    info!(
        "[Timer] Sui to Eth bridge transfer claimed in {:?}",
        timer.elapsed()
    );
    let balance_after =
        eth_signer.get_balance(eth_address_1, None).await.unwrap() / U256::exp10(18);
    info!("before balance: {:?}", balance_before);
    info!("after balance: {:?}", balance_after);
    // Assert eth_address_1 has received ETH
    assert_eq!(balance_after - balance_before, U256::from(amount));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_add_external_admin() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(false)
        .with_num_validators(3)
        .build()
        .await;

    let sender = bridge_test_cluster.sui_user_address();
    let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();

    let add_admin_action = BridgeAction::AddExternalCoinAdminAction(AddExternalCoinAdminAction {
        nonce: 0,
        chain_id: BridgeChainId::SuiCustom,
        coin_type: "test".to_string(),
        admin_address: "0x1234567890123456789012345678901234567890".to_string(),
    });

    let remove_admin_action =
        BridgeAction::RemoveExternalCoinAdminAction(RemoveExternalCoinAdminAction {
            nonce: 0,
            chain_id: BridgeChainId::SuiCustom,
            coin_type: "test".to_string(),
            admin_address: "0x1234567890123456789012345678901234567890".to_string(),
        });

    info!("Starting bridge cluster");

    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![add_admin_action.clone(), remove_admin_action.clone()],
        vec![add_admin_action.clone()],
        vec![remove_admin_action.clone()],
    ]);
    bridge_test_cluster.start_bridge_cluster(false,false,true).await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("Bridge cluster is up");

    let bridge_committee = Arc::new(
        bridge_test_cluster
            .bridge_client()
            .get_bridge_committee()
            .await
            .expect("Failed to get bridge committee"),
    );
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_action1 = agg
        .request_committee_signatures(add_admin_action)
        .await
        .expect("Failed to request committee signatures for AddExternalCoinAdminAction");

    let tx = build_add_external_coin_admin_transaction(
        sender,
        &bridge_test_cluster
            .wallet()
            .get_one_gas_object_owned_by_address(sender)
            .await
            .unwrap()
            .unwrap(),
        certified_action1,
        bridge_arg,
        1000,
    )
    .unwrap();

    let response = bridge_test_cluster.sign_and_execute_transaction(&tx).await;
    let effects = response.effects.unwrap();
    assert_eq!(effects.status(), &SuiExecutionStatus::Success);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_add_external_witness() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(false)
        .with_num_validators(3)
        .build()
        .await;

    let sender = bridge_test_cluster.sui_user_address();
    let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();
    let hex_str = EthAddress::from_str("7518085822fAA839EeB59035a74A87b4220C6629").unwrap();

    //let addr=decode(hex_str.trim_start_matches("0x")).unwrap();

    let add_witness_action =
        BridgeAction::AddExternalCoinWitnessAction(AddExternalCoinWitnessAction {
            nonce: 0,
            chain_id: BridgeChainId::SuiCustom,
            coin_type: "test".to_string(),
            witness_address: hex_str.clone(),
        });

    let remove_witness_action =
        BridgeAction::RemoveExternalCoinWitnessAction(RemoveExternalCoinWitnessAction {
            nonce: 0,
            chain_id: BridgeChainId::SuiCustom,
            coin_type: "test".to_string(),
            witness_address: hex_str.clone(),
        });

    info!("Starting bridge cluster");

    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![add_witness_action.clone(), remove_witness_action.clone()],
        vec![add_witness_action.clone()],
        vec![remove_witness_action.clone()],
    ]);
    bridge_test_cluster.start_bridge_cluster(false,false,true).await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("Bridge cluster is up");

    let bridge_committee = Arc::new(
        bridge_test_cluster
            .bridge_client()
            .get_bridge_committee()
            .await
            .expect("Failed to get bridge committee"),
    );
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_action1 = agg
        .request_committee_signatures(add_witness_action)
        .await
        .expect("Failed to request committee signatures for AddExternalCoinWitnessAction");

    let tx = build_add_external_coin_witness_transaction(
        sender,
        &bridge_test_cluster
            .wallet()
            .get_one_gas_object_owned_by_address(sender)
            .await
            .unwrap()
            .unwrap(),
        certified_action1,
        bridge_arg,
        1000,
    )
    .unwrap();

    let response = bridge_test_cluster.sign_and_execute_transaction(&tx).await;
    let effects = response.effects.unwrap();
    assert_eq!(effects.status(), &SuiExecutionStatus::Success);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_add_external_target() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(false)
        .with_num_validators(3)
        .build()
        .await;

    let sender = bridge_test_cluster.sui_user_address();
    let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();

    let add_target_action =
        BridgeAction::AddExternalCoinTargetAction(AddExternalCoinTargetAction {
            nonce: 0,
            chain_id: BridgeChainId::SuiCustom,
            coin_type: "test".to_string(),
            target_address: "0x1234567890123456789012345678901234567890".to_string(),
        });

    let remove_target_action =
        BridgeAction::RemoveExternalCoinTargetAction(RemoveExternalCoinTargetAction {
            nonce: 0,
            chain_id: BridgeChainId::SuiCustom,
            coin_type: "test".to_string(),
            target_address: "0x1234567890123456789012345678901234567890".to_string(),
        });

    info!("Starting bridge cluster");

    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![add_target_action.clone(), remove_target_action.clone()],
        vec![add_target_action.clone()],
        vec![remove_target_action.clone()],
    ]);
    bridge_test_cluster.start_bridge_cluster(false,false,true).await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("Bridge cluster is up");

    let bridge_committee = Arc::new(
        bridge_test_cluster
            .bridge_client()
            .get_bridge_committee()
            .await
            .expect("Failed to get bridge committee"),
    );
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_action1 = agg
        .request_committee_signatures(add_target_action)
        .await
        .expect("Failed to request committee signatures for AddExternalCoinTargetAction");

    let tx = build_add_external_coin_target_transaction(
        sender,
        &bridge_test_cluster
            .wallet()
            .get_one_gas_object_owned_by_address(sender)
            .await
            .unwrap()
            .unwrap(),
        certified_action1,
        bridge_arg,
        1000,
    )
    .unwrap();

    let response = bridge_test_cluster.sign_and_execute_transaction(&tx).await;
    let effects = response.effects.unwrap();
    assert_eq!(effects.status(), &SuiExecutionStatus::Success);
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_add_refund_admin() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(false)
        .with_num_validators(3)
        .build()
        .await;

    let sender = bridge_test_cluster.sui_user_address();
    let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();

    let add_refund_action = BridgeAction::RefundAdminAction(RefundAdminAction {
        nonce: 0,
        chain_id: BridgeChainId::SuiCustom,
        op_type: 0,
        sui_address: sender.to_string(),
    });

    info!("Starting bridge cluster");

    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![add_refund_action.clone()],
        vec![add_refund_action.clone()],
        vec![add_refund_action.clone()],
    ]);
    bridge_test_cluster.start_bridge_cluster(false,false,true).await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("Bridge cluster is up");

    let bridge_committee = Arc::new(
        bridge_test_cluster
            .bridge_client()
            .get_bridge_committee()
            .await
            .expect("Failed to get bridge committee"),
    );
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_action1 = agg
        .request_committee_signatures(add_refund_action)
        .await
        .expect("Failed to request committee signatures for AddExternalCoinAdminAction");

    let tx = build_refund_admin_operate_transaction(
        sender,
        &bridge_test_cluster
            .wallet()
            .get_one_gas_object_owned_by_address(sender)
            .await
            .unwrap()
            .unwrap(),
        certified_action1,
        bridge_arg,
        1000,
    )
    .unwrap();

    let response = bridge_test_cluster.sign_and_execute_transaction(&tx).await;
    let effects = response.effects.unwrap();
    assert_eq!(effects.status(), &SuiExecutionStatus::Success);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_remove_external_admin() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(false)
        .with_num_validators(3)
        .build()
        .await;

    let sender = bridge_test_cluster.sui_user_address();
    let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();

    let add_admin_action = BridgeAction::AddExternalCoinAdminAction(AddExternalCoinAdminAction {
        nonce: 0,
        chain_id: BridgeChainId::SuiCustom,
        coin_type: "test".to_string(),
        admin_address: "0x1234567890123456789012345678901234567890".to_string(),
    });

    let remove_admin_action =
        BridgeAction::RemoveExternalCoinAdminAction(RemoveExternalCoinAdminAction {
            nonce: 0,
            chain_id: BridgeChainId::SuiCustom,
            coin_type: "test".to_string(),
            admin_address: "0x1234567890123456789012345678901234567890".to_string(),
        });

    info!("Starting bridge cluster");

    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![add_admin_action.clone(), remove_admin_action.clone()],
        vec![add_admin_action.clone()],
        vec![remove_admin_action.clone()],
    ]);
    bridge_test_cluster.start_bridge_cluster(false,false,true).await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("Bridge cluster is up");

    let bridge_committee = Arc::new(
        bridge_test_cluster
            .bridge_client()
            .get_bridge_committee()
            .await
            .expect("Failed to get bridge committee"),
    );
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_action1 = agg
        .request_committee_signatures(remove_admin_action)
        .await
        .expect("Failed to request committee signatures for AddExternalCoinAdminAction");

    let tx = build_remove_external_coin_admin_transaction(
        sender,
        &bridge_test_cluster
            .wallet()
            .get_one_gas_object_owned_by_address(sender)
            .await
            .unwrap()
            .unwrap(),
        certified_action1,
        bridge_arg,
        1000,
    )
    .unwrap();

    let response = bridge_test_cluster.sign_and_execute_transaction(&tx).await;
    let effects = response.effects.unwrap();
    assert_eq!(effects.status(), &SuiExecutionStatus::Success);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_remove_external_witness() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(false)
        .with_num_validators(3)
        .build()
        .await;

    let sender = bridge_test_cluster.sui_user_address();
    let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();
    let hex_str = EthAddress::from_str("7518085822fAA839EeB59035a74A87b4220C6629").unwrap();

    let add_witness_action =
        BridgeAction::AddExternalCoinWitnessAction(AddExternalCoinWitnessAction {
            nonce: 0,
            chain_id: BridgeChainId::SuiCustom,
            coin_type: "test".to_string(),
            witness_address: hex_str.clone(),
        });

    let remove_witness_action =
        BridgeAction::RemoveExternalCoinWitnessAction(RemoveExternalCoinWitnessAction {
            nonce: 0,
            chain_id: BridgeChainId::SuiCustom,
            coin_type: "test".to_string(),
            witness_address: hex_str.clone(),
        });

    info!("Starting bridge cluster");

    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![add_witness_action.clone(), remove_witness_action.clone()],
        vec![add_witness_action.clone()],
        vec![remove_witness_action.clone()],
    ]);
    bridge_test_cluster.start_bridge_cluster(false,false,true).await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("Bridge cluster is up");

    let bridge_committee = Arc::new(
        bridge_test_cluster
            .bridge_client()
            .get_bridge_committee()
            .await
            .expect("Failed to get bridge committee"),
    );
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_action1 = agg
        .request_committee_signatures(remove_witness_action)
        .await
        .expect("Failed to request committee signatures for RemoveExternalCoinWitnessAction");

    let tx = build_remove_external_coin_witness_transaction(
        sender,
        &bridge_test_cluster
            .wallet()
            .get_one_gas_object_owned_by_address(sender)
            .await
            .unwrap()
            .unwrap(),
        certified_action1,
        bridge_arg,
        1000,
    )
    .unwrap();

    let response = bridge_test_cluster.sign_and_execute_transaction(&tx).await;
    let effects = response.effects.unwrap();
    assert_eq!(effects.status(), &SuiExecutionStatus::Success);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_remove_external_target() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(false)
        .with_num_validators(3)
        .build()
        .await;

    let sender = bridge_test_cluster.sui_user_address();
    let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();

    let add_target_action =
        BridgeAction::AddExternalCoinTargetAction(AddExternalCoinTargetAction {
            nonce: 0,
            chain_id: BridgeChainId::SuiCustom,
            coin_type: "test".to_string(),
            target_address: "0x1234567890123456789012345678901234567890".to_string(),
        });

    let remove_target_action =
        BridgeAction::RemoveExternalCoinTargetAction(RemoveExternalCoinTargetAction {
            nonce: 0,
            chain_id: BridgeChainId::SuiCustom,
            coin_type: "test".to_string(),
            target_address: "0x1234567890123456789012345678901234567890".to_string(),
        });

    info!("Starting bridge cluster");

    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![add_target_action.clone(), remove_target_action.clone()],
        vec![add_target_action.clone()],
        vec![remove_target_action.clone()],
    ]);
    bridge_test_cluster.start_bridge_cluster(false,false,true).await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("Bridge cluster is up");

    let bridge_committee = Arc::new(
        bridge_test_cluster
            .bridge_client()
            .get_bridge_committee()
            .await
            .expect("Failed to get bridge committee"),
    );
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_action1 = agg
        .request_committee_signatures(remove_target_action)
        .await
        .expect("Failed to request committee signatures for AddExternalCoinTargetAction");

    let tx = build_remove_external_coin_target_transaction(
        sender,
        &bridge_test_cluster
            .wallet()
            .get_one_gas_object_owned_by_address(sender)
            .await
            .unwrap()
            .unwrap(),
        certified_action1,
        bridge_arg,
        1000,
    )
    .unwrap();

    let response = bridge_test_cluster.sign_and_execute_transaction(&tx).await;
    let effects = response.effects.unwrap();
    assert_eq!(effects.status(), &SuiExecutionStatus::Success);
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_set_bridge_fee_on_cross_out() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(false)
        .with_num_validators(3)
        .build()
        .await;

    let sender = bridge_test_cluster.sui_user_address();
    let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();

    let add_action =
        BridgeAction::UpdateBridgeFeeOnCrossOutAction(UpdateBridgeFeeOnCrossOutAction {
            nonce: 0,
            chain_id: BridgeChainId::SuiCustom,
            to_chain_id: BridgeChainId::ArbCustom,
            token_id: 11,
            mode: 1,
            amount: 800,
        });
    let remove_action =
        BridgeAction::UpdateBridgeFeeOnCrossInAction(UpdateBridgeFeeOnCrossInAction {
            nonce: 0,
            chain_id: BridgeChainId::SuiCustom,
            from_chain_id: BridgeChainId::ArbCustom,
            token_id: 11,
            mode: 1,
            amount: 800,
        });

    info!("Starting bridge cluster");

    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![add_action.clone(), remove_action.clone()],
        vec![add_action.clone()],
        vec![remove_action.clone()],
    ]);
    bridge_test_cluster.start_bridge_cluster(false,false,true).await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("Bridge cluster is up");

    let bridge_committee = Arc::new(
        bridge_test_cluster
            .bridge_client()
            .get_bridge_committee()
            .await
            .expect("Failed to get bridge committee"),
    );
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_action1 = agg
        .request_committee_signatures(add_action)
        .await
        .expect("Failed to request committee signatures for UpdateBridgeFeeOnCrossOutAction");

    let tx = build_set_cross_out_bridge_fee_transaction(
        sender,
        &bridge_test_cluster
            .wallet()
            .get_one_gas_object_owned_by_address(sender)
            .await
            .unwrap()
            .unwrap(),
        certified_action1,
        bridge_arg,
        1000,
    )
    .unwrap();
info!("bridge_test_cluster.sign_and_execute_transaction before");

    let response = bridge_test_cluster.sign_and_execute_transaction(&tx).await;
    let effects = response.effects.unwrap();
    assert_eq!(effects.status(), &SuiExecutionStatus::Success);
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_add_lp_token_on_evm(){
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(false)
        .with_num_validators(3)
        .build()
        .await;
    // pub nonce: u64,
    // pub chain_id: BridgeChainId,
    // pub protocol_type: u64,
    // pub token_id: u64,
    // pub lp_token_id: u64,
    let add_lp_token =
        BridgeAction::AddLpTokenIdAction(AddLpTokenIdAction {
            nonce: 0,
            chain_id: BridgeChainId::EthCustom,
            protocol_type: 0, //aave
            token_id: 4, //usdt
            lp_token_id: 10000, //lp_token
    });

    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![add_lp_token.clone()],
        vec![add_lp_token.clone()],
        vec![add_lp_token.clone()],
    ]);

    bridge_test_cluster.start_bridge_cluster(false,false,true).await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("Bridge cluster is up");

    let bridge_committee = Arc::new(
        bridge_test_cluster
            .bridge_client()
            .get_bridge_committee()
            .await
            .expect("Failed to get bridge committee AddLpTokenIdAction"),
    );

    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_eth_action = agg
        .request_committee_signatures(add_lp_token)
        .await
        .expect("Failed to request committee signatures for AddLpTokenIdAction");

    let config_contract = bridge_test_cluster.contracts().bridge_config;


    let eth_signer = bridge_test_cluster.get_eth_signer().await;
    let eth_call = build_eth_transaction(config_contract, eth_signer, certified_eth_action)
        .await
        .unwrap();
    let eth_receipt = send_eth_tx_and_get_tx_receipt(eth_call).await;
    assert_eq!(eth_receipt.status.unwrap().as_u64(), 1);


    //Verify
    let lp_token_id = bridge_test_cluster
        .eth_env().get_protocol_type_lp_token_id(0, 4)
        .await;

    assert_eq!(lp_token_id, 10000);

} 


#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async  fn test_update_invest_address(){
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(false)
        .with_num_validators(3)
        .build()
        .await;
    let invest_address=EthAddress::from_str("0x1234567890123456789012345678901234567890").unwrap();
    let update_action =
        BridgeAction::UpdateInvestAddressAction(UpdateInvestAddressAction {
            nonce: 0,
            chain_id: BridgeChainId::EthCustom,
            invest_address: invest_address,
    });

    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![update_action.clone(), update_action.clone()],
        vec![update_action.clone()],
        vec![update_action.clone()],
    ]);


    bridge_test_cluster.start_bridge_cluster(false,false,true).await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("Bridge cluster is up");
   

    let bridge_committee = Arc::new(
        bridge_test_cluster
            .bridge_client()
            .get_bridge_committee()
            .await
            .expect("Failed to get bridge committee UpdateInvestAddressAction"),
    );
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_eth_action = agg
        .request_committee_signatures(update_action)
        .await
        .expect("Failed to request committee signatures for UpdateInvestAddressAction");

    let sui_bridge = bridge_test_cluster.contracts().sui_bridge;
    let eth_signer = bridge_test_cluster.get_eth_signer().await;
    let eth_call = build_eth_transaction(sui_bridge, eth_signer, certified_eth_action)
        .await
        .unwrap();
    let eth_receipt = send_eth_tx_and_get_tx_receipt(eth_call).await;
    assert_eq!(eth_receipt.status.unwrap().as_u64(), 1);

    //Verify
    let expect_invest_addr = bridge_test_cluster
        .eth_env().get_invest_address()
        .await;

    assert_eq!(expect_invest_addr, invest_address);


}


#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_set_bridge_fee_on_cross_in() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(false)
        .with_num_validators(3)
        .build()
        .await;

    let sender = bridge_test_cluster.sui_user_address();
    let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();

    let add_action =
        BridgeAction::UpdateBridgeFeeOnCrossOutAction(UpdateBridgeFeeOnCrossOutAction {
            nonce: 0,
            chain_id: BridgeChainId::SuiCustom,
            to_chain_id: BridgeChainId::ArbCustom,
            token_id: 11,
            mode: 1,
            amount: 800,
        });
    let remove_action =
        BridgeAction::UpdateBridgeFeeOnCrossInAction(UpdateBridgeFeeOnCrossInAction {
            nonce: 0,
            chain_id: BridgeChainId::SuiCustom,
            from_chain_id: BridgeChainId::ArbCustom,
            token_id: 11,
            mode: 1,
            amount: 800,
        });

    info!("Starting bridge cluster");

    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![add_action.clone(), remove_action.clone()],
        vec![add_action.clone()],
        vec![remove_action.clone()],
    ]);
    bridge_test_cluster.start_bridge_cluster(false,false,true).await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("Bridge cluster is up");

    let bridge_committee = Arc::new(
        bridge_test_cluster
            .bridge_client()
            .get_bridge_committee()
            .await
            .expect("Failed to get bridge committee"),
    );
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_action1 = agg
        .request_committee_signatures(remove_action)
        .await
        .expect("Failed to request committee signatures for UpdateBridgeFeeOnCrossInAction");

    let tx = build_set_cross_in_bridge_fee_transaction(
        sender,
        &bridge_test_cluster
            .wallet()
            .get_one_gas_object_owned_by_address(sender)
            .await
            .unwrap()
            .unwrap(),
        certified_action1,
        bridge_arg,
        1000,
    )
    .unwrap();

    let response = bridge_test_cluster.sign_and_execute_transaction(&tx).await;
    let effects = response.effects.unwrap();
    assert_eq!(effects.status(), &SuiExecutionStatus::Success);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_withdraw_fee_cap() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(false)
        .with_num_validators(3)
        .build()
        .await;

    let sender = bridge_test_cluster.sui_user_address();
    let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();

    // let eth_coin = bridge_test_cluster
    //     .sui_client()
    //     .coin_read_api()
    //     .get_all_coins(sender, None, None)
    //     .await
    //     .unwrap()
    //     .data
    //     .iter()
    //     .find(|c| c.coin_type.contains("ETH"))
    //     .expect("Recipient should have received ETH coin now")
    //     .clone();

    let add_action = BridgeAction::WithdrawBridgeFeeAction(WithdrawBridgeFeeAction {
        nonce: 0,
        chain_id: BridgeChainId::SuiCustom,
        addr: sender,
        coin_type: "test".to_string(),
        amount: 1_000_000_0000,
    });
    let remove_action =
        BridgeAction::UpdateBridgeFeeOnCrossInAction(UpdateBridgeFeeOnCrossInAction {
            nonce: 0,
            chain_id: BridgeChainId::SuiCustom,
            from_chain_id: BridgeChainId::ArbCustom,
            token_id: 11,
            mode: 1,
            amount: 800,
        });

    info!("Starting bridge cluster");

    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![add_action.clone(), remove_action.clone()],
        vec![add_action.clone()],
        vec![remove_action.clone()],
    ]);
    bridge_test_cluster.start_bridge_cluster(false,false,true).await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("Bridge cluster is up");

    let bridge_committee = Arc::new(
        bridge_test_cluster
            .bridge_client()
            .get_bridge_committee()
            .await
            .expect("Failed to get bridge committee"),
    );
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_action1 = agg
        .request_committee_signatures(add_action)
        .await
        .expect("Failed to request committee signatures for WithdrawBridgeFeeAction");

    let tx = build_withdraw_fee_cap_transaction(
        sender,
        &bridge_test_cluster
            .wallet()
            .get_one_gas_object_owned_by_address(sender)
            .await
            .unwrap()
            .unwrap(),
        certified_action1,
        bridge_arg,
        1000,
    )
    .unwrap();

    let response = bridge_test_cluster.sign_and_execute_transaction(&tx).await;
    let effects = response.effects.unwrap();
    let cap = bridge_test_cluster
        .sui_client()
        .read_api()
        .get_object_with_options(
            effects.created()[0].object_id(),
            SuiObjectDataOptions {
                show_type: true,
                show_owner: true,
                show_previous_transaction: true,
                show_display: true,
                show_content: true,
                show_bcs: true,
                show_storage_rebate: true,
            },
        )
        .await.unwrap();
    let obj_data= cap.data.expect("cap is nonce");
    let coin_type=obj_data.object_type().expect("object_type is nonce").to_string();
    assert_eq!(effects.status(), &SuiExecutionStatus::Success);
    assert_eq!(coin_type,"0x000000000000000000000000000000000000000000000000000000000000000b::bridge_fee::WithdrawBridgeFeeCap")
    //assert_eq!(obj_data.object_type()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_add_token_on_token_list() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(false)
        .with_num_validators(3)
        .build()
        .await;

    let sender = bridge_test_cluster.sui_user_address();
    let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();

    let add_action = BridgeAction::AddTokenOnTokenListAction(AddTokenOnTokenListAction {
        nonce: 0,
        chain_id: BridgeChainId::SuiCustom,
        from_chain_id: BridgeChainId::SuiCustom,
        to_chain_id: BridgeChainId::ArbCustom,
        token_id: 11,
    });

    let remove_action = BridgeAction::RemoveTokenOnTokenListAction(RemoveTokenOnTokenListAction {
        nonce: 0,
        chain_id: BridgeChainId::SuiCustom,
        from_chain_id: BridgeChainId::SuiCustom,
        to_chain_id: BridgeChainId::ArbCustom,
        token_id: 11,
    });

    info!("Starting bridge cluster");

    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![add_action.clone(), remove_action.clone()],
        vec![add_action.clone()],
        vec![remove_action.clone()],
    ]);
    bridge_test_cluster.start_bridge_cluster(false,false,true).await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("Bridge cluster is up");

    let bridge_committee = Arc::new(
        bridge_test_cluster
            .bridge_client()
            .get_bridge_committee()
            .await
            .expect("Failed to get bridge committee"),
    );
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_action1 = agg
        .request_committee_signatures(add_action)
        .await
        .expect("Failed to request committee signatures for AddTokenOnTokenListAction");

    let tx = build_add_token_on_token_list_transaction(
        sender,
        &bridge_test_cluster
            .wallet()
            .get_one_gas_object_owned_by_address(sender)
            .await
            .unwrap()
            .unwrap(),
        certified_action1,
        bridge_arg,
        1000,
    )
    .unwrap();

    let response = bridge_test_cluster.sign_and_execute_transaction(&tx).await;
    let effects = response.effects.unwrap();
    assert_eq!(effects.status(), &SuiExecutionStatus::Success);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_remove_token_on_token_list() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(false)
        .with_num_validators(3)
        .build()
        .await;

    let sender = bridge_test_cluster.sui_user_address();
    let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();

    let add_action = BridgeAction::AddTokenOnTokenListAction(AddTokenOnTokenListAction {
        nonce: 0,
        chain_id: BridgeChainId::SuiCustom,
        from_chain_id: BridgeChainId::SuiCustom,
        to_chain_id: BridgeChainId::ArbCustom,
        token_id: 11,
    });

    let remove_action = BridgeAction::RemoveTokenOnTokenListAction(RemoveTokenOnTokenListAction {
        nonce: 0,
        chain_id: BridgeChainId::SuiCustom,
        from_chain_id: BridgeChainId::SuiCustom,
        to_chain_id: BridgeChainId::ArbCustom,
        token_id: 11,
    });

    info!("Starting bridge cluster");

    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![add_action.clone(), remove_action.clone()],
        vec![add_action.clone()],
        vec![remove_action.clone()],
    ]);
    bridge_test_cluster.start_bridge_cluster(false,false,true).await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("Bridge cluster is up");

    let bridge_committee = Arc::new(
        bridge_test_cluster
            .bridge_client()
            .get_bridge_committee()
            .await
            .expect("Failed to get bridge committee"),
    );
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_action1 = agg
        .request_committee_signatures(remove_action)
        .await
        .expect("Failed to request committee signatures for RemoveTokenOnTokenListAction");

    let tx = build_remove_token_on_token_list_transaction(
        sender,
        &bridge_test_cluster
            .wallet()
            .get_one_gas_object_owned_by_address(sender)
            .await
            .unwrap()
            .unwrap(),
        certified_action1,
        bridge_arg,
        1000,
    )
    .unwrap();

    let response = bridge_test_cluster.sign_and_execute_transaction(&tx).await;
    let effects = response.effects.unwrap();
    assert_eq!(effects.status(), &SuiExecutionStatus::Success);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_set_single_transfer_limit_on_eth() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(false)
        .with_num_validators(3)
        .build()
        .await;
    let limit = 70_000_000_000;

    let eth_action =
        BridgeAction::SingleTransferLimitUpdateAction(SingleTransferLimitUpdateAction {
            nonce: 0,
            chain_id: BridgeChainId::EthCustom,
            sending_chain_id: BridgeChainId::SuiCustom,
            new_usd_limit: limit,
        });

    let remove_action = BridgeAction::RemoveTokenOnTokenListAction(RemoveTokenOnTokenListAction {
        nonce: 0,
        chain_id: BridgeChainId::SuiCustom,
        from_chain_id: BridgeChainId::SuiCustom,
        to_chain_id: BridgeChainId::ArbCustom,
        token_id: 11,
    });

    info!("Starting bridge cluster");

    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![eth_action.clone(), remove_action.clone()],
        vec![eth_action.clone()],
        vec![remove_action.clone()],
    ]);

    bridge_test_cluster.start_bridge_cluster(false,false,true).await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("Bridge cluster is up");

    let bridge_committee = Arc::new(
        bridge_test_cluster
            .bridge_client()
            .get_bridge_committee()
            .await
            .expect("Failed to get bridge committee SingleTransferLimitUpdateAction"),
    );
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_eth_action = agg
        .request_committee_signatures(eth_action)
        .await
        .expect("Failed to request committee signatures for SingleTransferLimitUpdateAction");

    let config_address = bridge_test_cluster.contracts().bridge_limiter;
    let eth_signer = bridge_test_cluster.get_eth_signer().await;
    let eth_call = build_eth_transaction(config_address, eth_signer, certified_eth_action)
        .await
        .unwrap();
    let eth_receipt = send_eth_tx_and_get_tx_receipt(eth_call).await;
    assert_eq!(eth_receipt.status.unwrap().as_u64(), 1);

    //Verify
    let amount = bridge_test_cluster
        .eth_env()
        .get_single_transfer_limit()
        .await;

    assert_eq!(amount, limit);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_add_new_coin_on_solana() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_solana_env(true)
        .with_solana_chain_id(BridgeChainId::SolanaTestnet)
        .with_bridge_cluster(false)
        .with_num_validators(3) 
        .build()
        .await;
    let env=bridge_test_cluster
        .solana_env();

    let solana_signer = env.get_signer().await.expect("Failed to get solana signer");

    let client=env.client.clone();
    let usdc_deployment_result = crate::utils::deploy_usdc_in_anchor_client(
        &client,
        env.contract(), 
        &solana_signer, 
        1000000,
        6
    ).await.expect("Failed to deploy USDT");

    let token_id=4;
    let token_address = usdc_deployment_result.mint;   
    let benfen_decimal = 9;
    let token_price = 10000;
    let nonce = 1;

    let add_solana_coin_action =
        BridgeAction::AddTokenOnSolanaAction(
            AddTokenOnSolanaAction {
                nonce,
                chain_id: BridgeChainId::SolanaTestnet,
                native: false,
                token_id,
                token_address,
                benfen_decimal,
                token_price,
            }
    );

    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![add_solana_coin_action.clone(), add_solana_coin_action.clone()],
        vec![add_solana_coin_action.clone()],
        vec![add_solana_coin_action.clone()],
    ]);

    bridge_test_cluster.start_bridge_cluster(false,false,true).await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("Bridge cluster is up");

    let bridge_committee = Arc::new(
        bridge_test_cluster
            .bridge_client()
            .get_bridge_committee()
            .await
            .expect("Failed to get bridge committee"),
    );
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_solana_action = agg
        .request_committee_signatures(add_solana_coin_action)
        .await
        .expect("Failed to request committee signatures for AddTokenOnSolanaAction");

    let program = Arc::new(client.program(benfen_bridge::ID).expect("Failed to get program"));

    let add_token_ix=build_solana_transaction(
        program.clone(),
        BridgeChainId::SolanaTestnet,
        BridgeChainId::SuiCustom,
        &solana_signer,
        certified_solana_action,
    ).await.expect("Failed to build solana transaction");

     let signature = program
        .request()
        .instruction(add_token_ix)
        .signer(solana_signer.clone())
        .send()
        .await.expect("Failed to send and confirm transaction");

    info!("add_token signature: {:?}", signature);

    let add_token_accounts = 
        query_solana_account::get_add_token_account(
            program.clone().id(), 
            BridgeChainId::SuiCustom as u8, 
            token_id, 
            AddTokensOnSolana as u8,
    );

    // 校验token 是否已经存在
    let new_token_account = program
        .account::<TokenConfigAccount>(add_token_accounts.token_config)
        .await.expect("Failed to get token config account");

    let actual_token_id: u64 = new_token_account.token_id;
    assert_eq!(actual_token_id, token_id);

}

// Test add new coins on both Sui and Eth
// Also test bridge ndoe handling `NewTokenEvent``
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_add_new_coins_on_sui_and_eth() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(false)
        .with_num_validators(3)
        .build()
        .await;

    let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();

    // Register tokens on Sui
    let token_id = 7;
    let token_sui_decimal = 9; // this needs to match ka.move
    let token_price = 10000;
    let sender = bridge_test_cluster.sui_user_address();
    info!("Published new token");
    let sui_action = publish_and_register_coins_return_add_coins_on_sui_action(
        bridge_test_cluster.wallet(),
        bridge_arg,
        vec![Path::new("../../bridge/move/tokens/mock/ka").into()],
        vec![token_id],
        vec![token_price],
        1, // seq num
    )
    .await;
    let new_token_erc_address = bridge_test_cluster.contracts().ka;
    let eth_action = BridgeAction::AddTokensOnEvmAction(AddTokensOnEvmAction {
        nonce: 0,
        chain_id: BridgeChainId::EthCustom,
        native: true,
        token_ids: vec![token_id],
        token_addresses: vec![new_token_erc_address],
        token_sui_decimals: vec![token_sui_decimal],
        token_prices: vec![token_price],
    });

    info!("Starting bridge cluster");

    bridge_test_cluster.set_approved_governance_actions_for_next_start(vec![
        vec![sui_action.clone(), eth_action.clone()],
        vec![sui_action.clone()],
        vec![eth_action.clone()],
    ]);
    bridge_test_cluster.start_bridge_cluster(false,false,true).await;
    bridge_test_cluster
        .wait_for_bridge_cluster_to_be_up(10)
        .await;
    info!("Bridge cluster is up");

    let bridge_committee = Arc::new(
        bridge_test_cluster
            .bridge_client()
            .get_bridge_committee()
            .await
            .expect("Failed to get bridge committee"),
    );
    let agg = BridgeAuthorityAggregator::new_for_testing(bridge_committee);
    let certified_sui_action = agg
        .request_committee_signatures(sui_action)
        .await
        .expect("Failed to request committee signatures for AddTokensOnSuiAction");
    let certified_eth_action = agg
        .request_committee_signatures(eth_action.clone())
        .await
        .expect("Failed to request committee signatures for AddTokensOnEvmAction");

    let tx = build_add_tokens_on_sui_transaction(
        sender,
        &bridge_test_cluster
            .wallet()
            .get_one_gas_object_owned_by_address(sender)
            .await
            .unwrap()
            .unwrap(),
        certified_sui_action,
        bridge_arg,
        1000,
    )
    .unwrap();

    let response = bridge_test_cluster.sign_and_execute_transaction(&tx).await;
    let effects = response.effects.unwrap();
    assert_eq!(effects.status(), &SuiExecutionStatus::Success);
    assert!(response.events.unwrap().data.iter().any(|e| {
        let sui_bridge_event = SuiBridgeEvent::try_from_sui_event(e).unwrap().unwrap();
        match sui_bridge_event {
            SuiBridgeEvent::NewTokenEvent(e) => {
                assert_eq!(e.token_id, token_id);
                true
            }
            _ => false,
        }
    }));
    info!("Approved new token on Sui");

    // Assert new token is correctly added
    let treasury_summary = bridge_test_cluster
        .bridge_client()
        .get_treasury_summary()
        .await
        .unwrap();
    dbg!(&treasury_summary.id_token_type_map);
    assert_eq!(treasury_summary.id_token_type_map.len(), 7); // 5 + 1 new token
    let (id, _type) = treasury_summary
        .id_token_type_map
        .iter()
        .find(|(id, _)| id == &token_id)
        .unwrap();
    let (_type, metadata) = treasury_summary
        .supported_tokens
        .iter()
        .find(|(_type_, _)| _type == _type_)
        .unwrap();
    assert_eq!(
        metadata,
        &BridgeTokenMetadata {
            id: *id,
            decimal_multiplier: 1_000_000_000,
            notional_value: token_price,
            native_token: false,
        }
    );

    // Add new token on EVM
    let config_address = bridge_test_cluster.contracts().bridge_config;
    let eth_signer = bridge_test_cluster.get_eth_signer().await;
    let eth_call = build_eth_transaction(config_address, eth_signer, certified_eth_action)
        .await
        .unwrap();
    let eth_receipt = send_eth_tx_and_get_tx_receipt(eth_call).await;
    assert_eq!(eth_receipt.status.unwrap().as_u64(), 1);

    // Verify new tokens are added on EVM
    let (address, dp, price) = bridge_test_cluster
        .eth_env()
        .get_supported_token(token_id)
        .await;
    assert_eq!(address, new_token_erc_address);
    assert_eq!(dp, 9);
    assert_eq!(price, token_price);

    initiate_bridge_erc20_to_sui(
        &bridge_test_cluster,
        100,
        new_token_erc_address,
        token_id,
        0,
    )
    .await
    .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_bridge_usdt_to_sui() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(true)
        .with_num_validators(3)
        .build()
        .await;

    let timer = std::time::Instant::now();

    // let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();

    let treasury_summary = bridge_test_cluster
        .bridge_client()
        .get_treasury_summary()
        .await
        .unwrap();
    assert_eq!(treasury_summary.id_token_type_map.len(), 6); // 4 + 1 new token
    let (_id, _type) = treasury_summary
        .id_token_type_map
        .iter()
        .find(|(id, _)| id == &TOKEN_ID_USDT)
        .unwrap();
    let (_type, _metadata) = treasury_summary
        .supported_tokens
        .iter()
        .find(|(_type_, _)| _type == _type_)
        .unwrap();
    let new_token_erc_address = bridge_test_cluster.contracts().usdt;
    initiate_bridge_erc20_to_sui(
        &bridge_test_cluster,
        100,
        new_token_erc_address,
        TOKEN_ID_USDT,
        0,
    )
    .await
    .unwrap();
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                TokenTransferApproved.get().unwrap().clone(),
                TokenTransferClaimed.get().unwrap().clone(),
            ]),
            true,
        )
        .await; // There are exactly 1 approved and 1 claimed event
    assert_eq!(events.len(), 2);
    sleep(Duration::from_secs(10));
    let sui_address = bridge_test_cluster.sui_user_address();
    let all_coins = bridge_test_cluster
        .sui_client()
        .coin_read_api()
        .get_all_coins(sui_address, None, None)
        .await
        .unwrap()
        .data;
    let busd_coin = all_coins
        .iter()
        .find(|c| c.coin_type.contains("BUSD"))
        .expect("Recipient should have received BUSD coin now")
        .clone();
    assert_eq!(busd_coin.balance, 100_000_000_000);
    info!(
        "[Timer] Eth to Sui bridge USDT transfer finished in {:?}",
        timer.elapsed()
    );

    let timer = std::time::Instant::now();

    // Now let the recipient send the coin back to ETH
    let eth_address_1 = EthAddress::random();
    let nonce = 0;

    let _sui_to_eth_bridge_action = initiate_bridge_sui_to_eth(
        &bridge_test_cluster,
        eth_address_1,
        busd_coin.object_ref(),
        nonce,
        100_000_000_000,
        TOKEN_ID_USDT,
    )
    .await
    .unwrap();
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                SuiToEthTokenBridgeV2.get().unwrap().clone(),
                TokenTransferApproved.get().unwrap().clone(),
                TokenTransferClaimed.get().unwrap().clone(),
            ]),
            true,
        )
        .await;
    // There are exactly 1 deposit and 1 approved event
    assert_eq!(events.len(), 2);
    info!(
        "[Timer] Sui to Eth bridge transfer approved in {:?}",
        timer.elapsed()
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_bridge_usdt_to_sui_fast_path() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(true)
        .with_num_validators(3)
        .with_enable_fast_path_latest(true)
        .with_enable_fast_path_safe(false)
        .with_enable_fast_path_finalized(false)
        .build()
        .await;

    let timer = std::time::Instant::now();

    // let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();

    let treasury_summary = bridge_test_cluster
        .bridge_client()
        .get_treasury_summary()
        .await
        .unwrap();
    assert_eq!(treasury_summary.id_token_type_map.len(), 6); // 4 + 1 new token
    let (_id, _type) = treasury_summary
        .id_token_type_map
        .iter()
        .find(|(id, _)| id == &TOKEN_ID_USDT)
        .unwrap();
    let (_type, _metadata) = treasury_summary
        .supported_tokens
        .iter()
        .find(|(_type_, _)| _type == _type_)
        .unwrap();
    let new_token_erc_address = bridge_test_cluster.contracts().usdt;
    initiate_bridge_erc20_to_sui(
        &bridge_test_cluster,
        50,
        new_token_erc_address,
        TOKEN_ID_USDT,
        0,
    )
    .await
    .unwrap();
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                TokenTransferApproved.get().unwrap().clone(),
                TokenTransferClaimed.get().unwrap().clone(),
            ]),
            true,
        )
        .await; // There are exactly 1 approved and 1 claimed event
    assert_eq!(events.len(), 2);
    sleep(Duration::from_secs(10));
    let sui_address = bridge_test_cluster.sui_user_address();
    let all_coins = bridge_test_cluster
        .sui_client()
        .coin_read_api()
        .get_all_coins(sui_address, None, None)
        .await
        .unwrap()
        .data;
    let busd_coin = all_coins
        .iter()
        .find(|c| c.coin_type.contains("BUSD"))
        .expect("Recipient should have received BUSD coin now")
        .clone();
    assert_eq!(busd_coin.balance, 50_000_000_000);
    info!(
        "[Timer] Eth to Sui bridge USDT transfer finished in {:?}",
        timer.elapsed()
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_bridge_usdt_to_sui_fast_path_limit() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(true)
        .with_num_validators(3)
        .with_enable_fast_path_latest(true)
        .with_enable_fast_path_safe(false)
        .with_enable_fast_path_finalized(false)
        .build()
        .await;

    let timer = std::time::Instant::now();

    // let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();

    let treasury_summary = bridge_test_cluster
        .bridge_client()
        .get_treasury_summary()
        .await
        .unwrap();
    assert_eq!(treasury_summary.id_token_type_map.len(), 6); // 4 + 1 new token
    let (_id, _type) = treasury_summary
        .id_token_type_map
        .iter()
        .find(|(id, _)| id == &TOKEN_ID_USDT)
        .unwrap();
    let (_type, _metadata) = treasury_summary
        .supported_tokens
        .iter()
        .find(|(_type_, _)| _type == _type_)
        .unwrap();
    let new_token_erc_address = bridge_test_cluster.contracts().usdt;
    initiate_bridge_erc20_to_sui(
        &bridge_test_cluster,
        50,
        new_token_erc_address,
        TOKEN_ID_USDT,
        0,
    )
    .await
    .unwrap();
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                TokenTransferApproved.get().unwrap().clone(),
                TokenTransferClaimed.get().unwrap().clone(),
            ]),
            true,
        )
        .await; // There are exactly 1 approved and 1 claimed event
    assert_eq!(events.len(), 2);
    sleep(Duration::from_secs(10));
    let sui_address = bridge_test_cluster.sui_user_address();
    let all_coins = bridge_test_cluster
        .sui_client()
        .coin_read_api()
        .get_all_coins(sui_address, None, None)
        .await
        .unwrap()
        .data;
    let busd_coin = all_coins
        .iter()
        .find(|c| c.coin_type.contains("BUSD"))
        .expect("Recipient should have received BUSD coin now")
        .clone();
    assert_eq!(busd_coin.balance, 50_000_000_000);
    info!(
        "[Timer] Eth to Sui bridge USDT transfer finished in {:?}",
        timer.elapsed()
    );
    let _result = initiate_bridge_erc20_to_sui(
        &bridge_test_cluster,
        40,
        new_token_erc_address,
        TOKEN_ID_USDT,
        1,
    )
    .await
    .unwrap();


    sleep(Duration::from_secs(10));
    let sui_address = bridge_test_cluster.sui_user_address();
    let all_coins = bridge_test_cluster
        .sui_client()
        .coin_read_api()
        .get_all_coins(sui_address, None, None)
        .await
        .unwrap()
        .data;
    let busd_coin = all_coins
        .iter()
        .filter(|c| c.coin_type.contains("BUSD"))
        .collect::<Vec<_>>();
    assert_eq!(busd_coin.len(), 2);
    //should fail
    let result = initiate_bridge_erc20_to_sui(
        &bridge_test_cluster,
        51,
        new_token_erc_address,
        TOKEN_ID_USDT,
        2,
    )
    .await
    .unwrap_err();
    info!("result: {:?}", result);
    let all_coins = bridge_test_cluster
        .sui_client()
        .coin_read_api()
        .get_all_coins(sui_address, None, None)
        .await
        .unwrap()
        .data;
    let busd_coin = all_coins
        .iter()
        .filter(|c| c.coin_type.contains("BUSD"))
        .collect::<Vec<_>>();
    assert_eq!(busd_coin.len(), 2);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_bridge_usdt_to_sui_from_bsc() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_eth_chain_id(BridgeChainId::BscCustom)
        .with_bridge_cluster(true)
        .with_num_validators(3)
        .build()
        .await;

    let timer = std::time::Instant::now();

    // let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();

    let treasury_summary = bridge_test_cluster
        .bridge_client()
        .get_treasury_summary()
        .await
        .unwrap();
    assert_eq!(treasury_summary.id_token_type_map.len(), 6); // 4 + 1 new token
    let (_id, _type) = treasury_summary
        .id_token_type_map
        .iter()
        .find(|(id, _)| id == &TOKEN_ID_USDT)
        .unwrap();
    let (_type, _metadata) = treasury_summary
        .supported_tokens
        .iter()
        .find(|(_type_, _)| _type == _type_)
        .unwrap();
    let new_token_erc_address = bridge_test_cluster.contracts().usdt;
    initiate_bridge_erc20_to_sui(
        &bridge_test_cluster,
        3,
        new_token_erc_address,
        TOKEN_ID_USDT,
        0,
    )
    .await
    .unwrap();
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                TokenTransferApproved.get().unwrap().clone(),
                TokenTransferClaimed.get().unwrap().clone(),
            ]),
            true,
        )
        .await; // There are exactly 1 approved and 1 claimed event
    assert_eq!(events.len(), 2);
    sleep(Duration::from_secs(10));
    let sui_address = bridge_test_cluster.sui_user_address();
    let all_coins = bridge_test_cluster
        .sui_client()
        .coin_read_api()
        .get_all_coins(sui_address, None, None)
        .await
        .unwrap()
        .data;
    let busd_coin = all_coins
        .iter()
        .find(|c| c.coin_type.contains("BUSD"))
        .expect("Recipient should have received BUSD coin now")
        .clone();
    assert_eq!(busd_coin.balance, 3_000_000_000);
    info!(
        "[Timer] Eth to Sui bridge USDT transfer finished in {:?}",
        timer.elapsed()
    );

    let timer = std::time::Instant::now();

    // Now let the recipient send the coin back to ETH
    let eth_address_1 = EthAddress::random();
    let nonce = 0;

    let _sui_to_eth_bridge_action = initiate_bridge_sui_to_eth(
        &bridge_test_cluster,
        eth_address_1,
        busd_coin.object_ref(),
        nonce,
        3_000_000_000,
        TOKEN_ID_USDT,
    )
    .await
    .unwrap();
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                SuiToEthTokenBridgeV2.get().unwrap().clone(),
                TokenTransferApproved.get().unwrap().clone(),
                TokenTransferClaimed.get().unwrap().clone(),
            ]),
            true,
        )
        .await;
    // There are exactly 1 deposit and 1 approved event
    assert_eq!(events.len(), 2);
    info!(
        "[Timer] Sui to Eth bridge transfer approved in {:?}",
        timer.elapsed()
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_eth_to_sui_limit() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(true)
        .with_num_validators(3)
        .build()
        .await;

    let timer = std::time::Instant::now();

    // let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();

    let treasury_summary = bridge_test_cluster
        .bridge_client()
        .get_treasury_summary()
        .await
        .unwrap();
    assert_eq!(treasury_summary.id_token_type_map.len(), 6); // 4 + 1 new token
    let (_id, _type) = treasury_summary
        .id_token_type_map
        .iter()
        .find(|(id, _)| id == &TOKEN_ID_USDT)
        .unwrap();
    let (_type, _metadata) = treasury_summary
        .supported_tokens
        .iter()
        .find(|(_type_, _)| _type == _type_)
        .unwrap();
    let new_token_erc_address = bridge_test_cluster.contracts().usdt;
    initiate_bridge_erc20_to_sui(
        &bridge_test_cluster,
        100,
        new_token_erc_address,
        TOKEN_ID_USDT,
        0,
    )
    .await
    .unwrap();
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                TokenTransferApproved.get().unwrap().clone(),
                TokenTransferClaimed.get().unwrap().clone(),
            ]),
            true,
        )
        .await; // There are exactly 1 approved and 1 claimed event
    assert_eq!(events.len(), 2);
    sleep(Duration::from_secs(10));
    let sui_address = bridge_test_cluster.sui_user_address();
    let all_coins = bridge_test_cluster
        .sui_client()
        .coin_read_api()
        .get_all_coins(sui_address, None, None)
        .await
        .unwrap()
        .data;
    let busd_coin = all_coins
        .iter()
        .find(|c| c.coin_type.contains("BUSD"))
        .expect("Recipient should have received BUSD coin now")
        .clone();
    assert_eq!(busd_coin.balance, 100_000_000_000);
    info!(
        "[Timer] Eth to Sui bridge USDT transfer finished in {:?}",
        timer.elapsed()
    );

    let treasury_summary_after = bridge_test_cluster
        .bridge_client()
        .get_bridge_summary()
        .await
        .unwrap();

    let limit = treasury_summary_after.limiter;
    let limit_record = limit.transfer_records.first().unwrap();
    let limit_record_total_amount = limit_record.2.total_amount();
    assert_eq!(limit_record_total_amount, 10_000_000_000);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_eth_to_sui_limit_with_new_token() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(true)
        .with_num_validators(3)
        .build()
        .await;

    let timer = std::time::Instant::now();

    // let bridge_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();

    let treasury_summary = bridge_test_cluster
        .bridge_client()
        .get_treasury_summary()
        .await
        .unwrap();
    assert_eq!(treasury_summary.id_token_type_map.len(), 6); // 4 + 1 new token
    let (_id, _type) = treasury_summary
        .id_token_type_map
        .iter()
        .find(|(id, _)| id == &TOKEN_ID_USDT)
        .unwrap();
    let (_type, _metadata) = treasury_summary
        .supported_tokens
        .iter()
        .find(|(_type_, _)| _type == _type_)
        .unwrap();
    let new_token_erc_address = bridge_test_cluster.contracts().usdt;
    initiate_bridge_erc20_to_sui(
        &bridge_test_cluster,
        100,
        new_token_erc_address,
        TOKEN_ID_USDT,
        0,
    )
    .await
    .unwrap();
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                TokenTransferApproved.get().unwrap().clone(),
                TokenTransferClaimed.get().unwrap().clone(),
            ]),
            true,
        )
        .await; // There are exactly 1 approved and 1 claimed event
    assert_eq!(events.len(), 2);
    sleep(Duration::from_secs(10));
    let sui_address = bridge_test_cluster.sui_user_address();
    let all_coins = bridge_test_cluster
        .sui_client()
        .coin_read_api()
        .get_all_coins(sui_address, None, None)
        .await
        .unwrap()
        .data;
    let busd_coin = all_coins
        .iter()
        .find(|c| c.coin_type.contains("BUSD"))
        .expect("Recipient should have received BUSD coin now")
        .clone();
    assert_eq!(busd_coin.balance, 100_000_000_000);
    info!(
        "[Timer] Eth to Sui bridge USDT transfer finished in {:?}",
        timer.elapsed()
    );

    assert_eq!(treasury_summary.id_token_type_map.len(), 6); // 4 + 1 new token
    let (_id, _type) = treasury_summary
        .id_token_type_map
        .iter()
        .find(|(id, _)| id == &TOKEN_ID_USDT)
        .unwrap();

    let bridge_object_arg = bridge_test_cluster
        .sui_client()
        .get_mutable_bridge_object_arg()
        .await
        .unwrap();
    let source_chain_id = BridgeChainId::EthCustom as u8;
    let token_type = TOKEN_ID_BUSD;
    let mut token_type_map = HashMap::new();
    for (id, type_) in treasury_summary.id_token_type_map.iter() {
        println!("id: {}, type_: {}", id, type_);
        token_type_map.insert(*id, TypeTag::from_str(&format!("0x{}", type_)).unwrap());
    }
    let limit = bridge_test_cluster
        .bridge_client()
        .sui_client()
        .get_eth_to_sui_limit(
            bridge_object_arg,
            source_chain_id,
            token_type,
            token_type_map,
        )
        .await
        .unwrap();
    assert_eq!(limit, 18_446_744_073_709_551_615 - 100 * 100_000_000);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_create_bridge_state_object() {
    let test_cluster = TestClusterBuilder::new()
        .with_protocol_version((BRIDGE_ENABLE_PROTOCOL_VERSION - 1).into())
        .with_epoch_duration_ms(20000)
        .build()
        .await;

    let handles = test_cluster.all_node_handles();

    // no node has the bridge state object yet
    for h in &handles {
        h.with(|node| {
            assert!(node
                .state()
                .get_object_cache_reader()
                .get_latest_object_ref_or_tombstone(SUI_BRIDGE_OBJECT_ID)
                .is_none());
        });
    }

    // wait until feature is enabled
    test_cluster
        .wait_for_protocol_version(BRIDGE_ENABLE_PROTOCOL_VERSION.into())
        .await;
    // wait until next epoch - authenticator state object is created at the end of the first epoch
    // in which it is supported.
    test_cluster.wait_for_epoch_all_nodes(2).await; // protocol upgrade completes in epoch 1

    for h in &handles {
        h.with(|node| {
            node.state()
                .get_object_cache_reader()
                .get_latest_object_ref_or_tombstone(SUI_BRIDGE_OBJECT_ID)
                .expect("auth state object should exist");
        });
    }
}

#[tokio::test]
async fn test_committee_registration() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_keys = vec![];
    for _ in 0..=3 {
        let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
        bridge_keys.push(kp);
    }
    let test_cluster = TestClusterWrapperBuilder::new()
        .with_bridge_authority_keys(bridge_keys)
        .build()
        .await;

    let bridge = get_bridge(
        test_cluster
            .inner
            .fullnode_handle
            .sui_node
            .state()
            .get_object_store(),
    )
    .unwrap();

    // Member should be empty before end of epoch
    assert!(bridge.committee().members.contents.is_empty());
    assert_eq!(
        test_cluster.inner.swarm.active_validators().count(),
        bridge.committee().member_registrations.contents.len()
    );

    test_cluster
        .trigger_reconfiguration_if_not_yet_and_assert_bridge_committee_initialized()
        .await;
}

#[tokio::test]
async fn test_bridge_api_compatibility() {
    let test_cluster: test_cluster::TestCluster = TestClusterBuilder::new()
        .with_protocol_version(BRIDGE_ENABLE_PROTOCOL_VERSION.into())
        .build()
        .await;

    test_cluster.trigger_reconfiguration().await;
    let client = test_cluster.rpc_client();
    client.get_latest_bridge().await.unwrap();
    // TODO: assert fields in summary

    client
        .get_bridge_object_initial_shared_version()
        .await
        .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_solana_client_with_test_validator() {
    telemetry_subscribers::init_for_testing();
    let bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_solana_env(true)
        .with_solana_chain_id(BridgeChainId::SolanaTestnet)
        .with_bridge_cluster(false)
        .with_num_validators(1)
        .build()
        .await;

    let rpc_url = bridge_test_cluster.solana_env().rpc_url.clone();
    let client = SolanaClient::new(&rpc_url);

    // test get block height
    let n = client.get_block_height(None).await.unwrap();
    assert!(n > 0, "Block height should be greater than 0");
    info!("block height: {:?}", n);

    // test get slot
    let slot = client.get_slot(None).await.unwrap();
    assert!(slot > 0, "Slot should be greater than 0");
    info!("slot: {:?}", slot);

    // Make a transaction to ensure the signer has at least one signature
    let solana_signer = bridge_test_cluster.solana_env().get_signer().await.unwrap();
    let rpc_client = RpcClient::new(rpc_url.clone());
    let from_pubkey = solana_signer.pubkey();
    let to_pubkey = solana_sdk::pubkey::new_rand();

    use solana_sdk::system_instruction;
    use solana_sdk::transaction::Transaction;
    use solana_sdk::signer::Signer;

    let instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, rpc_client.get_minimum_balance_for_rent_exemption(0).unwrap()); // transfer rent-exempt amount
    let recent_blockhash = rpc_client.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&from_pubkey),
        &[&*solana_signer],
        recent_blockhash,
    );

    rpc_client.send_and_confirm_transaction(&tx).unwrap();

    // test get signatures for address
    let cfg = GetSignaturesConfig {
        limit: Some(1),
        ..Default::default()
    };
    let v = client
        .get_signatures_for_address(&from_pubkey.to_string(), Some(cfg))
        .await
        .unwrap();

    assert!(!v.is_empty(), "Expected to find at least one signature");
    info!("signatures: {:?}", v);

    // test get transaction
    let signature = &v[0].signature;
    let tx_info = client.get_transaction(signature).await.unwrap();
    assert!(tx_info.slot.is_some(), "Transaction info should have a slot");
    assert!(tx_info.transaction.is_some(), "Transaction info should have transaction data");
    assert!(tx_info.meta.is_some(), "Transaction info should have meta data");
    info!("transaction info for {}: {:?}", signature, tx_info);

    let tx_info = client.get_transaction("5BM32aN75RmaxpPr8gSTGt4poQdCKEAzowRgZo7ZJL6wn5rW9kWCiL58dWma4e5dm2u4cBvdrbd8wXRYCWR4gPNi").await.unwrap();

    assert!(false);
}
