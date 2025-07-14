# QRASL Core Data Structures

This document defines the fundamental data structures of the QRASL blockchain.

## BlockHeader

The `BlockHeader` is the immutable fingerprint of a moment in time on the ledger.

| Field | Type | Description |
|---|---|---|
| `parent_hash` | `Hash` | The cryptographic link to the previous block, forming the chain. |
| `state_root` | `Hash` | A Merkle root committing to the entire state of the shard. |
| `transactions_root` | `Hash` | A Merkle root committing to all transactions included in the block. |
| `timestamp` | `u64` | A secure, on-chain timestamp. |
| `shard_id` | `u32` | The identifier of the shard this block belongs to. |
| `nonce` / `seal` | `Vec<u8>` | A field for the consensus mechanism's proof. |

## Transaction

The `Transaction` is the fundamental unit of state change.

| Field | Type | Description |
|---|---|---|
| `sender` | `Address` | The originating account. |
| `signature` | `Signature` | The cryptographic proof of the sender's authorization. |
| `recipient` | `Address` | The destination account or contract. |
| `value` | `u128` | The amount of native currency being transferred. |
| `payload` | `Vec<u8>` | Arbitrary data for smart contract interactions. |
| `gas_limit` / `fees` | `u64` | Parameters for the economic model. |
