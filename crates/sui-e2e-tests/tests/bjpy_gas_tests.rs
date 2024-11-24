//
// mod upgrade_treasury_tests;
// mod auth;
//
// use std::path::PathBuf;
// use std::str::FromStr;
// use anyhow::Error;
// use chrono::Utc;
// use jsonrpsee::http_client::HttpClient;
// use move_core_types::parser::parse_struct_tag;
// use sui::client_commands::{OptsWithGas, SuiClientCommandResult, SuiClientCommands};
// use sui_json_rpc_types::{ObjectChange, SuiExecutionStatus, SuiMoveStruct, SuiMoveValue, SuiObjectData, SuiObjectDataFilter, SuiObjectDataOptions, SuiObjectResponse, SuiObjectResponseQuery, SuiParsedData, SuiTransactionBlockEffects};
// use sui_json_rpc_types::{SuiTransactionBlockResponseOptions, SuiTypeTag, TransactionBlockBytes};
// use sui_macros::sim_test;
// use sui_move_build::BuildConfig;
// use sui_sdk::json::{type_args, SuiJsonValue};
// use sui_sdk::wallet_context::WalletContext;
// use sui_test_transaction_builder::TestTransactionBuilder;
// use sui_types::base_types::{ObjectID, ObjectRef, SuiAddress};
// use sui_types::sui_serde::BigInt;
// use sui_types::transaction::TEST_ONLY_GAS_UNIT_FOR_PUBLISH;
// use test_cluster::{TestCluster, TestClusterBuilder};
// use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
// use sui_types::{parse_sui_struct_tag, BFC_SYSTEM_PACKAGE_ID,BFC_SYSTEM_STATE_OBJECT_ID, SUI_CLOCK_OBJECT_ID};
// use serde_json::json;
// use sui_json_rpc_api::{IndexerApiClient, WriteApiClient};
// use sui_json_rpc_api::TransactionBuilderClient;
// use tracing::error;
//
// #[sim_test]
// async fn sim_test_operate_use_bjpy_gas() -> Result<(), anyhow::Error> {
//     // init
//     let mut test_cluster = TestClusterBuilder::new()
//         .with_epoch_duration_ms(6000)
//         .with_num_validators(5)
//         .build()
//         .await;
//
//     test_cluster.wait_for_epoch(Some(2)).await;
//     test_cluster
//     .swarm
//     .validator_nodes()
//     .next()
//     .unwrap()
//     .get_node_handle()
//     .unwrap()
//     .with(|node| {
//         let _state = node
//             .state()
//             .get_bfc_system_state_object_for_testing().unwrap();
//         let _oracle_address = _state.get_oracle_address();
//         assert!(_oracle_address.is_none());
//
//         //rate_map
//         let _rate_map = _state.get_rate_map();
//         // println!("=============rate_map: {:?}", &_rate_map);
//
//         let mut pass = false;
//         for entry in _rate_map.clone().contents.into_iter() {
//             if entry.key == "00000000000000000000000000000000000000000000000000000000000000c8::bjpy::BJPY" {
//                 println!("bjpy before {:?}", entry.value);
//                 pass = true;
//             }
//         }
//         assert!(pass);
//     });
//     let mut  http_client = test_cluster.rpc_client().clone();
//     let address = test_cluster.get_address_0();
//     let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
//     let (_, package) = do_publish(&mut test_cluster).await?;
//
//     auth::auth_setup(&mut test_cluster, &mut http_client, address, &bfc_status_address, "MINT-OTHER-STABLECOIN-POLLY").await?;
//     //add oracle price
//     get_bjpy(&test_cluster, &mut http_client, address, &bfc_status_address).await?;
//     check_oracle_price(&mut test_cluster, package).await;
//     // wait to get oracle price and call bfc_round_v2
//     test_cluster.wait_for_epoch(Some(3)).await;
//
//     swap_bfc_to_stablecoin(&test_cluster, &mut http_client, address, 100000000000).await?;
//     swap_stablecoin_to_bfc_by_bjpy_gas(&test_cluster, &mut http_client, address, 100000).await?;
//
//     test_cluster
//     .swarm
//     .validator_nodes()
//     .next()
//     .unwrap()
//     .get_node_handle()
//     .unwrap()
//     .with(|node| {
//         let _state = node
//             .state()
//             .get_bfc_system_state_object_for_testing().unwrap();
//         let _oracle_address = _state.get_oracle_address();
//         assert!(_oracle_address.is_some());
//         // println!("=============oracle_address: {}", _oracle_address.unwrap());
//
//         //rate_map
//         let _rate_map = _state.get_rate_map();
//         // println!("=============rate_map: {:?}", &_rate_map);
//
//         let mut pass = false;
//         for entry in _rate_map.clone().contents.into_iter() {
//             if entry.key == "00000000000000000000000000000000000000000000000000000000000000c8::bjpy::BJPY" {
//                 println!("bjpy after {:?}", entry.value);
//                 pass = true;
//             }
//         }
//         assert!(pass);
//     });
//     Ok(())
// }
//
// async fn get_bjpy(test_cluster: &TestCluster, http_client: &mut HttpClient, address: SuiAddress, bfc_status_address: &SuiAddress) -> Result<(), Error> {
//     let modify_cap_vec = get_owned_objects("0xc8::bfc_system_state_inner::BfcSystemModifyCap", http_client, address).await.unwrap();
//     let modify_cap = modify_cap_vec.first().unwrap().object().unwrap();
//     mint_stable_coin(test_cluster, http_client, address, &bfc_status_address, &modify_cap,"0xc8::bjpy::BJPY").await?;
//     Ok(())
// }
//
// async fn swap_bfc_to_stablecoin(
//     test_cluster: &TestCluster,
//     http_client: &mut HttpClient,
//     address: SuiAddress,
//     amount: u64,
// ) -> Result<(), anyhow::Error> {
//     swap_bfc_to_stablecoin_with_tag(test_cluster, http_client, address, amount,
//                                     SuiTypeTag::new("0xc8::busd::BUSD".to_string())).await?;
//     Ok(())
// }
//
// async fn swap_bfc_to_stablecoin_with_tag(
//     test_cluster: &TestCluster,
//     http_client: &mut HttpClient,
//     address: SuiAddress,
//     amount: u64,
//     type_tag: SuiTypeTag,
// ) -> Result<(), anyhow::Error> {
//     let objects = http_client
//         .get_owned_objects(address, Some(SuiObjectResponseQuery::new_with_filter(
//             SuiObjectDataFilter::StructType(
//                 parse_struct_tag("0x2::coin::Coin<0x2::bfc::BFC>").unwrap(),
//             )
//         )), None, None).await?.data;
//     // api ： https://docs.sui.io/sui-api-ref#suix_getownedobjects
//     let coin = objects.first().unwrap().object().unwrap();
//
//     let bfc_system_address: SuiAddress = BFC_SYSTEM_STATE_OBJECT_ID.into();
//     let module = "bfc_system".to_string();
//     let package_id = BFC_SYSTEM_PACKAGE_ID;
//     let function = "swap_bfc_to_stablecoin".to_string();
//     let timestamp = Utc::now().timestamp() * 1000 + 600000;
//     let deadtime = timestamp.to_string();
//
//     let args = vec![
//         SuiJsonValue::from_str(&bfc_system_address.to_string())?,
//         SuiJsonValue::from_str(&coin.object_id.to_string())?,
//         SuiJsonValue::from_str(&SUI_CLOCK_OBJECT_ID.to_string())?,
//         SuiJsonValue::new(json!(amount.to_string()))?,
//         SuiJsonValue::new(json!("0"))?,
//         SuiJsonValue::new(json!(&deadtime))?,
//     ];
//
//     let transaction_bytes: TransactionBlockBytes = http_client
//         .move_call(
//             address,
//             package_id,
//             module,
//             function,
//             vec![type_tag],
//             args,
//             None,
//             10_000_00000.into(),
//             None,
//         )
//         .await?;
//
//     let tx = test_cluster
//         .wallet
//         .sign_transaction(&transaction_bytes.to_data()?);
//     let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();
//     let tx_response = http_client
//         .execute_transaction_block(
//             tx_bytes,
//             signatures,
//             Some(SuiTransactionBlockResponseOptions::new().with_effects()),
//             Some(ExecuteTransactionRequestType::WaitForLocalExecution),
//         )
//         .await?;
//     let effects = tx_response.effects.unwrap().clone();
//
//     match effects {
//         SuiTransactionBlockEffects::V1(_effects) => {
//             if _effects.status.is_err() {
//                 error!("effects is {:?}",_effects);
//             }
//             assert!(_effects.status.is_ok());
//         }
//     };
//     Ok(())
// }
//
// async fn add_auth_key(test_cluster: &TestCluster, http_client: &HttpClient, address: SuiAddress, bfc_status_address: &&SuiAddress, admin_cap: &&SuiObjectData,auth_key: &str) -> Result<(), Error> {
//     let args1 = vec![
//         SuiJsonValue::from_str(&bfc_status_address.to_string())?,
//         SuiJsonValue::from_str(&admin_cap.object_id.to_string())?,
//         SuiJsonValue::new(json!(auth_key))?,
//         SuiJsonValue::new(json!(address.to_string()))?,
//     ];
//     let transaction_bytes1: TransactionBlockBytes = http_client
//         .move_call(
//             address,
//             BFC_SYSTEM_PACKAGE_ID,
//             "bfc_system".to_string(),
//             "set_single_operation_capability".to_string(),
//             vec![],
//             args1,
//             None,
//             10_000_00000.into(),
//             None,
//         )
//         .await?;
//     let tx1 = test_cluster
//         .wallet
//         .sign_transaction(&transaction_bytes1.to_data()?);
//     let (tx_bytes1, signatures1) = tx1.to_tx_bytes_and_signatures();
//     http_client
//         .execute_transaction_block(
//             tx_bytes1,
//             signatures1,
//             Some(SuiTransactionBlockResponseOptions::new().with_effects()),
//             Some(ExecuteTransactionRequestType::WaitForLocalExecution),
//         )
//         .await?;
//     Ok(())
// }
//
// async fn mint_stable_coin(test_cluster: &TestCluster, http_client: &HttpClient, address: SuiAddress, bfc_status_address: &&SuiAddress, modify_cap: &&SuiObjectData,coint_type: &str) -> Result<(), Error> {
//     let args = vec![
//         SuiJsonValue::from_str(&bfc_status_address.to_string())?,
//         SuiJsonValue::new(json!(25000000000u64.to_string()))?,
//         SuiJsonValue::from_str(&modify_cap.object_id.to_string())?,
//     ];
//     let transaction_bytes: TransactionBlockBytes = http_client
//         .move_call(
//             address,
//             BFC_SYSTEM_PACKAGE_ID,
//             "bfc_system".to_string(),
//             "mint_stable_entry".to_string(),
//             vec![SuiTypeTag::new(coint_type.to_string())],
//             args,
//             None,
//             10_000_00000.into(),
//             None,
//         )
//         .await?;
//     let tx = test_cluster
//         .wallet
//         .sign_transaction(&transaction_bytes.to_data()?);
//     let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();
//     let tx_response = http_client
//         .execute_transaction_block(
//             tx_bytes,
//             signatures,
//             Some(SuiTransactionBlockResponseOptions::new().with_effects()),
//             Some(ExecuteTransactionRequestType::WaitForLocalExecution),
//         )
//         .await?;
//     let effects = tx_response.effects.unwrap().clone();
//     effect_success(effects);
//     Ok(())
// }
//
// async fn swap_stablecoin_to_bfc_by_bjpy_gas(test_cluster: &TestCluster, http_client: &HttpClient, address: SuiAddress, amount: u64) -> Result<(), anyhow::Error> {
//     let bjpy_response_vec = do_get_owned_objects_with_filter("0x2::coin::Coin<0xc8::bjpy::BJPY>", http_client, address).await.unwrap();
//     let bjpy_coin = bjpy_response_vec.last().unwrap().object().unwrap();
//     let gas_budget = 25_000_000_000;
//     let split_coin_txn_bytes = http_client.split_coin(address, bjpy_coin.object_id, vec![BigInt::from(gas_budget)],
//                                                       None, BigInt::from(gas_budget)).await?.to_data()?;
//     let split_coin_txn = test_cluster.wallet.sign_transaction(&split_coin_txn_bytes);
//     let _response = test_cluster.wallet.execute_transaction_must_succeed(split_coin_txn).await;
//     let bjpy_response_vec = do_get_owned_objects_with_filter(
//         "0x2::coin::Coin<0xc8::bjpy::BJPY>",
//         http_client,
//         address,
//     ).await?;
//     assert_eq!(bjpy_response_vec.len(), 2);
//     let mut gas_id = None;
//     // let mut coin_id = None;
//     for bjpy_response in bjpy_response_vec {
//         let bjpy_data = bjpy_response.data.as_ref().unwrap();
//         let balance = get_balance(&bjpy_data);
//         if balance == gas_budget {
//             gas_id = Some(bjpy_data.object_id);
//         }
//     }
//     println!("gas_id is {:?}",gas_id);
//     assert!(gas_id.is_some());
//     //get busd
//     let busd_response_vec = do_get_owned_objects_with_filter(
//         "0x2::coin::Coin<0xc8::busd::BUSD>",
//         http_client,
//         address,
//     ).await?;
//     let busd_coin_id = busd_response_vec.first().unwrap().data.as_ref().unwrap().object_id;
//     println!("busd_coin_id is {:?}",busd_coin_id);
//
//     // let balance = get_balance(&gas);
//     // tracing::error!("balance is {:?} objid {:?}",balance,gas.object_id);
//     let bfc_system_address: SuiAddress = BFC_SYSTEM_STATE_OBJECT_ID.into();
//     let module = "bfc_system".to_string();
//     let package_id = BFC_SYSTEM_PACKAGE_ID;
//     let function = "swap_stablecoin_to_bfc".to_string();
//     let args = vec![
//         SuiJsonValue::from_str(&bfc_system_address.to_string())?,
//         SuiJsonValue::from_str(&busd_coin_id.to_string())?,
//         SuiJsonValue::from_str(&SUI_CLOCK_OBJECT_ID.to_string())?,
//         SuiJsonValue::new(json!(&amount.to_string()))?,
//         SuiJsonValue::new(json!("0"))?,
//         SuiJsonValue::new(json!("1709622441776884"))?,
//     ];
//     let transaction_bytes: TransactionBlockBytes = http_client
//         .move_call(
//             address,
//             package_id,
//             module,
//             function,
//             vec![SuiTypeTag::new("0xc8::busd::BUSD".to_string())],
//             args,
//             gas_id,
//             1_000_000_000.into(),
//             None,
//         )
//         .await?;
//     let tx = test_cluster
//         .wallet
//         .sign_transaction(&transaction_bytes.to_data()?);
//     let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();
//     let tx_response = http_client
//         .execute_transaction_block(
//             tx_bytes,
//             signatures,
//             Some(SuiTransactionBlockResponseOptions::new().with_effects()),
//             Some(ExecuteTransactionRequestType::WaitForLocalExecution),
//         )
//         .await?;
//     let effects = tx_response.effects.unwrap().clone();
//     println!("effects is {:?}",effects);
//     match effects {
//         SuiTransactionBlockEffects::V1(_effects) => {
//             assert!(_effects.status.is_ok());
//         }
//     };
//     Ok(())
// }
//
// async fn do_get_owned_objects_with_filter(filter_tag: &str, http_client: &HttpClient, address: SuiAddress) -> Result<Vec<SuiObjectResponse>, anyhow::Error> {
//     let filter = SuiObjectDataFilter::StructType(parse_sui_struct_tag(filter_tag).unwrap());
//     let data_option = SuiObjectDataOptions::new()
//         .with_type()
//         .with_owner()
//         .with_previous_transaction()
//         .with_content();
//     let objects = http_client
//         .get_owned_objects(
//             address,
//             Some(SuiObjectResponseQuery::new(
//                 Option::Some(filter),
//                 Option::Some(data_option),
//             )),
//             None,
//             None,
//         )
//         .await?
//         .data;
//     Ok(objects)
// }
//
// fn get_balance(busd_data: &SuiObjectData) -> u64 {
//     if let SuiParsedData::MoveObject(move_object) = busd_data.content.clone().unwrap() {
//         if let SuiMoveStruct::WithFields(data) = move_object.fields {
//             match data.get("balance").unwrap() {
//                 SuiMoveValue::String(balance) => return balance.parse().unwrap(),
//                 _ => return 0,
//             }
//         }
//     }
//     0
// }
//
// async fn get_owned_objects(filter_tag: &str, http_client: &HttpClient, address: SuiAddress) -> Result<Vec<SuiObjectResponse>, anyhow::Error> {
//     let filter = SuiObjectDataFilter::StructType(parse_sui_struct_tag(filter_tag).unwrap());
//     let data_option = SuiObjectDataOptions::new()
//         .with_type()
//         .with_owner()
//         .with_previous_transaction()
//         .with_content();
//     let objects = http_client
//         .get_owned_objects(
//             address,
//             Some(SuiObjectResponseQuery::new(
//                 Some(filter),
//                 Some(data_option),
//             )),
//             None,
//             None,
//         )
//         .await?
//         .data;
//     Ok(objects)
// }
//
// // fn effect_fail(effects: SuiTransactionBlockEffects, s: &str) {
// //     match effects {
// //         SuiTransactionBlockEffects::V1(_effects) => {
// //             match _effects.status {
// //                 SuiExecutionStatus::Success => {
// //                     assert!(false);
// //                 }
// //                 SuiExecutionStatus::Failure { error } => {
// //                     assert!(error.contains(s));
// //                 }
// //             }
// //         }
// //     };
// // }
//
// fn effect_success(effects: SuiTransactionBlockEffects) {
//     match effects {
//         SuiTransactionBlockEffects::V1(_effects) => {
//             match _effects.status {
//                 SuiExecutionStatus::Success => {
//                     assert!(true);
//                 }
//                 SuiExecutionStatus::Failure { error } => {
//                     assert!(false, "{}", error);
//                 }
//             }
//         }
//     };
// }
//
// async fn check_oracle_price(test_cluster: &mut TestCluster, package: ObjectID) {
//     let context = &test_cluster.wallet;
//     let address = test_cluster.get_address_0();
//     println!("address: {:?}", address);
//     let gas = context
//         .get_one_gas_object_owned_by_address(address)
//         .await
//         .unwrap()
//         .unwrap();
//     let tx = context.sign_transaction(
//         &TestTransactionBuilder::new(address, gas, context.get_reference_gas_price().await.unwrap())
//             .move_call(
//                 package,
//                 "test_oracle",
//                 "oracle",
//                 vec![],
//             )
//             .build(),
//     );
//     let resp = test_cluster.execute_transaction(tx).await;
//     println!("resp: {:#?}", resp.clone().object_changes.unwrap());
//
//     let oracle_id = resp.object_changes.unwrap().iter()
//         .find(|change| match change {
//             ObjectChange::Created {
//                 object_type, owner, ..
//             } => {
//                 object_type.to_string().contains("dynamic_field")
//             }
//             _ => false,
//         }).unwrap().object_id();
//
//     set_oracle_address(test_cluster, oracle_id.clone().to_bfc_address()).await.unwrap();
//
//     let state = test_cluster.fullnode_handle.sui_node.state().clone();
//     let bfc_sys_state = state.get_bfc_system_state_object_for_testing().unwrap();
//
//     println!("bfc_sys_state.get_oracle_address(): {:#?}", &bfc_sys_state.get_oracle_address().unwrap());
//     let price = state.get_oracle_price_by_id(ObjectID::from(bfc_sys_state.get_oracle_address().unwrap())).unwrap();
//     println!("price: {:?}", price);
//     assert!(price.value.len() > 0);
// }
//
// async fn set_oracle_address(test_cluster: &mut TestCluster, oracle_address: String) -> Result<(), Error> {
//     let module = "bfc_system".to_string();
//     let package_id = BFC_SYSTEM_PACKAGE_ID;
//     let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
//     let context = &test_cluster.wallet;
//
//     let http_client = test_cluster.rpc_client();
//     let address = test_cluster.get_address_0();
//     println!("address: {:?}", address);
//
//     let bfc_objects = do_get_owned_objects_with_filter("0x2::coin::Coin<0x2::bfc::BFC>", http_client, address).await?;
//     let gas = bfc_objects.first().unwrap().object().unwrap();
//
//     let tx = context.sign_transaction(
//         &http_client.move_call(
//             address,
//             package_id,
//             module,
//             "set_oracle_address".to_string(),
//             type_args![]?,
//             vec![
//                 SuiJsonValue::from_str(&bfc_status_address.to_string())?,
//                 SuiJsonValue::from_str(&oracle_address)?,
//             ],
//             Some(gas.object_id),
//             10_000_00000.into(),
//             None,
//         ).await?.to_data()?,
//     );
//     let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();
//     let tx_response = http_client
//         .execute_transaction_block(
//             tx_bytes,
//             signatures,
//             Some(SuiTransactionBlockResponseOptions::new().with_effects()),
//             Some(ExecuteTransactionRequestType::WaitForLocalExecution),
//         )
//         .await?;
//     println!("set_oracle_address tx_response: {:#?}", tx_response);
//     Ok(())
// }
//
// async fn do_publish(test_cluster: &mut TestCluster) -> Result<(ObjectRef, ObjectID), Error> {
//     let address = test_cluster.get_address_0();
//
//     let rgp = test_cluster.get_reference_gas_price().await;
//     let mut context = &mut test_cluster.wallet;
//     let client = context.get_client().await?;
//     let object_refs = client
//         .read_api()
//         .get_owned_objects(
//             address,
//             Some(SuiObjectResponseQuery::new_with_options(
//                 SuiObjectDataOptions::new()
//                     .with_type()
//                     .with_owner()
//                     .with_previous_transaction(),
//             )),
//             None,
//             None,
//         )
//         .await?
//         .data;
//
//     // Check log output contains all object ids.
//     let gas_obj = object_refs.first().unwrap().object().unwrap();
//     let gas_obj_id = &gas_obj.object_id;
//     //step 1: publish coin
//     let resp = do_publish_inner(rgp, &mut context, gas_obj_id).await?;
//
//     // // Print it out to CLI/logs
//     // resp.print(true);
//
//     let SuiClientCommandResult::Publish(response) = resp else {
//         unreachable!("Invalid response");
//     };
//
//     let SuiTransactionBlockEffects::V1(effects) = response.effects.unwrap();
//     assert!(effects.status.is_ok());
//     // assert_eq!(effects.gas_object().object_id(), gas_obj_id);
//     let cap = effects.created.get(1).unwrap().reference.to_object_ref();
//     println!("cap:{:?}", cap);
//     let mut published = vec![];
//     let obj_changed = &response.object_changes.unwrap();
//     for obj in obj_changed {
//         match obj {
//             ObjectChange::Published { .. } => published.push(obj),
//             _ => {}
//         };
//     }
//     let package = published.first().unwrap();
//     Ok((cap, package.object_id()))
// }
//
// async fn do_publish_inner(rgp: u64, context: &mut WalletContext, gas_obj_id: &ObjectID) -> Result<SuiClientCommandResult, Error> {
//     let mut package_path = PathBuf::from("tests/test_oracle_price/");
//     package_path.push("sources");
//     let build_config = BuildConfig::new_for_testing().config;
//     let resp = SuiClientCommands::Publish {
//         package_path: package_path.clone(),
//         build_config,
//         skip_dependency_verification: false,
//         with_unpublished_dependencies: false,
//         opts: OptsWithGas::for_testing(Some(*gas_obj_id), rgp * TEST_ONLY_GAS_UNIT_FOR_PUBLISH),
//     }
//         .execute(context)
//         .await?;
//     Ok(resp)
// }