use moka::future::Cache;
use std::time::Duration;

use crate::models::block::{BlockDetail, BlockSummary};
use crate::models::masternode::MasternodeSummary;
use crate::models::network::StatusResponse;
use crate::models::transaction::TransactionDetail;

pub struct AppCache {
    /// Confirmed blocks by hash
    pub blocks: Cache<String, BlockDetail>,
    /// Block hash by height
    pub block_hash_by_height: Cache<u64, String>,
    /// Confirmed transactions by txid
    pub transactions: Cache<String, TransactionDetail>,
    /// Latest blocks list
    pub latest_blocks: Cache<String, Vec<BlockSummary>>,
    /// Network status (quick stats)
    pub status: Cache<String, StatusResponse>,
    /// Masternode list
    pub masternode_list: Cache<String, Vec<MasternodeSummary>>,
}

impl AppCache {
    pub fn new() -> Self {
        Self {
            blocks: Cache::builder()
                .max_capacity(1000)
                .time_to_live(Duration::from_secs(3600))
                .build(),
            block_hash_by_height: Cache::builder()
                .max_capacity(10000)
                .time_to_live(Duration::from_secs(3600))
                .build(),
            transactions: Cache::builder()
                .max_capacity(5000)
                .time_to_live(Duration::from_secs(3600))
                .build(),
            latest_blocks: Cache::builder()
                .max_capacity(10)
                .time_to_live(Duration::from_secs(10))
                .build(),
            status: Cache::builder()
                .max_capacity(5)
                .time_to_live(Duration::from_secs(5))
                .build(),
            masternode_list: Cache::builder()
                .max_capacity(10)
                .time_to_live(Duration::from_secs(120))
                .build(),
        }
    }
}
