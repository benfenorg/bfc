use std::str::FromStr;
use std::time::Duration;
use anyhow::Error;
use jsonrpsee::http_client::HttpClient;
use sui_json_rpc_types::{SuiObjectData, SuiObjectDataFilter, SuiObjectDataOptions, SuiObjectResponse, SuiObjectResponseQuery};
use sui_json_rpc_types::{SuiTransactionBlockResponseOptions, TransactionBlockBytes};
use sui_sdk::json::SuiJsonValue;
use sui_types::base_types::SuiAddress;
use test_cluster::TestCluster;
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use sui_types::{parse_sui_struct_tag, BFC_SYSTEM_PACKAGE_ID};
use serde_json::json;
use sui_json_rpc_api::{IndexerApiClient,  WriteApiClient};
use sui_json_rpc_api::TransactionBuilderClient;
use tokio::time::sleep;

pub async fn auth_setup(test_cluster: &mut TestCluster, http_client: &mut HttpClient, address: SuiAddress, auth_key: &str) -> Result<(), Error> {
    
    let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
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
    add_auth_key(test_cluster, http_client, address, &&bfc_status_address, &admin_cap,auth_key).await?;
    Ok(())
}

pub async fn auth_setup_imut(test_cluster: &TestCluster, http_client: &HttpClient, address: SuiAddress, auth_key: &str) -> Result<(), Error> {
    
    let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
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
    let auth_result = http_client
        .execute_transaction_block(
            tx_bytes0,
            signatures0,
            Some(SuiTransactionBlockResponseOptions::new().with_effects()),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    println!("auth_result: {:?}", auth_result);
    let _ = sleep(Duration::from_secs(5)).await;
    let admin_cap_vec = get_owned_objects("0xc8::bfc_system_state_inner::BfcSystemAdminCap", http_client, address).await.unwrap();
    let admin_cap = admin_cap_vec.first().unwrap().object().unwrap();
    add_auth_key(test_cluster, http_client, address, &&bfc_status_address, &admin_cap,auth_key).await?;
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