use super::client::DashRpcClient;
use super::types::*;
use crate::AppError;
use serde_json::json;
use std::collections::HashMap;

impl DashRpcClient {
    pub async fn get_masternode_list(
        &self,
    ) -> Result<HashMap<String, RpcMasternodeListEntry>, AppError> {
        self.call("masternodelist", json!(["json"])).await
    }

    pub async fn get_masternode_count(&self) -> Result<RpcMasternodeCount, AppError> {
        self.call("masternode", json!(["count"])).await
    }

    pub async fn get_protx_list(&self) -> Result<Vec<RpcProtx>, AppError> {
        self.call("protx", json!(["list", "registered", 1])).await
    }

    pub async fn get_protx_info(&self, protx_hash: &str) -> Result<RpcProtx, AppError> {
        self.call("protx", json!(["info", protx_hash])).await
    }
}
