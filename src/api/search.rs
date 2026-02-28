use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::AppError;
use crate::AppState;

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: String,
}

#[derive(Serialize)]
pub struct SearchResult {
    #[serde(rename = "type")]
    pub result_type: String,
    pub value: String,
}

pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Json<SearchResult>, AppError> {
    let q = params.q.trim();

    if q.is_empty() {
        return Err(AppError::BadRequest("Search query is empty".into()));
    }

    // 1. Pure numeric -> block height
    if q.chars().all(|c| c.is_ascii_digit()) {
        let height: u64 = q.parse().unwrap_or(0);
        if let Ok(hash) = state.rpc.get_block_hash(height).await {
            return Ok(Json(SearchResult {
                result_type: "block".to_string(),
                value: hash,
            }));
        }
    }

    // 2. 64 hex chars -> could be block hash or txid
    if q.len() == 64 && q.chars().all(|c| c.is_ascii_hexdigit()) {
        // Try block first
        if let Ok(_block) = state.rpc.get_block_header(q).await {
            return Ok(Json(SearchResult {
                result_type: "block".to_string(),
                value: q.to_string(),
            }));
        }

        // Try transaction
        if let Ok(_tx) = state.rpc.get_raw_transaction(q).await {
            return Ok(Json(SearchResult {
                result_type: "tx".to_string(),
                value: q.to_string(),
            }));
        }

        // Try masternode protx
        if let Ok(_mn) = state.rpc.get_protx_info(q).await {
            return Ok(Json(SearchResult {
                result_type: "masternode".to_string(),
                value: q.to_string(),
            }));
        }
    }

    // 3. Starts with y or X (Dash address prefixes, testnet uses y)
    if q.starts_with('y') || q.starts_with('X') || q.starts_with('8') || q.starts_with('7') {
        if let Ok(_balance) = state.rpc.get_address_balance(q).await {
            return Ok(Json(SearchResult {
                result_type: "address".to_string(),
                value: q.to_string(),
            }));
        }
    }

    Ok(Json(SearchResult {
        result_type: "none".to_string(),
        value: String::new(),
    }))
}
