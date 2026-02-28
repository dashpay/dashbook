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

    pub async fn get_governance_object(
        &self,
        hash: &str,
    ) -> Result<serde_json::Value, AppError> {
        self.call("gobject", json!(["get", hash])).await
    }

    pub async fn get_gobject_count(&self) -> Result<serde_json::Value, AppError> {
        self.call("gobject", json!(["count"])).await
    }

    pub async fn gobject_check(&self, hex: &str) -> Result<serde_json::Value, AppError> {
        self.call("gobject", json!(["check", hex])).await
    }

    pub async fn gobject_deserialize(&self, hex: &str) -> Result<serde_json::Value, AppError> {
        self.call("gobject", json!(["deserialize", hex])).await
    }

    pub async fn get_governance_object_votes(
        &self,
        hash: &str,
    ) -> Result<serde_json::Value, AppError> {
        self.call("gobject", json!(["getcurrentvotes", hash])).await
    }

    pub async fn gobject_submit(
        &self,
        parent_hash: &str,
        revision: u32,
        time: u64,
        data_hex: &str,
        fee_txid: &str,
    ) -> Result<String, AppError> {
        self.call(
            "gobject",
            json!(["submit", parent_hash, revision, time, data_hex, fee_txid]),
        )
        .await
    }
}
