use crate::primitives::{Address, Block, BlockHeader, Transaction};
use crate::state::StateMachine;
use anyhow::{Context, Result};
use sled::Db;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChainError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sled::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),
    #[error("Block validation failed: {0}")]
    ValidationFailed(String),
}

pub struct Chain {
    pub db: Db,
    pub state_machine: StateMachine,
    pub shard_id: u64,
}

impl Chain {
    pub fn new(shard_id: u64) -> Result<Self> {
        let db = sled::open(format!("db/shard_{}", shard_id))?;
        let state_machine = StateMachine::new(db.clone());

        if db.is_empty() {
            let genesis_block = Block::new(
                [0; 32],
                [0; 32],
                0,
                shard_id as u32,
                vec![0],
                vec![],
                vec![],
            );
            let block_hash = genesis_block.header.hash();
            db.insert(b"tip", &block_hash)?;
            db.insert(&block_hash, bincode::serialize(&genesis_block)?)?;
        }

        Ok(Self {
            db,
            state_machine,
            shard_id,
        })
    }

    pub fn add_block(&mut self, block: Block, active_validators: &[Address]) -> Result<()> {
        let tip_hash = self.db.get(b"tip")?.context("Failed to get tip hash")?;
        let last_block_bytes = self.db.get(&tip_hash)?.context("Failed to get last block")?;
        let last_block: Block = bincode::deserialize(&last_block_bytes)?;

        if block.header.parent_hash != last_block.header.hash() {
            return Err(ChainError::ValidationFailed(
                "Block's parent hash does not match the last block's hash".to_string(),
            )
            .into());
        }

        // Verify the block's signature
        let validator_address_vec = &block.header.nonce;
        let mut validator_address = [0u8; 32];
        validator_address.copy_from_slice(validator_address_vec);

        if !active_validators.contains(&validator_address) {
            return Err(ChainError::ValidationFailed(
                "Block signed by non-active validator".to_string(),
            )
            .into());
        }

        for tx in &block.transactions {
            if self.shard_id == 1
                && (String::from_utf8_lossy(&tx.payload).starts_with("PROPOSE")
                    || String::from_utf8_lossy(&tx.payload).starts_with("VOTE"))
            {
                self.state_machine
                    .process_governance_tx(tx, self.shard_id)?;
            } else {
                self.state_machine.process_transaction(tx)?;
            }
        }

        for msg in &block.cross_shard_messages {
            self.state_machine.process_cross_shard_message(msg)?;
        }

        let block_hash = block.header.hash();
        self.db.insert(&block_hash, bincode::serialize(&block)?)?;
        self.db.insert(b"tip", &block_hash)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::Block;

    #[test]
    fn test_new_chain() {
        let chain = Chain::new();
        assert_eq!(chain.blocks.len(), 1);
        assert!(chain.block_hashes.contains_key(&chain.blocks[0].header.hash()));
    }

    #[test]
    fn test_add_block_valid() {
        let mut chain = Chain::new();
        let last_block_hash = chain.blocks.last().unwrap().header.hash();

        let mut nonce = vec![];
        let mut block;
        loop {
            block = Block::new(
                last_block_hash,
                [1; 32],
                1,
                0,
                nonce.clone(),
                vec![],
            );
            if block.header.hash()[0] == 0 {
                break;
            }
            nonce.push(0);
        }

        assert!(chain.add_block(block).is_ok());
        assert_eq!(chain.blocks.len(), 2);
    }

    #[test]
    fn test_add_block_invalid_parent_hash() {
        let mut chain = Chain::new();
        let block = Block::new(
            [99; 32], // Invalid parent hash
            [1; 32],
            1,
            0,
            vec![0],
            vec![],
        );
        assert_eq!(
            chain.add_block(block),
            Err("Block's parent hash does not match the last block's hash")
        );
    }

    #[test]
    fn test_add_block_failed_consensus() {
        let mut chain = Chain::new();
        let last_block_hash = chain.blocks.last().unwrap().header.hash();
        let block = Block::new(
            last_block_hash,
            [1; 32],
            1,
            0,
            vec![0], // Nonce that won't meet consensus
            vec![],
        );
        if block.header.hash()[0] != 0 {
            assert_eq!(
                chain.add_block(block),
                Err("Block does not meet consensus rule")
            );
        }
    }
}
