// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::env;
use config::Parameters;
use fastcrypto::traits::KeyPair;
use mysten_metrics::RegistryService;
use narwhal_node::execution_state::SimpleExecutionState;
use narwhal_node::primary_node::PrimaryNode;
use narwhal_node::worker_node::WorkerNodes;
use network::client::NetworkClient;
use prometheus::Registry;
use std::num::NonZeroUsize;
use std::path::{Path};
use std::time::Duration;
use storage::NodeStorage;
use test_utils::{latest_protocol_version, temp_dir, CommitteeFixture};
use tokio::sync::mpsc::channel;
use tokio::time::sleep;
use tracing::log::info;
use tracing_subscriber::fmt; // 如果要输出到控制台，还需要导入这个模块

use worker::TrivialTransactionValidator;

#[tokio::test]
async fn simple_primary_worker_node_start_stop() {
    telemetry_subscribers::init_for_testing();

    // GIVEN
    let parameters = Parameters::default();
    let registry_service = RegistryService::new(Registry::new());
    let fixture = CommitteeFixture::builder()
        .number_of_workers(NonZeroUsize::new(1).unwrap())
        .randomize_ports(true)
        .build();
    let committee = fixture.committee();
    let worker_cache = fixture.worker_cache();

    let authority = fixture.authorities().next().unwrap();
    let key_pair = authority.keypair();
    let network_key_pair = authority.network_keypair();
    let client = NetworkClient::new_from_keypair(&network_key_pair);

    let store = NodeStorage::reopen(temp_dir(), None);

    let (tx_confirmation, _rx_confirmation) = channel(10);
    let execution_state = SimpleExecutionState::new(tx_confirmation);

    // WHEN
    let primary_node = PrimaryNode::new(parameters.clone(), registry_service.clone());
    primary_node
        .start(
            key_pair.copy(),
            network_key_pair.copy(),
            committee.clone(),
            latest_protocol_version(),
            worker_cache.clone(),
            client.clone(),
            &store,
            execution_state,
        )
        .await
        .unwrap();

    // AND
    let workers = WorkerNodes::new(registry_service, parameters.clone());

    workers
        .start(
            key_pair.public().clone(),
            vec![(0, authority.worker(0).keypair().copy())],
            committee,
            latest_protocol_version(),
            worker_cache,
            client,
            &store,
            TrivialTransactionValidator,
        )
        .await
        .unwrap();

    tokio::task::yield_now().await;

    sleep(Duration::from_secs(2)).await;

    // THEN
    // unfortunately we don't have strong signal to check whether a node is up and running complete,
    // so just use the admin endpoint to check it's running
    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "http://127.0.0.1:{}/known_peers",
            parameters
                .network_admin_server
                .worker_network_admin_server_base_port
        ))
        .send()
        .await
        .unwrap();
    let result = response.text().await.unwrap();

    assert_ne!(result, "");

    // AND
    primary_node.shutdown().await;
    workers.shutdown().await;
}

#[tokio::test]
async fn primary_node_restart() {
    telemetry_subscribers::init_for_testing();

    // GIVEN
    let parameters = Parameters::default();
    let registry_service = RegistryService::new(Registry::new());
    let fixture = CommitteeFixture::builder()
        .number_of_workers(NonZeroUsize::new(1).unwrap())
        .randomize_ports(true)
        .build();
    let committee = fixture.committee();
    let worker_cache = fixture.worker_cache();

    let authority = fixture.authorities().next().unwrap();
    let key_pair = authority.keypair();
    let network_key_pair = authority.network_keypair();
    let client = NetworkClient::new_from_keypair(&network_key_pair);

    let store = NodeStorage::reopen(temp_dir(), None);

    let (tx_confirmation, _rx_confirmation) = channel(10);
    let execution_state = SimpleExecutionState::new(tx_confirmation.clone());

    // AND
    let primary_node = PrimaryNode::new(parameters.clone(), registry_service.clone());
    primary_node
        .start(
            key_pair.copy(),
            network_key_pair.copy(),
            committee.clone(),
            latest_protocol_version(),
            worker_cache.clone(),
            client.clone(),
            &store,
            execution_state,
        )
        .await
        .unwrap();

    tokio::task::yield_now().await;

    sleep(Duration::from_secs(2)).await;

    // WHEN
    primary_node.shutdown().await;

    // AND start again the node
    let execution_state = SimpleExecutionState::new(tx_confirmation.clone());
    primary_node
        .start(
            key_pair.copy(),
            network_key_pair.copy(),
            committee.clone(),
            latest_protocol_version(),
            worker_cache.clone(),
            client.clone(),
            &store,
            execution_state,
        )
        .await
        .unwrap();

    tokio::task::yield_now().await;

    sleep(Duration::from_secs(2)).await;

    // THEN can query/confirm that node is running
    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "http://127.0.0.1:{}/known_peers",
            parameters
                .network_admin_server
                .primary_network_admin_server_port
        ))
        .send()
        .await
        .unwrap();
    let result = response.text().await.unwrap();

    assert_ne!(result, "");
}



#[tokio::test]
async fn read_from_store() {
    //telemetry_subscribers::init_for_testing();
    // 初始化日志记录器
    let subscriber = fmt::Subscriber::new();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    let current_dir = env::current_dir().expect("Failed to get current directory");
    tracing::info!("the current dir is {}", current_dir.display());
    let path = Path::new("tests/data/11");
    let path2 = current_dir.join(path);
    let node_store =  NodeStorage::reopen(path2, None);

    //nodeStore.
    let last = node_store.consensus_store.get_latest_sub_dag_index();
    info!("the last proposed batch index is {}", last);

    let summary = node_store
        .batch_store.table_summary();
    tracing::info!("the summary is {},", summary.unwrap().num_keys);




}