use std::str::FromStr;
use jsonrpsee::http_client::HttpClient;
use sui_json_rpc_api::{IndexerApiClient, TransactionBuilderClient, WriteApiClient};
use sui_json_rpc_types::{SuiObjectData, SuiObjectDataFilter, SuiObjectDataOptions, SuiObjectResponse, SuiObjectResponseQuery, SuiTransactionBlockResponse, SuiTransactionBlockResponseOptions, SuiTypeTag, TransactionBlockBytes};
use sui_macros::sim_test;
use sui_sdk::json::{SuiJsonValue};
//use sui_types::transaction::CallArg;

use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::{parse_sui_struct_tag};
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use test_cluster::{TestCluster, TestClusterBuilder};
use sui_json_rpc_types::SuiTransactionBlockEffectsAPI;
//use sui_types::transaction::ProgrammableTransaction;
//use std::str::FromStr;
use sui_json_rpc_types::SuiExecutionStatus;
//use serde_json::json;
use sui_types::SUI_FRAMEWORK_PACKAGE_ID;
use move_core_types::annotated_value::MoveTypeLayout;
use tracing::info;
// use move_core_types::{
//     account_address::AccountAddress, ident_str, identifier::Identifier, language_storage::TypeTag,
// };
//use sui_types::transaction::TransactionKind;
mod publish_coin;

#[sim_test]
async fn sim_test_do_publish_anonymous_test_usd(){
    telemetry_subscribers::init_for_testing();

    println!("=====start to publish anonymous coin");
    let mut test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(6000)
        .with_num_validators(5)
        .build()
        .await;
    let http_client = test_cluster.rpc_client().clone();
    let address = test_cluster.get_address_0();
    let (package, _change_objs)
        = publish_coin::do_publish(&mut test_cluster,"tests/test_anonymous_usd").await.unwrap();

    //mint
    publish_coin::do_mint_anonymous(&mut test_cluster,package,30000000000000000).await;



    let objects = do_get_owned_objects_without_filter(&http_client, address).await.unwrap();
    for data in objects  {
        info!("=========sui object response: {:?}", data.object().unwrap().type_.as_ref().unwrap());
    }

    let filter=format!("{}{}{}","0x2::anonymous_coin::Anonymous_Coin<",package,"::testabfc::TESTABFC>");

    info!("=======the filter is {:?}", filter);
    let abfc_objects =
        do_get_owned_objects_with_filter(&*filter, &http_client, address).await.unwrap();
    assert!(abfc_objects.len() != 0);
    let testabfc_object = abfc_objects.first().unwrap().object();
    assert!(testabfc_object.is_ok());
    //panic!("the package id is {:?}", package);
}

#[sim_test]
async fn sim_test_mint_anonymous_usd() -> Result<(), anyhow::Error>{
    telemetry_subscribers::init_for_testing();
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let http_client = test_cluster.rpc_client().clone();
    let address = test_cluster.get_address_0();
    info!("the address is {:?}", address);

    let bfc_objects =
        do_get_owned_objects_with_filter("0x2::coin::Coin<0x2::bfc::BFC>", &http_client, address).await.unwrap();
    let gas = bfc_objects.get(0).unwrap().object().unwrap();
    let bfc_object = bfc_objects.get(1).unwrap().object().unwrap();
    info!("the gas object is {:?}", gas.object_id);
    info!("the bfc object is {:?}", bfc_object.object_id);


    //publish
    let (package, _change_objs)
        = publish_coin::do_publish(&mut test_cluster, "tests/test_anonymous_usd").await.unwrap();

    //mint
    let move_call_response = publish_coin::do_mint_anonymous(&mut test_cluster,package,30000000000000000).await;

    let filter=format!("{}{}{}","0x2::anonymous_coin::Anonymous_Coin<",package,"::testabfc::TESTABFC>");
    let abfc_objects =
        do_get_owned_objects_with_filter(&*filter, &http_client, address).await.unwrap();
    let testabfc_object = abfc_objects.first().unwrap().object().unwrap();
    info!("=======the abfc object is {:?}", testabfc_object.object_id);

    // public entry fun bind_swap_pool<T1, T2> (anonymous_coin: Anonymous_Coin<T1>, coin: Coin<T2>, ctx: &mut TxContext){
    // now do the call
    assert!(*move_call_response.effects.unwrap().status() ==  SuiExecutionStatus::Success);
    Ok(())
}

pub async fn do_move_call(http_client: &HttpClient, gas: &SuiObjectData,
                          address: SuiAddress, cluster: &TestCluster,
                          package_id: ObjectID, module: String,
                          function: String,
                          type_argument: Vec<SuiTypeTag>,
                          arg: Vec<SuiJsonValue>) -> Result<SuiTransactionBlockResponse, anyhow::Error> {
    let transaction_bytes: TransactionBlockBytes = http_client
        .move_call(
            address,
            package_id,
            module,
            function,
            type_argument,
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
    Ok(tx_response)
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



async fn do_get_owned_objects_without_filter(http_client: &HttpClient, address: SuiAddress) -> Result<Vec<SuiObjectResponse>, anyhow::Error> {
    let data_option = SuiObjectDataOptions::new()
        .with_type()
        .with_owner()
        .with_previous_transaction()
        .with_content();
    let objects = http_client
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new(
                None,
                Option::Some(data_option),
            )),
            None,
            None,
        )
        .await?
        .data;
    Ok(objects)
}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i+2], 16).unwrap())
        .collect()
}


