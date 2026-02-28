use serde::Serialize;

use super::transaction::TransactionSummary;
use crate::rpc::types::RpcBlock;

#[derive(Debug, Serialize, Clone)]
pub struct BlockSummary {
    pub hash: String,
    pub height: u64,
    pub time: u64,
    pub n_tx: u32,
    pub size: u64,
    pub difficulty: f64,
    pub chainlock: bool,
    pub credit_pool_balance: Option<f64>,
}

#[derive(Debug, Serialize, Clone)]
pub struct BlockDetail {
    pub hash: String,
    pub height: u64,
    pub version: u32,
    pub merkle_root: String,
    pub time: u64,
    pub median_time: u64,
    pub nonce: u64,
    pub bits: String,
    pub difficulty: f64,
    pub chainwork: String,
    pub n_tx: u32,
    pub confirmations: i64,
    pub size: u64,
    pub previous_block_hash: Option<String>,
    pub next_block_hash: Option<String>,
    pub chainlock: bool,
    pub cb_tx: Option<CbTxInfo>,
    pub transactions: Vec<TransactionSummary>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CbTxInfo {
    pub version: u32,
    pub height: u64,
    pub merkle_root_mn_list: String,
    pub merkle_root_quorums: String,
    pub best_cl_height_diff: u64,
    pub best_cl_signature: String,
    pub credit_pool_balance: f64,
}

impl BlockSummary {
    pub fn from_rpc(block: &RpcBlock) -> Self {
        Self {
            hash: block.hash.clone(),
            height: block.height,
            time: block.time,
            n_tx: block.n_tx,
            size: block.size.unwrap_or(0),
            difficulty: block.difficulty,
            chainlock: block.chainlock,
            credit_pool_balance: block.cb_tx.as_ref().map(|cb| cb.credit_pool_balance),
        }
    }
}

impl CbTxInfo {
    pub fn from_rpc(cb: &crate::rpc::types::RpcCbTxPayload) -> Self {
        Self {
            version: cb.version,
            height: cb.height,
            merkle_root_mn_list: cb.merkle_root_mn_list.clone(),
            merkle_root_quorums: cb.merkle_root_quorums.clone(),
            best_cl_height_diff: cb.best_cl_height_diff,
            best_cl_signature: cb.best_cl_signature.clone(),
            credit_pool_balance: cb.credit_pool_balance,
        }
    }
}
