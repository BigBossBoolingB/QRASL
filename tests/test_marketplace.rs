use qrasl::primitives::{Address, Transaction};
use qrasl::state::StateMachine;
use sled::Config;

#[test]
fn test_list_and_buy_nft() {
    let db = Config::new().temporary(true).open().unwrap();
    let mut state_machine = StateMachine::new(db);
    let seller: Address = [0; 32];
    let buyer: Address = [1; 32];
    state_machine.db.insert(b"balance_of_seller", &bincode::serialize(&100u128).unwrap()).unwrap();
    state_machine.db.insert(b"balance_of_buyer", &bincode::serialize(&100u128).unwrap()).unwrap();

    // Mint NFT
    let mint_tx = Transaction {
        sender: seller,
        signature: [0; 64],
        recipient: [2; 32],
        value: 0,
        payload: "MINT_NFT:1:1:http://example.com/nft1".into(),
        gas_limit: 0,
        fees: 0,
    };
    assert!(state_machine.process_transaction(&mint_tx).is_ok());

    // List NFT
    let list_tx = Transaction {
        sender: seller,
        signature: [0; 64],
        recipient: [3; 32], // Marketplace address
        value: 0,
        payload: "LIST_NFT:1:1:50".into(),
        gas_limit: 0,
        fees: 0,
    };
    assert!(state_machine.process_transaction(&list_tx).is_ok());

    // Buy NFT
    let buy_tx = Transaction {
        sender: buyer,
        signature: [0; 64],
        recipient: [3; 32], // Marketplace address
        value: 0,
        payload: "BUY_NFT:1:1".into(),
        gas_limit: 0,
        fees: 0,
    };
    assert!(state_machine.process_transaction(&buy_tx).is_ok());
}
