use jsonrpsee::http_client::HttpClient;
use sui_json_rpc_api::{IndexerApiClient, TransactionBuilderClient, WriteApiClient};
use sui_json_rpc_types::{SuiObjectData, SuiObjectDataFilter, SuiObjectDataOptions, SuiObjectResponse, SuiObjectResponseQuery, SuiTransactionBlockResponse, SuiTransactionBlockResponseOptions, SuiTypeTag, TransactionBlockBytes};
use sui_macros::sim_test;
use sui_sdk::json::{SuiJsonValue};
use sui_types::base_types::{ObjectID, SuiAddress};
use sui_types::{parse_sui_struct_tag};
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;
use test_cluster::{TestCluster, TestClusterBuilder};
use sui_json_rpc_types::SuiTransactionBlockEffectsAPI;
use std::str::FromStr;
use sui_json_rpc_types::SuiExecutionStatus;
use serde_json::json;
use sui_types::SUI_FRAMEWORK_PACKAGE_ID;
use tracing::info;
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
    let http_client = test_cluster.rpc_client().clone();
    let address = test_cluster.get_address_0();
    let (package, _change_objs)
        = publish_coin::do_publish(&mut test_cluster,"tests/test_anonymous_coin").await.unwrap();

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
async fn sim_test_binding_anonymous_coin() {
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
        = publish_coin::do_publish(&mut test_cluster, "tests/test_anonymous_coin").await.unwrap();

    //mint
    publish_coin::do_mint_anonymous(&mut test_cluster,package,30000000000000000).await;

    let filter=format!("{}{}{}","0x2::anonymous_coin::Anonymous_Coin<",package,"::testabfc::TESTABFC>");
    let abfc_objects =
        do_get_owned_objects_with_filter(&*filter, &http_client, address).await.unwrap();
    assert!(abfc_objects.len() != 0);
    let testabfc_object = abfc_objects.first().unwrap().object();
    assert!(testabfc_object.is_ok());

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
        SuiJsonValue::from_str(&testabfc_object.unwrap().object_id.to_string()).unwrap(),
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

    assert!(*result.effects.unwrap().status() ==  SuiExecutionStatus::Success);
}

