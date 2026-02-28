use serde::{Deserialize, Serialize};
use serde_json::Value;

// ============ Blocks ============

#[derive(Debug, Deserialize, Clone)]
pub struct RpcBlock {
    pub hash: String,
    pub confirmations: i64,
    pub height: u64,
    pub version: u32,
    #[serde(rename = "versionHex")]
    pub version_hex: Option<String>,
    pub merkleroot: String,
    pub time: u64,
    pub mediantime: u64,
    pub nonce: u64,
    pub bits: String,
    pub difficulty: f64,
    pub chainwork: String,
    #[serde(rename = "nTx")]
    pub n_tx: u32,
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: Option<String>,
    #[serde(rename = "nextblockhash")]
    pub next_block_hash: Option<String>,
    pub chainlock: bool,
    pub size: Option<u64>,
    #[serde(rename = "cbTx")]
    pub cb_tx: Option<RpcCbTxPayload>,
    /// verbosity 1: Vec<String> (txids), verbosity 2: Vec<RpcTransaction>
    /// We use Value to handle both cases
    #[serde(default)]
    pub tx: Option<serde_json::Value>,
}

impl RpcBlock {
    /// Get transactions when block was fetched with verbosity 2
    pub fn transactions(&self) -> Option<Vec<RpcTransaction>> {
        self.tx.as_ref().and_then(|v| {
            if let serde_json::Value::Array(arr) = v {
                if arr.first().map_or(false, |v| v.is_object()) {
                    serde_json::from_value(serde_json::Value::Array(arr.clone())).ok()
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    /// Get txids when block was fetched with verbosity 1
    pub fn txids(&self) -> Option<Vec<String>> {
        self.tx.as_ref().and_then(|v| {
            if let serde_json::Value::Array(arr) = v {
                if arr.first().map_or(false, |v| v.is_string()) {
                    serde_json::from_value(serde_json::Value::Array(arr.clone())).ok()
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RpcCbTxPayload {
    pub version: u32,
    pub height: u64,
    #[serde(rename = "merkleRootMNList")]
    pub merkle_root_mn_list: String,
    #[serde(rename = "merkleRootQuorums")]
    pub merkle_root_quorums: String,
    #[serde(rename = "bestCLHeightDiff")]
    pub best_cl_height_diff: u64,
    #[serde(rename = "bestCLSignature")]
    pub best_cl_signature: String,
    #[serde(rename = "creditPoolBalance")]
    pub credit_pool_balance: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcBlockStats {
    pub avgfee: f64,
    pub avgfeerate: f64,
    pub avgtxsize: u64,
    pub blockhash: String,
    pub height: u64,
    pub ins: u32,
    pub maxfee: f64,
    pub maxtxsize: u64,
    pub medianfee: f64,
    pub mediantime: u64,
    pub minfee: f64,
    pub mintxsize: u64,
    pub outs: u32,
    pub subsidy: i64,
    pub time: u64,
    pub total_out: i64,
    pub total_size: u64,
    pub totalfee: f64,
    pub txs: u32,
    pub utxo_increase: i32,
    pub utxo_size_inc: i64,
}

// ============ Transactions ============

#[derive(Debug, Deserialize, Clone)]
pub struct RpcTransaction {
    pub txid: String,
    pub version: u32,
    #[serde(rename = "type")]
    pub tx_type: u32,
    pub size: u64,
    pub locktime: u64,
    pub vin: Vec<RpcTxInput>,
    pub vout: Vec<RpcTxOutput>,
    #[serde(rename = "extraPayloadSize")]
    pub extra_payload_size: Option<u32>,
    #[serde(rename = "extraPayload")]
    pub extra_payload: Option<String>,
    pub blockhash: Option<String>,
    pub height: Option<u64>,
    pub confirmations: Option<i64>,
    pub time: Option<u64>,
    pub blocktime: Option<u64>,
    pub instantlock: bool,
    pub instantlock_internal: bool,
    pub chainlock: Option<bool>,
    // Special tx payloads
    #[serde(rename = "proRegTx")]
    pub pro_reg_tx: Option<RpcProRegTxPayload>,
    #[serde(rename = "proUpServTx")]
    pub pro_up_serv_tx: Option<Value>,
    #[serde(rename = "proUpRegTx")]
    pub pro_up_reg_tx: Option<Value>,
    #[serde(rename = "proUpRevTx")]
    pub pro_up_rev_tx: Option<Value>,
    #[serde(rename = "cbTx")]
    pub cb_tx: Option<RpcCbTxPayload>,
    #[serde(rename = "qcTx")]
    pub qc_tx: Option<RpcQcTxPayload>,
    #[serde(rename = "assetUnlockTx")]
    pub asset_unlock_tx: Option<RpcAssetUnlockTxPayload>,
    pub hex: Option<String>,
    pub fee: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcTxInput {
    pub txid: Option<String>,
    pub vout: Option<u32>,
    pub coinbase: Option<String>,
    #[serde(rename = "scriptSig")]
    pub script_sig: Option<RpcScriptSig>,
    pub value: Option<f64>,
    #[serde(rename = "valueSat")]
    pub value_sat: Option<i64>,
    pub address: Option<String>,
    pub sequence: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcScriptSig {
    pub asm: String,
    pub hex: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcTxOutput {
    pub value: f64,
    #[serde(rename = "valueSat")]
    pub value_sat: i64,
    pub n: u32,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: RpcScriptPubKey,
    #[serde(rename = "spentTxId")]
    pub spent_tx_id: Option<String>,
    #[serde(rename = "spentIndex")]
    pub spent_index: Option<u32>,
    #[serde(rename = "spentHeight")]
    pub spent_height: Option<i64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcScriptPubKey {
    pub asm: String,
    pub desc: Option<String>,
    pub hex: String,
    pub address: Option<String>,
    #[serde(rename = "type")]
    pub script_type: String,
}

// ============ Special TX Payloads ============

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RpcProRegTxPayload {
    pub version: u32,
    #[serde(rename = "type")]
    pub mn_type: Option<u32>,
    #[serde(rename = "collateralHash")]
    pub collateral_hash: String,
    #[serde(rename = "collateralIndex")]
    pub collateral_index: u32,
    pub service: String,
    pub addresses: Option<RpcAddresses>,
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    #[serde(rename = "votingAddress")]
    pub voting_address: String,
    #[serde(rename = "payoutAddress")]
    pub payout_address: String,
    #[serde(rename = "pubKeyOperator")]
    pub pub_key_operator: String,
    #[serde(rename = "operatorReward")]
    pub operator_reward: f64,
    #[serde(rename = "platformNodeID")]
    pub platform_node_id: Option<String>,
    #[serde(rename = "platformP2PPort")]
    pub platform_p2p_port: Option<u16>,
    #[serde(rename = "platformHTTPPort")]
    pub platform_http_port: Option<u16>,
    #[serde(rename = "inputsHash")]
    pub inputs_hash: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RpcAddresses {
    pub core_p2p: Option<Vec<String>>,
    pub platform_https: Option<Vec<String>>,
    pub platform_p2p: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RpcQcTxPayload {
    pub version: u32,
    pub height: u64,
    pub commitment: RpcQcCommitment,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RpcQcCommitment {
    pub version: u32,
    #[serde(rename = "llmqType")]
    pub llmq_type: u32,
    #[serde(rename = "quorumHash")]
    pub quorum_hash: String,
    #[serde(rename = "quorumIndex")]
    pub quorum_index: u32,
    #[serde(rename = "signersCount")]
    pub signers_count: u32,
    pub signers: String,
    #[serde(rename = "validMembersCount")]
    pub valid_members_count: u32,
    #[serde(rename = "validMembers")]
    pub valid_members: String,
    #[serde(rename = "quorumPublicKey")]
    pub quorum_public_key: String,
    #[serde(rename = "quorumVvecHash")]
    pub quorum_vvec_hash: String,
    #[serde(rename = "quorumSig")]
    pub quorum_sig: String,
    #[serde(rename = "membersSig")]
    pub members_sig: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RpcAssetUnlockTxPayload {
    pub version: u32,
    pub index: u64,
    pub fee: u64,
    #[serde(rename = "requestedHeight")]
    pub requested_height: u64,
    #[serde(rename = "quorumHash")]
    pub quorum_hash: String,
    #[serde(rename = "quorumSig")]
    pub quorum_sig: String,
}

// ============ Address Index ============

#[derive(Debug, Deserialize, Clone)]
pub struct RpcAddressBalance {
    pub balance: i64,
    pub balance_immature: i64,
    pub balance_spendable: i64,
    pub received: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcAddressDelta {
    pub satoshis: i64,
    pub txid: String,
    pub index: u32,
    pub blockindex: u32,
    pub height: u64,
    pub address: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcAddressUtxo {
    pub address: String,
    pub txid: String,
    #[serde(rename = "outputIndex")]
    pub output_index: u32,
    pub script: String,
    pub satoshis: i64,
    pub height: u64,
}

// ============ Masternodes ============

#[derive(Debug, Deserialize, Clone)]
pub struct RpcMasternodeListEntry {
    #[serde(rename = "proTxHash")]
    pub pro_tx_hash: String,
    pub address: String,
    pub addresses: Option<RpcAddresses>,
    pub payee: String,
    pub status: String,
    #[serde(rename = "type")]
    pub mn_type: String,
    #[serde(rename = "pospenaltyscore")]
    pub pose_penalty_score: u32,
    #[serde(rename = "consecutivePayments")]
    pub consecutive_payments: u32,
    pub lastpaidtime: u64,
    pub lastpaidblock: u64,
    pub owneraddress: String,
    pub votingaddress: String,
    pub collateraladdress: String,
    pub pubkeyoperator: String,
    #[serde(rename = "platformNodeID")]
    pub platform_node_id: Option<String>,
    #[serde(rename = "platformP2PPort")]
    pub platform_p2p_port: Option<u16>,
    #[serde(rename = "platformHTTPPort")]
    pub platform_http_port: Option<u16>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcProtx {
    #[serde(rename = "type")]
    pub mn_type: String,
    #[serde(rename = "proTxHash")]
    pub pro_tx_hash: String,
    #[serde(rename = "collateralHash")]
    pub collateral_hash: String,
    #[serde(rename = "collateralIndex")]
    pub collateral_index: u32,
    #[serde(rename = "collateralAddress")]
    pub collateral_address: String,
    #[serde(rename = "operatorReward")]
    pub operator_reward: f64,
    pub state: RpcMasternodeState,
    pub confirmations: i64,
    #[serde(rename = "metaInfo")]
    pub meta_info: RpcMasternodeMetaInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RpcMasternodeState {
    pub version: u32,
    pub service: String,
    pub addresses: Option<RpcAddresses>,
    #[serde(rename = "registeredHeight")]
    pub registered_height: u64,
    #[serde(rename = "lastPaidHeight")]
    pub last_paid_height: u64,
    #[serde(rename = "consecutivePayments")]
    pub consecutive_payments: u32,
    #[serde(rename = "PoSePenalty")]
    pub pose_penalty: u32,
    #[serde(rename = "PoSeRevivedHeight")]
    pub pose_revived_height: i64,
    #[serde(rename = "PoSeBanHeight")]
    pub pose_ban_height: i64,
    #[serde(rename = "revocationReason")]
    pub revocation_reason: u32,
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    #[serde(rename = "votingAddress")]
    pub voting_address: String,
    #[serde(rename = "payoutAddress")]
    pub payout_address: String,
    #[serde(rename = "pubKeyOperator")]
    pub pub_key_operator: String,
    #[serde(rename = "platformNodeID")]
    pub platform_node_id: Option<String>,
    #[serde(rename = "platformP2PPort")]
    pub platform_p2p_port: Option<u16>,
    #[serde(rename = "platformHTTPPort")]
    pub platform_http_port: Option<u16>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RpcMasternodeMetaInfo {
    #[serde(rename = "lastDSQ")]
    pub last_dsq: i64,
    #[serde(rename = "mixingTxCount")]
    pub mixing_tx_count: u32,
    #[serde(rename = "outboundAttemptCount")]
    pub outbound_attempt_count: u32,
    #[serde(rename = "lastOutboundAttempt")]
    pub last_outbound_attempt: u64,
    #[serde(rename = "lastOutboundAttemptElapsed")]
    pub last_outbound_attempt_elapsed: u64,
    #[serde(rename = "lastOutboundSuccess")]
    pub last_outbound_success: u64,
    #[serde(rename = "lastOutboundSuccessElapsed")]
    pub last_outbound_success_elapsed: u64,
    pub is_platform_banned: Option<bool>,
    pub platform_ban_height_updated: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcMasternodeCount {
    pub total: u32,
    pub enabled: u32,
    pub detailed: RpcMasternodeCountDetailed,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcMasternodeCountDetailed {
    pub regular: RpcMnTypeCount,
    pub evo: RpcMnTypeCount,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcMnTypeCount {
    pub total: u32,
    pub enabled: u32,
}

// ============ Network ============

#[derive(Debug, Deserialize, Clone)]
pub struct RpcNetworkInfo {
    pub version: u64,
    pub buildversion: String,
    pub subversion: String,
    pub protocolversion: u64,
    pub connections: u32,
    pub connections_in: u32,
    pub connections_out: u32,
    pub connections_mn: u32,
    pub connections_mn_in: u32,
    pub connections_mn_out: u32,
    pub relayfee: f64,
    pub warnings: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcBlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub bestblockhash: String,
    pub difficulty: f64,
    pub time: u64,
    pub mediantime: u64,
    pub verificationprogress: f64,
    pub initialblockdownload: bool,
    pub chainwork: String,
    pub size_on_disk: u64,
    pub pruned: bool,
    pub warnings: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcMempoolInfo {
    pub loaded: bool,
    pub size: u64,
    pub bytes: u64,
    pub usage: u64,
    pub total_fee: f64,
    pub maxmempool: u64,
    pub mempoolminfee: f64,
    pub minrelaytxfee: f64,
    pub instantsendlocks: u64,
    pub unbroadcastcount: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcChainTxStats {
    pub time: u64,
    pub txcount: u64,
    pub window_final_block_height: u64,
    pub window_block_count: u64,
    pub window_tx_count: u64,
    pub window_interval: u64,
    pub txrate: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcChainLock {
    pub blockhash: String,
    pub height: u64,
    pub signature: String,
    pub known_block: bool,
}

// ============ Governance ============

#[derive(Debug, Deserialize, Clone)]
pub struct RpcGovernanceInfo {
    pub governanceminquorum: u32,
    pub proposalfee: f64,
    pub superblockcycle: u32,
    pub superblockmaturitywindow: u32,
    pub lastsuperblock: u64,
    pub nextsuperblock: u64,
    pub fundingthreshold: u32,
    pub governancebudget: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcGovernanceObject {
    #[serde(rename = "DataHex")]
    pub data_hex: String,
    #[serde(rename = "DataString")]
    pub data_string: String,
    #[serde(rename = "Hash")]
    pub hash: String,
    #[serde(rename = "CollateralHash")]
    pub collateral_hash: String,
    #[serde(rename = "ObjectType")]
    pub object_type: u32,
    #[serde(rename = "CreationTime")]
    pub creation_time: u64,
    #[serde(rename = "fBlockchainValidity")]
    pub f_blockchain_validity: bool,
    #[serde(rename = "IsValidReason")]
    pub is_valid_reason: String,
    #[serde(rename = "fCachedValid")]
    pub f_cached_valid: bool,
    #[serde(rename = "fCachedFunding")]
    pub f_cached_funding: bool,
    #[serde(rename = "fCachedDelete")]
    pub f_cached_delete: bool,
    #[serde(rename = "fCachedEndorsed")]
    pub f_cached_endorsed: bool,
    #[serde(rename = "AbsoluteYesCount")]
    pub absolute_yes_count: i32,
    #[serde(rename = "YesCount")]
    pub yes_count: i32,
    #[serde(rename = "NoCount")]
    pub no_count: i32,
    #[serde(rename = "AbstainCount")]
    pub abstain_count: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcProposalData {
    pub end_epoch: u64,
    pub name: String,
    pub payment_address: String,
    pub payment_amount: f64,
    pub start_epoch: u64,
    #[serde(rename = "type")]
    pub _proposal_type: u32,
    pub url: String,
}

// ============ Quorums ============

#[derive(Debug, Deserialize, Clone)]
pub struct RpcQuorumInfo {
    pub height: u64,
    #[serde(rename = "type")]
    pub quorum_type: String,
    #[serde(rename = "quorumHash")]
    pub quorum_hash: String,
    #[serde(rename = "quorumIndex")]
    pub quorum_index: u32,
    #[serde(rename = "minedBlock")]
    pub mined_block: String,
    #[serde(rename = "quorumPublicKey")]
    pub quorum_public_key: String,
    pub members: Option<Vec<RpcQuorumMember>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RpcQuorumMember {
    #[serde(rename = "proTxHash")]
    pub pro_tx_hash: String,
    pub service: Option<String>,
    pub addresses: Option<RpcAddresses>,
    #[serde(rename = "pubKeyOperator")]
    pub pub_key_operator: Option<String>,
    pub valid: bool,
}
