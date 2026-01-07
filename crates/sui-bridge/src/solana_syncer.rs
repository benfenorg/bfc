use crate::error::BridgeResult;
use crate::metrics::BridgeMetrics;
use crate::retry_with_max_elapsed_time;
use crate::solana_client::{GetSignaturesConfig, SignatureInfo, SolanaClient};
use crate::solana_events::SolanaBridgeEvent;
use mysten_metrics::metered_channel::Sender;
use mysten_metrics::spawn_logged_monitored_task;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tokio::time::{self, Duration, Instant};
use tracing::{error, info, warn};

/// Maximum number of signatures to fetch per query
const SOLANA_SIGNATURE_QUERY_MAX_LIMIT: u64 = 1000;
/// Maximum retry duration for RPC calls
const RPC_RETRY_MAX_DURATION: Duration = Duration::from_secs(600);

pub struct SolanaSyncer {
    sol_client: Arc<SolanaClient>,
    target_addresses: SolanaTargetAddresses,
    event_tx: Sender<(SolanaSyncerCursorsKey, SolanaEventWrapper)>,
}

/// Configuration for a Solana address to watch
#[derive(Clone, Debug)]
pub struct SolanaTargetAddressInfo {
    /// The starting slot for initial sync (used as a lower bound filter)
    pub start_slot: u64,
    /// The last processed signature (used as cursor for pagination)
    /// When set, only signatures newer than this will be fetched
    pub until_signature: Option<String>,
}

/// Map from contract address to their start info
pub type SolanaTargetAddresses = HashMap<String, SolanaTargetAddressInfo>;
/// Key for cursor storage - the contract address being watched
pub type SolanaSyncerCursorsKey = String;

#[allow(clippy::new_without_default)]
impl SolanaSyncer {
    pub fn new(
        sol_client: Arc<SolanaClient>,
        target_addresses: SolanaTargetAddresses,
        event_tx: Sender<(SolanaSyncerCursorsKey, SolanaEventWrapper)>,
    ) -> Self {
        Self {
            sol_client,
            target_addresses,
            event_tx,
        }
    }

    pub async fn run(
        self,
        metrics: Arc<BridgeMetrics>,
        finalized_slot_query_interval: Duration,
    ) -> BridgeResult<(Vec<JoinHandle<()>>, watch::Receiver<u64>)> {
        // Get initial finalized slot
        let last_finalized_slot = self.sol_client.get_slot(Some("finalized")).await?;
        let (last_finalized_slot_tx, last_finalized_slot_rx) =
            watch::channel(last_finalized_slot);

        let mut task_handles = vec![];

        // Spawn finalized slot refresh task
        let sol_client_clone = self.sol_client.clone();
        let metrics_clone = metrics.clone();
        let finalized_slot_query_interval_clone = finalized_slot_query_interval;
        task_handles.push(spawn_logged_monitored_task!(
            Self::run_finalized_slot_refresh_task(
                last_finalized_slot_tx,
                sol_client_clone,
                metrics_clone,
                finalized_slot_query_interval_clone,
            )
        ));

        // Spawn event listening task for each address
        for (address, target_info) in self.target_addresses {
            let events_tx_clone = self.event_tx.clone();
            let last_finalized_slot_rx_clone = last_finalized_slot_rx.clone();
            let sol_client_clone = self.sol_client.clone();
            let metrics_clone = metrics.clone();
            task_handles.push(spawn_logged_monitored_task!(
                Self::run_signature_listening_task(
                    address,
                    target_info,
                    last_finalized_slot_rx_clone,
                    events_tx_clone,
                    sol_client_clone,
                    metrics_clone,
                )
            ));
        }

        Ok((task_handles, last_finalized_slot_rx))
    }

    /// Task that periodically refreshes the last finalized slot
    async fn run_finalized_slot_refresh_task(
        last_finalized_slot_sender: watch::Sender<u64>,
        sol_client: Arc<SolanaClient>,
        metrics: Arc<BridgeMetrics>,
        finalized_slot_query_interval: Duration,
    ) {
        info!("[SolanaSyncer] Starting Solana finalized slot refresh task");
        let mut last_slot = 0u64;
        let mut interval = time::interval(finalized_slot_query_interval);
        interval.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

        loop {
            interval.tick().await;

            let Ok(Ok(new_slot)) = retry_with_max_elapsed_time!(
                sol_client.get_slot(Some("finalized")),
                RPC_RETRY_MAX_DURATION
            ) else {
                error!("[SolanaSyncer] Failed to get finalized slot from Solana client after retry");
                continue;
            };

            metrics.last_finalized_solana_slot.set(new_slot as i64);

            if new_slot > last_slot {
                last_finalized_slot_sender
                    .send(new_slot)
                    .expect("last_finalized_slot channel receiver is closed");
                info!(
                    old_slot = last_slot,
                    new_slot = new_slot,
                    "[SolanaSyncer] Observed new finalized Solana slot"
                );
                last_slot = new_slot;
            }
        }
    }


