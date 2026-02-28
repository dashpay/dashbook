use super::client::DashRpcClient;
use super::types::*;
use crate::AppError;
use serde_json::json;
use std::collections::HashMap;

impl DashRpcClient {
    pub async fn get_raw_transaction(
        &self,
        txid: &str,
    ) -> Result<RpcTransaction, AppError> {
        self.call("getrawtransaction", json!([txid, true])).await
    }

    pub async fn get_raw_mempool_verbose(
        &self,
    ) -> Result<HashMap<String, serde_json::Value>, AppError> {
        self.call("getrawmempool", json!([true])).await
    }

    pub async fn get_raw_mempool_txids(&self) -> Result<Vec<String>, AppError> {
        self.call("getrawmempool", json!([false])).await
    }

    pub async fn send_raw_transaction(&self, hex: &str) -> Result<String, AppError> {
        self.call("sendrawtransaction", json!([hex])).await
    }
}
