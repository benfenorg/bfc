// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use futures::future::join_all;
use rand::rngs::OsRng;
use std::collections::{BTreeSet, HashSet};
use std::option;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use anyhow::Error;
use jsonrpsee::http_client::HttpClient;
use move_core_types::account_address::AccountAddress;
use sui_core::consensus_adapter::position_submit_certificate;
use sui_json_rpc_types::{SuiObjectData, SuiObjectDataFilter, SuiObjectDataOptions, SuiObjectResponseQuery, SuiTransactionBlockEffects, SuiTransactionBlockEffectsAPI, SuiTransactionBlockResponseOptions, TransactionBlockBytes};
use sui_macros::sim_test;
use sui_node::SuiNodeHandle;
use sui_protocol_config::{ProtocolConfig, ProtocolVersion, SupportedProtocolVersions};
use sui_swarm_config::genesis_config::{ValidatorGenesisConfig, ValidatorGenesisConfigBuilder};
use sui_test_transaction_builder::{make_transfer_sui_transaction, TestTransactionBuilder};
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::effects::TransactionEffectsAPI;
use sui_types::error::SuiError;
use sui_types::gas::GasCostSummary;
use sui_types::governance::MIN_VALIDATOR_JOINING_STAKE_MIST;
use sui_types::message_envelope::Message;
use sui_types::sui_system_state::{
    get_validator_from_table, sui_system_state_summary::get_validator_by_pool_id,
    SuiSystemStateTrait,
};
use sui_types::transaction::{CallArg, TransactionDataAPI, TransactionExpiration};
use test_cluster::{TestCluster, TestClusterBuilder};
use tokio::time::sleep;
use tracing::info;
use sui_json_rpc::api::{IndexerApiClient, ReadApiClient, TransactionBuilderClient, WriteApiClient};
use sui_sdk::json::{call_args, SuiJsonValue, type_args};
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_types::{OBC_SYSTEM_PACKAGE_ID, parse_sui_struct_tag};
use serde_json::json;
use sui_types::dao::DaoRPC;
use sui_types::dao_manager::MANAGE_MODULE_NAME;
use sui_types::OBC_SYSTEM_STATE_ADDRESS;


#[sim_test]
async fn advance_epoch_tx_test() {
    let test_cluster = TestClusterBuilder::new().build().await;
    let states = test_cluster
        .swarm
        .validator_node_handles()
        .into_iter()
        .map(|handle| handle.with(|node| node.state()))
        .collect::<Vec<_>>();
    let tasks: Vec<_> = states
        .iter()
        .map(|state| async {
            let (_system_state, effects) = state
                .create_and_execute_advance_epoch_tx(
                    &state.epoch_store_for_testing(),
                    &GasCostSummary::new(0, 0, 0, 0),
                    0, // checkpoint
                    0, // epoch_start_timestamp_ms
                )
                .await
                .unwrap();
            // Check that the validator didn't commit the transaction yet.
            assert!(state
                .get_signed_effects_and_maybe_resign(
                    effects.transaction_digest(),
                    &state.epoch_store_for_testing()
                )
                .unwrap()
                .is_none());
            effects
        })
        .collect();
    let results: HashSet<_> = join_all(tasks)
        .await
        .into_iter()
        .map(|result| result.digest())
        .collect();
    // Check that all validators have the same result.
    assert_eq!(results.len(), 1);
}

#[sim_test]
async fn basic_reconfig_end_to_end_test() {
    // TODO remove this sleep when this test passes consistently
    sleep(Duration::from_secs(1)).await;
    let test_cluster = TestClusterBuilder::new().build().await;
    test_cluster.trigger_reconfiguration().await;
}

#[sim_test]
async fn test_transaction_expiration() {
    let test_cluster = TestClusterBuilder::new().build().await;
    test_cluster.trigger_reconfiguration().await;

    let (sender, gas) = test_cluster
        .wallet
        .get_one_gas_object()
        .await
        .unwrap()
        .unwrap();
    let rgp = test_cluster.get_reference_gas_price().await;
    let mut data = TestTransactionBuilder::new(sender, gas, rgp)
        .transfer_sui(Some(1), sender)
        .build();
    // Expired transaction returns an error
    let mut expired_data = data.clone();
    *expired_data.expiration_mut_for_testing() = TransactionExpiration::Epoch(0);
    let expired_transaction = test_cluster.wallet.sign_transaction(&expired_data);
    let authority = test_cluster.swarm.validator_node_handles().pop().unwrap();
    let result = authority
        .with_async(|node| async {
            let epoch_store = node.state().epoch_store_for_testing();
            let state = node.state();
            let expired_transaction = state.verify_transaction(expired_transaction).unwrap();
            state
                .handle_transaction(&epoch_store, expired_transaction)
                .await
        })
        .await;
    assert!(matches!(result.unwrap_err(), SuiError::TransactionExpired));

    // Non expired transaction signed without issue
    *data.expiration_mut_for_testing() = TransactionExpiration::Epoch(10);
    let transaction = test_cluster.wallet.sign_transaction(&data);
    authority
        .with_async(|node| async {
            let epoch_store = node.state().epoch_store_for_testing();
            let state = node.state();
            let transaction = state.verify_transaction(transaction).unwrap();
            state.handle_transaction(&epoch_store, transaction).await
        })
        .await
        .unwrap();
}

