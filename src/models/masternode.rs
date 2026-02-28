use serde::Serialize;

use crate::rpc::types::{RpcMasternodeListEntry, RpcProtx};

#[derive(Debug, Serialize, Clone)]
pub struct MasternodeSummary {
    pub pro_tx_hash: String,
    pub mn_type: String,
    pub status: String,
    pub service: String,
    pub pose_penalty: u32,
    pub last_paid_block: u64,
    pub last_paid_time: u64,
    pub registered_height: Option<u64>,
    pub collateral_address: String,
    pub payout_address: String,
    pub platform_node_id: Option<String>,
    pub platform_http_port: Option<u16>,
    pub platform_p2p_port: Option<u16>,
}

#[derive(Debug, Serialize, Clone)]
pub struct MasternodeDetail {
    pub pro_tx_hash: String,
    pub mn_type: String,
    pub collateral_hash: String,
    pub collateral_index: u32,
    pub collateral_address: String,
    pub operator_reward: f64,
    pub service: String,
    pub registered_height: u64,
    pub last_paid_height: u64,
    pub consecutive_payments: u32,
    pub pose_penalty: u32,
    pub pose_revived_height: i64,
    pub pose_ban_height: i64,
    pub revocation_reason: u32,
    pub owner_address: String,
    pub voting_address: String,
    pub payout_address: String,
    pub pub_key_operator: String,
    pub platform_node_id: Option<String>,
    pub platform_http_port: Option<u16>,
    pub platform_p2p_port: Option<u16>,
    pub is_platform_banned: Option<bool>,
    pub confirmations: i64,
    pub last_dsq: i64,
    pub mixing_tx_count: u32,
}

impl MasternodeSummary {
    pub fn from_list_entry(entry: &RpcMasternodeListEntry) -> Self {
        Self {
            pro_tx_hash: entry.pro_tx_hash.clone(),
            mn_type: entry.mn_type.clone(),
            status: entry.status.clone(),
            service: entry.address.clone(),
            pose_penalty: entry.pose_penalty_score,
            last_paid_block: entry.lastpaidblock,
            last_paid_time: entry.lastpaidtime,
            registered_height: None,
            collateral_address: entry.collateraladdress.clone(),
            payout_address: entry.payee.clone(),
            platform_node_id: entry.platform_node_id.clone(),
            platform_http_port: entry.platform_http_port,
            platform_p2p_port: entry.platform_p2p_port,
        }
    }
}

impl MasternodeDetail {
    pub fn from_protx(protx: &RpcProtx) -> Self {
        Self {
            pro_tx_hash: protx.pro_tx_hash.clone(),
            mn_type: protx.mn_type.clone(),
            collateral_hash: protx.collateral_hash.clone(),
            collateral_index: protx.collateral_index,
            collateral_address: protx.collateral_address.clone(),
            operator_reward: protx.operator_reward,
            service: protx.state.service.clone(),
            registered_height: protx.state.registered_height,
            last_paid_height: protx.state.last_paid_height,
            consecutive_payments: protx.state.consecutive_payments,
            pose_penalty: protx.state.pose_penalty,
            pose_revived_height: protx.state.pose_revived_height,
            pose_ban_height: protx.state.pose_ban_height,
            revocation_reason: protx.state.revocation_reason,
            owner_address: protx.state.owner_address.clone(),
            voting_address: protx.state.voting_address.clone(),
            payout_address: protx.state.payout_address.clone(),
            pub_key_operator: protx.state.pub_key_operator.clone(),
            platform_node_id: protx.state.platform_node_id.clone(),
            platform_http_port: protx.state.platform_http_port,
            platform_p2p_port: protx.state.platform_p2p_port,
            is_platform_banned: protx.meta_info.is_platform_banned,
            confirmations: protx.confirmations,
            last_dsq: protx.meta_info.last_dsq,
            mixing_tx_count: protx.meta_info.mixing_tx_count,
        }
    }
}
