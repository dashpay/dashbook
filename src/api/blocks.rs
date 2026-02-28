use axum::extract::{Path, Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::models::block::{BlockDetail, BlockSummary, CbTxInfo};
use crate::models::transaction::TransactionSummary;
use crate::AppError;
use crate::AppState;

#[derive(Deserialize)]
pub struct BlockListParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Serialize)]
pub struct BlockListResponse {
    pub blocks: Vec<BlockSummary>,
    pub total: u64,
    pub page: u32,
    pub pages: u64,
}

pub async fn list_blocks(
    State(state): State<AppState>,
    Query(params): Query<BlockListParams>,
) -> Result<Json<BlockListResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).min(100);

    let tip = state.rpc.get_block_count().await?;
    let total_pages = (tip / limit as u64) + 1;

    let start = tip.saturating_sub((page as u64 - 1) * limit as u64);
    let end = start.saturating_sub(limit as u64 - 1);

    let mut blocks = Vec::with_capacity(limit as usize);
    for h in (end..=start).rev() {
        let hash = state.rpc.get_block_hash(h).await?;
        let block = state.rpc.get_block_header(&hash).await?;
        blocks.push(BlockSummary::from_rpc(&block));
    }

    Ok(Json(BlockListResponse {
        blocks,
        total: tip,
        page,
        pages: total_pages,
    }))
}

pub async fn get_block(
    State(state): State<AppState>,
    Path(hash_or_height): Path<String>,
) -> Result<Json<BlockDetail>, AppError> {
    // Determine if input is height (numeric) or hash
    let hash = if hash_or_height.chars().all(|c| c.is_ascii_digit()) {
        let height: u64 = hash_or_height
            .parse()
            .map_err(|_| AppError::BadRequest("Invalid block height".into()))?;
        state.rpc.get_block_hash(height).await?
    } else {
        hash_or_height
    };

    // Check cache first
    if let Some(cached) = state.cache.blocks.get(&hash).await {
        return Ok(Json(cached));
    }

    // Fetch block with full transaction details
    let block = state.rpc.get_block(&hash, 2).await?;

    let transactions: Vec<TransactionSummary> = block
        .transactions()
        .map(|txs| txs.iter().map(TransactionSummary::from_rpc).collect())
        .unwrap_or_default();

    let detail = BlockDetail {
        hash: block.hash.clone(),
        height: block.height,
        version: block.version,
        merkle_root: block.merkleroot.clone(),
        time: block.time,
        median_time: block.mediantime,
        nonce: block.nonce,
        bits: block.bits.clone(),
        difficulty: block.difficulty,
        chainwork: block.chainwork.clone(),
        n_tx: block.n_tx,
        confirmations: block.confirmations,
        size: block.size.unwrap_or(0),
        previous_block_hash: block.previous_block_hash.clone(),
        next_block_hash: block.next_block_hash.clone(),
        chainlock: block.chainlock,
        cb_tx: block.cb_tx.as_ref().map(CbTxInfo::from_rpc),
        transactions,
    };

    // Cache if deeply confirmed
    if block.confirmations > 6 {
        state.cache.blocks.insert(hash.clone(), detail.clone()).await;
        state
            .cache
            .block_hash_by_height
            .insert(block.height, hash)
            .await;
    }

    Ok(Json(detail))
}
