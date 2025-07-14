use crate::primitives::{CrossShardMessage, Hash};
use std::collections::HashMap;

pub struct BeaconChain {
    finalized_shard_blocks: HashMap<u64, Vec<Hash>>,
    message_queues: HashMap<u64, Vec<CrossShardMessage>>,
}

impl BeaconChain {
    pub fn new() -> Self {
        Self {
            finalized_shard_blocks: HashMap::new(),
            message_queues: HashMap::new(),
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

    pub fn submit_cross_shard_message(&mut self, message: CrossShardMessage) {
        let queue = self.message_queues.entry(message.target_shard_id).or_default();
        queue.push(message);
        println!(
            "Beacon Chain: Queued message from Shard {} to Shard {}",
            message.source_shard_id, message.target_shard_id
        );
    }

    pub fn get_messages_for_shard(&mut self, shard_id: u64) -> Vec<CrossShardMessage> {
        self.message_queues.remove(&shard_id).unwrap_or_default()
    }

    pub fn assign_validators_for_epoch(&self, shard_id: u64) {
        // Placeholder for validator assignment logic
        println!(
            "Beacon Chain: Assigning validators for Shard {} for the next epoch.",
            shard_id
        );
    }
}
