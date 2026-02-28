use super::client::DashRpcClient;
use super::types::*;
use crate::AppError;
use serde_json::json;

impl DashRpcClient {
    pub async fn get_block_count(&self) -> Result<u64, AppError> {
        self.call("getblockcount", json!([])).await
    }

    pub async fn get_block_hash(&self, height: u64) -> Result<String, AppError> {
        self.call("getblockhash", json!([height])).await
    }

    /// Get block with specified verbosity (1 = header + tx ids, 2 = header + full txs)
    pub async fn get_block(&self, hash: &str, verbosity: u8) -> Result<RpcBlock, AppError> {
        self.call("getblock", json!([hash, verbosity])).await
    }

    pub async fn get_block_header(&self, hash: &str) -> Result<RpcBlock, AppError> {
        self.call("getblockheader", json!([hash, true])).await
    }

    pub async fn get_best_block_hash(&self) -> Result<String, AppError> {
        self.call("getbestblockhash", json!([])).await
    }

    pub async fn get_block_stats(&self, height: u64) -> Result<RpcBlockStats, AppError> {
        self.call("getblockstats", json!([height])).await
    }

    pub async fn get_blockchain_info(&self) -> Result<RpcBlockchainInfo, AppError> {
        self.call("getblockchaininfo", json!([])).await
    }

    pub async fn get_raw_block(&self, hash: &str) -> Result<String, AppError> {
        self.call("getblock", json!([hash, 0])).await
    }

    pub async fn get_chain_tx_stats(
        &self,
        n_blocks: Option<u32>,
    ) -> Result<RpcChainTxStats, AppError> {
        match n_blocks {
            Some(n) => self.call("getchaintxstats", json!([n])).await,
            None => self.call("getchaintxstats", json!([])).await,
        }
    }
}
