use sui_macros::sim_test;
use sui_sdk::json::SuiJsonValue;
use sui_types::base_types::SuiAddress;
use sui_types::{BFC_SYSTEM_PACKAGE_ID, SUI_FRAMEWORK_PACKAGE_ID};
use test_cluster::TestClusterBuilder;

#[sim_test]
async fn sim_binding_anonymous_coin() {
    //binding A_Busd
    let test_cluster = TestClusterBuilder::new().build().await;

}

#[sim_test]
async fn sim_binding_anonymous_coin_swap_in(){
    let test_cluster = TestClusterBuilder::new().build().await;
    let address = test_cluster.get_address_0();

    // now do the call
    let package_id = SUI_FRAMEWORK_PACKAGE_ID;
    let module = "bfc_system".to_string();
    let function = "create_voting_bfc".to_string();
    let bfc_status_address = SuiAddress::from_str("0x00000000000000000000000000000000000000000000000000000000000000c9").unwrap();
    let arg = vec![
        SuiJsonValue::from_str(&bfc_status_address.to_string())?,
        SuiJsonValue::from_str(&coin_obj.object_id.to_string())?,
        SuiJsonValue::from_str(&clock.to_string())?,
    ];

    do_move_call(http_client, gas, address, &cluster, package_id, module, function, arg).await?;


}

#[sim_test]
async fn sim_binding_anonymous_coin_swap_out(){
    let test_cluster = TestClusterBuilder::new().build().await;
}

#[sim_test]
async fn sim_binding_anonymous_coin_restore(){
    let test_cluster = TestClusterBuilder::new().build().await;

}