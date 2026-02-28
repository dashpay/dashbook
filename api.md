# Dashbook API Documentation

Base URL: `https://dashbook.testnet.networks.dash.org`

All endpoints return JSON. No authentication required.

---

## Status

### `GET /api/status`

Quick network stats overview.

```json
{
  "block_height": 1429565,
  "best_block_hash": "0000006b...",
  "chainlock_height": 1429565,
  "difficulty": 0.004186,
  "credit_pool_balance": 277108.207,
  "masternode_count": {
    "total": 541,
    "enabled": 86,
    "regular_total": 480,
    "regular_enabled": 56,
    "evo_total": 61,
    "evo_enabled": 30
  },
  "mempool_size": 3,
  "mempool_bytes": 617,
  "tx_rate": 0.0161,
  "chain": "test"
}
```

---

## Blocks

### `GET /api/blocks`

Paginated block list (most recent first).

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page` | u32 | 1 | Page number |
| `limit` | u32 | 20 | Blocks per page (max 100) |

```json
{
  "blocks": [
    {
      "hash": "0000006b...",
      "height": 1429565,
      "time": 1772294069,
      "n_tx": 2,
      "size": 0,
      "difficulty": 0.004312,
      "chainlock": true,
      "credit_pool_balance": 277109.649
    }
  ],
  "total": 1429565,
  "page": 1,
  "pages": 71479
}
```

### `GET /api/block/:hash_or_height`

Full block details with transaction list. Accepts either a block hash or numeric height.

```json
{
  "hash": "0000006b...",
  "height": 1429565,
  "version": 536870912,
  "merkle_root": "abc123...",
  "time": 1772294069,
  "median_time": 1772292963,
  "nonce": 12345,
  "bits": "1e0fffff",
  "difficulty": 0.004312,
  "chainwork": "0000000000000000...",
  "n_tx": 2,
  "confirmations": 10,
  "size": 1234,
  "previous_block_hash": "00000043...",
  "next_block_hash": "00000012...",
  "chainlock": true,
  "cb_tx": {
    "version": 3,
    "height": 1429565,
    "merkle_root_mn_list": "abc...",
    "merkle_root_quorums": "def...",
    "best_cl_height_diff": 0,
    "best_cl_signature": "...",
    "credit_pool_balance": 277109.649
  },
  "transactions": [
    {
      "txid": "abc123...",
      "tx_type": 5,
      "size": 345,
      "total_output": 1.234,
      "instantlock": true
    }
  ]
}
```

---

## Transactions

### `GET /api/tx/:txid`

Full transaction details including inputs, outputs, and special payload.

**Transaction types:**

| Type | Name | DIP |
|------|------|-----|
| 0 | Standard | - |
| 1 | ProRegTx | DIP-3 |
| 2 | ProUpServTx | DIP-3 |
| 3 | ProUpRegTx | DIP-3 |
| 4 | ProUpRevTx | DIP-3 |
| 5 | CoinBase | DIP-4 |
| 6 | QuorumCommitment | DIP-6 |
| 8 | AssetLock | DIP-27 |
| 9 | AssetUnlock | DIP-27 |

```json
{
  "txid": "abc123...",
  "version": 3,
  "tx_type": 0,
  "tx_type_label": "Standard",
  "size": 226,
  "locktime": 0,
  "block_hash": "0000006b...",
  "block_height": 1429565,
  "confirmations": 10,
  "time": 1772294069,
  "fee": 0.00000226,
  "instantlock": true,
  "instantlock_internal": false,
  "chainlock": true,
  "inputs": [
    {
      "txid": "def456...",
      "vout": 0,
      "is_coinbase": false,
      "coinbase_hex": null,
      "address": "yWz1...",
      "value": 1.0,
      "value_sat": 100000000
    }
  ],
  "outputs": [
    {
      "n": 0,
      "value": 0.5,
      "value_sat": 50000000,
      "address": "yXa2...",
      "script_type": "pubkeyhash",
      "script_asm": "OP_DUP OP_HASH160 ...",
      "spent_tx_id": null,
      "spent_height": null,
      "is_spent": false
    }
  ],
  "special_tx_payload": null
}
```

When `special_tx_payload` is present (for types 1-9), it contains:
```json
{
  "type": "ProRegTx",
  "data": { ... }
}
```

---

## Addresses

### `GET /api/address/:address`

Address balance, transaction history, and UTXOs.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page` | u32 | 1 | Page number for transactions |
| `limit` | u32 | 50 | Transactions per page (max 200) |

```json
{
  "address": "yWz1...",
  "balance": 100.5,
  "balance_sat": 10050000000,
  "balance_immature": 0.0,
  "balance_spendable": 100.5,
  "total_received": 200.0,
  "total_received_sat": 20000000000,
  "tx_count": 15,
  "transactions": [
    {
      "txid": "abc123...",
      "height": 1429500,
      "delta_sat": 50000000,
      "delta": 0.5
    }
  ],
  "utxos": [
    {
      "txid": "abc123...",
      "output_index": 0,
      "satoshis": 50000000,
      "value": 0.5,
      "height": 1429500
    }
  ]
}
```

---

## Masternodes

