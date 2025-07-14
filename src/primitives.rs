use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use ed25519_dalek::{Keypair, Signer, Verifier};

pub type Hash = [u8; 32];
pub type Address = [u8; 32];
pub type Signature = [u8; 64];

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BlockHeader {
    pub parent_hash: Hash,
    pub state_root: Hash,
    pub transactions_root: Hash,
    pub timestamp: u64,
    pub shard_id: u32,
    pub nonce: Vec<u8>,
}

impl BlockHeader {
    pub fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(self.parent_hash);
        hasher.update(self.state_root);
        hasher.update(self.transactions_root);
        hasher.update(self.timestamp.to_be_bytes());
        hasher.update(self.shard_id.to_be_bytes());
        hasher.update(&self.nonce);
        hasher.finalize().into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CrossShardMessage {
    pub source_shard_id: u64,
    pub target_shard_id: u64,
    pub payload: Vec<u8>,
}

use rs_merkle::{MerkleTree, algorithms::Sha256 as MerkleSha256};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub cross_shard_messages: Vec<CrossShardMessage>,
}

impl Block {
    pub fn new(
        parent_hash: Hash,
        state_root: Hash,
        timestamp: u64,
        shard_id: u32,
        nonce: Vec<u8>,
        transactions: Vec<Transaction>,
        cross_shard_messages: Vec<CrossShardMessage>,
    ) -> Self {
        let transactions_root = Self::calculate_transactions_root(&transactions);
        let header = BlockHeader {
            parent_hash,
            state_root,
            transactions_root,
            timestamp,
            shard_id,
            nonce,
        };
        Self {
            header,
            transactions,
            cross_shard_messages,
        }
    }

    fn calculate_transactions_root(transactions: &[Transaction]) -> Hash {
        let leaves: Vec<[u8; 32]> = transactions.iter().map(|tx| tx.hash()).collect();
        let merkle_tree = MerkleTree::<MerkleSha256>::from_leaves(&leaves);
        merkle_tree.root().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::Keypair;
    use rand::rngs::OsRng;

    #[test]
    fn test_block_header_hash() {
        let header = BlockHeader {
            parent_hash: [0; 32],
            state_root: [1; 32],
            transactions_root: [2; 32],
            timestamp: 3,
            shard_id: 4,
            nonce: vec![5],
        };
        let hash = header.hash();
        assert_ne!(hash, [0; 32]);
    }

    #[test]
    fn test_transaction_signing_and_verification() {
        let mut csprng = OsRng{};
        let keypair: Keypair = Keypair::generate(&mut csprng);

        let mut tx = Transaction {
            sender: keypair.public.to_bytes(),
            signature: [0; 64],
            recipient: [1; 32],
            value: 100,
            payload: vec![2, 3, 4],
            gas_limit: 50000,
            fees: 10,
        };

        tx.sign(&keypair);
        assert!(tx.verify());

        // Tamper with the transaction
        tx.value = 200;
        assert!(!tx.verify());
    }

    #[test]
    fn test_block_construction() {
        let mut csprng = OsRng{};
        let keypair1: Keypair = Keypair::generate(&mut csprng);
        let keypair2: Keypair = Keypair::generate(&mut csprng);

        let tx1 = Transaction {
            sender: keypair1.public.to_bytes(),
            signature: [0; 64],
            recipient: [1; 32],
            value: 100,
            payload: vec![2, 3, 4],
            gas_limit: 50000,
            fees: 10,
        };

        let tx2 = Transaction {
            sender: keypair2.public.to_bytes(),
            signature: [0; 64],
            recipient: [2; 32],
            value: 200,
            payload: vec![5, 6, 7],
            gas_limit: 60000,
            fees: 20,
        };

        let transactions = vec![tx1.clone(), tx2.clone()];
        let block = Block::new([0; 32], [1; 32], 2, 3, vec![4], transactions);

        let leaves: Vec<[u8; 32]> = vec![tx1.hash(), tx2.hash()];
        let merkle_tree = MerkleTree::<MerkleSha256>::from_leaves(&leaves);
        let expected_transactions_root = merkle_tree.root().unwrap_or_default();

        assert_eq!(block.header.transactions_root, expected_transactions_root);
    }
}
