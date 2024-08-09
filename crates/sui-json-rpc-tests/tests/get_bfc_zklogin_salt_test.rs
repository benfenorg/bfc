// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sui_sdk::SuiClient;
use test_cluster::TestClusterBuilder;

#[tokio::test]
async fn test_get_bfc_zklogin_salt() -> Result<(), anyhow::Error> {
    let cluster = TestClusterBuilder::new().build().await;
    let context = &cluster.wallet;

    let client: SuiClient = context.get_client().await.unwrap();

    let result = client
        .read_api()
        .get_bfc_zklogin_salt(String::from("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b"),
                              String::from("000102030405060708090a0b0c"),
                              String::from("f0f1f2f3f4f5f6f7f8f9"))
        .await;
    assert!(result.is_ok());

    Ok(())
}
