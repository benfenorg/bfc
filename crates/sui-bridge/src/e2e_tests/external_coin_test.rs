use crate::e2e_tests::test_utils::BridgeTestClusterBuilder;
use crate::sui_client::SuiBridgeClient;
use crate::test_utils::{
    approve_action_with_validator_secrets, get_test_external_bridge_action,
};
use sui_types::bridge::TOKEN_ID_USDT;
use tracing::info;

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_external_coin_usdt_deposit() {
    telemetry_subscribers::init_for_testing();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(true)
        .with_num_validators(3)
        .build()
        .await;

    let sender = bridge_test_cluster.sui_user_address();
    let bridge_object_arg = bridge_test_cluster.get_mut_bridge_arg().await.unwrap();
    
    // Get all validator keys
    let mut bridge_authority_keys = vec![];
    for i in 0..3 {
        bridge_authority_keys.push(bridge_test_cluster.bridge_authority_key(i));
    }

    // Construct the action: External deposit of USDT
    // This triggers build_external_token_bridge_approve_and_claim_transaction
    // with a non-BUSD token (USDT), hitting the desired code path.
    let action = get_test_external_bridge_action(
        None, 
        None, 
        None, 
        sender, 
        Some(TOKEN_ID_USDT)
    );

    let sui_client = bridge_test_cluster.sui_client();
    let bridge_client = SuiBridgeClient::new_for_testing(sui_client.clone());
    let id_token_map = bridge_client.get_token_id_map().await.unwrap();

    // Execute
    info!("Approving external deposit action for USDT");
    approve_action_with_validator_secrets(
        bridge_test_cluster.wallet_mut(),
        bridge_object_arg,
        action,
        &bridge_authority_keys,
        Some(sender),
        &id_token_map,
    )
    .await
    .unwrap();

    info!("USDT external deposit transaction executed successfully");
}