// TODO: This test does not guarantee that tx would be reverted, and hence the code path
// may not always be tested.
#[sim_test]
async fn reconfig_with_revert_end_to_end_test() {
    let test_cluster = TestClusterBuilder::new().build().await;
    let authorities = test_cluster.swarm.validator_node_handles();
    let rgp = test_cluster.get_reference_gas_price().await;
    let (sender, mut gas_objects) = test_cluster.wallet.get_one_account().await.unwrap();

    // gas1 transaction is committed
    let gas1 = gas_objects.pop().unwrap();
    let tx = test_cluster.wallet.sign_transaction(
        &TestTransactionBuilder::new(sender, gas1, rgp)
            .transfer_sui(None, sender)
            .build(),
    );
    let effects1 = test_cluster.execute_transaction(tx).await;
    assert_eq!(0, effects1.effects.unwrap().executed_epoch());

    // gas2 transaction is (most likely) reverted
    let gas2 = gas_objects.pop().unwrap();
    let tx = test_cluster.wallet.sign_transaction(
        &TestTransactionBuilder::new(sender, gas2, rgp)
            .transfer_sui(None, sender)
            .build(),
    );
    let net = test_cluster
        .fullnode_handle
        .sui_node
        .with(|node| node.clone_authority_aggregator().unwrap());
    let cert = net
        .process_transaction(tx.clone())
        .await
        .unwrap()
        .into_cert_for_testing();

    // Close epoch on 3 (2f+1) validators.
    let mut reverting_authority_idx = None;
    for (i, handle) in authorities.iter().enumerate() {
        handle
            .with_async(|node| async {
                if position_submit_certificate(&net.committee, &node.state().name, tx.digest())
                    < (authorities.len() - 1)
                {
                    node.close_epoch_for_testing().await.unwrap();
                } else {
                    // remember the authority that wouild submit it to consensus last.
                    reverting_authority_idx = Some(i);
                }
            })
            .await;
    }

    let reverting_authority_idx = reverting_authority_idx.unwrap();
    let client = net
        .get_client(&authorities[reverting_authority_idx].with(|node| node.state().name))
        .unwrap();
    client.handle_certificate(cert.clone()).await.unwrap();

    authorities[reverting_authority_idx]
        .with_async(|node| async {
            let object = node
                .state()
                .get_objects(&[gas2.0])
                .await
                .unwrap()
                .into_iter()
                .next()
                .unwrap()
                .unwrap();
            // verify that authority 0 advanced object version
            assert_eq!(2, object.version().value());
        })
        .await;

    // Wait for all nodes to reach the next epoch.
    let handles: Vec<_> = authorities
        .iter()
        .map(|handle| {
            handle.with_async(|node| async {
                loop {
                    if node.state().current_epoch_for_testing() == 1 {
                        break;
                    }
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            })
        })
        .collect();
    join_all(handles).await;

    let mut epoch = None;
    for handle in authorities.iter() {
        handle
            .with_async(|node| async {
                let object = node
                    .state()
                    .get_objects(&[gas1.0])
                    .await
                    .unwrap()
                    .into_iter()
                    .next()
                    .unwrap()
                    .unwrap();
                assert_eq!(2, object.version().value());
                // Due to race conditions, it's possible that tx2 went in
                // before 2f+1 validators sent EndOfPublish messages and close
                // the curtain of epoch 0. So, we are asserting that
                // the object version is either 1 or 2, but needs to be
                // consistent in all validators.
                // Note that previously test checked that object version == 2 on authority 0
                let object = node
                    .state()
                    .get_objects(&[gas2.0])
                    .await
                    .unwrap()
                    .into_iter()
                    .next()
                    .unwrap()
                    .unwrap();
                let object_version = object.version().value();
                if epoch.is_none() {
                    assert!(object_version == 1 || object_version == 2);
                    epoch.replace(object_version);
                } else {
                    assert_eq!(epoch, Some(object_version));
                }
            })
            .await;
    }
}

// This test just starts up a cluster that reconfigures itself under 0 load.
#[sim_test]
async fn test_passive_reconfig() {
    telemetry_subscribers::init_for_testing();
    let _commit_root_state_digest = ProtocolConfig::apply_overrides_for_testing(|_, mut config| {
        config.set_commit_root_state_digest_supported(true);
        config
    });
    ProtocolConfig::poison_get_for_min_version();

    let test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(1000)
        .build()
        .await;

    let target_epoch: u64 = std::env::var("RECONFIG_TARGET_EPOCH")
        .ok()
        .map(|v| v.parse().unwrap())
        .unwrap_or(4);

    test_cluster.wait_for_epoch(Some(target_epoch)).await;

    test_cluster
        .swarm
        .validator_nodes()
        .next()
        .unwrap()
        .get_node_handle()
        .unwrap()
        .with(|node| {
            let commitments = node
                .state()
                .get_epoch_state_commitments(0)
                .unwrap()
                .unwrap();
            assert_eq!(commitments.len(), 1);
        });
}

#[sim_test]
async fn test_change_obc_round() {
    telemetry_subscribers::init_for_testing();
    let _commit_root_state_digest = ProtocolConfig::apply_overrides_for_testing(|_, mut config| {
        config.set_commit_root_state_digest_supported(true);
        config
    });
    ProtocolConfig::poison_get_for_min_version();

    let test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(1000)
        .with_num_validators(5)
        .build()
        .await;

    let target_epoch: u64 = std::env::var("RECONFIG_TARGET_EPOCH")
        .ok()
        .map(|v| v.parse().unwrap())
        .unwrap_or(1);

    test_cluster
        .swarm
        .validator_nodes()
        .next()
        .unwrap()
        .get_node_handle()
        .unwrap()
        .with(|node| {
            let state = node
                .state()
                .get_obc_system_state_object_for_testing().unwrap();
            assert_eq!(state.inner_state().round, 0);
        });

    test_cluster.wait_for_epoch(Some(target_epoch)).await;

    test_cluster
        .swarm
        .validator_nodes()
        .next()
        .unwrap()
        .get_node_handle()
        .unwrap()
        .with(|node| {
            let state = node
                .state()
                .get_obc_system_state_object_for_testing().unwrap();
            //assert_eq!(state.inner_state().round, 1);
        });

}


#[sim_test]
async fn test_obc_dao_update_system_package_blocked(){

    telemetry_subscribers::init_for_testing();
    // let _commit_root_state_digest = ProtocolConfig::apply_overrides_for_testing(|_, mut config| {
    //     config.set_commit_root_state_digest_supported(true);
    //     config
    // });
    ProtocolConfig::poison_get_for_min_version();

    let START_VERSION= 18u64;
    let UPDATE_TARGET_VERSION = 19u64;
    let test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(1000)
        .with_protocol_version(ProtocolVersion::new(START_VERSION))
        .build()
        .await;



    let  node = test_cluster
        .swarm
        .validator_nodes()
        .next()
        .unwrap()
        .get_node_handle()
        .unwrap();
    let epoch_store = node.state().load_epoch_store_one_call_per_task();


    let mut epochid =  node.state().current_epoch_for_testing();
    let mut protocol_version = epoch_store.protocol_version();
    info!("=============epochid: {}", epochid);
    info!("=============protocol_version:{:?} ", protocol_version);


    let target_epoch: u64 = std::env::var("RECONFIG_TARGET_EPOCH")
        .ok()
        .map(|v| v.parse().unwrap())
        .unwrap_or(1);
    info!("=============target_epoch: {}", target_epoch);

    test_cluster.wait_for_epoch_all_nodes(target_epoch).await;



    epochid =  node.state().current_epoch_for_testing();
    protocol_version = epoch_store.protocol_version();
    info!("=============epochid: {}", epochid);
    info!("=============protocol_version:{:?} ", protocol_version);



    //waiting for....

    //test_cluster.wait_for_all_nodes_upgrade_to(19u64).await;


    sleep(Duration::from_secs(10)).await;


    epochid =  node.state().current_epoch_for_testing();
    protocol_version = epoch_store.protocol_version();

    info!("=============epochid: {}", epochid);
    info!("=============protocol_version:{:?} ", protocol_version);

    assert_eq!(protocol_version, ProtocolVersion::new(START_VERSION));

}


#[sim_test]
async fn test_obc_dao_update_system_package_pass(){

    telemetry_subscribers::init_for_testing();
    let _commit_root_state_digest = ProtocolConfig::apply_overrides_for_testing(|_, mut config| {
        config.set_commit_root_state_digest_supported(true);
        config
    });
    ProtocolConfig::poison_get_for_min_version();

    let START_VERSION= 18u64;
    let UPDATE_TARGET_VERSION = 19u64;
    let test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(1000)
        .with_protocol_version(ProtocolVersion::new(START_VERSION))
        .build()
        .await;



    let  node = test_cluster
        .swarm
        .validator_nodes()
        .next()
        .unwrap()
        .get_node_handle()
        .unwrap();
    let epoch_store = node.state().load_epoch_store_one_call_per_task();


    let mut epochid =  node.state().current_epoch_for_testing();
    let mut protocol_version = epoch_store.protocol_version();
    info!("=============epochid: {}", epochid);
    info!("=============protocol_version:{:?} ", protocol_version);


    let target_epoch: u64 = std::env::var("RECONFIG_TARGET_EPOCH")
        .ok()
        .map(|v| v.parse().unwrap())
        .unwrap_or(1);
    info!("=============target_epoch: {}", target_epoch);

    test_cluster.wait_for_epoch_all_nodes(target_epoch).await;



    epochid =  node.state().current_epoch_for_testing();
    protocol_version = epoch_store.protocol_version();
    info!("=============epochid: {}", epochid);
    info!("=============protocol_version:{:?} ", protocol_version);



    //waiting for....

    //test_cluster.wait_for_all_nodes_upgrade_to(19u64).await;

    sleep(Duration::from_secs(10)).await;


    epochid =  node.state().current_epoch_for_testing();
    protocol_version = epoch_store.protocol_version();

    info!("=============epochid: {}", epochid);
    info!("=============protocol_version:{:?} ", protocol_version);

    assert_eq!(protocol_version, ProtocolVersion::new(START_VERSION));

}



async fn add_cluster_admin(http_client: &HttpClient, gas: &SuiObjectData, address: SuiAddress, cluster: &TestCluster) -> Result<ObjectID, anyhow::Error> {
    // now do the call
    let module = "obc_system".to_string();
    let function = "cluster_add_admin".to_string();
    let package_id = OBC_SYSTEM_PACKAGE_ID;

    let arg = vec![ SuiJsonValue::from_str(&address.to_string())?];

    let transaction_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            package_id,
            module,
            function,
            type_args![]?,
            arg,
            Some(gas.object_id),
            10_000_00000.into(),
            None,
        )
        .await?;

    let tx = cluster
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
    let current_effects = tx_response.effects.unwrap() as SuiTransactionBlockEffects;



    info!("============finish add admin");

    //            SuiObjectDataFilter::StructType(parse_sui_struct_tag("0x2::test::Test").unwrap()),
    let filter =  SuiObjectDataFilter::StructType(parse_sui_struct_tag("0xc8::obc_dao_manager::OBCDaoManageKey").unwrap());
    let dataOption = SuiObjectDataOptions::new()
        .with_type()
        .with_owner()
        .with_previous_transaction();

    let objects = http_client
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new(
                Option::Some(filter),
                Option::Some(dataOption),
            )),
            None,
            None,
        )
        .await?
        .data;


    let managerObj = objects.get(0).unwrap().object().unwrap();
    info!("============finish get obcdao manager {}", managerObj.object_id);

    Ok(managerObj.object_id)
}
async fn do_move_call(http_client: &HttpClient, gas: &SuiObjectData, address: SuiAddress, cluster: &TestCluster, package_id: ObjectID, module: String, function: String, arg: Vec<SuiJsonValue>) -> Result<(), anyhow::Error> {

    let transaction_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            package_id,
            module,
            function,
            type_args![]?,
            arg,
            Some(gas.object_id),
            10_000_00000.into(),
            None,
        )
        .await?;

    let tx = cluster
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
    Ok(())
}
#[sim_test]
async fn test_obc_dao_create_create_action() -> Result<(), anyhow::Error>{
    telemetry_subscribers::init_for_testing();

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

    let gas = objects.first().unwrap().object().unwrap();

    let managerObj = add_cluster_admin(http_client, gas, address, &cluster).await?;


    // now do the call
    let package_id = OBC_SYSTEM_PACKAGE_ID;
    let module = "obc_system".to_string();
    let function = "create_obcdao_action".to_string();
    let obc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let arg = vec![
        SuiJsonValue::from_str(&obc_status_address.to_string())?,
        SuiJsonValue::from_str(&managerObj.to_string())?,
        SuiJsonValue::new(json!("hello world"))?,
    ];


   do_move_call(&http_client, &gas, address, &cluster, package_id, module, function, arg).await?;


    let result = http_client.get_inner_dao_info().await?;

    let dao = result as DaoRPC;

    info!("============finish get dao actions {:?}", dao.action_record);
    assert!(dao.action_record.len() > 0);

    Ok(())
}


