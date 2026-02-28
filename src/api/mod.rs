mod addresses;
mod blocks;
mod events;
mod governance;
pub mod insight;
mod masternodes;
mod network;
mod search;
mod transactions;

use axum::routing::{get, post};
use axum::Router;
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

    let insight_routes = Router::new()
        // Blocks
        .route("/block/{hash}", get(insight::get_block))
        .route("/block-index/{height}", get(insight::get_block_index))
        .route("/rawblock/{hash}", get(insight::get_raw_block))
        .route("/blocks", get(insight::get_blocks))
        // Transactions
        .route("/tx/{txid}", get(insight::get_tx))
        .route("/rawtx/{txid}", get(insight::get_raw_tx))
        .route("/txs", get(insight::get_txs))
        .route("/tx/send", post(insight::send_tx))
        .route("/tx/sendix", post(insight::send_tx_ix))
        // Single address
        .route("/addr/{addr}", get(insight::get_addr))
        .route("/addr/{addr}/balance", get(insight::get_addr_balance))
        .route("/addr/{addr}/totalReceived", get(insight::get_addr_total_received))
        .route("/addr/{addr}/totalSent", get(insight::get_addr_total_sent))
        .route("/addr/{addr}/unconfirmedBalance", get(insight::get_addr_unconfirmed_balance))
        .route("/addr/{addr}/utxo", get(insight::get_addr_utxo))
        // Multi-address
        .route("/addrs/{addrs}/utxo", get(insight::get_addrs_utxo))
        .route("/addrs/utxo", post(insight::post_addrs_utxo))
        .route("/addrs/{addrs}/txs", get(insight::get_addrs_txs))
        .route("/addrs/txs", post(insight::post_addrs_txs))
        .route("/addrs/{addrs}/balance", get(insight::get_addrs_balance))
        .route("/addrs/{addrs}/totalReceived", get(insight::get_addrs_total_received))
        .route("/addrs/{addrs}/totalSent", get(insight::get_addrs_total_sent))
        .route("/addrs/{addrs}/unconfirmedBalance", get(insight::get_addrs_unconfirmed_balance))
        // Governance
        .route("/gobject/info", get(insight::gobject_info))
        .route("/gobject/count", get(insight::gobject_count))
        .route("/gobject/list", get(insight::gobject_list))
        .route("/gobject/list/{obj_type}", get(insight::gobject_list_typed))
        .route("/gobject/get/{hash}", get(insight::gobject_get))
        .route("/gobject/check/{hex}", get(insight::gobject_check))
        .route("/gobject/deserialize/{hex}", get(insight::gobject_deserialize))
        .route("/gobject/votes/current/{hash}", get(insight::gobject_votes))
        .route("/gobject/submit", post(insight::gobject_submit))
        .route("/governance/budget/{block_index}", get(insight::governance_budget))
        // Network / Status
        .route("/status", get(insight::status))
        .route("/sporks", get(insight::sporks))
        .route("/sync", get(insight::sync))
        .route("/peer", get(insight::peer))
        .route("/utils/estimatefee", get(insight::estimate_fee));

    Router::new()
        .nest("/api", api_routes)
        .nest("/insight-api", insight_routes)
        .fallback_service(
            ServeDir::new(&static_dir)
                .not_found_service(ServeFile::new(index_path)),
        )
        .layer(CompressionLayer::new())
        .with_state(state)
}
