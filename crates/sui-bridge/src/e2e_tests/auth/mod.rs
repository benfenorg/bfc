use std::str::FromStr;
use std::time::Duration;
use anyhow::Error;
use jsonrpsee::http_client::HttpClient;
use shared_crypto::intent::{Intent, IntentMessage};
use sui_json_rpc_types::{SuiObjectData, SuiObjectDataFilter, SuiObjectDataOptions, SuiObjectResponse, SuiObjectResponseQuery, SuiTransactionBlockEffectsAPI};
use sui_json_rpc_types::SuiTransactionBlockResponseOptions;
use sui_sdk::json::SuiJsonValue;
use sui_types::base_types::SuiAddress;
use sui_types::crypto::{Signature, SuiKeyPair};
use sui_types::transaction::Transaction;
use sui_json_rpc_types::{SuiMoveStruct, SuiMoveValue, SuiParsedData, TransactionBlockBytes};
use test_cluster::TestCluster;
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_types::{parse_sui_struct_tag, BFC_SYSTEM_PACKAGE_ID};
use serde_json::{json, Value};
use sui_json_rpc_api::{IndexerApiClient,  WriteApiClient};
use sui_json_rpc_api::TransactionBuilderClient;
use tokio::time::sleep;
use tracing::info;

#[allow(unused)]
pub async fn auth_setup(test_cluster: &mut TestCluster, http_client: &mut HttpClient, address: SuiAddress, auth_key: &str) -> Result<(), Error> {
    
    let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let args0 = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::new(Value::Array(vec![json!(address.to_string())]))?,
    ];
    let transaction_bytes0: TransactionBlockBytes = http_client.move_call(
        address,
        BFC_SYSTEM_PACKAGE_ID,
        "bfc_system".to_string(),
        "init_admin_capability".to_string(),
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
    let tx_response = http_client
        .execute_transaction_block(
            tx_bytes0,
            signatures0,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    println!("auth_setup tx_response is {:?}",tx_response);
    assert!(tx_response.effects.unwrap().status().is_ok());
    let admin_cap_vec = get_owned_objects("0xc8::bfc_system_state_inner::BfcSystemAdminCap", http_client, address).await.unwrap();
    let admin_cap = admin_cap_vec.first().unwrap().object().unwrap();
    add_auth_key2(test_cluster, http_client, address, &&bfc_status_address, &admin_cap,auth_key).await?;
    Ok(())
}

async fn add_auth_key2(test_cluster: &TestCluster, http_client: &HttpClient, address: SuiAddress, bfc_status_address: &&SuiAddress, admin_cap: &&SuiObjectData,auth_key: &str) -> Result<(), Error> {
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
    let tx_response = http_client
        .execute_transaction_block(
            tx_bytes1,
            signatures1,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    // println!("add_auth_key tx_response is {:?}",tx_response);
    Ok(())
}


#[allow(unused)]
pub async fn auth_setup_imut(http_client: &HttpClient, address: SuiAddress, sui_key: &SuiKeyPair, auth_key: &str) -> Result<(), Error> {
    let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let args0 = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::new(Value::Array(vec![json!(address.to_string())]))?,
    ];
    let tx_data = http_client.move_call(
        address,
        BFC_SYSTEM_PACKAGE_ID,
        "bfc_system".to_string(),
        "init_admin_capability".to_string(),
        vec![],
        args0,
        None,
        10_000_00000.into(),
        None,
    )
        .await?.to_data()?;
    let sig = Signature::new_secure(
        &IntentMessage::new(Intent::sui_transaction(), &tx_data),
        sui_key,
    );
    let tx0 = Transaction::from_data(tx_data, vec![sig]);
    let (tx_bytes0, signatures0) = tx0.to_tx_bytes_and_signatures();
    let auth_result = http_client
        .execute_transaction_block(
            tx_bytes0,
            signatures0,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    // println!("auth_result: {:?}", auth_result);
    let _ = sleep(Duration::from_secs(5)).await;
    let admin_cap_vec = get_owned_objects("0xc8::bfc_system_state_inner::BfcSystemAdminCap", http_client, address).await.unwrap();
    let admin_cap = admin_cap_vec.first().unwrap().object().unwrap();
    let result = add_auth_key(http_client, address,sui_key, &&bfc_status_address, &admin_cap,auth_key).await?;

    Ok(())
}

async fn add_auth_key(http_client: &HttpClient, address: SuiAddress, sui_key: &SuiKeyPair, bfc_status_address: &&SuiAddress, admin_cap: &&SuiObjectData,auth_key: &str) -> Result<(), Error> {
    let args1 = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::from_str(&admin_cap.object_id.to_string())?,
        SuiJsonValue::new(json!(auth_key))?,
        SuiJsonValue::new(json!(address.to_string()))?,
    ];
    let tx_data = http_client
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
        .await?.to_data()?;
    let sig = Signature::new_secure(
        &IntentMessage::new(Intent::sui_transaction(), &tx_data),
        sui_key,
    );
    let tx1 = Transaction::from_data(tx_data, vec![sig]);
        // let tx1 = test_cluster
        //     .wallet
        //     .sign_transaction(&transaction_bytes1.to_data()?);
    let (tx_bytes1, signatures1) = tx1.to_tx_bytes_and_signatures();
    let tx_response = http_client
        .execute_transaction_block(
            tx_bytes1,
            signatures1,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    info!("add_auth_key tx_response is {:?}",tx_response);
    Ok(())
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

pub async fn do_get_owned_objects_with_filter(filter_tag: &str, http_client: &HttpClient, address: SuiAddress) -> Result<Vec<SuiObjectResponse>, anyhow::Error> {
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

pub fn get_balance(busd_data: &SuiObjectData) -> u64 {
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