#[sim_test]
async fn test_obc_dao_create_create_propose() -> Result<(), anyhow::Error> {



    Ok(())
}



#[sim_test]
async fn test_obc_dao_create_votingobc()  -> Result<(), anyhow::Error> {
    telemetry_subscribers::init_for_testing();

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

    info!("============finish get owned objects {}", objects.len());

    let gas = objects.first().unwrap().object().unwrap();

    let amount  = 1_000_000_000u64* 100;
    let tx = make_transfer_sui_transaction(&cluster.wallet,
                                           Option::Some(address),
                                           Option::Some(amount)).await;
    let effects0 = cluster
        .execute_transaction(tx.clone())
        .await
        .effects
        .unwrap();

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

    info!("============finish get owned objects {}", objects.len());
    let coinObj = objects.get(2).unwrap().object().unwrap();


    // now do the call
    let package_id = OBC_SYSTEM_PACKAGE_ID;
    let module = "obc_system".to_string();
    let function = "create_voting_obc".to_string();
    let obc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let arg = vec![
        SuiJsonValue::from_str(&obc_status_address.to_string())?,
        SuiJsonValue::from_str(&coinObj.object_id.to_string())?,
    ];


    let transaction_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            package_id,
            module,
            function,
            type_args![]?,
            arg,
            Some(gas.object_id),
            10_000_00000.into(),
            None,
        )
        .await?;

    let tx = cluster
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




    let filter =  SuiObjectDataFilter::StructType(parse_sui_struct_tag("0xc8::voting_pool::VotingObc").unwrap());
    let dataOption = SuiObjectDataOptions::new()
        .with_type()
        .with_owner()
        .with_previous_transaction();
    let objects = http_client
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new(
                Option::Some(filter),
                Option::Some(dataOption),
            )),
            None,
            None,
        )
        .await?
        .data;


    //should be size = 1. pass.
    info!("============finish get owned objects {}", objects.len());

    Ok(())
}

