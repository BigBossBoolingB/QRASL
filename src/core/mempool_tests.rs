use crate::core::mempool::Mempool;
use crate::core::types::Transaction;
use crate::crypto;

fn create_dummy_transaction() -> Transaction {
    let (pk, _) = crypto::generate_keypair();
    let (pk2, _) = crypto::generate_keypair();
    Transaction {
        sender: pk,
        recipient: pk2,
        amount: 10,
        timestamp: 0,
        signature: [0; crypto::SIGNATURE_LENGTH],
    }
}

#[test]
fn test_mempool_add_and_get_transactions() {
    let mut mempool = Mempool::new();

    let tx1 = create_dummy_transaction();
    let tx2 = create_dummy_transaction();
    let tx3 = create_dummy_transaction();

    mempool.add_transaction(tx1.clone());
    mempool.add_transaction(tx2.clone());
    mempool.add_transaction(tx3.clone());

    assert_eq!(mempool.len(), 3);

    let batch = mempool.get_transactions(2);
    assert_eq!(batch.len(), 2);
    assert_eq!(mempool.len(), 1);

    let remaining = mempool.get_transactions(2);
    assert_eq!(remaining.len(), 1);
    assert_eq!(mempool.len(), 0);
}
