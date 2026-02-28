use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::AppError;
use crate::AppState;

// ============ Blocks ============

pub async fn get_block(
    State(state): State<AppState>,
    Path(hash): Path<String>,
) -> Result<Json<Value>, AppError> {
    let block = state.rpc.get_block(&hash, 1).await?;

    // Build tx list (just txids at verbosity 1)
    let txids: Vec<Value> = block
        .txids()
        .map(|ids| ids.iter().map(|id| json!(id)).collect())
        .unwrap_or_default();

    // Compute reward from coinbase output
    let reward = if let Ok(full_block) = state.rpc.get_block(&hash, 2).await {
        if let Some(txs) = full_block.transactions() {
            if let Some(cb_tx) = txs.first() {
                let total: f64 = cb_tx.vout.iter().map(|o| o.value).sum();
                format!("{:.8}", total)
            } else {
                "0.00000000".to_string()
            }
        } else {
            "0.00000000".to_string()
        }
    } else {
        "0.00000000".to_string()
    };

    Ok(Json(json!({
        "hash": block.hash,
        "size": block.size.unwrap_or(0),
        "height": block.height,
        "version": block.version,
        "merkleroot": block.merkleroot,
        "tx": txids,
        "time": block.time,
        "nonce": block.nonce,
        "bits": block.bits,
        "difficulty": block.difficulty,
        "chainwork": block.chainwork,
        "confirmations": block.confirmations,
        "previousblockhash": block.previous_block_hash,
        "nextblockhash": block.next_block_hash,
        "reward": reward,
        "isMainChain": true,
        "poolInfo": {}
    })))
}

pub async fn get_block_index(
    State(state): State<AppState>,
    Path(height): Path<u64>,
) -> Result<Json<Value>, AppError> {
    let hash = state.rpc.get_block_hash(height).await?;
    Ok(Json(json!({ "blockHash": hash })))
}

pub async fn get_raw_block(
    State(state): State<AppState>,
    Path(hash): Path<String>,
) -> Result<Json<Value>, AppError> {
    let raw = state.rpc.get_raw_block(&hash).await?;
    Ok(Json(json!({ "rawblock": raw })))
}

#[derive(Deserialize)]
pub struct BlocksParams {
    pub limit: Option<u32>,
    #[serde(rename = "blockDate")]
    pub block_date: Option<String>,
}

pub async fn get_blocks(
    State(state): State<AppState>,
    Query(params): Query<BlocksParams>,
) -> Result<Json<Value>, AppError> {
    let limit = params.limit.unwrap_or(200).min(200);

    // Get the current date info
    let tip = state.rpc.get_block_count().await?;

    // If blockDate is provided, find blocks for that date
    // Otherwise use today's blocks
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let target_date = params.block_date.as_deref().unwrap_or(&today);

    // Parse date to get start/end timestamps
    let date_ts_start = parse_date_to_ts(target_date);
    let date_ts_end = date_ts_start + 86400;

    // Walk backwards from tip finding blocks in the date range
    let mut blocks = Vec::new();
    let mut h = tip;
    while h > 0 && blocks.len() < limit as usize {
        let hash = state.rpc.get_block_hash(h).await?;
        let header = state.rpc.get_block_header(&hash).await?;

        if header.time < date_ts_start {
            break;
        }

        if header.time < date_ts_end {
            blocks.push(json!({
                "height": header.height,
                "size": header.size.unwrap_or(0),
                "hash": header.hash,
                "time": header.time,
                "txlength": header.n_tx,
                "poolInfo": {}
            }));
        }

        h -= 1;
    }

    // Build pagination
    let is_today = target_date == today;
    let prev_date = shift_date(target_date, -1);
    let next_date = shift_date(target_date, 1);

    Ok(Json(json!({
        "blocks": blocks,
        "length": blocks.len(),
        "pagination": {
            "next": next_date,
            "prev": prev_date,
            "currentTs": date_ts_end - 1,
            "current": target_date,
            "isToday": is_today,
            "more": true,
            "moreTs": date_ts_end
        }
    })))
}

// ============ Transactions ============

