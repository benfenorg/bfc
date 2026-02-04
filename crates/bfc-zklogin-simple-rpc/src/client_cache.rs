use std::sync::OnceLock;
use std::time::Duration;
use anyhow::anyhow;
use dashmap::DashMap;
use sui_sdk::{SuiClient, SuiClientBuilder};

static SUI_CLIENTS: OnceLock<DashMap<String, SuiClient>> = OnceLock::new();

fn clients_map() -> &'static DashMap<String, SuiClient> {
    SUI_CLIENTS.get_or_init(DashMap::new)
}

pub async fn get_sui_client(rpc_url: &str) -> Result<SuiClient, anyhow::Error> {
    if let Some(entry) = clients_map().get(rpc_url) {
        return Ok(entry.value().clone());
    }

    let entry = clients_map().entry(rpc_url.to_string());
    match entry {
        dashmap::mapref::entry::Entry::Occupied(e) => Ok(e.get().clone()),
        dashmap::mapref::entry::Entry::Vacant(v) => {
            let client = SuiClientBuilder::default()
                .request_timeout(Duration::from_secs(10))
                .max_concurrent_requests(64)
                .build(rpc_url)
                .await
                .map_err(|e| anyhow!("build sui client error {:?}, rpc_url={}", e, rpc_url))?;

            v.insert(client.clone());
            Ok(client)
        }
    }
}
