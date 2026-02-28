use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

use crate::models::address::{AddressInfo, AddressTxEntry, AddressUtxo};
use crate::AppError;
use crate::AppState;

#[derive(Deserialize)]
pub struct AddressParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

pub async fn get_address(
    State(state): State<AppState>,
    Path(address): Path<String>,
    Query(params): Query<AddressParams>,
) -> Result<Json<AddressInfo>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(50).min(200);

    // Fetch balance, txids, and utxos in parallel
    let (balance_res, txids_res, utxos_res) = tokio::join!(
        state.rpc.get_address_balance(&address),
        state.rpc.get_address_txids(&address),
        state.rpc.get_address_utxos(&address),
    );

    let balance = balance_res?;
    let all_txids = txids_res?;
    let raw_utxos = utxos_res?;

    let tx_count = all_txids.len();

    // Paginate txids (most recent first)
    let start = ((page - 1) * limit) as usize;
    let page_txids: Vec<&String> = all_txids.iter().rev().skip(start).take(limit as usize).collect();

    // Get deltas for the page transactions
    let deltas = state.rpc.get_address_deltas(&address, None, None).await?;

    // Build a txid -> delta map
    let mut tx_delta_map: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
    let mut tx_height_map: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
    for d in &deltas {
        *tx_delta_map.entry(d.txid.clone()).or_insert(0) += d.satoshis;
        tx_height_map.entry(d.txid.clone()).or_insert(d.height);
    }

    let transactions: Vec<AddressTxEntry> = page_txids
        .iter()
        .map(|txid| {
            let delta_sat = tx_delta_map.get(*txid).copied().unwrap_or(0);
            let height = tx_height_map.get(*txid).copied().unwrap_or(0);
            AddressTxEntry {
                txid: (*txid).clone(),
                height,
                delta_sat,
                delta: delta_sat as f64 / 100_000_000.0,
            }
        })
        .collect();

    let utxos: Vec<AddressUtxo> = raw_utxos
        .iter()
        .map(|u| AddressUtxo {
            txid: u.txid.clone(),
            output_index: u.output_index,
            satoshis: u.satoshis,
            value: u.satoshis as f64 / 100_000_000.0,
            height: u.height,
        })
        .collect();

    let info = AddressInfo {
        address,
        balance: balance.balance as f64 / 100_000_000.0,
        balance_sat: balance.balance,
        balance_immature: balance.balance_immature as f64 / 100_000_000.0,
        balance_spendable: balance.balance_spendable as f64 / 100_000_000.0,
        total_received: balance.received as f64 / 100_000_000.0,
        total_received_sat: balance.received,
        tx_count,
        transactions,
        utxos,
    };

    Ok(Json(info))
}
