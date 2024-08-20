// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use std::env;
use std::sync::Arc;
use sui_config::sui_config_dir;
use sui_faucet::{create_wallet_context, start_faucet, AppState};
use sui_faucet::{FaucetConfig, SimpleFaucet};
use tracing::info;

const CONCURRENCY_LIMIT: usize = 30;
const DEFAULT_AMOUNT: u64 = 1000 * 1_000_000_000;

struct AppState<F = Arc<SimpleFaucet>> {
    faucet: F,
    config: FaucetConfig,
}

//const PROM_PORT_ADDR: &str = "0.0.0.0:9184";

const PROM_PORT_ADDR: &str = "0.0.0.0:9185";
const PROM_PORT_ADDR: &str = "0.0.0.0:9184";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // initialize tracing
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    let max_concurrency = match env::var("MAX_CONCURRENCY") {
        Ok(val) => val.parse::<usize>().unwrap(),
        _ => CONCURRENCY_LIMIT,
    };
    info!("Max concurrency: {max_concurrency}.");

    let mut config: FaucetConfig = FaucetConfig::parse();
    let config: FaucetConfig = FaucetConfig::parse();
    let FaucetConfig {
        wallet_client_timeout_secs,
        ref write_ahead_log,
        ..
    } = config;
    config.amount = DEFAULT_AMOUNT;
    let context = create_wallet_context(wallet_client_timeout_secs)?;

    let context = create_wallet_context(wallet_client_timeout_secs, sui_config_dir()?)?;

    let max_concurrency = match env::var("MAX_CONCURRENCY") {
        Ok(val) => val.parse::<usize>().unwrap(),
        _ => CONCURRENCY_LIMIT,
    };
    info!("Max concurrency: {max_concurrency}.");

    let prom_binding = PROM_PORT_ADDR.parse().unwrap();
    info!("Starting Prometheus HTTP endpoint at {}", prom_binding);
    let registry_service = mysten_metrics::start_prometheus_server(prom_binding);
    let prometheus_registry = registry_service.default_registry();
    let app_state = Arc::new(AppState {
        faucet: SimpleFaucet::new(
            context,
            &prometheus_registry,
            write_ahead_log,
            config.clone(),
        )
        .await
        .unwrap(),
        config,
    });

    // TODO: restrict access if needed
    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(health))
        .route("/gas", post(request_gas))
        .route("/v1/gas", post(batch_request_gas))
        .route("/v1/status", post(request_status))
        .route("/v1/status/:task_id", get(request_status))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .layer(RequestMetricsLayer::new(&prometheus_registry))
                .layer(cors)
                .load_shed()
                .buffer(request_buffer_size)
                .layer(RateLimitLayer::new(
                    max_request_per_second,
                    Duration::from_secs(1),
                ))
                .concurrency_limit(max_concurrency)
                .layer(Extension(app_state.clone()))
                .into_inner(),
        );

    spawn_monitored_task!(async move {
        info!("Starting task to clear WAL.");
        loop {
            // Every config.wal_retry_interval (Default: 300 seconds) we try to clear the wal coins
            tokio::time::sleep(Duration::from_secs(wal_retry_interval)).await;
            app_state.faucet.retry_wal_coins().await.unwrap();
        }
    });

    let addr = SocketAddr::new(IpAddr::V4(host_ip), port);
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

/// basic handler that responds with a static string
async fn health() -> &'static str {
    "OK"
}

/// handler for batch_request_gas requests
async fn batch_request_gas(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<FaucetRequest>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    // ID for traceability
    info!(uuid = ?id, "Got new gas request.");

    let FaucetRequest::FixedAmountRequest(request) = payload else {
        return (
            StatusCode::BAD_REQUEST,
            Json(BatchFaucetResponse::from(FaucetError::Internal(
                "Input Error.".to_string(),
            ))),
        );
    };

    if state.config.batch_enabled {
        let result = spawn_monitored_task!(async move {
            state
                .faucet
                .batch_send(
                    id,
                    request.recipient,
                    &vec![state.config.amount; state.config.num_coins],
                )
                .await
        })
        .await
        .unwrap();

        match result {
            Ok(v) => {
                info!(uuid =?id, "Request is successfully served");
                (StatusCode::ACCEPTED, Json(BatchFaucetResponse::from(v)))
            }
            Err(v) => {
                warn!(uuid =?id, "Failed to request gas: {:?}", v);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(BatchFaucetResponse::from(v)),
                )
            }
        }
    } else {
        // TODO (jian): remove this feature gate when batch has proven to be baked long enough
        info!(uuid = ?id, "Falling back to v1 implementation");
        let result = spawn_monitored_task!(async move {
            state
                .faucet
                .send(
                    id,
                    request.recipient,
                    &vec![state.config.amount; state.config.num_coins],
                )
                .await
        })
        .await
        .unwrap();

        match result {
            Ok(_) => {
                info!(uuid =?id, "Request is successfully served");
                (StatusCode::ACCEPTED, Json(BatchFaucetResponse::from(id)))
            }
            Err(v) => {
                warn!(uuid =?id, "Failed to request gas: {:?}", v);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(BatchFaucetResponse::from(v)),
                )
            }
        }
    }
}

/// handler for batch_get_status requests
async fn request_status(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match Uuid::parse_str(&id) {
        Ok(task_id) => {
            let result = state.faucet.get_batch_send_status(task_id).await;
            match result {
                Ok(v) => (
                    StatusCode::CREATED,
                    Json(BatchStatusFaucetResponse::from(v)),
                ),
                Err(v) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(BatchStatusFaucetResponse::from(v)),
                ),
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(BatchStatusFaucetResponse::from(FaucetError::Internal(
                e.to_string(),
            ))),
        ),
    }
}

/// handler for all the request_gas requests
async fn request_gas(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<FaucetRequest>,
) -> impl IntoResponse {
    // ID for traceability
    let id = Uuid::new_v4();
    info!(uuid = ?id, "Got new gas request.");
    let result = match payload {
        FaucetRequest::FixedAmountRequest(requests) => {
            // We spawn a tokio task for this such that connection drop will not interrupt
            // it and impact the recycling of coins
            spawn_monitored_task!(async move {
                state
                    .faucet
                    .send(
                        id,
                        requests.recipient,
                        &vec![state.config.amount; state.config.num_coins],
                    )
                    .await
            })
            .await
            .unwrap()
        }
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(FaucetResponse::from(FaucetError::Internal(
                    "Input Error.".to_string(),
                ))),
            )
        }
    };
    match result {
        Ok(v) => {
            info!(uuid =?id, "Request is successfully served");
            (StatusCode::CREATED, Json(FaucetResponse::from(v)))
        }
        Err(v) => {
            warn!(uuid =?id, "Failed to request gas: {:?}", v);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(FaucetResponse::from(v)),
            )
        }
    }
}

fn create_wallet_context(timeout_secs: u64) -> Result<WalletContext, anyhow::Error> {
    let wallet_conf = sui_config_dir()?.join(SUI_CLIENT_CONFIG);
    info!("Initialize wallet from config path: {:?}", wallet_conf);
    WalletContext::new(
        &wallet_conf,
        Some(Duration::from_secs(timeout_secs)),
        Some(1000),
    )
}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, please try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {}", error)),
    )
    start_faucet(app_state, max_concurrency, &prometheus_registry).await
}
