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
use sui_json_rpc_types::SuiTransactionBlockEffectsAPI;
use hex::FromHex;
use move_core_types::annotated_value::MoveTypeLayout;
mod publish_coin;

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

    println!("=====wubin do_publish {:?}", change_objs);


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
    //panic!("the result is {:?}", result);
}

#[sim_test]
async fn sim_test_anonymous_coin_swap_in() -> Result<(), anyhow::Error>{
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
        module.clone(),
        function,
        type_arguments.clone(),
        arg,
    ).await?;

    let swap_pool = result.effects.clone().unwrap().created().to_vec()[0].reference.object_id;
    let len = result.effects.unwrap().created().len();

    let bfc_object = bfc_objects.get(2).unwrap().object().unwrap();


    let bfc_objects =
        do_get_owned_objects_with_filter("0x2::coin::Coin<0x2::bfc::BFC>", &http_client, address).await.unwrap();
    let gas = bfc_objects.get(0).unwrap().object().unwrap();
    let bfc_object = bfc_objects.get(1).unwrap().object().unwrap();

    // now do the call
    let function = "swap_in".to_string();

    let arg = vec![
        SuiJsonValue::from_str(&bfc_object.object_id.to_string()).unwrap(),
        SuiJsonValue::from_str(&swap_pool.to_string()).unwrap(),
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
    ).await?;


    Ok(())
}

#[sim_test]
async fn sim_test_anonymous_coin_swap_out() -> Result<(), anyhow::Error>{
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
        module.clone(),
        function,
        type_arguments.clone(),
        arg,
    ).await?;

    let swap_pool = result.effects.clone().unwrap().created().to_vec()[0].reference.object_id;
    let len = result.effects.unwrap().created().len();

    let bfc_object = bfc_objects.get(2).unwrap().object().unwrap();


    let bfc_objects =
        do_get_owned_objects_with_filter("0x2::coin::Coin<0x2::bfc::BFC>", &http_client, address).await.unwrap();
    let gas = bfc_objects.get(0).unwrap().object().unwrap();
    let bfc_object = bfc_objects.get(1).unwrap().object().unwrap();

    // now do the call
    let function = "swap_in".to_string();

    let arg = vec![
        SuiJsonValue::from_str(&bfc_object.object_id.to_string()).unwrap(),
        SuiJsonValue::from_str(&swap_pool.to_string()).unwrap(),
    ];
    let result = do_move_call(
        &http_client,
        &gas,
        address,
        &test_cluster,
        package_id,
        module.clone(),
        function,
        type_arguments.clone(),
        arg,
    ).await?;

    let anonymous_coin = result.effects.clone().unwrap().created().to_vec()[0].reference.object_id;

    let function = "swap_out".to_string();

    let publickey = "8496d3d932986b43bb64b5d5c7548d5c97a73aebf4301447f3746680b2114ae1";
    let signature = "cd5f94646b13eaa370a55fe9c084d6b266e1c3856c16e43fbc8b2e9a28076ffe7b73fec594974a0ff7a7ebac2cf9ad2196ff89fdd97c14c0883159ec0181730c";

    let bcs_bytes_signature =  hex_to_bytes(signature);
    let bcs_bytes_public_key = hex_to_bytes(publickey);

    let signature = SuiJsonValue::from_bcs_bytes(Some(&MoveTypeLayout::Signer), &bcs_bytes_signature).unwrap();
    let public_key = SuiJsonValue::from_bcs_bytes(Some(&MoveTypeLayout::U32), &bcs_bytes_public_key).unwrap();
    let object_id = SuiAddress::from_str("0x33a2598b7c5e22d03967b42671926c5c18e82f0be9973c077041e9912696f910").unwrap();

    let arg = vec![
        SuiJsonValue::from_str(&anonymous_coin.to_string()).unwrap(),
        SuiJsonValue::from_str(&swap_pool.to_string()).unwrap(),
        signature,
        SuiJsonValue::new(json!(object_id.to_string()))?,
        public_key,
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
    ).await?;

    println!("=====wubin result {:?}", result);
    panic!("the package id is {:?}", package);
    Ok(())
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

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i+2], 16).unwrap())
        .collect()
}