pub async fn get_tx(
    State(state): State<AppState>,
    Path(txid): Path<String>,
) -> Result<Json<Value>, AppError> {
    let tx = state.rpc.get_raw_transaction(&txid).await?;
    Ok(Json(format_insight_tx(&tx)))
}

pub async fn get_raw_tx(
    State(state): State<AppState>,
    Path(txid): Path<String>,
) -> Result<Json<Value>, AppError> {
    let tx = state.rpc.get_raw_transaction(&txid).await?;
    Ok(Json(json!({ "rawtx": tx.hex.unwrap_or_default() })))
}

#[derive(Deserialize)]
pub struct TxsParams {
    pub block: Option<String>,
    pub address: Option<String>,
    #[serde(rename = "pageNum")]
    pub page_num: Option<u32>,
}

pub async fn get_txs(
    State(state): State<AppState>,
    Query(params): Query<TxsParams>,
) -> Result<Json<Value>, AppError> {
    let page = params.page_num.unwrap_or(0);
    let page_size = 10;

    if let Some(ref block_hash) = params.block {
        let block = state.rpc.get_block(block_hash, 2).await?;
        let block_info = (
            block_hash.as_str(),
            block.height,
            block.time,
            block.confirmations as i64,
        );
        let txs: Vec<Value> = block
            .transactions()
            .map(|txs| {
                txs.iter()
                    .map(|tx| format_insight_tx_with_block(tx, Some(block_info)))
                    .collect()
            })
            .unwrap_or_default();

        let total = txs.len();
        let start = (page as usize) * page_size;
        let paged: Vec<Value> = txs.into_iter().skip(start).take(page_size).collect();

        return Ok(Json(json!({
            "pagesTotal": (total + page_size - 1) / page_size,
            "txs": paged
        })));
    }

    if let Some(ref address) = params.address {
        let all_txids = state.rpc.get_address_txids(address).await?;
        let total = all_txids.len();
        let start = (page as usize) * page_size;
        let page_txids: Vec<&String> = all_txids.iter().rev().skip(start).take(page_size).collect();

        let mut txs = Vec::new();
        for txid in page_txids {
            if let Ok(tx) = state.rpc.get_raw_transaction(txid).await {
                txs.push(format_insight_tx(&tx));
            }
        }

        return Ok(Json(json!({
            "pagesTotal": (total + page_size - 1) / page_size,
            "txs": txs
        })));
    }

    Ok(Json(json!({ "pagesTotal": 0, "txs": [] })))
}

#[derive(Deserialize)]
pub struct SendTxBody {
    pub rawtx: String,
}

pub async fn send_tx(
    State(state): State<AppState>,
    Json(body): Json<SendTxBody>,
) -> Result<Json<Value>, AppError> {
    let txid = state.rpc.send_raw_transaction(&body.rawtx).await?;
    Ok(Json(json!({ "txid": txid })))
}

pub async fn send_tx_ix(
    State(state): State<AppState>,
    Json(body): Json<SendTxBody>,
) -> Result<Json<Value>, AppError> {
    // InstantSend is default in modern Dash, same RPC call
    let txid = state.rpc.send_raw_transaction(&body.rawtx).await?;
    Ok(Json(json!({ "txid": txid })))
}

// ============ Addresses ============

pub async fn get_addr(
    State(state): State<AppState>,
    Path(addr): Path<String>,
    Query(params): Query<AddrParams>,
) -> Result<Json<Value>, AppError> {
    let (balance_res, txids_res) = tokio::join!(
        state.rpc.get_address_balance(&addr),
        state.rpc.get_address_txids(&addr),
    );

    let balance = balance_res?;
    let txids = txids_res?;

    let balance_dash = balance.balance as f64 / 1e8;
    let received_dash = balance.received as f64 / 1e8;
    let sent_sat = balance.received - balance.balance;
    let sent_dash = sent_sat as f64 / 1e8;
    let tx_count = txids.len();

    let no_tx_list = params.no_tx_list.unwrap_or(0);
    let from = params.from.unwrap_or(0) as usize;
    let to = params.to.unwrap_or(tx_count as u32) as usize;

    let transactions = if no_tx_list == 1 {
        Value::Null
    } else {
        let page: Vec<Value> = txids
            .iter()
            .rev()
            .skip(from)
            .take(to.saturating_sub(from).min(50))
            .map(|id| json!(id))
            .collect();
        Value::Array(page)
    };

    let mut resp = json!({
        "addrStr": addr,
        "balance": balance_dash,
        "balanceSat": balance.balance,
        "totalReceived": received_dash,
        "totalReceivedSat": balance.received,
        "totalSent": sent_dash,
        "totalSentSat": sent_sat,
        "unconfirmedBalance": 0,
        "unconfirmedBalanceSat": 0,
        "unconfirmedTxApperances": 0,
        "unconfirmedAppearances": 0,
        "txApperances": tx_count,
        "txAppearances": tx_count,
    });

    if !transactions.is_null() {
        resp["transactions"] = transactions;
    }

    Ok(Json(resp))
}

