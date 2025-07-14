use qrasl::primitives::{Address, Transaction};
use qrasl::state::StateMachine;
use sled::Config;

#[test]
fn test_mint_and_transfer_nft() {
    let db = Config::new().temporary(true).open().unwrap();
    let mut state_machine = StateMachine::new(db);
    let owner1: Address = [0; 32];
    let owner2: Address = [1; 32];

    // Mint NFT
    let mint_tx = Transaction {
        sender: owner1,
        signature: [0; 64],
        recipient: [2; 32],
        value: 0,
        payload: "MINT_NFT:1:1:http://example.com/nft1".into(),
        gas_limit: 0,
        fees: 0,
    };
    assert!(state_machine.process_transaction(&mint_tx).is_ok());

    // Transfer NFT
    let transfer_tx = Transaction {
        sender: owner1,
        signature: [0; 64],
        recipient: [2; 32],
        value: 0,
        payload: format!(
            "TRANSFER_NFT:1:1:{}",
            serde_json::to_string(&owner2).unwrap()
        )
        .into(),
        gas_limit: 0,
        fees: 0,
    };
    assert!(state_machine.process_transaction(&transfer_tx).is_ok());

    // Invalid transfer
    let invalid_transfer_tx = Transaction {
        sender: owner1, // owner1 no longer owns the NFT
        signature: [0; 64],
        recipient: [2; 32],
        value: 0,
        payload: format!(
            "TRANSFER_NFT:1:1:{}",
            serde_json::to_string(&owner1).unwrap()
        )
        .into(),
        gas_limit: 0,
        fees: 0,
    };
    assert!(state_machine
        .process_transaction(&invalid_transfer_tx)
        .is_err());
}
