use super::client::DashRpcClient;
use super::types::*;
use crate::AppError;
use serde_json::json;

impl DashRpcClient {
    pub async fn get_network_info(&self) -> Result<RpcNetworkInfo, AppError> {
        self.call("getnetworkinfo", json!([])).await
    }

    pub async fn get_mempool_info(&self) -> Result<RpcMempoolInfo, AppError> {
        self.call("getmempoolinfo", json!([])).await
    }

    pub async fn get_best_chain_lock(&self) -> Result<RpcChainLock, AppError> {
        self.call("getbestchainlock", json!([])).await
    }

    pub async fn get_sporks(&self) -> Result<serde_json::Value, AppError> {
        self.call("spork", json!(["show"])).await
    }

    pub async fn estimate_fee(&self, n_blocks: u32) -> Result<serde_json::Value, AppError> {
        self.call("estimatesmartfee", json!([n_blocks])).await
    }
}
