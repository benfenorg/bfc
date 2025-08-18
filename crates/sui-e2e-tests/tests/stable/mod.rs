use std::str::FromStr;

use anyhow::Error;
use jsonrpsee::http_client::HttpClient;
use serde_json::json;
use sui_json_rpc_api::{IndexerApiClient, TransactionBuilderClient, WriteApiClient};
use sui_json_rpc_types::{SuiExecutionStatus, SuiObjectDataFilter, SuiObjectDataOptions, SuiObjectResponse, SuiObjectResponseQuery, SuiTransactionBlockEffects, SuiTransactionBlockResponseOptions, SuiTypeTag, TransactionBlockBytes};
use sui_sdk::json::SuiJsonValue;
use sui_types::{base_types::SuiAddress, parse_sui_struct_tag, quorum_driver_types::ExecuteTransactionRequestType, BFC_SYSTEM_PACKAGE_ID};
use test_cluster::TestCluster;

#[allow(dead_code)]
pub async fn mint_stable_coin_to_address(amount: u64, test_cluster: &TestCluster, http_client: &HttpClient, address: SuiAddress,coint_type: &str,to_address: SuiAddress) -> Result<(), Error> {
    let modify_cap_vec = get_owned_objects("0xc8::bfc_system_state_inner::BfcSystemModifyCap", http_client, address).await.unwrap();
    let modify_cap = modify_cap_vec.first().unwrap().object().unwrap();
    let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let args = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::new(json!(amount.to_string()))?,
        SuiJsonValue::from_str(&modify_cap.object_id.to_string())?,
        SuiJsonValue::from_str(&to_address.to_string())?,
    ];
    let transaction_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "mint_stable_entry_to_address".to_string(),
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

#[allow(dead_code)]
pub async fn mint_stable_coin(amount: u64, test_cluster: &TestCluster, http_client: &HttpClient, address: SuiAddress,coint_type: &str) -> Result<(), Error> {
    let modify_cap_vec = get_owned_objects("0xc8::bfc_system_state_inner::BfcSystemModifyCap", http_client, address).await.unwrap();
    let modify_cap = modify_cap_vec.first().unwrap().object().unwrap();
    let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let args = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::new(json!(amount.to_string()))?,
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

#[allow(unused)]
pub async fn mint_stable_coin_with_gas(amount: u64, test_cluster: &TestCluster, http_client: &HttpClient, address: SuiAddress,coint_type: &str,gas_filter: &str) -> Result<(), Error> {
    let modify_cap_vec = get_owned_objects("0xc8::bfc_system_state_inner::BfcSystemModifyCap", http_client, address).await.unwrap();
    let modify_cap = modify_cap_vec.first().unwrap().object().unwrap();
    let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let args = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::new(json!(amount.to_string()))?,
        SuiJsonValue::from_str(&modify_cap.object_id.to_string())?,
    ];

    //获取 gas 对象
    let objects = get_owned_objects(gas_filter, &mut http_client.clone(), address).await?;
    let gas_object = objects.first().unwrap().object().unwrap();

    println!("mint_stable_coin_with_gas gas_object: {:?}", gas_object.to_string());

    let gas_object_id = gas_object.object_id;

    let transaction_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            BFC_SYSTEM_PACKAGE_ID,
            "bfc_system".to_string(),
            "mint_stable_entry".to_string(),
            vec![SuiTypeTag::new(coint_type.to_string())],
            args,
            Some(gas_object_id),
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

    println!("tx_response: {:#?}", tx_response);

    Ok(())
}

#[allow(dead_code)]
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