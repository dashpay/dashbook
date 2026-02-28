use serde::Serialize;

use crate::rpc::types::{RpcGovernanceInfo, RpcGovernanceObject, RpcProposalData};

#[derive(Debug, Serialize, Clone)]
pub struct GovernanceOverview {
    pub info: GovernanceInfo,
    pub proposals: Vec<Proposal>,
}

#[derive(Debug, Serialize, Clone)]
pub struct GovernanceInfo {
    pub governance_min_quorum: u32,
    pub proposal_fee: f64,
    pub superblock_cycle: u32,
    pub last_superblock: u64,
    pub next_superblock: u64,
    pub funding_threshold: u32,
    pub governance_budget: f64,
}

#[derive(Debug, Serialize, Clone)]
pub struct Proposal {
    pub hash: String,
    pub name: String,
    pub url: String,
    pub payment_address: String,
    pub payment_amount: f64,
    pub start_epoch: u64,
    pub end_epoch: u64,
    pub creation_time: u64,
    pub yes_count: i32,
    pub no_count: i32,
    pub abstain_count: i32,
    pub absolute_yes_count: i32,
    pub is_funded: bool,
    pub is_valid: bool,
    pub collateral_hash: String,
}

impl GovernanceInfo {
    pub fn from_rpc(info: &RpcGovernanceInfo) -> Self {
        Self {
            governance_min_quorum: info.governanceminquorum,
            proposal_fee: info.proposalfee,
            superblock_cycle: info.superblockcycle,
            last_superblock: info.lastsuperblock,
            next_superblock: info.nextsuperblock,
            funding_threshold: info.fundingthreshold,
            governance_budget: info.governancebudget,
        }
    }
}

impl Proposal {
    pub fn from_rpc(hash: &str, obj: &RpcGovernanceObject) -> Option<Self> {
        // Only handle type 1 (proposals)
        if obj.object_type != 1 {
            return None;
        }

        let data: RpcProposalData = serde_json::from_str(&obj.data_string).ok()?;

        Some(Self {
            hash: hash.to_string(),
            name: data.name,
            url: data.url,
            payment_address: data.payment_address,
            payment_amount: data.payment_amount,
            start_epoch: data.start_epoch,
            end_epoch: data.end_epoch,
            creation_time: obj.creation_time,
            yes_count: obj.yes_count,
            no_count: obj.no_count,
            abstain_count: obj.abstain_count,
            absolute_yes_count: obj.absolute_yes_count,
            is_funded: obj.f_cached_funding,
            is_valid: obj.f_cached_valid,
            collateral_hash: obj.collateral_hash.clone(),
        })
    }
}
