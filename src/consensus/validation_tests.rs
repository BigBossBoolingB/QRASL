use crate::consensus::validation::validate_block;
use crate::core::state::ShardState;
use crate::core::types::{SimplerAdaptiveDAGBlock, Transaction};
use crate::crypto::{self, sign};

#[test]
fn test_validate_and_apply_valid_transaction() {
    let mut state = ShardState::new();
    let (pk, sk) = crypto::generate_keypair();
    let (pk2, _) = crypto::generate_keypair();
    state.set_balance(pk, 100);

    let mut tx = Transaction {
        sender: pk,
        recipient: pk2,
        amount: 50,
        timestamp: 0,
        signature: [0; crypto::SIGNATURE_LENGTH],
    };
    tx.signature = sign(&tx.to_signable_bytes(), &sk);

    assert!(state.validate_and_apply_transaction(&tx).is_ok());
    assert_eq!(state.get_balance(&pk), 50);
    assert_eq!(state.get_balance(&pk2), 50);
}

#[test]
fn test_validate_insufficient_funds() {
    let mut state = ShardState::new();
    let (pk, sk) = crypto::generate_keypair();
    let (pk2, _) = crypto::generate_keypair();
    state.set_balance(pk, 20);

    let mut tx = Transaction {
        sender: pk,
        recipient: pk2,
        amount: 50,
        timestamp: 0,
        signature: [0; crypto::SIGNATURE_LENGTH],
    };
    tx.signature = sign(&tx.to_signable_bytes(), &sk);

    assert_eq!(state.validate_and_apply_transaction(&tx), Err("Insufficient funds"));
    assert_eq!(state.get_balance(&pk), 20);
}

#[test]
fn test_validate_invalid_signature() {
    let mut state = ShardState::new();
    let (pk, _) = crypto::generate_keypair();
    let (pk2, _) = crypto::generate_keypair();
    let (_, sk_fake) = crypto::generate_keypair();
    state.set_balance(pk, 100);

    let mut tx = Transaction {
        sender: pk,
        recipient: pk2,
        amount: 50,
        timestamp: 0,
        signature: [0; crypto::SIGNATURE_LENGTH],
    };
    tx.signature = sign(&tx.to_signable_bytes(), &sk_fake);

    assert_eq!(state.validate_and_apply_transaction(&tx), Err("Invalid signature"));
    assert_eq!(state.get_balance(&pk), 100);
}

#[test]
fn test_validate_valid_block() {
    let mut state = ShardState::new();
    let (pk, sk) = crypto::generate_keypair();
    let (pk2, _) = crypto::generate_keypair();
    state.set_balance(pk, 100);

    let mut tx = Transaction {
        sender: pk,
        recipient: pk2,
        amount: 50,
        timestamp: 0,
        signature: [0; crypto::SIGNATURE_LENGTH],
    };
    tx.signature = sign(&tx.to_signable_bytes(), &sk);

    let block = SimplerAdaptiveDAGBlock::new(vec![], vec![tx], [0; 32]);

    assert!(validate_block(&block, &mut state).is_ok());
    assert_eq!(state.get_balance(&pk), 50);
}

#[test]
fn test_validate_block_with_invalid_tx() {
    let mut state = ShardState::new();
    let (pk, sk) = crypto::generate_keypair();
    let (pk2, _) = crypto::generate_keypair();
    state.set_balance(pk, 100);

    let mut tx_valid = Transaction {
        sender: pk,
        recipient: pk2,
        amount: 50,
        timestamp: 0,
        signature: [0; crypto::SIGNATURE_LENGTH],
    };
    tx_valid.signature = sign(&tx_valid.to_signable_bytes(), &sk);

    let mut tx_invalid = Transaction {
        sender: pk,
        recipient: pk2,
        amount: 100, // Insufficient funds
        timestamp: 1,
        signature: [0; crypto::SIGNATURE_LENGTH],
    };
    tx_invalid.signature = sign(&tx_invalid.to_signable_bytes(), &sk);

    let block = SimplerAdaptiveDAGBlock::new(vec![], vec![tx_valid, tx_invalid], [0; 32]);

    assert!(validate_block(&block, &mut state).is_err());
    // State should not have changed
    assert_eq!(state.get_balance(&pk), 100);
}