#[sim_test]
async fn test_obc_dao_cast_voting() -> Result<(), anyhow::Error>{


    Ok(())
}


#[sim_test]
async fn test_obc_dao_withdraw_obc() -> Result<(), anyhow::Error>{
    telemetry_subscribers::init_for_testing();


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

    info!("============finish get owned objects {}", objects.len());
    assert!(objects.len() > 0);

    let gas = objects.first().unwrap().object().unwrap();

    let amount  = 1_000_000_000u64* 100;
    let tx = make_transfer_sui_transaction(&cluster.wallet,
                                           Option::Some(address),
                                           Option::Some(amount)).await;
    let effects0 = cluster
        .execute_transaction(tx.clone())
        .await
        .effects
        .unwrap();

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

    info!("============finish get owned objects {}", objects.len());
    let coinObj = objects.get(2).unwrap().object().unwrap();


    // now do the call
    let package_id = OBC_SYSTEM_PACKAGE_ID;
    let module = "obc_system".to_string();
    let function = "create_voting_obc".to_string();
    let obc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let arg = vec![
        SuiJsonValue::from_str(&obc_status_address.to_string())?,
        SuiJsonValue::from_str(&coinObj.object_id.to_string())?,
    ];

    do_move_call(http_client, gas, address, &cluster, package_id, module, function, arg).await?;




    let filter =  SuiObjectDataFilter::StructType(parse_sui_struct_tag("0xc8::voting_pool::VotingObc").unwrap());
    let dataOption = SuiObjectDataOptions::new()
        .with_type()
        .with_owner()
        .with_previous_transaction();
    let objects = http_client
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new(
                Option::Some(filter),
                Option::Some(dataOption),
            )),
            None,
            None,
        )
        .await?
        .data;


    //should be size = 1. pass.
    info!("============finish get owned objects {}", objects.len());
    assert_eq!(objects.len(), 1);


    let votingObc = objects.get(0).unwrap().object().unwrap();

    //with draw the voting obc,,,
    // now do the call
    //public entry fun withdraw_voting(   wrapper: &mut ObcSystemState voting_obc: VotingObc)
    let package_id = OBC_SYSTEM_PACKAGE_ID;
    let module = "obc_system".to_string();
    let function = "withdraw_voting".to_string();
    let obc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let arg = vec![
        SuiJsonValue::from_str(&obc_status_address.to_string())?,
        SuiJsonValue::from_str(&votingObc.object_id.to_string())?,
    ];

   do_move_call(http_client, gas, address, &cluster, package_id, module, function, arg).await?;



    let filter =  SuiObjectDataFilter::StructType(parse_sui_struct_tag("0xc8::voting_pool::VotingObc").unwrap());
    let dataOption = SuiObjectDataOptions::new()
        .with_type()
        .with_owner()
        .with_previous_transaction();
    let objects = http_client
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new(
                Option::Some(filter),
                Option::Some(dataOption),
            )),
            None,
            None,
        )
        .await?
        .data;


    //should be size = 0. pass.
    info!("============finish get owned objects {}", objects.len());
    assert_eq!(objects.len(), 0);

    Ok(())
}


