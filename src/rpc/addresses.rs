use super::client::DashRpcClient;
use super::types::*;
use crate::AppError;
use serde_json::json;

impl DashRpcClient {
    pub async fn get_address_balance(
        &self,
        address: &str,
    ) -> Result<RpcAddressBalance, AppError> {
        self.call(
            "getaddressbalance",
            json!([{"addresses": [address]}]),
        )
        .await
    }

    pub async fn get_address_txids(
        &self,
        address: &str,
    ) -> Result<Vec<String>, AppError> {
        self.call(
            "getaddresstxids",
            json!([{"addresses": [address]}]),
        )
        .await
    }

    pub async fn get_address_utxos(
        &self,
        address: &str,
    ) -> Result<Vec<RpcAddressUtxo>, AppError> {
        self.call(
            "getaddressutxos",
            json!([{"addresses": [address]}]),
        )
        .await
    }

    pub async fn get_address_deltas(
        &self,
        address: &str,
        start: Option<u64>,
        end: Option<u64>,
    ) -> Result<Vec<RpcAddressDelta>, AppError> {
        let mut params = json!({"addresses": [address]});
        if let Some(s) = start {
            params["start"] = json!(s);
        }
        if let Some(e) = end {
            params["end"] = json!(e);
        }
        self.call("getaddressdeltas", json!([params])).await
    }
}
