use serde::Serialize;
use serde_json::Value;

use crate::rpc::types::RpcTransaction;

#[derive(Debug, Serialize, Clone)]
pub struct TransactionSummary {
    pub txid: String,
    pub tx_type: u32,
    pub tx_type_label: String,
    pub size: u64,
    pub fee: Option<f64>,
    pub instantlock: bool,
    pub total_input: Option<f64>,
    pub total_output: f64,
}

#[derive(Debug, Serialize, Clone)]
pub struct TransactionDetail {
    pub txid: String,
    pub version: u32,
    pub tx_type: u32,
    pub tx_type_label: String,
    pub size: u64,
    pub locktime: u64,
    pub block_hash: Option<String>,
    pub block_height: Option<u64>,
    pub confirmations: Option<i64>,
    pub time: Option<u64>,
    pub fee: Option<f64>,
    pub instantlock: bool,
    pub instantlock_internal: bool,
    pub chainlock: Option<bool>,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub special_tx_payload: Option<Value>,
}

#[derive(Debug, Serialize, Clone)]
pub struct TxInput {
    pub txid: Option<String>,
    pub vout: Option<u32>,
    pub is_coinbase: bool,
    pub coinbase_hex: Option<String>,
    pub address: Option<String>,
    pub value: Option<f64>,
    pub value_sat: Option<i64>,
}

#[derive(Debug, Serialize, Clone)]
pub struct TxOutput {
    pub n: u32,
    pub value: f64,
    pub value_sat: i64,
    pub address: Option<String>,
    pub script_type: String,
    pub script_asm: String,
    pub spent_tx_id: Option<String>,
    pub spent_height: Option<i64>,
    pub is_spent: bool,
}

pub fn tx_type_label(tx_type: u32) -> String {
    match tx_type {
        0 => "Standard".to_string(),
        1 => "ProRegTx".to_string(),
        2 => "ProUpServTx".to_string(),
        3 => "ProUpRegTx".to_string(),
        4 => "ProUpRevTx".to_string(),
        5 => "CoinBase".to_string(),
        6 => "QuorumCommitment".to_string(),
        8 => "AssetLock".to_string(),
        9 => "AssetUnlock".to_string(),
        _ => format!("Type {}", tx_type),
    }
}

impl TransactionSummary {
    pub fn from_rpc(tx: &RpcTransaction) -> Self {
        let total_input: Option<f64> = if tx.vin.iter().any(|v| v.coinbase.is_some()) {
            None
        } else {
            let sum: f64 = tx.vin.iter().filter_map(|v| v.value).sum();
            if sum > 0.0 {
                Some(sum)
            } else {
                None
            }
        };
        let total_output: f64 = tx.vout.iter().map(|v| v.value).sum();

        Self {
            txid: tx.txid.clone(),
            tx_type: tx.tx_type,
            tx_type_label: tx_type_label(tx.tx_type),
            size: tx.size,
            fee: tx.fee,
            instantlock: tx.instantlock,
            total_input,
            total_output,
        }
    }
}

impl TransactionDetail {
    pub fn from_rpc(tx: &RpcTransaction) -> Self {
        let inputs: Vec<TxInput> = tx
            .vin
            .iter()
            .map(|vin| TxInput {
                txid: vin.txid.clone(),
                vout: vin.vout,
                is_coinbase: vin.coinbase.is_some(),
                coinbase_hex: vin.coinbase.clone(),
                address: vin.address.clone(),
                value: vin.value,
                value_sat: vin.value_sat,
            })
            .collect();

        let outputs: Vec<TxOutput> = tx
            .vout
            .iter()
            .map(|vout| TxOutput {
                n: vout.n,
                value: vout.value,
                value_sat: vout.value_sat,
                address: vout.script_pub_key.address.clone(),
                script_type: vout.script_pub_key.script_type.clone(),
                script_asm: vout.script_pub_key.asm.clone(),
                spent_tx_id: vout.spent_tx_id.clone(),
                spent_height: vout.spent_height,
                is_spent: vout.spent_tx_id.is_some(),
            })
            .collect();

        // Build special payload
        let special_tx_payload = build_special_payload(tx);

        Self {
            txid: tx.txid.clone(),
            version: tx.version,
            tx_type: tx.tx_type,
            tx_type_label: tx_type_label(tx.tx_type),
            size: tx.size,
            locktime: tx.locktime,
            block_hash: tx.blockhash.clone(),
            block_height: tx.height,
            confirmations: tx.confirmations,
            time: tx.time,
            fee: tx.fee,
            instantlock: tx.instantlock,
            instantlock_internal: tx.instantlock_internal,
            chainlock: tx.chainlock,
            inputs,
            outputs,
            special_tx_payload,
        }
    }
}

fn build_special_payload(tx: &RpcTransaction) -> Option<Value> {
    if let Some(ref payload) = tx.pro_reg_tx {
        return Some(serde_json::json!({
            "type": "ProRegTx",
            "data": serde_json::to_value(payload).unwrap_or_default()
        }));
    }
    if let Some(ref payload) = tx.pro_up_serv_tx {
        return Some(serde_json::json!({
            "type": "ProUpServTx",
            "data": payload
        }));
    }
    if let Some(ref payload) = tx.pro_up_reg_tx {
        return Some(serde_json::json!({
            "type": "ProUpRegTx",
            "data": payload
        }));
    }
    if let Some(ref payload) = tx.pro_up_rev_tx {
        return Some(serde_json::json!({
            "type": "ProUpRevTx",
            "data": payload
        }));
    }
    if let Some(ref payload) = tx.cb_tx {
        return Some(serde_json::json!({
            "type": "CbTx",
            "data": serde_json::to_value(payload).unwrap_or_default()
        }));
    }
    if let Some(ref payload) = tx.qc_tx {
        return Some(serde_json::json!({
            "type": "QcTx",
            "data": serde_json::to_value(payload).unwrap_or_default()
        }));
    }
    if let Some(ref payload) = tx.asset_unlock_tx {
        return Some(serde_json::json!({
            "type": "AssetUnlockTx",
            "data": serde_json::to_value(payload).unwrap_or_default()
        }));
    }
    None
}
