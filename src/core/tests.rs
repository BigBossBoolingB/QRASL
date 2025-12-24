use crate::core::types::{SimplerAdaptiveDAGBlock, Transaction};
use crate::crypto::{self, sign};

#[test]
fn test_transaction_signing_bytes() {
    let (pk, _) = crypto::generate_keypair();
    let (pk2, _) = crypto::generate_keypair();

    let tx = Transaction {
        sender: pk,
        recipient: pk2,
        amount: 100,
        timestamp: 1234567890,
        signature: [0; crypto::SIGNATURE_LENGTH],
    };

    let bytes = tx.to_signable_bytes();
    assert!(!bytes.is_empty());
}

#[test]
fn test_block_creation_and_hash() {
    let (pk, sk) = crypto::generate_keypair();
    let (pk2, _) = crypto::generate_keypair();

    let mut tx = Transaction {
        sender: pk,
        recipient: pk2,
        amount: 100,
        timestamp: 1234567890,
        signature: [0; crypto::SIGNATURE_LENGTH],
    };
    let signable_bytes = tx.to_signable_bytes();
    tx.signature = sign(&signable_bytes, &sk);

    let parent_hash = crypto::hash(b"genesis");
    let state_root = crypto::hash(b"state");

    let block = SimplerAdaptiveDAGBlock::new(vec![parent_hash], vec![tx], state_root);

    assert_eq!(block.parent_hashes.len(), 1);
    assert_eq!(block.transactions.len(), 1);
    assert_ne!(block.block_hash, [0; 32]);

    // Verify that the hash is deterministic
    let block2 = SimplerAdaptiveDAGBlock {
        parent_hashes: block.parent_hashes.clone(),
        timestamp: block.timestamp,
        transactions: block.transactions.clone(),
        state_root: block.state_root,
        block_hash: [0; 32],
    };
    let hash2 = block2.calculate_hash();
    assert_eq!(block.block_hash, hash2);
}