#[sim_test]
async fn test_obc_dao_change_setting_config() -> Result<(), anyhow::Error> {

    //telemetry_subscribers::init_for_testing();

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

    let gas = objects.first().unwrap().object().unwrap();


    let managerObj = add_cluster_admin(http_client, gas, address, &cluster).await?;


    // now do the call  public entry fun set_voting_period(
    //         wrapper: &mut ObcSystemState,
    //         manager_key: &OBCDaoManageKey,
    //         value: u64,
    //     )
    let package_id = OBC_SYSTEM_PACKAGE_ID;
    let module = "obc_system".to_string();
    let function = "set_voting_period".to_string();
    let obc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let arg = vec![
        SuiJsonValue::from_str(&obc_status_address.to_string())?,
        SuiJsonValue::from_str(&managerObj.to_string())?,
        SuiJsonValue::new(json!("888888"))?,
    ];

    do_move_call(http_client, gas, address, &cluster, package_id, module, function, arg).await?;
    info!("============finish set_voting_period");


    let module = "obc_system".to_string();
    let function = "set_min_action_delay".to_string();
    let obc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let arg = vec![
        SuiJsonValue::from_str(&obc_status_address.to_string())?,
        SuiJsonValue::from_str(&managerObj.to_string())?,
        SuiJsonValue::new(json!("888888"))?,
    ];

    do_move_call(http_client, gas, address, &cluster, package_id, module, function, arg).await?;
    info!("============finish set_min_action_delay");



    let module = "obc_system".to_string();
    let function = "set_voting_delay".to_string();
    let obc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let arg = vec![
        SuiJsonValue::from_str(&obc_status_address.to_string())?,
        SuiJsonValue::from_str(&managerObj.to_string())?,
        SuiJsonValue::new(json!("888888"))?,
    ];

    do_move_call(http_client, gas, address, &cluster, package_id, module, function, arg).await?;
    info!("============finish set_voting_delay");


    let module = "obc_system".to_string();
    let function = "set_voting_quorum_rate".to_string();
    let obc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let arg = vec![
        SuiJsonValue::from_str(&obc_status_address.to_string())?,
        SuiJsonValue::from_str(&managerObj.to_string())?,
        SuiJsonValue::new(json!("88"))?,
    ];

    do_move_call(http_client, gas, address, &cluster, package_id, module, function, arg).await?;
    info!("============finish set_voting_quorum_rate");




    let result = http_client.get_inner_dao_info().await?;

    let dao = result as DaoRPC;

    info!("============finish get dao info {:?}", dao.config);
    assert_eq!(dao.config.voting_period, 888888);
    assert_eq!(dao.config.min_action_delay, 888888);
    assert_eq!(dao.config.voting_delay, 888888);
    assert_eq!(dao.config.voting_quorum_rate, 88);


    Ok(())
}

#[sim_test]
async fn test_obc_dao_unvote_votingobc(){

}

#[sim_test]
async fn test_obc_dao_change_vote(){

}

#[sim_test]
async fn test_obc_dao_revoke_vote(){

}

