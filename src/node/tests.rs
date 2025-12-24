use super::ShardNode;
use crate::core::types::Transaction;
use crate::crypto;
use std::fs;

#[test]
fn test_node_save_and_load() {
    let node = ShardNode::new();
    node.save().expect("Failed to save node");

    let loaded_node = ShardNode::load();
    assert_eq!(node.chain.len(), loaded_node.chain.len());
    assert_eq!(node.mempool.len(), loaded_node.mempool.len());

    fs::remove_file(super::NODE_STATE_FILE).expect("Failed to remove test node state file");
}

#[test]
fn test_produce_block() {
    let mut node = ShardNode::new();
    let (pk, sk) = crypto::generate_keypair();
    let (pk2, _) = crypto::generate_keypair();
    node.state.set_balance(pk, 100);

    let mut tx = Transaction {
        sender: pk,
        recipient: pk2,
        amount: 50,
        timestamp: 0,
        signature: [0; crypto::SIGNATURE_LENGTH],
    };
    tx.signature = crypto::sign(&tx.to_signable_bytes(), &sk);

    node.mempool.add_transaction(tx);
    assert_eq!(node.mempool.len(), 1);

    node.produce_block().expect("Failed to produce block");

    assert_eq!(node.chain.len(), 2);
    assert_eq!(node.mempool.len(), 0);
    assert_eq!(node.state.get_balance(&pk), 50);
}
