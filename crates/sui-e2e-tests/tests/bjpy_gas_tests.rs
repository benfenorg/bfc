

use std::str::FromStr;
use anyhow::Error;
use jsonrpsee::http_client::HttpClient;
use sui_json_rpc_types::{SuiExecutionStatus, SuiObjectData, SuiObjectDataFilter, SuiObjectDataOptions, SuiObjectResponse, SuiObjectResponseQuery, SuiTransactionBlockEffects};
use sui_json_rpc_types::{SuiTransactionBlockResponseOptions, SuiTypeTag, TransactionBlockBytes};
use sui_macros::sim_test;
use sui_sdk::json::SuiJsonValue;
use sui_types::base_types::SuiAddress;
use test_cluster::{TestCluster, TestClusterBuilder};
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_types::{parse_sui_struct_tag, BFC_SYSTEM_PACKAGE_ID};
use serde_json::json;
use sui_json_rpc_api::{IndexerApiClient, WriteApiClient};
use sui_json_rpc_api::TransactionBuilderClient;




#[sim_test]
async fn sim_test_operate_use_bjpy_gas() -> Result<(), anyhow::Error> {
    // init
    let test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(6000)
        .with_num_validators(5)
        .build()
        .await;
    let http_client = test_cluster.rpc_client();
    let address = test_cluster.get_address_0();
    let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    get_bjpy_and_busd(&test_cluster, http_client, address, &bfc_status_address).await?;
    Ok(())
}

async fn get_bjpy_and_busd(test_cluster: &TestCluster, http_client: &HttpClient, address: SuiAddress, bfc_status_address: &SuiAddress) -> Result<(), Error> {
    let args0 = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::new(json!(address.to_string()))?,
    ];
    let transaction_bytes0: TransactionBlockBytes = http_client.move_call(
        address,
        BFC_SYSTEM_PACKAGE_ID,
        "bfc_system".to_string(),
        "init_single_admin_capability".to_string(),
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
    add_auth_key(test_cluster, http_client, address, &bfc_status_address, &admin_cap,"MINT-OTHER-STABLECOIN-right_key").await?;
    add_auth_key(test_cluster, http_client, address, &bfc_status_address, &admin_cap,"MINT-USDT-USDC-right_key").await?;
    let modify_cap_vec = get_owned_objects("0xc8::bfc_system_state_inner::BfcSystemModifyCap", http_client, address).await.unwrap();
    let modify_cap = modify_cap_vec.first().unwrap().object().unwrap();
    mint_stable_coin(test_cluster, http_client, address, &bfc_status_address, &modify_cap,"0xc8::bjpy::BJPY").await?;
    mint_stable_coin(test_cluster, http_client, address, &bfc_status_address, &modify_cap,"0xc8::usdc::USDC").await?;
    Ok(())
}

async fn add_auth_key(test_cluster: &TestCluster, http_client: &HttpClient, address: SuiAddress, bfc_status_address: &&SuiAddress, admin_cap: &&SuiObjectData,auth_key: &str) -> Result<(), Error> {
    let args1 = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::from_str(&admin_cap.object_id.to_string())?,
        SuiJsonValue::new(json!(auth_key))?,
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

async fn mint_stable_coin(test_cluster: &TestCluster, http_client: &HttpClient, address: SuiAddress, bfc_status_address: &&SuiAddress, modify_cap: &&SuiObjectData,coint_type: &str) -> Result<(), Error> {
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
            vec![SuiTypeTag::new(coint_type.to_string())],
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
    Ok(())
}

// async fn swap_stablecoin_to_bfc_by_bjpy_gas(test_cluster: &TestCluster, http_client: &HttpClient, address: SuiAddress, amount: u64) -> Result<(), anyhow::Error> {
//     let bfc_response_vec = do_get_owned_objects_with_filter("0x2::coin::Coin<0xc8::bjpy::BJPY>", http_client, address).await.unwrap();
//     let stable_coin = bfc_response_vec.last().unwrap().object().unwrap();
//     let gas_budget = 1_000_000_000;
//     let split_coin_txn_bytes = http_client.split_coin(address, stable_coin.object_id, vec![BigInt::from(gas_budget)],
//                                                       None, BigInt::from(10000000)).await?.to_data()?;
//     let split_coin_txn = test_cluster.wallet.sign_transaction(&split_coin_txn_bytes);
//     let _response = test_cluster.wallet.execute_transaction_must_succeed(split_coin_txn).await;
//     let busd_response_vec = do_get_owned_objects_with_filter(
//         "0x2::coin::Coin<0xc8::busd::BUSD>",
//         http_client,
//         address,
//     ).await?;
//     assert_eq!(busd_response_vec.len(), 2);
//     let mut gas_id = None;
//     let mut coin_id = None;
//     for busd_response in busd_response_vec {
//         let busd_data = busd_response.data.as_ref().unwrap();
//         let balance = get_balance(&busd_data);
//         if balance == gas_budget {
//             gas_id = Some(busd_data.object_id);
//         } else {
//             coin_id = Some(busd_data.object_id);
//         }
//     }
//     // let balance = get_balance(&gas);
//     // tracing::error!("balance is {:?} objid {:?}",balance,gas.object_id);
//     let bfc_system_address: SuiAddress = BFC_SYSTEM_STATE_OBJECT_ID.into();
//     let module = "bfc_system".to_string();
//     let package_id = BFC_SYSTEM_PACKAGE_ID;
//     let function = "swap_stablecoin_to_bfc".to_string();
//     let args = vec![
//         SuiJsonValue::from_str(&bfc_system_address.to_string())?,
//         SuiJsonValue::from_str(&coin_id.unwrap().to_string())?,
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
//     match effects {
//         SuiTransactionBlockEffects::V1(_effects) => {
//             assert!(_effects.status.is_ok());
//         }
//     };
//     Ok(())
// }


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

// fn effect_fail(effects: SuiTransactionBlockEffects, s: &str) {
//     match effects {
//         SuiTransactionBlockEffects::V1(_effects) => {
//             match _effects.status {
//                 SuiExecutionStatus::Success => {
//                     assert!(false);
//                 }
//                 SuiExecutionStatus::Failure { error } => {
//                     assert!(error.contains(s));
//                 }
//             }
//         }
//     };
// }

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