    /// Main task that listens for new signatures on a Solana address
    ///
    /// This task:
    /// 1. Waits for new finalized slots before querying
    /// 2. Polls for new signatures using `getSignaturesForAddress` API
    /// 3. Uses `until` parameter to only fetch signatures newer than the last processed one
    /// 4. Filters by `start_slot` to ensure we don't process old transactions
    /// 5. Fetches full transaction details and parses bridge events
    /// 6. Sends events in chronological order (oldest to newest)
    async fn run_signature_listening_task(
        address: String,
        mut target_info: SolanaTargetAddressInfo,
        mut last_finalized_slot_rx: watch::Receiver<u64>,
        events_sender: mysten_metrics::metered_channel::Sender<(
            SolanaSyncerCursorsKey,
            SolanaEventWrapper,
        )>,
        sol_client: Arc<SolanaClient>,
        metrics: Arc<BridgeMetrics>,
    ) {
        let commitment = "finalized".to_string();
        let address_str = address.clone();

        info!(
            address = %address_str,
            start_slot = target_info.start_slot,
            until_signature = ?target_info.until_signature,
            "[SolanaSyncer] Starting Solana signature listening task"
        );

        loop {
            // Wait for new finalized slot
            last_finalized_slot_rx
                .changed()
                .await
                .expect("last_finalized_slot channel sender is closed");

            let current_finalized_slot = *last_finalized_slot_rx.borrow();

            // Skip if finalized slot is before our start slot
            if current_finalized_slot < target_info.start_slot {
                info!(
                    address = %address_str,
                    finalized_slot = current_finalized_slot,
                    start_slot = target_info.start_slot,
                    "[SolanaSyncer] Finalized slot is before start slot, skipping"
                );
                continue;
            }

            let timer = Instant::now();

            // Collect signatures for this polling window
            let (collected, window_ok) = Self::collect_signatures(
                &sol_client,
                &address_str,
                &target_info,
                current_finalized_slot,
                &commitment,
            )
            .await;

            if !window_ok {
                error!(
                    address = %address_str,
                    "[SolanaSyncer] Failed to collect signatures for Solana address, retrying in next window"
                );

                continue;
            }

            // If no new signatures, continue waiting
            if collected.is_empty() {
                info!(
                    address = %address_str,
                    "[SolanaSyncer] No new Solana signatures observed, continuing"
                );

                continue;
            }

            // Fetch transactions and parse events
            let (parsed_events, fetch_ok) =
                Self::fetch_transactions_and_parse_events(&sol_client, &collected).await;

            if !fetch_ok {
                // Don't advance cursor on failure
                error!(
                    address = %address_str,
                    "[SolanaSyncer] Failed to fetch transactions or parse events, retrying in next window"
                );

                continue;
            }

            info!(
                address = %address_str,
                signature_count = collected.len(),
                "[SolanaSyncer] Querying Solana signatures took {:?}",
                timer.elapsed()
            );

            // After collect_signatures returns data in chronological order (oldest first),
            // the last element is the newest
            let newest_signature = collected.last().map(|s| s.signature.clone());
            let newest_slot = collected.last().map(|s| s.slot);
            let tx_count = collected.len();
            let event_count = parsed_events.len();

            // Update cursor for next iteration
            if let Some(ref sig) = newest_signature {
                target_info.until_signature = Some(sig.clone());
            }

            // Also update start_slot to the newest slot for resilience
            // This helps if the until_signature is pruned from RPC history
            if let Some(slot) = newest_slot {
                target_info.start_slot = slot;

                metrics
                    .last_synced_solana_slot
                    .with_label_values(&[&address_str])
                    .set(slot as i64);
            }

            if tx_count != 0 {
                info!(
                    address = %address_str,
                    tx_count = tx_count,
                    event_count = event_count,
                    newest_signature = ?newest_signature,
                    newest_slot = ?newest_slot,
                    "[SolanaSyncer] Observed {} new Solana transactions",
                    tx_count
                );
            }

            let wrapper = SolanaEventWrapper {
                parsed_events,
                newest_signature,
                newest_slot,
            };

            events_sender
                .send((address.clone(), wrapper))
                .await
                .expect("All Solana event channel receivers are closed");
        }
    }

    /// Collect signatures from Solana RPC with pagination
    ///
    /// Returns signatures in chronological order (oldest to newest)
    async fn collect_signatures(
        sol_client: &Arc<SolanaClient>,
        address: &str,
        target_info: &SolanaTargetAddressInfo,
        _current_finalized_slot: u64,
        commitment: &str,
    ) -> (Vec<SignatureInfo>, bool) {
        let mut before: Option<String> = None;
        let until: Option<String> = target_info.until_signature.clone();
        let mut collected: Vec<SignatureInfo> = Vec::new();

        // Paginate through signatures (API returns newest to oldest)
        loop {
            let cfg = GetSignaturesConfig {
                commitment: Some(commitment.to_string()),
                min_context_slot: Some(target_info.start_slot),
                limit: Some(SOLANA_SIGNATURE_QUERY_MAX_LIMIT),
                before: before.clone(),
                until: until.clone(),
                ..Default::default()
            };

            let Ok(Ok(page)) = retry_with_max_elapsed_time!(
                sol_client.get_signatures_for_address(address, Some(cfg.clone())),
                RPC_RETRY_MAX_DURATION
            ) else {
                error!(
                    address = %address,
                    "[SolanaSyncer] Failed to get signatures for address from Solana client"
                );
                return (collected, false);
            };

            if page.is_empty() {
                break;
            }

            // Filter by start_slot and exclude failed transactions
            let filtered: Vec<SignatureInfo> = page
                .iter()
                .cloned()
                .filter(|s| {
                    // Note: min_context_slot in the API is just a hint for RPC context,
                    // it doesn't filter results. We must filter manually.
                    if s.slot < target_info.start_slot {
                        warn!(
                            signature = %s.signature,
                            slot = s.slot,
                            start_slot = target_info.start_slot,
                            "[SolanaSyncer] Skipping Solana transaction before start_slot"
                        );

                        return false;
                    }
                    // Skip failed transactions
                    if s.err.is_some() {
                        warn!(
                            signature = %s.signature,
                            slot = s.slot,
                            err = ?s.err,
                            "[SolanaSyncer] Skipping failed Solana transaction"
                        );
                        return false;
                    }

                    info!(
                        signature = %s.signature,
                        slot = s.slot,
                        "[SolanaSyncer] Collected Solana transaction signature"
                    );

                    true
                })
                .collect();

            collected.extend(filtered);
            before = page.last().map(|s| s.signature.clone());

            if page.len() < SOLANA_SIGNATURE_QUERY_MAX_LIMIT as usize {
                break;
            }
        }

        // Remove the until_signature itself (it was already processed)
        if let Some(until_sig) = until.as_ref() {
            collected.retain(|s| s.signature != *until_sig);
        }

        // Reverse to process from oldest to newest (chronological order)
        // getSignaturesForAddress returns newest -> oldest, but bridge needs chronological order
        collected.reverse();

        (collected, true)
    }

