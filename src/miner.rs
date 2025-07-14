use crate::primitives::{Block, CrossShardMessage, Transaction};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn mine_block(
    last_block: &Block,
    transactions: Vec<Transaction>,
    cross_shard_messages: Vec<CrossShardMessage>,
    difficulty: u32,
) -> Block {
    let mut nonce = 0;
    loop {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let block = Block::new(
            last_block.header.hash(),
            [0; 32], // Placeholder state root
            timestamp,
            0, // Placeholder shard ID
            nonce.to_string().into_bytes(),
            transactions.clone(),
            cross_shard_messages.clone(),
        );

        let hash = block.header.hash();
        let target = u32::MAX >> difficulty;
        let hash_value = u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]]);

        if hash_value < target {
            return block;
        }
        nonce += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::Block;

    #[test]
    fn test_mine_block() {
        let genesis_block = Block::new(
            [0; 32],
            [0; 32],
            0,
            0,
            vec![0],
            vec![],
        );
        let difficulty = 8;
        let block = mine_block(&genesis_block, vec![], difficulty);
        let hash = block.header.hash();
        let target = u32::MAX >> difficulty;
        let hash_value = u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]]);
        assert!(hash_value < target);
    }
}
