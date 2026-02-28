use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct AddressInfo {
    pub address: String,
    pub balance: f64,
    pub balance_sat: i64,
    pub balance_immature: f64,
    pub balance_spendable: f64,
    pub total_received: f64,
    pub total_received_sat: i64,
    pub tx_count: usize,
    pub transactions: Vec<AddressTxEntry>,
    pub utxos: Vec<AddressUtxo>,
}

#[derive(Debug, Serialize, Clone)]
pub struct AddressTxEntry {
    pub txid: String,
    pub height: u64,
    pub delta_sat: i64,
    pub delta: f64,
}

#[derive(Debug, Serialize, Clone)]
pub struct AddressUtxo {
    pub txid: String,
    pub output_index: u32,
    pub satoshis: i64,
    pub value: f64,
    pub height: u64,
}
