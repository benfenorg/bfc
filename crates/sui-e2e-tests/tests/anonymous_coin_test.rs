use std::fmt::format;
use jsonrpsee::http_client::HttpClient;
use serde_json::json;
use tracing::info;
use sui_json_rpc_api::{IndexerApiClient, TransactionBuilderClient, WriteApiClient};
use sui_json_rpc_types::{SuiObjectData, SuiObjectDataFilter, SuiObjectDataOptions, SuiObjectResponse, SuiObjectResponseQuery, SuiTransactionBlockResponse, SuiTransactionBlockResponseOptions, SuiTypeTag, TransactionBlockBytes};
use sui_macros::sim_test;
use sui_sdk::json::{type_args, SuiJsonValue};
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::{parse_sui_struct_tag, BFC_SYSTEM_PACKAGE_ID, SUI_FRAMEWORK_PACKAGE_ID};
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use test_cluster::{TestCluster, TestClusterBuilder};
use std::str::FromStr;
mod publish_coin;

async fn basic_do_publish_anonymous_test_coin(){

}
#[sim_test]
async fn sim_test_do_publish_anonymous_test_coin(){
    telemetry_subscribers::init_for_testing();

    println!("=====start to publish anonymous coin");
    let mut test_cluster = TestClusterBuilder::new()
        .with_epoch_duration_ms(6000)
        .with_num_validators(5)
        .build()
        .await;
    let mut http_client = test_cluster.rpc_client().clone();
    let address = test_cluster.get_address_0();
    let (package, change_objs)
        = publish_coin::do_publish(&mut test_cluster,"tests/test_anonymous_coin").await.unwrap();



    //mint
    publish_coin::do_mint_anonymous(&mut test_cluster,package).await;



    let objects = do_get_owned_objects_without_filter(&http_client, address).await.unwrap();
    for data in objects  {
        info!("=========sui object response: {:?}", data.object().unwrap().type_.as_ref().unwrap());
    }

    let filter=format!("{}{}{}","0x2::anonymous_coin::Anonymous_Coin<",package,"::testabfc::TESTABFC>");

    info!("=======the filter is {:?}", filter);
    let abfc_objects =
        do_get_owned_objects_with_filter(&*filter, &http_client, address).await.unwrap();
    let testabfc_object = abfc_objects.first().unwrap().object().unwrap();
    info!("=======the abfc object is {:?}", testabfc_object.object_id);


    info!("=====the package id is {:?}", package);
    //panic!("the package id is {:?}", package);

}

#[sim_test]
async fn sim_test_binding_anonymous_coin() {
    telemetry_subscribers::init_for_testing();
    let mut test_cluster = TestClusterBuilder::new().build().await;
    let mut http_client = test_cluster.rpc_client().clone();
    let address = test_cluster.get_address_0();
    info!("the address is {:?}", address);

    let bfc_objects =
        do_get_owned_objects_with_filter("0x2::coin::Coin<0x2::bfc::BFC>", &http_client, address).await.unwrap();
    let gas = bfc_objects.get(0).unwrap().object().unwrap();
    let bfc_object = bfc_objects.get(1).unwrap().object().unwrap();
    info!("the gas object is {:?}", gas.object_id);
    info!("the bfc object is {:?}", bfc_object.object_id);


    //publish
    let (package, change_objs)
        = publish_coin::do_publish(&mut test_cluster, "tests/test_anonymous_coin").await.unwrap();

    //mint
    publish_coin::do_mint_anonymous(&mut test_cluster,package).await;

    let filter=format!("{}{}{}","0x2::anonymous_coin::Anonymous_Coin<",package,"::testabfc::TESTABFC>");
    let abfc_objects =
        do_get_owned_objects_with_filter(&*filter, &http_client, address).await.unwrap();
    let testabfc_object = abfc_objects.first().unwrap().object().unwrap();
    info!("=======the abfc object is {:?}", testabfc_object.object_id);

    // public entry fun bind_swap_pool<T1, T2> (anonymous_coin: Anonymous_Coin<T1>, coin: Coin<T2>, ctx: &mut TxContext){
    // now do the call
    let package_id = SUI_FRAMEWORK_PACKAGE_ID;
    let module = "anonymous_coin".to_string();
    let function = "bind_swap_pool".to_string();
    let mut type_arguments = Vec::new();
    let anonymous_type = format!("{}{}", package, "::testabfc::TESTABFC");
    type_arguments.push(SuiTypeTag::new(anonymous_type.to_string()));
    type_arguments.push(SuiTypeTag::new("0x2::bfc::BFC".to_string()));


    let arg = vec![
        SuiJsonValue::from_str(&testabfc_object.object_id.to_string()).unwrap(),
        SuiJsonValue::from_str(&bfc_object.object_id.to_string()).unwrap(),
    ];
    let result = do_move_call(
        &http_client,
        &gas,
        address,
        &test_cluster,
        package_id,
        module,
        function,
        type_arguments,
        arg,
    ).await.unwrap();

    info!("the result is {:?}", result);
    panic!("the result is {:?}", result);
}

#[sim_test]
async fn sim_test_anonymous_coin_swap_in(){
    let test_cluster = TestClusterBuilder::new().build().await;
    let address = test_cluster.get_address_0();

    // now do the call
    let package_id = SUI_FRAMEWORK_PACKAGE_ID;
    let module = "anonymous_coin".to_string();
    let function = "swap_in".to_string();



}

#[sim_test]
async fn sim_test_anonymous_coin_swap_out(){
    let test_cluster = TestClusterBuilder::new().build().await;
}

#[sim_test]
async fn sim_test_anonymous_coin_restore(){
    let test_cluster = TestClusterBuilder::new().build().await;

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



