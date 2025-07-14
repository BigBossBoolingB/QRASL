use crate::primitives::{Block, BlockHeader, Transaction};
use std::collections::HashMap;

pub struct Chain {
    pub blocks: Vec<Block>,
    pub block_hashes: HashMap<[u8; 32], usize>,
}

impl Chain {
    pub fn new() -> Self {
        let genesis_block = Block::new(
            [0; 32],
            [0; 32],
            0,
            0,
            vec![0],
            vec![],
        );
        let mut block_hashes = HashMap::new();
        block_hashes.insert(genesis_block.header.hash(), 0);

        Self {
            blocks: vec![genesis_block],
            block_hashes,
        }
    }

    pub fn add_block(&mut self, block: Block) -> Result<(), &'static str> {
        let last_block = self.blocks.last().ok_or("Chain has no blocks")?;
        if block.header.parent_hash != last_block.header.hash() {
            return Err("Block's parent hash does not match the last block's hash");
        }

        // Placeholder for consensus rule is now handled in the miner

        self.block_hashes.insert(block.header.hash(), self.blocks.len());
        self.blocks.push(block);
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
