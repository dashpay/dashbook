use super::client::DashRpcClient;
use super::types::*;
use crate::AppError;
use serde_json::json;
use std::collections::HashMap;

impl DashRpcClient {
    pub async fn get_governance_info(&self) -> Result<RpcGovernanceInfo, AppError> {
        self.call("getgovernanceinfo", json!([])).await
    }

    pub async fn get_governance_objects(
        &self,
    ) -> Result<HashMap<String, RpcGovernanceObject>, AppError> {
        self.call("gobject", json!(["list"])).await
    }

    pub async fn get_superblock_budget(&self, height: u64) -> Result<f64, AppError> {
        self.call("getsuperblockbudget", json!([height])).await
    }
}