// This test just starts up a cluster that reconfigures itself under 0 load.
#[cfg(msim)]
#[sim_test]
async fn test_create_advance_epoch_tx_race() {
    use std::sync::Arc;
    use sui_macros::{register_fail_point, register_fail_point_async};
    use tokio::sync::broadcast;
    use tracing::info;

    telemetry_subscribers::init_for_testing();
    sui_protocol_config::ProtocolConfig::poison_get_for_min_version();

    // panic if we enter safe mode. If you remove the check for `is_tx_already_executed` in
    // AuthorityState::create_and_execute_advance_epoch_tx, this test should fail.
    register_fail_point("record_checkpoint_builder_is_safe_mode_metric", || {
        panic!("safe mode recorded");
    });

    // Intercept the specified async wait point on a given node, and wait there until a message
    // is sent from the given tx.
    let register_wait = |failpoint, node_id, tx: Arc<broadcast::Sender<()>>| {
        let node = sui_simulator::task::NodeId(node_id);
        register_fail_point_async(failpoint, move || {
            let cur_node = sui_simulator::current_simnode_id();
            let tx = tx.clone();
            async move {
                if cur_node == node {
                    let mut rx = tx.subscribe();

                    info!(
                        "waiting for test to send continuation signal for {}",
                        failpoint
                    );
                    rx.recv().await.unwrap();
                    info!("continuing {}", failpoint);
                }
            }
        });
    };

    // Set up wait points.
    let (change_epoch_delay_tx, _change_epoch_delay_rx) = broadcast::channel(1);
    let change_epoch_delay_tx = Arc::new(change_epoch_delay_tx);
    let (reconfig_delay_tx, _reconfig_delay_rx) = broadcast::channel(1);
    let reconfig_delay_tx = Arc::new(reconfig_delay_tx);

    // Test code runs in node 1 - node 2 is always a validator.
    let target_node = 2;
    register_wait(
        "change_epoch_tx_delay",
        target_node,
        change_epoch_delay_tx.clone(),
    );
    register_wait("reconfig_delay", target_node, reconfig_delay_tx.clone());

    let test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(1000)
        .build()
        .await;

    test_cluster.wait_for_epoch(None).await;

    // Allow time for paused node to execute change epoch tx via state sync.
    sleep(Duration::from_secs(5)).await;

    // now release the pause, node will find that change epoch tx has already been executed.
    info!("releasing change epoch delay tx");
    change_epoch_delay_tx.send(()).unwrap();

    // proceeded with reconfiguration.
    sleep(Duration::from_secs(1)).await;
    reconfig_delay_tx.send(()).unwrap();
}

#[sim_test]
async fn test_reconfig_with_failing_validator() {
    sui_protocol_config::ProtocolConfig::poison_get_for_min_version();

    let test_cluster = Arc::new(
        TestClusterBuilder::new()
            .with_epoch_duration_ms(5000)
            .build()
            .await,
    );

    test_cluster
        .random_node_restarter()
        .with_kill_interval_secs(2, 4)
        .with_restart_delay_secs(2, 4)
        .run();

    let target_epoch: u64 = std::env::var("RECONFIG_TARGET_EPOCH")
        .ok()
        .map(|v| v.parse().unwrap())
        .unwrap_or(4);

    // A longer timeout is required, as restarts can cause reconfiguration to take longer.
    test_cluster
        .wait_for_epoch_with_timeout(Some(target_epoch), Duration::from_secs(90))
        .await;
}

#[sim_test]
async fn test_validator_resign_effects() {
    // This test checks that validators are able to re-sign transaction effects that were finalized
    // in previous epochs. This allows authority aggregator to form a new effects certificate
    // in the new epoch.
    let test_cluster = TestClusterBuilder::new().build().await;
    let tx = make_transfer_sui_transaction(&test_cluster.wallet, None, None).await;
    let effects0 = test_cluster
        .execute_transaction(tx.clone())
        .await
        .effects
        .unwrap();
    assert_eq!(effects0.executed_epoch(), 0);
    test_cluster.trigger_reconfiguration().await;

    let net = test_cluster
        .fullnode_handle
        .sui_node
        .with(|node| node.clone_authority_aggregator().unwrap());
    let effects1 = net
        .process_transaction(tx)
        .await
        .unwrap()
        .into_effects_for_testing();
    // Ensure that we are able to form a new effects cert in the new epoch.
    assert_eq!(effects1.epoch(), 1);
    assert_eq!(effects1.executed_epoch(), 0);
}

#[sim_test]
async fn test_validator_candidate_pool_read() {
    let new_validator = ValidatorGenesisConfigBuilder::new().build(&mut OsRng);
    let address: SuiAddress = (&new_validator.account_key_pair.public()).into();
    let test_cluster = TestClusterBuilder::new()
        .with_validator_candidates([address])
        .build()
        .await;
    add_validator_candidate(&test_cluster, &new_validator).await;
    test_cluster.fullnode_handle.sui_node.with(|node| {
        let system_state = node
            .state()
            .get_sui_system_state_object_for_testing()
            .unwrap();
        let system_state_summary = system_state.clone().into_sui_system_state_summary();
        let staking_pool_id = get_validator_from_table(
            node.state().db().as_ref(),
            system_state_summary.validator_candidates_id,
            &address,
        )
        .unwrap()
        .staking_pool_id;
        let validator = get_validator_by_pool_id(
            node.state().db().as_ref(),
            &system_state,
            &system_state_summary,
            staking_pool_id,
        )
        .unwrap();
        assert_eq!(validator.sui_address, address);
    });
}

#[sim_test]
async fn test_inactive_validator_pool_read() {
    let test_cluster = TestClusterBuilder::new()
        .with_num_validators(5)
        .build()
        .await;
    // Pick the first validator.
    let validator = test_cluster.swarm.validator_node_handles().pop().unwrap();
    let address = validator.with(|node| node.get_config().sui_address());
    let staking_pool_id = test_cluster.fullnode_handle.sui_node.with(|node| {
        node.state()
            .get_sui_system_state_object_for_testing()
            .unwrap()
            .into_sui_system_state_summary()
            .active_validators
            .iter()
            .find(|v| v.sui_address == address)
            .unwrap()
            .staking_pool_id
    });
    test_cluster.fullnode_handle.sui_node.with(|node| {
        let system_state = node
            .state()
            .get_sui_system_state_object_for_testing()
            .unwrap();
        let system_state_summary = system_state.clone().into_sui_system_state_summary();
        // Validator is active. Check that we can find its summary by staking pool id.
        let validator = get_validator_by_pool_id(
            node.state().db().as_ref(),
            &system_state,
            &system_state_summary,
            staking_pool_id,
        )
        .unwrap();
        assert_eq!(validator.sui_address, address);
    });
    execute_remove_validator_tx(&test_cluster, &validator).await;

    test_cluster.trigger_reconfiguration().await;

    // Check that this node is no longer a validator.
    validator.with(|node| {
        assert!(node
            .state()
            .is_fullnode(&node.state().epoch_store_for_testing()));
    });

    // Check that the validator that just left now shows up in the inactive_validators,
    // and we can still deserialize it and get the inactive staking pool.
    test_cluster.fullnode_handle.sui_node.with(|node| {
        let system_state = node
            .state()
            .get_sui_system_state_object_for_testing()
            .unwrap();
        assert_eq!(
            system_state
                .get_current_epoch_committee()
                .committee
                .num_members(),
            4
        );
        let system_state_summary = system_state.clone().into_sui_system_state_summary();
        let validator = get_validator_by_pool_id(
            node.state().db().as_ref(),
            &system_state,
            &system_state_summary,
            staking_pool_id,
        )
        .unwrap();
        assert_eq!(validator.sui_address, address);
        assert!(validator.staking_pool_deactivation_epoch.is_some());
    })
}

