
mod upgrade_treasury_tests;
mod auth;
mod stable;
mod publish_coin;

use std::str::FromStr;
use std::time::Duration;
use std::vec;
use anyhow::{anyhow, Error};
use chrono::Utc;
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
use sui_types::transaction::{CallArg, ObjectArg};
use test_cluster::{TestCluster, TestClusterBuilder};
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_types::{parse_sui_struct_tag, BFC_SYSTEM_PACKAGE_ID, BFC_SYSTEM_STATE_OBJECT_ID, SUI_CLOCK_OBJECT_ID};
use serde_json::json;
use sui_json_rpc_api::{CoinReadApiClient, IndexerApiClient, WriteApiClient};
use sui_json_rpc_api::TransactionBuilderClient;
use tokio::time::sleep;
use tracing::error;
//use sui_network::tonic::codegen::StdError;

#[sim_test]
#[ignore]
async fn sim_test_operate_use_bjpy_gas() -> Result<(), anyhow::Error> {
    // init
    let mut test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(6000)
        .with_num_validators(5)
        .with_all_vault_init()
        .build()
        .await;

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
        assert!(_oracle_address.is_none());

        //rate_map
        let _rate_map = _state.get_rate_map();
        // println!("=============rate_map: {:?}", &_rate_map);

        let mut pass = false;
        for entry in _rate_map.clone().contents.into_iter() {
            if entry.key == "00000000000000000000000000000000000000000000000000000000000000c8::bjpy::BJPY" {
                println!("bjpy before {:?}", entry.value);
                pass = true;
            }
        }
        assert!(pass);
    });
    let mut  http_client = test_cluster.rpc_client().clone();
    let address = test_cluster.get_address_0();
    // let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let (package, _) = publish_coin::do_publish(&mut test_cluster,"tests/test_oracle_price").await?;
    auth::auth_setup(&mut test_cluster, &mut http_client, address, "MINT-OTHER-STABLECOIN-POLLY").await?;
    test_cluster.wait_for_epoch(Some(3)).await;
    //add oracle price
    get_bjpy(&test_cluster, &mut http_client, address).await?;
    check_oracle_price(&mut test_cluster, package).await;
    // wait to get oracle price and call bfc_round_v2
    test_cluster.wait_for_epoch(Some(3)).await;

    swap_bfc_to_stablecoin(&test_cluster, &mut http_client, address, 100000000000).await?;
    swap_stablecoin_to_bfc_by_bjpy_gas(&test_cluster, &mut http_client, address, 100000).await?;

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
        assert!(_oracle_address.is_some());
        // println!("=============oracle_address: {}", _oracle_address.unwrap());

        //rate_map
        let _rate_map = _state.get_rate_map();
        // println!("=============rate_map: {:?}", &_rate_map);

        let mut pass = false;
        for entry in _rate_map.clone().contents.into_iter() {
            if entry.key == "00000000000000000000000000000000000000000000000000000000000000c8::bjpy::BJPY" {
                println!("bjpy after {:?}", entry.value);
                pass = true;
            }
        }
        assert!(pass);
    });
    Ok(())
}

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
    auth::auth_setup(&mut test_cluster, &mut http_client, address, "MINT-OTHER-STABLECOIN-POLLY").await?;

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
    test_cluster.wait_for_epoch(Some(10)).await;

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

async fn get_bjpy(test_cluster: &TestCluster, http_client: &mut HttpClient, address: SuiAddress) -> Result<(), Error> {
    stable::mint_stable_coin(25_000_000_000,test_cluster, http_client, address,"0xc8::bjpy::BJPY").await?;
    Ok(())
}

async fn swap_bfc_to_stablecoin(
    test_cluster: &TestCluster,
    http_client: &mut HttpClient,
    address: SuiAddress,
    amount: u64,
) -> Result<(), anyhow::Error> {
    swap_bfc_to_stablecoin_with_tag(test_cluster, http_client, address, amount,
                                    SuiTypeTag::new("0xc8::busd::BUSD".to_string())).await?;
    Ok(())
}

