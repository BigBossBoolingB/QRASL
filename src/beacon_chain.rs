use crate::primitives::Hash;
use std::collections::HashMap;

pub struct BeaconChain {
    finalized_shard_blocks: HashMap<u64, Vec<Hash>>,
}

impl BeaconChain {
    pub fn new() -> Self {
        Self {
            finalized_shard_blocks: HashMap::new(),
        }
    }

    pub fn submit_shard_checkpoint(&mut self, shard_id: u64, block_hash: Hash) {
        let shard_blocks = self.finalized_shard_blocks.entry(shard_id).or_default();
        shard_blocks.push(block_hash);
        println!(
            "Beacon Chain: Received checkpoint from Shard {} - Block Hash: {:?}",
            shard_id, block_hash
        );
    }

    pub fn assign_validators_for_epoch(&self, shard_id: u64) {
        // Placeholder for validator assignment logic
        println!(
            "Beacon Chain: Assigning validators for Shard {} for the next epoch.",
            shard_id
        );
    }
}