#[derive(Deserialize)]
pub struct AddrParams {
    #[serde(rename = "noTxList")]
    pub no_tx_list: Option<u32>,
    pub from: Option<u32>,
    pub to: Option<u32>,
}

pub async fn get_addr_balance(
    State(state): State<AppState>,
    Path(addr): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let balance = state.rpc.get_address_balance(&addr).await?;
    Ok(balance.balance.to_string())
}

pub async fn get_addr_total_received(
    State(state): State<AppState>,
    Path(addr): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let balance = state.rpc.get_address_balance(&addr).await?;
    Ok(balance.received.to_string())
}

pub async fn get_addr_total_sent(
    State(state): State<AppState>,
    Path(addr): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let balance = state.rpc.get_address_balance(&addr).await?;
    let sent = balance.received - balance.balance;
    Ok(sent.to_string())
}

pub async fn get_addr_unconfirmed_balance(
    State(_state): State<AppState>,
    Path(_addr): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    Ok("0".to_string())
}

pub async fn get_addr_utxo(
    State(state): State<AppState>,
    Path(addr): Path<String>,
) -> Result<Json<Value>, AppError> {
    let utxos = state.rpc.get_address_utxos(&addr).await?;
    let tip = state.rpc.get_block_count().await?;

    let items: Vec<Value> = utxos
        .iter()
        .map(|u| {
            json!({
                "address": u.address,
                "txid": u.txid,
                "vout": u.output_index,
                "scriptPubKey": u.script,
                "amount": u.satoshis as f64 / 1e8,
                "satoshis": u.satoshis,
                "height": u.height,
                "confirmations": tip.saturating_sub(u.height) + 1
            })
        })
        .collect();

    Ok(Json(Value::Array(items)))
}

// Multi-address endpoints

pub async fn get_addrs_utxo(
    State(state): State<AppState>,
    Path(addrs): Path<String>,
) -> Result<Json<Value>, AppError> {
    let tip = state.rpc.get_block_count().await?;
    let mut all_utxos = Vec::new();

    for addr in addrs.split(',') {
        let addr = addr.trim();
        if addr.is_empty() {
            continue;
        }
        if let Ok(utxos) = state.rpc.get_address_utxos(addr).await {
            for u in &utxos {
                all_utxos.push(json!({
                    "address": u.address,
                    "txid": u.txid,
                    "vout": u.output_index,
                    "scriptPubKey": u.script,
                    "amount": u.satoshis as f64 / 1e8,
                    "satoshis": u.satoshis,
                    "height": u.height,
                    "confirmations": tip.saturating_sub(u.height) + 1
                }));
            }
        }
    }

    Ok(Json(Value::Array(all_utxos)))
}

#[derive(Deserialize)]
pub struct AddrsBody {
    pub addrs: String,
    pub from: Option<u32>,
    pub to: Option<u32>,
}

pub async fn post_addrs_utxo(
    State(state): State<AppState>,
    Json(body): Json<AddrsBody>,
) -> Result<Json<Value>, AppError> {
    let tip = state.rpc.get_block_count().await?;
    let mut all_utxos = Vec::new();

    for addr in body.addrs.split(',') {
        let addr = addr.trim();
        if addr.is_empty() {
            continue;
        }
        if let Ok(utxos) = state.rpc.get_address_utxos(addr).await {
            for u in &utxos {
                all_utxos.push(json!({
                    "address": u.address,
                    "txid": u.txid,
                    "vout": u.output_index,
                    "scriptPubKey": u.script,
                    "amount": u.satoshis as f64 / 1e8,
                    "satoshis": u.satoshis,
                    "height": u.height,
                    "confirmations": tip.saturating_sub(u.height) + 1
                }));
            }
        }
    }

    Ok(Json(Value::Array(all_utxos)))
}

