mod api;
mod cache;
mod config;
mod error;
mod live;
mod models;
mod rpc;

use std::sync::Arc;
use tokio::sync::broadcast;
use tracing_subscriber::EnvFilter;

pub use error::AppError;

#[derive(Clone)]
pub struct AppState {
    pub rpc: Arc<rpc::DashRpcClient>,
    pub cache: Arc<cache::AppCache>,
    pub live_tx: broadcast::Sender<live::LiveEvent>,
    pub config: Arc<config::Config>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config = config::Config::from_env();
    tracing::info!("Dashbook starting with RPC at {}", config.rpc.url);

    let rpc_client = rpc::DashRpcClient::new(
        config.rpc.url.clone(),
        config.rpc.username.clone(),
        config.rpc.password.clone(),
    );

    // Test RPC connection
    match rpc_client.get_block_count().await {
        Ok(height) => tracing::info!("Connected to Dash Core - block height: {}", height),
        Err(e) => {
            tracing::error!("Failed to connect to Dash Core RPC: {}", e);
            std::process::exit(1);
        }
    }

    let rpc = Arc::new(rpc_client);
    let app_cache = Arc::new(cache::AppCache::new());
    let (live_tx, _) = broadcast::channel(256);

    let state = AppState {
        rpc: rpc.clone(),
        cache: app_cache.clone(),
        live_tx: live_tx.clone(),
        config: Arc::new(config.clone()),
    };

    // Start background live updater
    let updater = live::LiveUpdater::new(rpc.clone(), app_cache.clone(), live_tx.clone());
    tokio::spawn(updater.run());

    let app = api::build_router(state);

    let bind_addr = &config.server.bind_address;
    tracing::info!("Dashbook listening on {}", bind_addr);

    let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
