//! The `node` module contains the `ShardNode` struct, which represents a single shard node.

use crate::core::mempool::Mempool;
use crate::core::state::ShardState;
use crate::core::types::SimplerAdaptiveDAGBlock;
use crate::crypto;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const NODE_STATE_FILE: &str = "node_state.json";
const GENESIS_KEYPAIR_FILE: &str = "genesis_keypair.json";

/// Represents a single shard node.
#[derive(Debug, Serialize, Deserialize)]
pub struct ShardNode {
    pub state: ShardState,
    pub mempool: Mempool,
    pub chain: Vec<SimplerAdaptiveDAGBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GenesisKeys {
    #[serde(with = "serde_bytes")]
    pk: crypto::PublicKeyBytes,
    #[serde(with = "serde_bytes")]
    sk: crypto::SecretKeyBytes,
}

impl ShardNode {
    /// Creates a new shard node with a genesis block.
    pub fn new() -> Self {
        let mut state = ShardState::new();
        let (genesis_pk, _) = Self::get_genesis_keypair();
        state.set_balance(genesis_pk, 1_000_000);

        let genesis_block = SimplerAdaptiveDAGBlock::new(vec![], vec![], [0; 32]);

        Self {
            state,
            mempool: Mempool::new(),
            chain: vec![genesis_block],
        }
    }

    // This is a temporary way to have a consistent genesis account for testing.
    pub fn get_genesis_keypair() -> (crypto::PublicKeyBytes, crypto::SecretKeyBytes) {
        if Path::new(GENESIS_KEYPAIR_FILE).exists() {
            let serialized = fs::read_to_string(GENESIS_KEYPAIR_FILE).expect("Failed to read genesis keypair file");
            let keys: GenesisKeys = serde_json::from_str(&serialized).expect("Failed to deserialize genesis keypair");
            (keys.pk, keys.sk)
        } else {
            let (pk, sk) = crypto::generate_keypair();
            let keys = GenesisKeys { pk, sk };
            let serialized = serde_json::to_string_pretty(&keys).expect("Failed to serialize genesis keypair");
            fs::write(GENESIS_KEYPAIR_FILE, serialized).expect("Failed to write genesis keypair file");
            (pk, sk)
        }
    }

    /// Saves the node's state to a file.
    pub fn save(&self) -> Result<(), std::io::Error> {
        let serialized = serde_json::to_string_pretty(self)?;
        fs::write(NODE_STATE_FILE, serialized)
    }

    /// Loads the node's state from a file, or creates a new one if it doesn't exist.
    pub fn load() -> Self {
        if Path::new(NODE_STATE_FILE).exists() {
            let serialized = fs::read_to_string(NODE_STATE_FILE).expect("Failed to read node state file");
            serde_json::from_str(&serialized).expect("Failed to deserialize node state")
        } else {
            Self::new()
        }
    }

    /// Produces a new block from the transactions in the mempool.
    pub fn produce_block(&mut self) -> Result<(), &'static str> {
        let transactions = self.mempool.get_transactions(10); // Arbitrary limit
        if transactions.is_empty() {
            return Err("No transactions in mempool to produce a block");
        }

        // For a DAG, we could have multiple parents. For simplicity, we'll use the last block.
        let parent_hashes = self.chain.last().map_or(vec![], |b| vec![b.block_hash]);

        // In a real implementation, the state root would be a Merkle root of the state.
        // For now, we'll use a placeholder.
        let state_root = [0; 32];

        let new_block = SimplerAdaptiveDAGBlock::new(parent_hashes, transactions, state_root);

        // Validate the new block against the current state
        if let Err(e) = crate::consensus::validation::validate_block(&new_block, &mut self.state) {
            // If validation fails, we should return the transactions to the mempool.
            // For now, we'll just error.
            return Err(e);
        }

        self.chain.push(new_block);

        Ok(())
    }
}

#[cfg(test)]
mod tests;