pub async fn get_addrs_txs(
    State(state): State<AppState>,
    Path(addrs): Path<String>,
    Query(params): Query<AddrParams>,
) -> Result<Json<Value>, AppError> {
    let from = params.from.unwrap_or(0) as usize;
    let to = params.to.unwrap_or(10) as usize;

    let mut all_txids: Vec<String> = Vec::new();
    for addr in addrs.split(',') {
        let addr = addr.trim();
        if addr.is_empty() {
            continue;
        }
        if let Ok(txids) = state.rpc.get_address_txids(addr).await {
            for txid in txids {
                if !all_txids.contains(&txid) {
                    all_txids.push(txid);
                }
            }
        }
    }

    let total = all_txids.len();
    let page_txids: Vec<&String> = all_txids.iter().rev().skip(from).take(to - from).collect();

    let mut items = Vec::new();
    for txid in page_txids {
        if let Ok(tx) = state.rpc.get_raw_transaction(txid).await {
            items.push(format_insight_tx(&tx));
        }
    }

    Ok(Json(json!({
        "totalItems": total,
        "from": from,
        "to": from + items.len(),
        "items": items
    })))
}

pub async fn post_addrs_txs(
    State(state): State<AppState>,
    Json(body): Json<AddrsBody>,
) -> Result<Json<Value>, AppError> {
    let from = body.from.unwrap_or(0) as usize;
    let to = body.to.unwrap_or(10) as usize;

    let mut all_txids: Vec<String> = Vec::new();
    for addr in body.addrs.split(',') {
        let addr = addr.trim();
        if addr.is_empty() {
            continue;
        }
        if let Ok(txids) = state.rpc.get_address_txids(addr).await {
            for txid in txids {
                if !all_txids.contains(&txid) {
                    all_txids.push(txid);
                }
            }
        }
    }

    let total = all_txids.len();
    let page_txids: Vec<&String> = all_txids.iter().rev().skip(from).take(to - from).collect();

    let mut items = Vec::new();
    for txid in page_txids {
        if let Ok(tx) = state.rpc.get_raw_transaction(txid).await {
            items.push(format_insight_tx(&tx));
        }
    }

    Ok(Json(json!({
        "totalItems": total,
        "from": from,
        "to": from + items.len(),
        "items": items
    })))
}

pub async fn get_addrs_balance(
    State(state): State<AppState>,
    Path(addrs): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mut total: i64 = 0;
    for addr in addrs.split(',') {
        let addr = addr.trim();
        if addr.is_empty() { continue; }
        if let Ok(b) = state.rpc.get_address_balance(addr).await {
            total += b.balance;
        }
    }
    Ok(total.to_string())
}

pub async fn get_addrs_total_received(
    State(state): State<AppState>,
    Path(addrs): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mut total: i64 = 0;
    for addr in addrs.split(',') {
        let addr = addr.trim();
        if addr.is_empty() { continue; }
        if let Ok(b) = state.rpc.get_address_balance(addr).await {
            total += b.received;
        }
    }
    Ok(total.to_string())
}

pub async fn get_addrs_total_sent(
    State(state): State<AppState>,
    Path(addrs): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mut total: i64 = 0;
    for addr in addrs.split(',') {
        let addr = addr.trim();
        if addr.is_empty() { continue; }
        if let Ok(b) = state.rpc.get_address_balance(addr).await {
            total += b.received - b.balance;
        }
    }
    Ok(total.to_string())
}

pub async fn get_addrs_unconfirmed_balance(
    State(_state): State<AppState>,
    Path(_addrs): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    Ok("0".to_string())
}

// ============ Governance ============