async fn swap_bfc_to_stablecoin_with_tag(
    test_cluster: &TestCluster,
    http_client: &mut HttpClient,
    address: SuiAddress,
    amount: u64,
    type_tag: SuiTypeTag,
) -> Result<(), anyhow::Error> {
    let objects = http_client
        .get_owned_objects(address, Some(SuiObjectResponseQuery::new_with_filter(
            SuiObjectDataFilter::StructType(
                parse_struct_tag("0x2::coin::Coin<0x2::bfc::BFC>").unwrap(),
            )
        )), None, None).await?.data;
    // api ： https://docs.sui.io/sui-api-ref#suix_getownedobjects
    let coin = objects.first().unwrap().object().unwrap();

    let bfc_system_address: SuiAddress = BFC_SYSTEM_STATE_OBJECT_ID.into();
    let module = "bfc_system".to_string();
    let package_id = BFC_SYSTEM_PACKAGE_ID;
    let function = "swap_bfc_to_stablecoin".to_string();
    let timestamp = Utc::now().timestamp() * 1000 + 600000;
    let deadtime = timestamp.to_string();

    let args = vec![
        SuiJsonValue::from_str(&bfc_system_address.to_string())?,
        SuiJsonValue::from_str(&coin.object_id.to_string())?,
        SuiJsonValue::from_str(&SUI_CLOCK_OBJECT_ID.to_string())?,
        SuiJsonValue::new(json!(amount.to_string()))?,
        SuiJsonValue::new(json!("0"))?,
        SuiJsonValue::new(json!(&deadtime))?,
    ];

    let transaction_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            package_id,
            module,
            function,
            vec![type_tag],
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

async fn swap_stablecoin_to_bfc_by_bjpy_gas(test_cluster: &TestCluster, http_client: &HttpClient, address: SuiAddress, amount: u64) -> Result<(), anyhow::Error> {
    let bjpy_response_vec = do_get_owned_objects_with_filter("0x2::coin::Coin<0xc8::bjpy::BJPY>", http_client, address).await.unwrap();
    let bjpy_coin = bjpy_response_vec.last().unwrap().object().unwrap();
    let gas_budget = 25_000_000_000;
    let split_coin_txn_bytes = http_client.split_coin(address, bjpy_coin.object_id, vec![BigInt::from(gas_budget)],
                                                      None, BigInt::from(gas_budget)).await?.to_data()?;
    let split_coin_txn = test_cluster.wallet.sign_transaction(&split_coin_txn_bytes);
    let _response = test_cluster.wallet.execute_transaction_must_succeed(split_coin_txn).await;
    let bjpy_response_vec = do_get_owned_objects_with_filter(
        "0x2::coin::Coin<0xc8::bjpy::BJPY>",
        http_client,
        address,
    ).await?;
    assert_eq!(bjpy_response_vec.len(), 2);
    let mut gas_id = None;
    // let mut coin_id = None;
    for bjpy_response in bjpy_response_vec {
        let bjpy_data = bjpy_response.data.as_ref().unwrap();
        let balance = get_balance(&bjpy_data);
        if balance == gas_budget {
            gas_id = Some(bjpy_data.object_id);
        }
    }
    println!("gas_id is {:?}",gas_id);
    assert!(gas_id.is_some());
    //get busd
    let busd_response_vec = do_get_owned_objects_with_filter(
        "0x2::coin::Coin<0xc8::busd::BUSD>",
        http_client,
        address,
    ).await?;
    let busd_coin_id = busd_response_vec.first().unwrap().data.as_ref().unwrap().object_id;
    println!("busd_coin_id is {:?}",busd_coin_id);

    // let balance = get_balance(&gas);
    // tracing::error!("balance is {:?} objid {:?}",balance,gas.object_id);
    let bfc_system_address: SuiAddress = BFC_SYSTEM_STATE_OBJECT_ID.into();
    let module = "bfc_system".to_string();
    let package_id = BFC_SYSTEM_PACKAGE_ID;
    let function = "swap_stablecoin_to_bfc".to_string();
    let args = vec![
        SuiJsonValue::from_str(&bfc_system_address.to_string())?,
        SuiJsonValue::from_str(&busd_coin_id.to_string())?,
        SuiJsonValue::from_str(&SUI_CLOCK_OBJECT_ID.to_string())?,
        SuiJsonValue::new(json!(&amount.to_string()))?,
        SuiJsonValue::new(json!("0"))?,
        SuiJsonValue::new(json!("1709622441776884"))?,
    ];
    let transaction_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            package_id,
            module,
            function,
            vec![SuiTypeTag::new("0xc8::busd::BUSD".to_string())],
            args,
            gas_id,
            1_000_000_000.into(),
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
    println!("effects is {:?}",effects);
    match effects {
        SuiTransactionBlockEffects::V1(_effects) => {
            assert!(_effects.status.is_ok());
        }
    };
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

fn get_balance(busd_data: &SuiObjectData) -> u64 {
    if let SuiParsedData::MoveObject(move_object) = busd_data.content.clone().unwrap() {
        if let SuiMoveStruct::WithFields(data) = move_object.fields {
            match data.get("balance").unwrap() {
                SuiMoveValue::String(balance) => return balance.parse().unwrap(),
                _ => return 0,
            }
        }
    }
    0
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

async fn check_oracle_price(test_cluster: &mut TestCluster, package: ObjectID) {
    let context = &test_cluster.wallet;
    let address = test_cluster.get_address_0();
    println!("address: {:?}", address);
    let gas = context
        .get_one_gas_object_owned_by_address(address)
        .await
        .unwrap()
        .unwrap();
    let tx = context.sign_transaction(
        &TestTransactionBuilder::new(address, gas, context.get_reference_gas_price().await.unwrap())
            .move_call(
                package,
                "test_oracle",
                "oracle",
                vec![],
            )
            .build(),
    );
    let resp = test_cluster.execute_transaction(tx).await;
    println!("resp: {:#?}", resp.clone().object_changes.unwrap());

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
    println!("price: {:?}", price);
    assert!(price.value.contents.len() > 0);
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

    let mut  http_client = test_cluster.rpc_client().clone();
    swap_bfc_to_stablecoin(&test_cluster, &mut http_client, address, 100000000000).await?;

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
    tracing::error!("txn_data is {:?}",txn_data);
    let tx = context.sign_transaction(&txn_data);
    let resp = test_cluster.wallet.execute_transaction_may_fail(tx).await;
    tracing::error!("test_move_call_use_new_test_coin resp: {:#?}", resp);

    if  resp.is_err() {
        println!("test_move_call_use_new_test_coin resp: {:#?}", resp);
        return Err(resp.unwrap_err());
    }

    let mut result_vec =  Vec::with_capacity(2);
    for ele in  resp?.object_changes.unwrap() {
        if let ObjectChange::Created { object_id, version,digest,object_type,.. } = ele {
            if object_type.name == Identifier::from_str("TestOraclePrice").unwrap(){
                result_vec.insert(0,(object_id,version,digest));
            }
            if object_type.name == Identifier::from_str("Global").unwrap(){
                result_vec.insert(1,(object_id,version,digest));
            }
        }
    }
    if result_vec.len() != 2 {
        Err(anyhow!("not found"))
    }else {
        Ok((result_vec[0],result_vec[1]))
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
