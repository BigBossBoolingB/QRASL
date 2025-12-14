//! The `types` module contains the core data structures for the QRASL blockchain.

use crate::crypto::{PublicKeyBytes, SignatureBytes};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// A simple representation of a transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(with = "serde_bytes")]
    pub sender: PublicKeyBytes,
    #[serde(with = "serde_bytes")]
    pub recipient: PublicKeyBytes, // For simplicity, using PublicKeyBytes as address
    pub amount: u64,
    pub timestamp: u64,
    #[serde(with = "serde_bytes")]
    pub signature: SignatureBytes,
}

impl Transaction {
    /// Returns the byte representation of the transaction to be signed.
    pub fn to_signable_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.sender);
        bytes.extend_from_slice(&self.recipient);
        bytes.extend_from_slice(&self.amount.to_be_bytes());
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());
        bytes
    }
}

/// A block in the Simpler Adaptive DAG.
#[derive(Debug, Serialize, Deserialize)]
pub struct SimplerAdaptiveDAGBlock {
    pub parent_hashes: Vec<[u8; 32]>,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub state_root: [u8; 32], // A commitment to the state of the shard
    pub block_hash: [u8; 32],
}

impl SimplerAdaptiveDAGBlock {
    /// Creates a new block. The block hash is calculated upon creation.
    pub fn new(parent_hashes: Vec<[u8; 32]>, transactions: Vec<Transaction>, state_root: [u8; 32]) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut block = Self {
            parent_hashes,
            timestamp,
            transactions,
            state_root,
            block_hash: [0; 32], // Placeholder for hash calculation
        };
        block.block_hash = block.calculate_hash();
        block
    }

    /// Calculates the hash of the block header and transactions.
    pub fn calculate_hash(&self) -> [u8; 32] {
        let mut bytes = Vec::new();
        for parent_hash in &self.parent_hashes {
            bytes.extend_from_slice(parent_hash);
        }
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());
        // In a real implementation, this would be a Merkle root of the transactions.
        for tx in &self.transactions {
            bytes.extend_from_slice(&tx.to_signable_bytes());
            bytes.extend_from_slice(&tx.signature);
        }
        bytes.extend_from_slice(&self.state_root);

        crate::crypto::hash(&bytes)
    }
}
