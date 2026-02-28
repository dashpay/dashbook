use axum::extract::State;
use axum::Json;
use serde::Serialize;

use crate::models::network::{MasternodeCountInfo, NetworkOverview, StatusResponse};
use crate::AppError;
use crate::AppState;

pub async fn status(State(state): State<AppState>) -> Result<Json<StatusResponse>, AppError> {
    // Check cache
    if let Some(cached) = state.cache.status.get("status").await {
        return Ok(Json(cached));
    }

    let (blockchain_res, chainlock_res, mempool_res, mn_count_res, chain_stats_res) = tokio::join!(
        state.rpc.get_blockchain_info(),
        state.rpc.get_best_chain_lock(),
        state.rpc.get_mempool_info(),
        state.rpc.get_masternode_count(),
        state.rpc.get_chain_tx_stats(None),
    );

    let blockchain = blockchain_res?;
    let chainlock = chainlock_res?;
    let mempool = mempool_res?;
    let mn_count = mn_count_res?;
    let chain_stats = chain_stats_res?;

    // Get credit pool from latest block
    let best_hash = &blockchain.bestblockhash;
    let latest_block = state.rpc.get_block(best_hash, 1).await?;
    let credit_pool = latest_block
        .cb_tx
        .as_ref()
        .map(|cb| cb.credit_pool_balance)
        .unwrap_or(0.0);

    let resp = StatusResponse {
        block_height: blockchain.blocks,
        best_block_hash: blockchain.bestblockhash,
        chainlock_height: chainlock.height,
        difficulty: blockchain.difficulty,
        credit_pool_balance: credit_pool,
        masternode_count: MasternodeCountInfo {
            total: mn_count.total,
            enabled: mn_count.enabled,
            regular_total: mn_count.detailed.regular.total,
            regular_enabled: mn_count.detailed.regular.enabled,
            evo_total: mn_count.detailed.evo.total,
            evo_enabled: mn_count.detailed.evo.enabled,
        },
        mempool_size: mempool.size,
        mempool_bytes: mempool.bytes,
        tx_rate: chain_stats.txrate,
        chain: blockchain.chain,
    };

    state
        .cache
        .status
        .insert("status".to_string(), resp.clone())
        .await;

    Ok(Json(resp))
}

pub async fn get_network(
    State(state): State<AppState>,
) -> Result<Json<NetworkOverview>, AppError> {
    let (
        blockchain_res,
        network_res,
        chainlock_res,
        mempool_res,
        mn_count_res,
        chain_stats_res,
    ) = tokio::join!(
        state.rpc.get_blockchain_info(),
        state.rpc.get_network_info(),
        state.rpc.get_best_chain_lock(),
        state.rpc.get_mempool_info(),
        state.rpc.get_masternode_count(),
        state.rpc.get_chain_tx_stats(None),
    );

    let blockchain = blockchain_res?;
    let network = network_res?;
    let chainlock = chainlock_res?;
    let mempool = mempool_res?;
    let mn_count = mn_count_res?;
    let chain_stats = chain_stats_res?;

    // Get credit pool
    let latest_block = state.rpc.get_block(&blockchain.bestblockhash, 1).await?;
    let credit_pool = latest_block
        .cb_tx
        .as_ref()
        .map(|cb| cb.credit_pool_balance)
        .unwrap_or(0.0);

    Ok(Json(NetworkOverview {
        chain: blockchain.chain,
        block_height: blockchain.blocks,
        best_block_hash: blockchain.bestblockhash,
        difficulty: blockchain.difficulty,
        chainlock_height: chainlock.height,
        chainlock_hash: chainlock.blockhash,
        tx_count: chain_stats.txcount,
        tx_rate: chain_stats.txrate,
        mempool_size: mempool.size,
        mempool_bytes: mempool.bytes,
        mempool_total_fee: mempool.total_fee,
        core_version: network.buildversion,
        protocol_version: network.protocolversion,
        connections: network.connections,
        connections_mn: network.connections_mn,
        credit_pool_balance: credit_pool,
        masternode_count: MasternodeCountInfo {
            total: mn_count.total,
            enabled: mn_count.enabled,
            regular_total: mn_count.detailed.regular.total,
            regular_enabled: mn_count.detailed.regular.enabled,
            evo_total: mn_count.detailed.evo.total,
            evo_enabled: mn_count.detailed.evo.enabled,
        },
    }))
}

#[derive(Serialize)]
pub struct MempoolResponse {
    pub size: u64,
    pub bytes: u64,
    pub total_fee: f64,
    pub min_fee: f64,
    pub instantsend_locks: u64,
    pub transactions: Vec<String>,
}

pub async fn get_mempool(
    State(state): State<AppState>,
) -> Result<Json<MempoolResponse>, AppError> {
    let (mempool_res, txids_res) = tokio::join!(
        state.rpc.get_mempool_info(),
        state.rpc.get_raw_mempool_txids(),
    );

    let mempool = mempool_res?;
    let txids = txids_res?;

    Ok(Json(MempoolResponse {
        size: mempool.size,
        bytes: mempool.bytes,
        total_fee: mempool.total_fee,
        min_fee: mempool.mempoolminfee,
        instantsend_locks: mempool.instantsendlocks,
        transactions: txids,
    }))
}
