use super::client::DashRpcClient;
use super::types::*;
use crate::AppError;
use serde_json::json;
use std::collections::HashMap;

impl DashRpcClient {
    pub async fn get_quorum_list(&self) -> Result<HashMap<String, Vec<String>>, AppError> {
        self.call("quorum", json!(["list"])).await
    }

    pub async fn get_quorum_info(
        &self,
        quorum_type: u32,
        quorum_hash: &str,
    ) -> Result<RpcQuorumInfo, AppError> {
        self.call("quorum", json!(["info", quorum_type, quorum_hash])).await
    }
}