    /// Fetch full transaction details and parse bridge events
    ///
    /// Returns (parsed_events_with_signatures, success)
    async fn fetch_transactions_and_parse_events(
        sol_client: &Arc<SolanaClient>,
        signatures: &[SignatureInfo],
    ) -> (Vec<SolanaParsedEvent>, bool) {
        let mut parsed_events: Vec<SolanaParsedEvent> = Vec::new();

        for sig in signatures {
            let Ok(Ok(tx)) = retry_with_max_elapsed_time!(
                sol_client.get_transaction(&sig.signature),
                RPC_RETRY_MAX_DURATION
            ) else {
                error!(
                    signature = %sig.signature,
                    slot = sig.slot,
                    "[SolanaSyncer] Failed to get transaction details from Solana client"
                );
                // Return what we have so far as failed - don't advance cursor
                return (parsed_events, false);
            };

            // Parse bridge events from transaction logs
            let events = SolanaBridgeEvent::try_from_client_transaction(&tx);
            if !events.is_empty() {
                info!(
                    signature = %sig.signature,
                    slot = sig.slot,
                    event_count = events.len(),
                    "[SolanaSyncer] Parsed bridge events from Solana transaction"
                );
                // Attach transaction signature to each event
                for event in events {
                    info!(
                        signature = %sig.signature,
                        slot = sig.slot,
                        event = ?event,
                        "[SolanaSyncer] Parsed Solana bridge event"
                    );

                    parsed_events.push(SolanaParsedEvent {
                        tx_signature: sig.signature.clone(),
                        event,
                    });
                }
            } else {
                info!(
                    signature = %sig.signature,
                    slot = sig.slot,
                    "[SolanaSyncer] No bridge events found in Solana transaction"
                );
            }
        }

        (parsed_events, true)
    }
}

/// Parsed event with its transaction signature
#[derive(Debug, Clone)]
pub struct SolanaParsedEvent {
    /// The transaction signature this event belongs to
    pub tx_signature: String,
    /// The parsed bridge event
    pub event: SolanaBridgeEvent,
}

/// Wrapper for Solana events to be sent through the channel
#[derive(Debug, Clone)]
pub struct SolanaEventWrapper {
    /// Parsed bridge events with their transaction signatures
    pub parsed_events: Vec<SolanaParsedEvent>,
    /// The newest signature in this batch (used as cursor for next query)
    pub newest_signature: Option<String>,
    /// The newest slot in this batch (used for metrics and fallback cursor)
    pub newest_slot: Option<u64>,
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::routing::post;
    use axum::{Json, Router};
    use prometheus::Registry;
    use std::net::SocketAddr;
    use std::sync::atomic::{AtomicU64, Ordering};
    use tokio::sync::oneshot;

    #[derive(Clone)]
    struct MockState {
        slot: Arc<AtomicU64>,
    }