#[sim_test]
async fn test_reconfig_with_committee_change_basic() {
    // This test exercise the full flow of a validator joining the network, catch up and then leave.

    let new_validator = ValidatorGenesisConfigBuilder::new().build(&mut OsRng);
    let address = (&new_validator.account_key_pair.public()).into();
    let mut test_cluster = TestClusterBuilder::new()
        .with_validator_candidates([address])
        .build()
        .await;

    execute_add_validator_transactions(&test_cluster, &new_validator).await;

    test_cluster.trigger_reconfiguration().await;

    // Check that a new validator has joined the committee.
    test_cluster.fullnode_handle.sui_node.with(|node| {
        assert_eq!(
            node.state()
                .epoch_store_for_testing()
                .committee()
                .num_members(),
            5
        );
    });
    let new_validator_handle = test_cluster.spawn_new_validator(new_validator).await;
    test_cluster.wait_for_epoch_all_nodes(1).await;

    new_validator_handle.with(|node| {
        assert!(node
            .state()
            .is_validator(&node.state().epoch_store_for_testing()));
    });

    execute_remove_validator_tx(&test_cluster, &new_validator_handle).await;
    test_cluster.trigger_reconfiguration().await;
    test_cluster.fullnode_handle.sui_node.with(|node| {
        assert_eq!(
            node.state()
                .epoch_store_for_testing()
                .committee()
                .num_members(),
            4
        );
    });
}

#[sim_test]
async fn test_reconfig_with_committee_change_stress() {
    let mut candidates = (0..6)
        .map(|_| ValidatorGenesisConfigBuilder::new().build(&mut OsRng))
        .collect::<Vec<_>>();
    let addresses = candidates
        .iter()
        .map(|c| (&c.account_key_pair.public()).into())
        .collect::<Vec<SuiAddress>>();
    let mut test_cluster = TestClusterBuilder::new()
        .with_num_validators(7)
        .with_validator_candidates(addresses)
        .with_num_unpruned_validators(2)
        .build()
        .await;

    while !candidates.is_empty() {
        let v1 = candidates.pop().unwrap();
        let v2 = candidates.pop().unwrap();
        execute_add_validator_transactions(&test_cluster, &v1).await;
        execute_add_validator_transactions(&test_cluster, &v2).await;
        let mut removed_validators = vec![];
        for v in test_cluster
            .swarm
            .active_validators()
            // Skip removal of any non-pruning validators from the committee.
            // Until we have archival solution, we need to have some validators that do not prune,
            // otherwise new validators to the committee will not be able to catch up to the network
            // TODO: remove and replace with usage of archival solution
            .filter(|node| {
                node.config
                    .authority_store_pruning_config
                    .num_epochs_to_retain_for_checkpoints()
                    .is_some()
            })
            .take(2)
        {
            let h = v.get_node_handle().unwrap();
            removed_validators.push(h.state().name);
            execute_remove_validator_tx(&test_cluster, &h).await;
        }
        let handle1 = test_cluster.spawn_new_validator(v1).await;
        let handle2 = test_cluster.spawn_new_validator(v2).await;
        test_cluster.trigger_reconfiguration().await;
        let committee = test_cluster
            .fullnode_handle
            .sui_node
            .with(|node| node.state().epoch_store_for_testing().committee().clone());
        assert_eq!(committee.num_members(), 7);
        assert!(committee.authority_exists(&handle1.state().name));
        assert!(committee.authority_exists(&handle2.state().name));
        removed_validators
            .iter()
            .all(|v| !committee.authority_exists(v));
    }
}

