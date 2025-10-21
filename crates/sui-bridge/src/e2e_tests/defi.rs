// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use crate::e2e_tests::test_utils::BridgeTestClusterBuilder;
use std::collections::HashSet;
use tracing::{info};
use crate::e2e_tests::test_utils::{
    initiate_defi_bridge_unstake_sui_to_eth,
};
use crate::events::TokenTransferApproved;

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_defi_unstake_sui_to_eth() {
    telemetry_subscribers::init_for_testing();

    // let eth_chain_id = BridgeChainId::EthCustom as u8;
    // let sui_chain_id = BridgeChainId::SuiCustom as u8;
    let timer = std::time::Instant::now();
    let mut bridge_test_cluster = BridgeTestClusterBuilder::new()
        .with_eth_env(true)
        .with_bridge_cluster(true)
        .with_num_validators(3)
        .build()
        .await;
    info!(
        "[Timer] Bridge test cluster started in {:?}",
        timer.elapsed()
    );
    let timer = std::time::Instant::now();
    let (eth_signer, _) = bridge_test_cluster
        .get_eth_signer_and_address()
        .await
        .unwrap();

    // let sui_address = bridge_test_cluster.sui_user_address();
    let amount = 42;
    let sui_amount = amount * 100_000_000;

    initiate_defi_bridge_unstake_sui_to_eth(&bridge_test_cluster, 1, 3, 3, amount)
        .await
        .unwrap();
    let events = bridge_test_cluster
        .new_bridge_events(
            HashSet::from_iter([
                TokenTransferApproved.get().unwrap().clone(),
            ]),
            true,
        )
        .await;
    // There are exactly 1 approved and 1 claimed event
    assert_eq!(events.len(), 1);
}