    async fn handler(
        State(state): State<MockState>,
        Json(body): Json<serde_json::Value>,
    ) -> (StatusCode, Json<serde_json::Value>) {
        let method = body
            .get("method")
            .and_then(|m| m.as_str())
            .unwrap_or_default();
        match method {
            "getSlot" => {
                let slot = state.slot.load(Ordering::SeqCst);
                let rsp = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "result": slot });
                (StatusCode::OK, Json(rsp))
            }
            "getSignaturesForAddress" => {
                let rsp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": [
                        {
                            "signature": "sig1",
                            "slot": 12340,
                            "err": null,
                            "memo": null,
                            "blockTime": 1700000000,
                            "confirmationStatus": "finalized"
                        }
                    ]
                });
                (StatusCode::OK, Json(rsp))
            }
            "getTransaction" => {
                let rsp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "slot": 12340,
                        "transaction": {
                            "signatures": ["sig1"],
                            "message": { "accountKeys": ["11111111111111111111111111111111"] }
                        },
                        "meta": {
                            "err": null,
                            "fee": 5000,
                            "preBalances": [0],
                            "postBalances": [0],
                            "preTokenBalances": [],
                            "postTokenBalances": []
                        }
                    }
                });
                (StatusCode::OK, Json(rsp))
            }
            _ => {
                let rsp = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "error": { "code": -32601, "message": "Method not found" }});
                (StatusCode::OK, Json(rsp))
            }
        }
    }

    async fn start_mock_server(
        initial_slot: u64,
    ) -> (
        SocketAddr,
        tokio::task::JoinHandle<()>,
        oneshot::Sender<()>,
        Arc<AtomicU64>,
    ) {
        let slot = Arc::new(AtomicU64::new(initial_slot));
        let state = MockState {
            slot: slot.clone(),
        };
        let app = Router::new().route("/", post(handler)).with_state(state);
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        let local_addr = listener.local_addr().unwrap();
        let (tx, rx) = oneshot::channel();
        let handle = tokio::spawn(async move {
            let server = axum::serve(listener, app);
            tokio::select! {
                _ = server => {},
                _ = rx => {},
            }
        });
        (local_addr, handle, tx, slot)
    }

    async fn handler_until_inclusive(
        State(state): State<MockState>,
        Json(body): Json<serde_json::Value>,
    ) -> (StatusCode, Json<serde_json::Value>) {
        let method = body
            .get("method")
            .and_then(|m| m.as_str())
            .unwrap_or_default();
        match method {
            "getSlot" => {
                let slot = state.slot.load(Ordering::SeqCst);
                let rsp = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "result": slot });
                (StatusCode::OK, Json(rsp))
            }
            "getSignaturesForAddress" => {
                let until = body
                    .get("params")
                    .and_then(|p| p.as_array())
                    .and_then(|p| p.get(1))
                    .and_then(|cfg| cfg.get("until"))
                    .and_then(|u| u.as_str())
                    .unwrap_or_default();

                if until != "sig_cursor" {
                    let rsp = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "result": [] });
                    return (StatusCode::OK, Json(rsp));
                }

                let rsp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": [
                        {
                            "signature": "sig_new",
                            "slot": 12340,
                            "err": null,
                            "memo": null,
                            "blockTime": 1700000002,
                            "confirmationStatus": "finalized"
                        },
                        {
                            "signature": "sig_cursor",
                            "slot": 12330,
                            "err": null,
                            "memo": null,
                            "blockTime": 1700000001,
                            "confirmationStatus": "finalized"
                        },
                        {
                            "signature": "sig_old",
                            "slot": 12000,
                            "err": null,
                            "memo": null,
                            "blockTime": 1600000000,
                            "confirmationStatus": "finalized"
                        }
                    ]
                });
                (StatusCode::OK, Json(rsp))
            }
            "getTransaction" => {
                let sig = body
                    .get("params")
                    .and_then(|p| p.as_array())
                    .and_then(|p| p.get(0))
                    .and_then(|s| s.as_str())
                    .unwrap_or_default();

                if sig == "sig_cursor" || sig == "sig_old" {
                    let rsp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": 1,
                        "error": { "code": -32601, "message": "unexpected getTransaction for filtered signature" }
                    });
                    return (StatusCode::OK, Json(rsp));
                }

                let rsp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "slot": 12340,
                        "transaction": {
                            "signatures": [sig],
                            "message": { "accountKeys": ["11111111111111111111111111111111"] }
                        },
                        "meta": {
                            "err": null,
                            "fee": 5000,
                            "preBalances": [0],
                            "postBalances": [0],
                            "preTokenBalances": [],
                            "postTokenBalances": [],
                            "logMessages": ["Program log: test"]
                        }
                    }
                });
                (StatusCode::OK, Json(rsp))
            }
            _ => {
                let rsp = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "error": { "code": -32601, "message": "Method not found" }});
                (StatusCode::OK, Json(rsp))
            }
        }
    }

    async fn start_mock_server_until_inclusive(
        initial_slot: u64,
    ) -> (
        SocketAddr,
        tokio::task::JoinHandle<()>,
        oneshot::Sender<()>,
        Arc<AtomicU64>,
    ) {
        let slot = Arc::new(AtomicU64::new(initial_slot));
        let state = MockState {
            slot: slot.clone(),
        };
        let app = Router::new()
            .route("/", post(handler_until_inclusive))
            .with_state(state);
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        let local_addr = listener.local_addr().unwrap();
        let (tx, rx) = oneshot::channel();
        let handle = tokio::spawn(async move {
            let server = axum::serve(listener, app);
            tokio::select! {
                _ = server => {},
                _ = rx => {},
            }
        });
        (local_addr, handle, tx, slot)
    }

    async fn handler_tx_err(
        State(state): State<MockState>,
        Json(body): Json<serde_json::Value>,
    ) -> (StatusCode, Json<serde_json::Value>) {
        let method = body
            .get("method")
            .and_then(|m| m.as_str())
            .unwrap_or_default();
        match method {
            "getSlot" => {
                let slot = state.slot.load(Ordering::SeqCst);
                let rsp = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "result": slot });
                (StatusCode::OK, Json(rsp))
            }
            "getSignaturesForAddress" => {
                let rsp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": [
                        {
                            "signature": "sig1",
                            "slot": 12340,
                            "err": null,
                            "memo": null,
                            "blockTime": 1700000000,
                            "confirmationStatus": "finalized"
                        }
                    ]
                });
                (StatusCode::OK, Json(rsp))
            }
            "getTransaction" => {
                let rsp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "error": { "code": -32601, "message": "Method not found" }
                });
                (StatusCode::OK, Json(rsp))
            }
            _ => {
                let rsp = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "error": { "code": -32601, "message": "Method not found" }});
                (StatusCode::OK, Json(rsp))
            }
        }
    }

    async fn start_mock_server_tx_err(
        initial_slot: u64,
    ) -> (
        SocketAddr,
        tokio::task::JoinHandle<()>,
        oneshot::Sender<()>,
        Arc<AtomicU64>,
    ) {
        let slot = Arc::new(AtomicU64::new(initial_slot));
        let state = MockState {
            slot: slot.clone(),
        };
        let app = Router::new()
            .route("/", post(handler_tx_err))
            .with_state(state);
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        let local_addr = listener.local_addr().unwrap();
        let (tx, rx) = oneshot::channel();
        let handle = tokio::spawn(async move {
            let server = axum::serve(listener, app);
            tokio::select! {
                _ = server => {},
                _ = rx => {},
            }
        });
        (local_addr, handle, tx, slot)
    }

    #[tokio::test]
    async fn test_solana_syncer_basic() {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mysten_metrics::init_metrics(&registry);

        let (addr, handle, tx, slot_control) = start_mock_server(12345).await;
        let base_url = format!("http://{addr}/");
        let client = Arc::new(SolanaClient::new(&base_url));

        let addresses = HashMap::from_iter(vec![(
            "Vote111111111111111111111111111111111111111".to_string(),
            SolanaTargetAddressInfo {
                start_slot: 12300u64,
                until_signature: None,
            },
        )]);

        const SOLANA_EVENTS_CHANNEL_SIZE: usize = 1000;
        let (sol_events_tx, mut rx) = mysten_metrics::metered_channel::channel(
            SOLANA_EVENTS_CHANNEL_SIZE,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["solana_events_queue"]),
        );

        let (_handles, mut finalized_slot_rx) = SolanaSyncer::new(client.clone(), addresses, sol_events_tx)
            .run(
                Arc::new(BridgeMetrics::new_for_testing()),
                Duration::from_millis(50),
            )
            .await
            .unwrap();

        // Initial finalized slot
        assert_eq!(*finalized_slot_rx.borrow(), 12345);

        // Trigger slot update to start processing
        slot_control.store(12346, Ordering::SeqCst);
        finalized_slot_rx.changed().await.unwrap();
        assert_eq!(*finalized_slot_rx.borrow(), 12346);

        let (_addr, wrapper) = rx.recv().await.unwrap();
        // No bridge events parsed from mock transaction, but we should receive the wrapper
        assert_eq!(wrapper.parsed_events.len(), 0);
        assert_eq!(wrapper.newest_slot, Some(12340));

        tx.send(()).ok();
        handle.abort();
    }

    #[tokio::test]
    async fn test_solana_syncer_tx_failure_no_cursor_advance() {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mysten_metrics::init_metrics(&registry);

        let (addr, handle, tx, slot_control) = start_mock_server_tx_err(12345).await;
        let base_url = format!("http://{addr}/");
        let client = Arc::new(SolanaClient::new(&base_url));

        let addresses = HashMap::from_iter(vec![(
            "Vote111111111111111111111111111111111111111".to_string(),
            SolanaTargetAddressInfo {
                start_slot: 12300u64,
                until_signature: None,
            },
        )]);

        const SOLANA_EVENTS_CHANNEL_SIZE: usize = 1000;
        let (sol_events_tx, mut rx) = mysten_metrics::metered_channel::channel(
            SOLANA_EVENTS_CHANNEL_SIZE,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["solana_events_queue"]),
        );

        let (_handles, _finalized_slot_rx) =
            SolanaSyncer::new(client.clone(), addresses, sol_events_tx)
                .run(
                    Arc::new(BridgeMetrics::new_for_testing()),
                    Duration::from_millis(50),
                )
                .await
                .unwrap();

        // Trigger slot update
        slot_control.store(12346, Ordering::SeqCst);

        let result = tokio::time::timeout(std::time::Duration::from_millis(500), rx.recv()).await;
        assert!(result.is_err());

        tx.send(()).ok();
        handle.abort();
    }

    #[tokio::test]
    async fn test_solana_syncer_dedupes_until_and_applies_start_slot() {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mysten_metrics::init_metrics(&registry);

        let (addr, handle, tx, slot_control) = start_mock_server_until_inclusive(12345).await;
        let base_url = format!("http://{addr}/");
        let client = Arc::new(SolanaClient::new(&base_url));

        let addresses = HashMap::from_iter(vec![(
            "Vote111111111111111111111111111111111111111".to_string(),
            SolanaTargetAddressInfo {
                start_slot: 12320u64,
                until_signature: Some("sig_cursor".to_string()),
            },
        )]);

        const SOLANA_EVENTS_CHANNEL_SIZE: usize = 1000;
        let (sol_events_tx, mut rx) = mysten_metrics::metered_channel::channel(
            SOLANA_EVENTS_CHANNEL_SIZE,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["solana_events_queue"]),
        );

        let (_handles, mut finalized_slot_rx) =
            SolanaSyncer::new(client.clone(), addresses, sol_events_tx)
                .run(
                    Arc::new(BridgeMetrics::new_for_testing()),
                    Duration::from_millis(50),
                )
                .await
                .unwrap();

        // Trigger slot update
        slot_control.store(12346, Ordering::SeqCst);
        finalized_slot_rx.changed().await.unwrap();

        let (_addr, wrapper) = rx.recv().await.unwrap();
        assert_eq!(wrapper.newest_signature.as_deref(), Some("sig_new"));
        // Only sig_new should be included (sig_cursor and sig_old filtered out)
        // No bridge events parsed from mock transaction
        assert_eq!(wrapper.parsed_events.len(), 0);

        tx.send(()).ok();
        handle.abort();
    }

    #[tokio::test]
    async fn test_solana_syncer_returns_finalized_slot_receiver() {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mysten_metrics::init_metrics(&registry);

        let (addr, handle, tx, slot_control) = start_mock_server(10000).await;
        let base_url = format!("http://{addr}/");
        let client = Arc::new(SolanaClient::new(&base_url));

        let addresses = HashMap::new();

        const SOLANA_EVENTS_CHANNEL_SIZE: usize = 1000;
        let (sol_events_tx, _rx) = mysten_metrics::metered_channel::channel(
            SOLANA_EVENTS_CHANNEL_SIZE,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["solana_events_queue"]),
        );

        let (_handles, mut finalized_slot_rx) =
            SolanaSyncer::new(client.clone(), addresses, sol_events_tx)
                .run(
                    Arc::new(BridgeMetrics::new_for_testing()),
                    Duration::from_millis(50),
                )
                .await
                .unwrap();

        // Initial slot
        assert_eq!(*finalized_slot_rx.borrow(), 10000);

        // Update slot and verify receiver gets the update
        slot_control.store(10001, Ordering::SeqCst);
        finalized_slot_rx.changed().await.unwrap();
        assert_eq!(*finalized_slot_rx.borrow(), 10001);

        slot_control.store(10005, Ordering::SeqCst);
        finalized_slot_rx.changed().await.unwrap();
        assert_eq!(*finalized_slot_rx.borrow(), 10005);

        tx.send(()).ok();
        handle.abort();
    }

    /// Test multiple addresses - similar to eth_syncer's test_multiple_addresses
    #[tokio::test]
    async fn test_solana_syncer_multiple_addresses() {
        use std::collections::HashSet;
        use tokio::sync::mpsc::error::TryRecvError;

        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mysten_metrics::init_metrics(&registry);

        // Create a mock server that returns different signatures for different addresses
        let slot = Arc::new(AtomicU64::new(12345));
        let state = MockState { slot: slot.clone() };

        async fn handler_multi_addr(
            State(state): State<MockState>,
            Json(body): Json<serde_json::Value>,
        ) -> (StatusCode, Json<serde_json::Value>) {
            let method = body
                .get("method")
                .and_then(|m| m.as_str())
                .unwrap_or_default();
            match method {
                "getSlot" => {
                    let slot = state.slot.load(Ordering::SeqCst);
                    let rsp = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "result": slot });
                    (StatusCode::OK, Json(rsp))
                }
                "getSignaturesForAddress" => {
                    let address = body
                        .get("params")
                        .and_then(|p| p.as_array())
                        .and_then(|p| p.get(0))
                        .and_then(|a| a.as_str())
                        .unwrap_or_default();

                    let (sig, slot) = if address.starts_with("Addr1") {
                        ("sig_addr1", 12340)
                    } else {
                        ("sig_addr2", 12350)
                    };

                    let rsp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": 1,
                        "result": [{
                            "signature": sig,
                            "slot": slot,
                            "err": null,
                            "memo": null,
                            "blockTime": 1700000000,
                            "confirmationStatus": "finalized"
                        }]
                    });
                    (StatusCode::OK, Json(rsp))
                }
                "getTransaction" => {
                    let sig = body
                        .get("params")
                        .and_then(|p| p.as_array())
                        .and_then(|p| p.get(0))
                        .and_then(|s| s.as_str())
                        .unwrap_or_default();

                    let slot = if sig == "sig_addr1" { 12340 } else { 12350 };

                    let rsp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": 1,
                        "result": {
                            "slot": slot,
                            "transaction": {
                                "signatures": [sig],
                                "message": { "accountKeys": ["11111111111111111111111111111111"] }
                            },
                            "meta": {
                                "err": null,
                                "fee": 5000,
                                "preBalances": [0],
                                "postBalances": [0],
                                "preTokenBalances": [],
                                "postTokenBalances": []
                            }
                        }
                    });
                    (StatusCode::OK, Json(rsp))
                }
                _ => {
                    let rsp = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "error": { "code": -32601, "message": "Method not found" }});
                    (StatusCode::OK, Json(rsp))
                }
            }
        }

        let app = Router::new()
            .route("/", post(handler_multi_addr))
            .with_state(state);
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        let local_addr = listener.local_addr().unwrap();
        let (tx, rx) = oneshot::channel();
        let handle = tokio::spawn(async move {
            let server = axum::serve(listener, app);
            tokio::select! {
                _ = server => {},
                _ = rx => {},
            }
        });

        let base_url = format!("http://{local_addr}/");
        let client = Arc::new(SolanaClient::new(&base_url));

        // Two addresses with different start slots
        let addresses = HashMap::from_iter(vec![
            (
                "Addr1111111111111111111111111111111111111111".to_string(),
                SolanaTargetAddressInfo {
                    start_slot: 12300u64,
                    until_signature: None,
                },
            ),
            (
                "Addr2222222222222222222222222222222222222222".to_string(),
                SolanaTargetAddressInfo {
                    start_slot: 12300u64,
                    until_signature: None,
                },
            ),
        ]);

        const SOLANA_EVENTS_CHANNEL_SIZE: usize = 1000;
        let (sol_events_tx, mut events_rx) = mysten_metrics::metered_channel::channel(
            SOLANA_EVENTS_CHANNEL_SIZE,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["solana_events_queue"]),
        );

        let (_handles, mut finalized_slot_rx) =
            SolanaSyncer::new(client.clone(), addresses, sol_events_tx)
                .run(
                    Arc::new(BridgeMetrics::new_for_testing()),
                    Duration::from_millis(50),
                )
                .await
                .unwrap();

        // Trigger slot update
        slot.store(12346, Ordering::SeqCst);
        finalized_slot_rx.changed().await.unwrap();

        // Should receive events from both addresses
        let mut received_sigs = HashSet::new();
        let (addr1, wrapper1) = events_rx.recv().await.unwrap();
        received_sigs.insert((addr1, wrapper1.newest_signature));

        let (addr2, wrapper2) = events_rx.recv().await.unwrap();
        received_sigs.insert((addr2, wrapper2.newest_signature));

        assert_eq!(received_sigs.len(), 2);

        // No more events
        assert_eq!(events_rx.try_recv().unwrap_err(), TryRecvError::Empty);

        tx.send(()).ok();
        handle.abort();
    }

    /// Test that finalized slot update triggers new query - similar to eth_syncer behavior
    #[tokio::test]
    async fn test_solana_syncer_slot_update_triggers_query() {
        use tokio::sync::mpsc::error::TryRecvError;

        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mysten_metrics::init_metrics(&registry);

        // Create a mock server that tracks query count
        let slot = Arc::new(AtomicU64::new(12345));
        let query_count = Arc::new(AtomicU64::new(0));
        
        #[derive(Clone)]
        struct MockStateWithCounter {
            slot: Arc<AtomicU64>,
            query_count: Arc<AtomicU64>,
        }

        async fn handler_with_counter(
            State(state): State<MockStateWithCounter>,
            Json(body): Json<serde_json::Value>,
        ) -> (StatusCode, Json<serde_json::Value>) {
            let method = body
                .get("method")
                .and_then(|m| m.as_str())
                .unwrap_or_default();
            match method {
                "getSlot" => {
                    let slot = state.slot.load(Ordering::SeqCst);
                    let rsp = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "result": slot });
                    (StatusCode::OK, Json(rsp))
                }
                "getSignaturesForAddress" => {
                    let count = state.query_count.fetch_add(1, Ordering::SeqCst);
                    let sig = format!("sig_{}", count);
                    let rsp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": 1,
                        "result": [{
                            "signature": sig,
                            "slot": 12340 + count,
                            "err": null,
                            "memo": null,
                            "blockTime": 1700000000,
                            "confirmationStatus": "finalized"
                        }]
                    });
                    (StatusCode::OK, Json(rsp))
                }
                "getTransaction" => {
                    let rsp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": 1,
                        "result": {
                            "slot": 12340,
                            "transaction": {
                                "signatures": ["sig"],
                                "message": { "accountKeys": ["11111111111111111111111111111111"] }
                            },
                            "meta": {
                                "err": null,
                                "fee": 5000,
                                "preBalances": [0],
                                "postBalances": [0],
                                "preTokenBalances": [],
                                "postTokenBalances": []
                            }
                        }
                    });
                    (StatusCode::OK, Json(rsp))
                }
                _ => {
                    let rsp = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "error": { "code": -32601, "message": "Method not found" }});
                    (StatusCode::OK, Json(rsp))
                }
            }
        }

        let state = MockStateWithCounter {
            slot: slot.clone(),
            query_count: query_count.clone(),
        };
        let app = Router::new()
            .route("/", post(handler_with_counter))
            .with_state(state);
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        let local_addr = listener.local_addr().unwrap();
        let (tx, rx) = oneshot::channel();
        let handle = tokio::spawn(async move {
            let server = axum::serve(listener, app);
            tokio::select! {
                _ = server => {},
                _ = rx => {},
            }
        });

        let base_url = format!("http://{local_addr}/");
        let client = Arc::new(SolanaClient::new(&base_url));

        let addresses = HashMap::from_iter(vec![(
            "Vote111111111111111111111111111111111111111".to_string(),
            SolanaTargetAddressInfo {
                start_slot: 12300u64,
                until_signature: None,
            },
        )]);

        const SOLANA_EVENTS_CHANNEL_SIZE: usize = 1000;
        let (sol_events_tx, mut events_rx) = mysten_metrics::metered_channel::channel(
            SOLANA_EVENTS_CHANNEL_SIZE,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["solana_events_queue"]),
        );

        let (_handles, mut finalized_slot_rx) =
            SolanaSyncer::new(client.clone(), addresses, sol_events_tx)
                .run(
                    Arc::new(BridgeMetrics::new_for_testing()),
                    Duration::from_millis(50),
                )
                .await
                .unwrap();

        // First slot update
        slot.store(12346, Ordering::SeqCst);
        finalized_slot_rx.changed().await.unwrap();
        let (_addr, wrapper1) = events_rx.recv().await.unwrap();
        assert_eq!(wrapper1.newest_signature.as_deref(), Some("sig_0"));

        // Second slot update should trigger another query
        slot.store(12347, Ordering::SeqCst);
        finalized_slot_rx.changed().await.unwrap();
        let (_addr, wrapper2) = events_rx.recv().await.unwrap();
        assert_eq!(wrapper2.newest_signature.as_deref(), Some("sig_1"));

        // No more events without slot update
        assert_eq!(events_rx.try_recv().unwrap_err(), TryRecvError::Empty);

        tx.send(()).ok();
        handle.abort();
    }

    /// Test that start_slot filters out old transactions
    #[tokio::test]
    async fn test_solana_syncer_start_slot_filter() {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mysten_metrics::init_metrics(&registry);

        // Create a mock server that returns transactions with various slots
        let slot = Arc::new(AtomicU64::new(12500));
        let state = MockState { slot: slot.clone() };

        async fn handler_mixed_slots(
            State(state): State<MockState>,
            Json(body): Json<serde_json::Value>,
        ) -> (StatusCode, Json<serde_json::Value>) {
            let method = body
                .get("method")
                .and_then(|m| m.as_str())
                .unwrap_or_default();
            match method {
                "getSlot" => {
                    let slot = state.slot.load(Ordering::SeqCst);
                    let rsp = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "result": slot });
                    (StatusCode::OK, Json(rsp))
                }
                "getSignaturesForAddress" => {
                    // Return signatures with mixed slots - some before start_slot, some after
                    let rsp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": 1,
                        "result": [
                            {
                                "signature": "sig_new",
                                "slot": 12400,  // After start_slot (12350)
                                "err": null,
                                "memo": null,
                                "blockTime": 1700000002,
                                "confirmationStatus": "finalized"
                            },
                            {
                                "signature": "sig_old",
                                "slot": 12300,  // Before start_slot (12350)
                                "err": null,
                                "memo": null,
                                "blockTime": 1700000001,
                                "confirmationStatus": "finalized"
                            }
                        ]
                    });
                    (StatusCode::OK, Json(rsp))
                }
                "getTransaction" => {
                    let sig = body
                        .get("params")
                        .and_then(|p| p.as_array())
                        .and_then(|p| p.get(0))
                        .and_then(|s| s.as_str())
                        .unwrap_or_default();

                    // sig_old should never be queried due to start_slot filter
                    if sig == "sig_old" {
                        let rsp = serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": 1,
                            "error": { "code": -32601, "message": "sig_old should be filtered by start_slot" }
                        });
                        return (StatusCode::OK, Json(rsp));
                    }

                    let rsp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": 1,
                        "result": {
                            "slot": 12400,
                            "transaction": {
                                "signatures": [sig],
                                "message": { "accountKeys": ["11111111111111111111111111111111"] }
                            },
                            "meta": {
                                "err": null,
                                "fee": 5000,
                                "preBalances": [0],
                                "postBalances": [0],
                                "preTokenBalances": [],
                                "postTokenBalances": []
                            }
                        }
                    });
                    (StatusCode::OK, Json(rsp))
                }
                _ => {
                    let rsp = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "error": { "code": -32601, "message": "Method not found" }});
                    (StatusCode::OK, Json(rsp))
                }
            }
        }

        let app = Router::new()
            .route("/", post(handler_mixed_slots))
            .with_state(state);
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        let local_addr = listener.local_addr().unwrap();
        let (tx, rx) = oneshot::channel();
        let handle = tokio::spawn(async move {
            let server = axum::serve(listener, app);
            tokio::select! {
                _ = server => {},
                _ = rx => {},
            }
        });

        let base_url = format!("http://{local_addr}/");
        let client = Arc::new(SolanaClient::new(&base_url));

        // Set start_slot to 12350, which should filter out sig_old (slot 12300)
        let addresses = HashMap::from_iter(vec![(
            "Vote111111111111111111111111111111111111111".to_string(),
            SolanaTargetAddressInfo {
                start_slot: 12350u64,
                until_signature: None,
            },
        )]);

        const SOLANA_EVENTS_CHANNEL_SIZE: usize = 1000;
        let (sol_events_tx, mut events_rx) = mysten_metrics::metered_channel::channel(
            SOLANA_EVENTS_CHANNEL_SIZE,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["solana_events_queue"]),
        );

        let (_handles, mut finalized_slot_rx) =
            SolanaSyncer::new(client.clone(), addresses, sol_events_tx)
                .run(
                    Arc::new(BridgeMetrics::new_for_testing()),
                    Duration::from_millis(50),
                )
                .await
                .unwrap();

        // Trigger slot update
        slot.store(12501, Ordering::SeqCst);
        finalized_slot_rx.changed().await.unwrap();

        let (_addr, wrapper) = events_rx.recv().await.unwrap();
        // Only sig_new should be included (sig_old filtered by start_slot)
        // No bridge events parsed from mock transaction
        assert_eq!(wrapper.parsed_events.len(), 0);
        assert_eq!(wrapper.newest_signature.as_deref(), Some("sig_new"));
        assert_eq!(wrapper.newest_slot, Some(12400));

        tx.send(()).ok();
        handle.abort();
    }

    /// Test that failed transactions are skipped
    #[tokio::test]
    async fn test_solana_syncer_skips_failed_transactions() {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mysten_metrics::init_metrics(&registry);

        let slot = Arc::new(AtomicU64::new(12345));
        let state = MockState { slot: slot.clone() };

        async fn handler_with_failed_tx(
            State(state): State<MockState>,
            Json(body): Json<serde_json::Value>,
        ) -> (StatusCode, Json<serde_json::Value>) {
            let method = body
                .get("method")
                .and_then(|m| m.as_str())
                .unwrap_or_default();
            match method {
                "getSlot" => {
                    let slot = state.slot.load(Ordering::SeqCst);
                    let rsp = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "result": slot });
                    (StatusCode::OK, Json(rsp))
                }
                "getSignaturesForAddress" => {
                    // Return one successful and one failed transaction
                    let rsp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": 1,
                        "result": [
                            {
                                "signature": "sig_success",
                                "slot": 12340,
                                "err": null,
                                "memo": null,
                                "blockTime": 1700000002,
                                "confirmationStatus": "finalized"
                            },
                            {
                                "signature": "sig_failed",
                                "slot": 12339,
                                "err": { "InstructionError": [0, "Custom"] },
                                "memo": null,
                                "blockTime": 1700000001,
                                "confirmationStatus": "finalized"
                            }
                        ]
                    });
                    (StatusCode::OK, Json(rsp))
                }
                "getTransaction" => {
                    let sig = body
                        .get("params")
                        .and_then(|p| p.as_array())
                        .and_then(|p| p.get(0))
                        .and_then(|s| s.as_str())
                        .unwrap_or_default();

                    // sig_failed should never be queried
                    if sig == "sig_failed" {
                        let rsp = serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": 1,
                            "error": { "code": -32601, "message": "sig_failed should be filtered" }
                        });
                        return (StatusCode::OK, Json(rsp));
                    }

                    let rsp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": 1,
                        "result": {
                            "slot": 12340,
                            "transaction": {
                                "signatures": [sig],
                                "message": { "accountKeys": ["11111111111111111111111111111111"] }
                            },
                            "meta": {
                                "err": null,
                                "fee": 5000,
                                "preBalances": [0],
                                "postBalances": [0],
                                "preTokenBalances": [],
                                "postTokenBalances": []
                            }
                        }
                    });
                    (StatusCode::OK, Json(rsp))
                }
                _ => {
                    let rsp = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "error": { "code": -32601, "message": "Method not found" }});
                    (StatusCode::OK, Json(rsp))
                }
            }
        }

        let app = Router::new()
            .route("/", post(handler_with_failed_tx))
            .with_state(state);
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        let local_addr = listener.local_addr().unwrap();
        let (tx, rx) = oneshot::channel();
        let handle = tokio::spawn(async move {
            let server = axum::serve(listener, app);
            tokio::select! {
                _ = server => {},
                _ = rx => {},
            }
        });

        let base_url = format!("http://{local_addr}/");
        let client = Arc::new(SolanaClient::new(&base_url));

        let addresses = HashMap::from_iter(vec![(
            "Vote111111111111111111111111111111111111111".to_string(),
            SolanaTargetAddressInfo {
                start_slot: 12300u64,
                until_signature: None,
            },
        )]);

        const SOLANA_EVENTS_CHANNEL_SIZE: usize = 1000;
        let (sol_events_tx, mut events_rx) = mysten_metrics::metered_channel::channel(
            SOLANA_EVENTS_CHANNEL_SIZE,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channel_inflight
                .with_label_values(&["solana_events_queue"]),
        );

        let (_handles, mut finalized_slot_rx) =
            SolanaSyncer::new(client.clone(), addresses, sol_events_tx)
                .run(
                    Arc::new(BridgeMetrics::new_for_testing()),
                    Duration::from_millis(50),
                )
                .await
                .unwrap();

        // Trigger slot update
        slot.store(12346, Ordering::SeqCst);
        finalized_slot_rx.changed().await.unwrap();

        let (_addr, wrapper) = events_rx.recv().await.unwrap();
        // Only sig_success should be included (sig_failed filtered due to err)
        // No bridge events parsed from mock transaction
        assert_eq!(wrapper.parsed_events.len(), 0);
        assert_eq!(wrapper.newest_signature.as_deref(), Some("sig_success"));

        tx.send(()).ok();
        handle.abort();
    }
}
