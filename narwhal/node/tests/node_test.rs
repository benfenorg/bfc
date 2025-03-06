
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