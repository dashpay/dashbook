use crate::AppError;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct DashRpcClient {
    client: Client,
    url: String,
    username: String,
    password: String,
    request_id: AtomicU64,
}

#[derive(Debug, serde::Deserialize)]
struct RpcResponse<T> {
    result: Option<T>,
    error: Option<RpcError>,
    #[allow(dead_code)]
    id: Value,
}

#[derive(Debug, serde::Deserialize)]
struct RpcError {
    code: i32,
    message: String,
}

impl DashRpcClient {
    pub fn new(url: String, username: String, password: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            url,
            username,
            password,
            request_id: AtomicU64::new(1),
        }
    }

    pub async fn call<T: DeserializeOwned>(
        &self,
        method: &str,
        params: Value,
    ) -> Result<T, AppError> {
        let id = self.request_id.fetch_add(1, Ordering::Relaxed);

        let body = json!({
            "jsonrpc": "1.0",
            "id": id,
            "method": method,
            "params": params,
        });

        let response = self
            .client
            .post(&self.url)
            .basic_auth(&self.username, Some(&self.password))
            .header("content-type", "text/plain")
            .json(&body)
            .send()
            .await?;

        let resp_text = response.text().await?;

        let rpc_resp: RpcResponse<T> =
            serde_json::from_str(&resp_text).map_err(|e| AppError::Internal(
                format!("Failed to parse RPC response for {}: {} -- response: {}", method, e, &resp_text[..resp_text.len().min(500)])
            ))?;

        if let Some(err) = rpc_resp.error {
            return Err(AppError::Rpc {
                code: err.code,
                message: err.message,
            });
        }

        rpc_resp
            .result
            .ok_or_else(|| AppError::Internal(format!("RPC {} returned null result", method)))
    }
}