### `GET /api/masternodes`

Filtered, paginated masternode list.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page` | u32 | 1 | Page number |
| `limit` | u32 | 50 | Per page (max 200) |
| `type` | string | all | Filter: `all`, `regular`, `evo` |
| `status` | string | all | Filter: `all`, `ENABLED`, `POSE_BANNED`, etc. |

```json
{
  "masternodes": [
    {
      "pro_tx_hash": "abc123...",
      "mn_type": "Regular",
      "status": "ENABLED",
      "service": "1.2.3.4:19999",
      "pose_penalty": 0,
      "last_paid_block": 1429400,
      "last_paid_time": 1772290000,
      "registered_height": 1000000,
      "collateral_address": "yWz1...",
      "payout_address": "yXa2...",
      "platform_node_id": null,
      "platform_http_port": null,
      "platform_p2p_port": null
    }
  ],
  "total": 541,
  "page": 1,
  "pages": 11
}
```

### `GET /api/masternode/:protxhash`

Full masternode details by ProRegTx hash.

```json
{
  "pro_tx_hash": "abc123...",
  "mn_type": "Evo",
  "collateral_hash": "def456...",
  "collateral_index": 0,
  "collateral_address": "yWz1...",
  "operator_reward": 0.0,
  "service": "1.2.3.4:19999",
  "registered_height": 1000000,
  "last_paid_height": 1429400,
  "consecutive_payments": 5,
  "pose_penalty": 0,
  "pose_revived_height": -1,
  "pose_ban_height": -1,
  "revocation_reason": 0,
  "owner_address": "yWz1...",
  "voting_address": "yXa2...",
  "payout_address": "yBc3...",
  "pub_key_operator": "abc...",
  "platform_node_id": "abc123",
  "platform_http_port": 443,
  "platform_p2p_port": 26656,
  "is_platform_banned": false,
  "confirmations": 429565,
  "last_dsq": 0,
  "mixing_tx_count": 0
}
```

---

## Governance

### `GET /api/governance`

Governance info and all active proposals.

```json
{
  "info": {
    "governance_min_quorum": 1,
    "proposal_fee": 1.0,
    "superblock_cycle": 24,
    "last_superblock": 1429536,
    "next_superblock": 1429560,
    "funding_threshold": 3,
    "governance_budget": 5.0
  },
  "proposals": [
    {
      "hash": "abc123...",
      "name": "Test Proposal",
      "url": "https://example.com",
      "payment_address": "yWz1...",
      "payment_amount": 10.0,
      "start_epoch": 1770000000,
      "end_epoch": 1780000000,
      "creation_time": 1770000000,
      "yes_count": 50,
      "no_count": 5,
      "abstain_count": 2,
      "absolute_yes_count": 45,
      "is_funded": true,
      "is_valid": true,
      "collateral_hash": "def456..."
    }
  ]
}
```

---

## Network

### `GET /api/network`

Detailed network information.

```json
{
  "chain": "test",
  "block_height": 1429565,
  "best_block_hash": "0000006b...",
  "difficulty": 0.004312,
  "chainlock_height": 1429565,
  "chainlock_hash": "0000006b...",
  "tx_count": 2500000,
  "tx_rate": 0.0161,
  "mempool_size": 3,
  "mempool_bytes": 617,
  "mempool_total_fee": 0.00001,
  "core_version": "/Dash Core:23.1.0/",
  "protocol_version": 70232,
  "connections": 16,
  "connections_mn": 10,
  "credit_pool_balance": 277108.207,
  "masternode_count": { ... }
}
```

### `GET /api/mempool`

Mempool details and transaction list.

```json
{
  "size": 3,
  "bytes": 617,
  "total_fee": 0.00001,
  "min_fee": 0.00001,
  "instantsend_locks": 0,
  "transactions": ["txid1...", "txid2...", "txid3..."]
}
```

---

## Search

### `GET /api/search`

Universal search across blocks, transactions, addresses, and masternodes.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `q` | string | yes | Search query |

**Search logic:**
- Numeric string → block height
- 64 hex characters → block hash, then txid, then masternode ProTxHash
- Starts with `X`, `Y`, `7`, or `8` → address

```json
{
  "type": "block",
  "value": "0000006b..."
}
```

Possible `type` values: `block`, `tx`, `address`, `masternode`, `none`

---

## WebSocket

### `WS /api/ws`

Live event stream. Connect via WebSocket upgrade.

```javascript
const ws = new WebSocket('wss://dashbook.testnet.networks.dash.org/api/ws');
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log(data.type, data.data);
};
```

#### Event: `NewBlock`

Sent when a new block is detected (polled every 2 seconds).

```json
{
  "type": "NewBlock",
  "data": {
    "hash": "0000006b...",
    "height": 1429566,
    "time": 1772294200,
    "n_tx": 1,
    "chainlock": true,
    "credit_pool_balance": 277109.649
  }
}
```

#### Event: `MempoolUpdate`

Sent when mempool size changes (polled every 10 seconds).

```json
{
  "type": "MempoolUpdate",
  "data": {
    "size": 5,
    "bytes": 1234,
    "total_fee": 0.00005
  }
}
```