#[sim_test]
async fn sim_test_anonymous_coin_swap_in_exceed_boundary() -> Result<(), anyhow::Error>{
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
        = publish_coin::do_publish(&mut test_cluster, "tests/test_anonymous_coin").await.unwrap();

    //mint
    publish_coin::do_mint_anonymous(&mut test_cluster,package,30000000000).await;

    let filter=format!("{}{}{}","0x2::anonymous_coin::Anonymous_Coin<",package,"::testabfc::TESTABFC>");
    let abfc_objects =
        do_get_owned_objects_with_filter(&*filter, &http_client, address).await.unwrap();
    assert!(abfc_objects.len() != 0);
    let testabfc_object = abfc_objects.first().unwrap().object();
    assert!(testabfc_object.is_ok());

    let package_id = SUI_FRAMEWORK_PACKAGE_ID;
    let module = "anonymous_coin".to_string();
    let function = "bind_swap_pool".to_string();
    let mut type_arguments = Vec::new();
    let anonymous_type = format!("{}{}", package, "::testabfc::TESTABFC");
    type_arguments.push(SuiTypeTag::new(anonymous_type.to_string()));
    type_arguments.push(SuiTypeTag::new("0x2::bfc::BFC".to_string()));

    let arg = vec![
        SuiJsonValue::from_str(&testabfc_object.unwrap().object_id.to_string()).unwrap(),
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

    assert!(*result.effects.clone().unwrap().status() ==  SuiExecutionStatus::Success);
    let swap_pool = result.effects.clone().unwrap().created().to_vec()[0].reference.object_id;
    let bfc_objects =
        do_get_owned_objects_with_filter("0x2::coin::Coin<0x2::bfc::BFC>", &http_client, address).await.unwrap();
    let gas = bfc_objects.get(0).unwrap().object().unwrap();
    let bfc_object = bfc_objects.get(1).unwrap().object().unwrap();

    // swap in out anonymous coin of boundary
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

    info!("========={:?}", result.effects.clone().unwrap());
    assert!(*result.effects.unwrap().status() !=  SuiExecutionStatus::Success);
    Ok(())
}

#[sim_test]
async fn sim_test_anonymous_coin_swap_in() -> Result<(), anyhow::Error>{
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
        = publish_coin::do_publish(&mut test_cluster, "tests/test_anonymous_coin").await.unwrap();

    //mint
    publish_coin::do_mint_anonymous(&mut test_cluster,package,30000000000000000).await;

    let filter=format!("{}{}{}","0x2::anonymous_coin::Anonymous_Coin<",package,"::testabfc::TESTABFC>");
    let abfc_objects =
        do_get_owned_objects_with_filter(&*filter, &http_client, address).await.unwrap();
    assert!(abfc_objects.len() != 0);
    let testabfc_object = abfc_objects.first().unwrap().object();
    assert!(testabfc_object.is_ok());


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
        SuiJsonValue::from_str(&testabfc_object.unwrap().object_id.to_string()).unwrap(),
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

    assert!(*result.effects.clone().unwrap().status() ==  SuiExecutionStatus::Success);
    let swap_pool = result.effects.clone().unwrap().created().to_vec()[0].reference.object_id;
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

    assert!(*result.effects.unwrap().status() ==  SuiExecutionStatus::Success);
    Ok(())
}

#[sim_test]
async fn sim_test_anonymous_coin_swap_out_with_amount() -> Result<(), anyhow::Error>{
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
        = publish_coin::do_publish(&mut test_cluster, "tests/test_anonymous_coin").await.unwrap();

    //mint
    publish_coin::do_mint_anonymous(&mut test_cluster,package,30000000000000000).await;

    let filter=format!("{}{}{}","0x2::anonymous_coin::Anonymous_Coin<",package,"::testabfc::TESTABFC>");
    let abfc_objects =
        do_get_owned_objects_with_filter(&*filter, &http_client, address).await.unwrap();
    assert!(abfc_objects.len() != 0);
    let testabfc_object = abfc_objects.first().unwrap().object();
    assert!(testabfc_object.is_ok());

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
        SuiJsonValue::from_str(&testabfc_object.unwrap().object_id.to_string()).unwrap(),
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
    assert!(*result.effects.clone().unwrap().status() ==  SuiExecutionStatus::Success);


    let swap_pool = result.effects.clone().unwrap().created().to_vec()[0].reference.object_id;

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
    assert!(*result.effects.clone().unwrap().status() ==  SuiExecutionStatus::Success);


    let anonymous_coin = result.effects.clone().unwrap().created().to_vec()[0].reference.object_id;

    let function = "swap_out_with_amount".to_string();

    let arg = vec![
        SuiJsonValue::from_str(&anonymous_coin.to_string()).unwrap(),
        SuiJsonValue::new(json!("10"))?,
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
    assert!(*result.effects.clone().unwrap().status() ==  SuiExecutionStatus::Success);

    Ok(())
}


#[sim_test]
async fn sim_test_anonymous_coin_swap_out_with_amount_exceed_boundary() -> Result<(), anyhow::Error>{
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
        = publish_coin::do_publish(&mut test_cluster, "tests/test_anonymous_coin").await.unwrap();

    //mint
    publish_coin::do_mint_anonymous(&mut test_cluster,package,30000000000000000).await;

    let filter=format!("{}{}{}","0x2::anonymous_coin::Anonymous_Coin<",package,"::testabfc::TESTABFC>");
    let abfc_objects =
        do_get_owned_objects_with_filter(&*filter, &http_client, address).await.unwrap();
    assert!(abfc_objects.len() != 0);
    let testabfc_object = abfc_objects.first().unwrap().object();
    assert!(testabfc_object.is_ok());

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
        SuiJsonValue::from_str(&testabfc_object.unwrap().object_id.to_string()).unwrap(),
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
    assert!(*result.effects.clone().unwrap().status() ==  SuiExecutionStatus::Success);


    let swap_pool = result.effects.clone().unwrap().created().to_vec()[0].reference.object_id;

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
    assert!(*result.effects.clone().unwrap().status() ==  SuiExecutionStatus::Success);


    let mint_response = publish_coin::do_mint_anonymous(&mut test_cluster,package,300000000000000000).await;

    let function = "swap_out_with_amount".to_string();

    let arg = vec![
        SuiJsonValue::from_str(&mint_response.effects.clone().unwrap().created().to_vec()[0].reference.object_id.to_string()).unwrap(),
        SuiJsonValue::new(json!("300000000000000000"))?,
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
    assert!(*result.effects.clone().unwrap().status() !=  SuiExecutionStatus::Success);
    Ok(())
}


// #[sim_test]
// async fn sim_test_anonymous_coin_restore() -> Result<(), anyhow::Error>{
//     telemetry_subscribers::init_for_testing();
//     let mut test_cluster = TestClusterBuilder::new().build().await;
//     let http_client = test_cluster.rpc_client().clone();
//     let address = test_cluster.get_address_0();
//     info!("the address is {:?}", address);
//
//     let bfc_objects =
//         do_get_owned_objects_with_filter("0x2::coin::Coin<0x2::bfc::BFC>", &http_client, address).await.unwrap();
//     let gas = bfc_objects.get(0).unwrap().object().unwrap();
//     let bfc_object = bfc_objects.get(1).unwrap().object().unwrap();
//     info!("the gas object is {:?}", gas.object_id);
//     info!("the bfc object is {:?}", bfc_object.object_id);
//
//
//     //publish
//     let (package, _change_objs)
//         = publish_coin::do_publish(&mut test_cluster, "tests/test_anonymous_coin").await.unwrap();
//
//     //mint
//     publish_coin::do_mint_anonymous(&mut test_cluster,package,30000000000000000).await;
//
//     let filter=format!("{}{}{}","0x2::anonymous_coin::Anonymous_Coin<",package,"::testabfc::TESTABFC>");
//     let abfc_objects =
//         do_get_owned_objects_with_filter(&*filter, &http_client, address).await.unwrap();
//     let testabfc_object = abfc_objects.first().unwrap().object().unwrap();
//     info!("=======the abfc object is {:?}", testabfc_object.object_id);
//
//     // public entry fun bind_swap_pool<T1, T2> (anonymous_coin: Anonymous_Coin<T1>, coin: Coin<T2>, ctx: &mut TxContext){
//     // now do the call
//     let package_id = SUI_FRAMEWORK_PACKAGE_ID;
//     let module = "anonymous_coin".to_string();
//     // let function = "bind_swap_pool".to_string();
//      let mut type_arguments = Vec::new();
//      let anonymous_type = format!("{}{}", package, "::testabfc::TESTABFC");
//     type_arguments.push(SuiTypeTag::new(anonymous_type.to_string()));
//
//     let function = "get_anonymous_value".to_string();
//
//     let signature = "cd5f94646b13eaa370a55fe9c084d6b266e1c3856c16e43fbc8b2e9a28076ffe7b73fec594974a0ff7a7ebac2cf9ad2196ff89fdd97c14c0883159ec0181730c";
//     let publickey = "8496d3d932986b43bb64b5d5c7548d5c97a73aebf4301447f3746680b2114ae1";
//
//     let signature_bytes = hex_to_bytes(signature);
//     let publickey_bytes = hex_to_bytes(publickey);
//
//
//     let arg = vec![
//         SuiJsonValue::from_str(&testabfc_object.object_id.to_string()).unwrap(),
//         SuiJsonValue::from_bcs_bytes(Some(&MoveTypeLayout::U32), &signature_bytes).unwrap(),
//         SuiJsonValue::from_bcs_bytes(Some(&MoveTypeLayout::U32), &publickey_bytes).unwrap(),
//     ];
//     let move_call_response = do_move_call(
//         &http_client,
//         &gas,
//         address,
//         &test_cluster,
//         package_id,
//         module,
//         function,
//         type_arguments,
//         arg,
//     ).await?;
//     //
//     assert!(*move_call_response.effects.unwrap().status() ==  SuiExecutionStatus::Success);
//     Ok(())
//
// }


#[sim_test]
async fn sim_test_ausd_coin_create() -> Result<(), anyhow::Error>{

    Ok(())
}
#[sim_test]
async fn sim_test_ausd_coin_swap_in() -> Result<(), anyhow::Error>{
    Ok(())
}

#[sim_test]
async fn sim_test_ausd_coin_swap_out() -> Result<(), anyhow::Error>{

    Ok(())
}
#[sim_test]
async fn sim_test_ausd_coin_restore() -> Result<(), anyhow::Error>{
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