#[cfg(msim)]
#[sim_test]
async fn safe_mode_reconfig_test() {
    use sui_test_transaction_builder::make_staking_transaction;
    use sui_types::sui_system_state::advance_epoch_result_injection;

    const EPOCH_DURATION: u64 = 10000;

    // Inject failure at epoch change 1 -> 2.
    advance_epoch_result_injection::set_override(Some((2, 3)));

    let test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(EPOCH_DURATION)
        .build()
        .await;

    let system_state = test_cluster
        .sui_client()
        .governance_api()
        .get_latest_sui_system_state()
        .await
        .unwrap();

    // On startup, we should be at V1.
    assert_eq!(system_state.system_state_version, 1);
    assert_eq!(system_state.epoch, 0);

    // Wait for regular epoch change to happen once. Migration from V1 to V2 should happen here.
    let system_state = test_cluster.wait_for_epoch(Some(1)).await;
    assert!(!system_state.safe_mode());
    assert_eq!(system_state.epoch(), 1);
    assert_eq!(system_state.system_state_version(), 2);

    let prev_epoch_start_timestamp = system_state.epoch_start_timestamp_ms();

    // We are going to enter safe mode so set the expectation right.
    test_cluster.set_safe_mode_expected(true);

    // Reconfig again and check that we are in safe mode now.
    let system_state = test_cluster.wait_for_epoch(Some(2)).await;
    assert!(system_state.safe_mode());
    assert_eq!(system_state.epoch(), 2);
    // Check that time is properly set even in safe mode.
    assert!(system_state.epoch_start_timestamp_ms() >= prev_epoch_start_timestamp + EPOCH_DURATION);

    // Try a staking transaction.
    let validator_address = system_state
        .into_sui_system_state_summary()
        .active_validators[0]
        .sui_address;
    let txn = make_staking_transaction(&test_cluster.wallet, validator_address).await;
    test_cluster.execute_transaction(txn).await;

    // Now remove the override and check that in the next epoch we are no longer in safe mode.
    test_cluster.set_safe_mode_expected(false);

    let system_state = test_cluster.wait_for_epoch(Some(3)).await;
    assert!(!system_state.safe_mode());
    assert_eq!(system_state.epoch(), 3);
    assert_eq!(system_state.system_state_version(), 2);
}

async fn add_validator_candidate(
    test_cluster: &TestCluster,
    new_validator: &ValidatorGenesisConfig,
) {
    let cur_validator_candidate_count = test_cluster.fullnode_handle.sui_node.with(|node| {
        node.state()
            .get_sui_system_state_object_for_testing()
            .unwrap()
            .into_sui_system_state_summary()
            .validator_candidates_size
    });
    let address = (&new_validator.account_key_pair.public()).into();
    let gas = test_cluster
        .wallet
        .get_one_gas_object_owned_by_address(address)
        .await
        .unwrap()
        .unwrap();

    let tx =
        TestTransactionBuilder::new(address, gas, test_cluster.get_reference_gas_price().await)
            .call_request_add_validator_candidate(
                &new_validator.to_validator_info_with_random_name().into(),
            )
            .build_and_sign(&new_validator.account_key_pair);
    test_cluster.execute_transaction(tx).await;

    // Check that the candidate can be found in the candidate table now.
    test_cluster.fullnode_handle.sui_node.with(|node| {
        let system_state = node
            .state()
            .get_sui_system_state_object_for_testing()
            .unwrap();
        let system_state_summary = system_state.into_sui_system_state_summary();
        assert_eq!(
            system_state_summary.validator_candidates_size,
            cur_validator_candidate_count + 1
        );
    });
}

async fn execute_remove_validator_tx(test_cluster: &TestCluster, handle: &SuiNodeHandle) {
    let address = handle.with(|node| node.get_config().sui_address());
    let gas = test_cluster
        .wallet
        .get_one_gas_object_owned_by_address(address)
        .await
        .unwrap()
        .unwrap();

    let rgp = test_cluster.get_reference_gas_price().await;
    let tx = handle.with(|node| {
        TestTransactionBuilder::new(address, gas, rgp)
            .call_request_remove_validator()
            .build_and_sign(node.get_config().account_key_pair.keypair())
    });
    test_cluster.execute_transaction(tx).await;
}

/// Execute a sequence of transactions to add a validator, including adding candidate, adding stake
/// and activate the validator.
/// It does not however trigger reconfiguration yet.
async fn execute_add_validator_transactions(
    test_cluster: &TestCluster,
    new_validator: &ValidatorGenesisConfig,
) {
    let pending_active_count = test_cluster.fullnode_handle.sui_node.with(|node| {
        let system_state = node
            .state()
            .get_sui_system_state_object_for_testing()
            .unwrap();
        system_state
            .get_pending_active_validators(node.state().db().as_ref())
            .unwrap()
            .len()
    });
    add_validator_candidate(test_cluster, new_validator).await;

    let address = (&new_validator.account_key_pair.public()).into();
    let stake_coin = test_cluster
        .wallet
        .gas_for_owner_budget(
            address,
            MIN_VALIDATOR_JOINING_STAKE_MIST,
            Default::default(),
        )
        .await
        .unwrap()
        .1
        .object_ref();
    let gas = test_cluster
        .wallet
        .gas_for_owner_budget(address, 0, BTreeSet::from([stake_coin.0]))
        .await
        .unwrap()
        .1
        .object_ref();

    let rgp = test_cluster.get_reference_gas_price().await;
    let stake_tx = TestTransactionBuilder::new(address, gas, rgp)
        .call_staking(stake_coin, address)
        .build_and_sign(&new_validator.account_key_pair);
    test_cluster.execute_transaction(stake_tx).await;

    let gas = test_cluster.wallet.get_object_ref(gas.0).await.unwrap();
    let tx = TestTransactionBuilder::new(address, gas, rgp)
        .call_request_add_validator()
        .build_and_sign(&new_validator.account_key_pair);
    test_cluster.execute_transaction(tx).await;

    // Check that we can get the pending validator from 0x5.
    test_cluster.fullnode_handle.sui_node.with(|node| {
        let system_state = node
            .state()
            .get_sui_system_state_object_for_testing()
            .unwrap();
        let pending_active_validators = system_state
            .get_pending_active_validators(node.state().db().as_ref())
            .unwrap();
        assert_eq!(pending_active_validators.len(), pending_active_count + 1);
        assert_eq!(
            pending_active_validators[pending_active_validators.len() - 1].sui_address,
            address
        );
    });
}
