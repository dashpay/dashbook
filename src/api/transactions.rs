use axum::extract::{Path, State};
use axum::Json;

use crate::models::transaction::TransactionDetail;
use crate::AppError;
use crate::AppState;

pub async fn get_transaction(
    State(state): State<AppState>,
    Path(txid): Path<String>,
) -> Result<Json<TransactionDetail>, AppError> {
    // Check cache
    if let Some(cached) = state.cache.transactions.get(&txid).await {
        return Ok(Json(cached));
    }

    let tx = state.rpc.get_raw_transaction(&txid).await?;
    let detail = TransactionDetail::from_rpc(&tx);

    // Cache if confirmed
    if tx.confirmations.unwrap_or(0) > 6 {
        state
            .cache
            .transactions
            .insert(txid, detail.clone())
            .await;
    }

    Ok(Json(detail))
}