pub async fn gobject_info(
    State(state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let info = state.rpc.get_governance_info().await?;
    Ok(Json(json!({
        "result": {
            "governanceminquorum": info.governanceminquorum,
            "proposalfee": info.proposalfee,
            "superblockcycle": info.superblockcycle,
            "superblockmaturitywindow": info.superblockmaturitywindow,
            "lastsuperblock": info.lastsuperblock,
            "nextsuperblock": info.nextsuperblock,
            "fundingthreshold": info.fundingthreshold,
            "governancebudget": info.governancebudget
        },
        "error": null,
        "id": 1
    })))
}

pub async fn gobject_count(
    State(state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let count = state.rpc.get_gobject_count().await?;
    Ok(Json(json!({
        "result": count,
        "error": null,
        "id": 1
    })))
}

#[derive(Deserialize)]
pub struct GobjectListParams {
    pub r#type: Option<String>,
}

pub async fn gobject_list(
    State(state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let objects = state.rpc.get_governance_objects().await?;
    let items: Vec<Value> = objects
        .into_iter()
        .map(|(hash, obj)| {
            json!({
                "Hash": hash,
                "DataHex": obj.data_hex,
                "DataObject": serde_json::from_str::<Value>(&obj.data_string).unwrap_or(json!({})),
                "AbsoluteYesCount": obj.absolute_yes_count,
                "YesCount": obj.yes_count,
                "NoCount": obj.no_count,
                "AbstainCount": obj.abstain_count
            })
        })
        .collect();
    Ok(Json(Value::Array(items)))
}

pub async fn gobject_list_typed(
    State(state): State<AppState>,
    Path(obj_type): Path<String>,
) -> Result<Json<Value>, AppError> {
    let objects = state.rpc.get_governance_objects().await?;
    let items: Vec<Value> = objects
        .into_iter()
        .filter(|(_, obj)| {
            let data: Value = serde_json::from_str(&obj.data_string).unwrap_or(json!({}));
            if obj_type == "proposal" {
                data.get("type").and_then(|t| t.as_u64()) == Some(1)
            } else if obj_type == "trigger" {
                data.get("type").and_then(|t| t.as_u64()) == Some(2)
            } else {
                true
            }
        })
        .map(|(hash, obj)| {
            json!({
                "Hash": hash,
                "DataHex": obj.data_hex,
                "DataObject": serde_json::from_str::<Value>(&obj.data_string).unwrap_or(json!({})),
                "AbsoluteYesCount": obj.absolute_yes_count,
                "YesCount": obj.yes_count,
                "NoCount": obj.no_count,
                "AbstainCount": obj.abstain_count
            })
        })
        .collect();
    Ok(Json(Value::Array(items)))
}

pub async fn gobject_get(
    State(state): State<AppState>,
    Path(hash): Path<String>,
) -> Result<Json<Value>, AppError> {
    let obj = state.rpc.get_governance_object(&hash).await?;
    Ok(Json(obj))
}

pub async fn gobject_check(
    State(state): State<AppState>,
    Path(hex): Path<String>,
) -> Result<Json<Value>, AppError> {
    let result = state.rpc.gobject_check(&hex).await?;
    Ok(Json(result))
}

pub async fn gobject_deserialize(
    State(state): State<AppState>,
    Path(hex): Path<String>,
) -> Result<Json<Value>, AppError> {
    let result = state.rpc.gobject_deserialize(&hex).await?;
    Ok(Json(json!({
        "result": result,
        "error": null,
        "id": 1
    })))
}

pub async fn gobject_votes(
    State(state): State<AppState>,
    Path(hash): Path<String>,
) -> Result<Json<Value>, AppError> {
    let votes = state.rpc.get_governance_object_votes(&hash).await?;
    Ok(Json(votes))
}

#[derive(Deserialize)]
pub struct GobjectSubmitBody {
    #[serde(rename = "parentHash")]
    pub parent_hash: String,
    pub revision: u32,
    pub time: u64,
    #[serde(rename = "dataHex")]
    pub data_hex: String,
    #[serde(rename = "feeTxId")]
    pub fee_tx_id: String,
}

pub async fn gobject_submit(
    State(state): State<AppState>,
    Json(body): Json<GobjectSubmitBody>,
) -> Result<Json<Value>, AppError> {
    let result = state
        .rpc
        .gobject_submit(
            &body.parent_hash,
            body.revision,
            body.time,
            &body.data_hex,
            &body.fee_tx_id,
        )
        .await?;
    Ok(Json(json!({ "result": result })))
}

pub async fn governance_budget(
    State(state): State<AppState>,
    Path(block_index): Path<u64>,
) -> Result<Json<Value>, AppError> {
    let budget = state.rpc.get_superblock_budget(block_index).await?;
    Ok(Json(json!({
        "result": format!("{:.2}", budget),
        "error": null,
        "id": 1
    })))
}

// ============ Network / Status ============

#[derive(Deserialize)]
pub struct StatusParams {
    pub q: Option<String>,
}

pub async fn status(
    State(state): State<AppState>,
    Query(params): Query<StatusParams>,
) -> Result<Json<Value>, AppError> {
    let q = params.q.as_deref().unwrap_or("getInfo");

    match q {
        "getInfo" => {
            let (blockchain, network) = tokio::join!(
                state.rpc.get_blockchain_info(),
                state.rpc.get_network_info(),
            );
            let blockchain = blockchain?;
            let network = network?;

            Ok(Json(json!({
                "info": {
                    "version": network.version,
                    "insightversion": "0.1.0",
                    "protocolversion": network.protocolversion,
                    "blocks": blockchain.blocks,
                    "timeoffset": 0,
                    "connections": network.connections,
                    "difficulty": blockchain.difficulty,
                    "relayfee": network.relayfee,
                    "errors": "",
                    "network": blockchain.chain
                }
            })))
        }
        "getDifficulty" => {
            let blockchain = state.rpc.get_blockchain_info().await?;
            Ok(Json(json!({ "difficulty": blockchain.difficulty })))
        }
        "getBestBlockHash" => {
            let hash = state.rpc.get_best_block_hash().await?;
            Ok(Json(json!({ "bestblockhash": hash })))
        }
        "getBestChainLock" => {
            let cl = state.rpc.get_best_chain_lock().await?;
            Ok(Json(json!({
                "bestchainlock": {
                    "blockhash": cl.blockhash,
                    "height": cl.height,
                    "signature": cl.signature,
                    "known_block": cl.known_block
                }
            })))
        }
        "getLastBlockHash" => {
            let hash = state.rpc.get_best_block_hash().await?;
            Ok(Json(json!({ "lastblockhash": hash, "syncTipHash": hash })))
        }
        _ => {
            Ok(Json(json!({ "error": "Invalid query" })))
        }
    }
}

pub async fn sporks(
    State(state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let sporks = state.rpc.get_sporks().await?;
    Ok(Json(json!({ "sporks": sporks })))
}

pub async fn sync(
    State(state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    let blockchain = state.rpc.get_blockchain_info().await?;
    let sync_pct = (blockchain.verificationprogress * 100.0) as u32;

    Ok(Json(json!({
        "status": if sync_pct >= 99 { "finished" } else { "syncing" },
        "blockChainHeight": blockchain.blocks,
        "syncPercentage": sync_pct.min(100),
        "height": blockchain.blocks,
        "error": null,
        "type": "dashbook"
    })))
}

pub async fn peer(
    State(_state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    Ok(Json(json!({
        "connected": true,
        "host": "127.0.0.1",
        "port": null
    })))
}

#[derive(Deserialize)]
pub struct EstimateFeeParams {
    #[serde(rename = "nbBlocks")]
    pub nb_blocks: Option<u32>,
}

pub async fn estimate_fee(
    State(state): State<AppState>,
    Query(params): Query<EstimateFeeParams>,
) -> Result<Json<Value>, AppError> {
    let n = params.nb_blocks.unwrap_or(2);
    let result = state.rpc.estimate_fee(n).await?;
    // estimatesmartfee returns {feerate: x, blocks: y}
    let fee = result
        .get("feerate")
        .and_then(|f| f.as_f64())
        .unwrap_or(-1.0);
    Ok(Json(json!({ n.to_string(): fee })))
}

// ============ Helpers ============

fn format_insight_tx(tx: &crate::rpc::types::RpcTransaction) -> Value {
    format_insight_tx_with_block(tx, None)
}

fn format_insight_tx_with_block(
    tx: &crate::rpc::types::RpcTransaction,
    block_info: Option<(&str, u64, u64, i64)>, // (blockhash, height, time, confirmations)
) -> Value {
    let vin: Vec<Value> = tx
        .vin
        .iter()
        .enumerate()
        .map(|(i, input)| {
            if let Some(ref coinbase) = input.coinbase {
                json!({
                    "coinbase": coinbase,
                    "sequence": input.sequence,
                    "n": i
                })
            } else {
                let mut v = json!({
                    "txid": input.txid,
                    "vout": input.vout,
                    "sequence": input.sequence,
                    "n": i,
                    "doubleSpentTxID": null,
                });
                if let Some(ref addr) = input.address {
                    v["addr"] = json!(addr);
                }
                if let Some(val) = input.value {
                    v["value"] = json!(val);
                    v["valueSat"] = json!(input.value_sat.unwrap_or((val * 1e8) as i64));
                }
                if let Some(ref sig) = input.script_sig {
                    v["scriptSig"] = json!({
                        "hex": sig.hex,
                        "asm": sig.asm
                    });
                }
                v
            }
        })
        .collect();

    let vout: Vec<Value> = tx
        .vout
        .iter()
        .map(|output| {
            let mut spk = json!({
                "hex": output.script_pub_key.hex,
                "asm": output.script_pub_key.asm,
                "type": output.script_pub_key.script_type,
            });
            if let Some(ref addr) = output.script_pub_key.address {
                spk["addresses"] = json!([addr]);
            }
            json!({
                "value": format!("{:.8}", output.value),
                "n": output.n,
                "scriptPubKey": spk,
                "spentTxId": output.spent_tx_id,
                "spentIndex": output.spent_index,
                "spentHeight": output.spent_height,
            })
        })
        .collect();

    let value_out: f64 = tx.vout.iter().map(|o| o.value).sum();
    let value_in: f64 = tx.vin.iter().filter_map(|i| i.value).sum();
    let is_coinbase = tx.vin.first().map(|i| i.coinbase.is_some()).unwrap_or(false);
    let fees = if is_coinbase { 0.0 } else { value_in - value_out };

    // Use block_info override when tx comes from getblock verbosity 2
    let (blockhash, blockheight, time, blocktime, confirmations) = match block_info {
        Some((bh, height, t, conf)) => (
            Some(bh.to_string()),
            Some(height),
            Some(t),
            Some(t),
            Some(conf),
        ),
        None => (
            tx.blockhash.clone(),
            tx.height,
            tx.time,
            tx.blocktime,
            tx.confirmations,
        ),
    };

    let mut result = json!({
        "txid": tx.txid,
        "version": tx.version,
        "locktime": tx.locktime,
        "vin": vin,
        "vout": vout,
        "blockhash": blockhash,
        "blockheight": blockheight,
        "confirmations": confirmations,
        "time": time,
        "blocktime": blocktime,
        "valueOut": value_out,
        "size": tx.size,
        "txlock": tx.instantlock
    });

    if !is_coinbase {
        result["valueIn"] = json!(value_in);
        result["fees"] = json!(fees);
    } else {
        result["isCoinBase"] = json!(true);
    }

    result
}

fn parse_date_to_ts(date_str: &str) -> u64 {
    // Parse YYYY-MM-DD to unix timestamp at midnight UTC
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        return 0;
    }
    let y: i32 = parts[0].parse().unwrap_or(2026);
    let m: u32 = parts[1].parse().unwrap_or(1);
    let d: u32 = parts[2].parse().unwrap_or(1);

    // Simple calculation using chrono if available, otherwise manual
    use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
    let date = NaiveDate::from_ymd_opt(y, m, d).unwrap_or(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
    let dt = NaiveDateTime::new(date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    dt.and_utc().timestamp() as u64
}

fn shift_date(date_str: &str, days: i64) -> String {
    use chrono::NaiveDate;
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        return date_str.to_string();
    }
    let y: i32 = parts[0].parse().unwrap_or(2026);
    let m: u32 = parts[1].parse().unwrap_or(1);
    let d: u32 = parts[2].parse().unwrap_or(1);

    let date = NaiveDate::from_ymd_opt(y, m, d).unwrap_or(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
    let shifted = date + chrono::Duration::days(days);
    shifted.format("%Y-%m-%d").to_string()
}
