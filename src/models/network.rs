use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct NetworkOverview {
    pub chain: String,
    pub block_height: u64,
    pub best_block_hash: String,
    pub difficulty: f64,
    pub chainlock_height: u64,
    pub chainlock_hash: String,
    pub tx_count: u64,
    pub tx_rate: f64,
    pub mempool_size: u64,
    pub mempool_bytes: u64,
    pub mempool_total_fee: f64,
    pub core_version: String,
    pub protocol_version: u64,
    pub connections: u32,
    pub connections_mn: u32,
    pub credit_pool_balance: f64,
    pub masternode_count: MasternodeCountInfo,
}

#[derive(Debug, Serialize, Clone)]
pub struct MasternodeCountInfo {
    pub total: u32,
    pub enabled: u32,
    pub regular_total: u32,
    pub regular_enabled: u32,
    pub evo_total: u32,
    pub evo_enabled: u32,
}

#[derive(Debug, Serialize, Clone)]
pub struct StatusResponse {
    pub block_height: u64,
    pub best_block_hash: String,
    pub chainlock_height: u64,
    pub difficulty: f64,
    pub credit_pool_balance: f64,
    pub masternode_count: MasternodeCountInfo,
    pub mempool_size: u64,
    pub mempool_bytes: u64,
    pub tx_rate: f64,
    pub chain: String,
}
