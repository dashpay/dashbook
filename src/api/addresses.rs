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

    // Fetch balance and txids in parallel (skip utxos initially)
    let (balance_res, txids_res) = tokio::join!(
        state.rpc.get_address_balance(&address),
        state.rpc.get_address_txids(&address),
    );

    let balance = balance_res?;
    let all_txids = txids_res?;

    let tx_count = all_txids.len();

    // Paginate txids (most recent first)
    let start_idx = ((page - 1) * limit) as usize;
    let page_txids: Vec<&String> = all_txids.iter().rev().skip(start_idx).take(limit as usize).collect();

    // Collect unique heights for page txids by fetching only the needed deltas
    // Use the page txids set to filter so we don't load all deltas for huge addresses
    let page_txid_set: std::collections::HashSet<&str> = page_txids.iter().map(|t| t.as_str()).collect();

    // Only fetch deltas if we have a reasonable tx count, otherwise just show txids without deltas
    let (tx_delta_map, tx_height_map) = if tx_count <= 10_000 {
        let deltas = state.rpc.get_address_deltas(&address, None, None).await?;
        let mut delta_map: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
        let mut height_map: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
        for d in &deltas {
            if page_txid_set.contains(d.txid.as_str()) {
                *delta_map.entry(d.txid.clone()).or_insert(0) += d.satoshis;
                height_map.entry(d.txid.clone()).or_insert(d.height);
            }
        }
        (delta_map, height_map)
    } else {
        // For very large addresses, fetch deltas individually per page txid via getrawtransaction
        let mut delta_map: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
        let mut height_map: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
        for txid in &page_txids {
            if let Ok(tx) = state.rpc.get_raw_transaction(txid).await {
                let height = tx.height.unwrap_or(0);
                height_map.insert((*txid).clone(), height);
                // Compute delta: sum outputs to this address minus inputs from this address
                let mut delta: i64 = 0;
                for output in &tx.vout {
                    if let Some(ref addr) = output.script_pub_key.address {
                        if addr == &address {
                            delta += output.value_sat;
                        }
                    }
                }
                for input in &tx.vin {
                    if let Some(ref input_addr) = input.address {
                        if input_addr == &address {
                            delta -= input.value_sat.unwrap_or(0);
                        }
                    }
                }
                delta_map.insert((*txid).clone(), delta);
            }
        }
        (delta_map, height_map)
    };

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

    // Only fetch UTXOs for addresses with a reasonable count
    let utxos: Vec<AddressUtxo> = if tx_count <= 50_000 {
        let raw_utxos = state.rpc.get_address_utxos(&address).await?;
        raw_utxos
            .iter()
            .map(|u| AddressUtxo {
                txid: u.txid.clone(),
                output_index: u.output_index,
                satoshis: u.satoshis,
                value: u.satoshis as f64 / 100_000_000.0,
                height: u.height,
            })
            .collect()
    } else {
        vec![]
    };

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
