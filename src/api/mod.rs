mod addresses;
mod blocks;
mod events;
mod governance;
mod masternodes;
mod network;
mod search;
mod transactions;

use axum::{routing::get, Router};
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};

use crate::AppState;

pub fn build_router(state: AppState) -> Router {
    let static_dir = state.config.server.static_dir.clone();
    let index_path = format!("{}/index.html", &static_dir);

    let api_routes = Router::new()
        .route("/status", get(network::status))
        .route("/blocks", get(blocks::list_blocks))
        .route("/block/{hash_or_height}", get(blocks::get_block))
        .route("/tx/{txid}", get(transactions::get_transaction))
        .route("/address/{address}", get(addresses::get_address))
        .route("/masternodes", get(masternodes::list_masternodes))
        .route(
            "/masternode/{protxhash}",
            get(masternodes::get_masternode),
        )
        .route("/governance", get(governance::get_governance))
        .route("/network", get(network::get_network))
        .route("/mempool", get(network::get_mempool))
        .route("/search", get(search::search))
        .route("/ws", get(events::websocket_handler));

    Router::new()
        .nest("/api", api_routes)
        .fallback_service(
            ServeDir::new(&static_dir)
                .not_found_service(ServeFile::new(index_path)),
        )
        .layer(CompressionLayer::new())
        .with_state(state)
}
