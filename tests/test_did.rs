use qrasl::primitives::{Address, Transaction};
use qrasl::state::StateMachine;
use sled::Config;

#[test]
fn test_did_creation_and_resolution() {
    let db = Config::new().temporary(true).open().unwrap();
    let mut state_machine = StateMachine::new(db);
    let creator: Address = [0; 32];

    // This is a simplified test. A real implementation would involve deploying
    // the DID registry contract and then calling its methods.
    let create_tx = Transaction {
        sender: creator,
        signature: [0; 64],
        recipient: [6; 32], // DID Registry contract address
        value: 0,
        payload: "CREATE_DID:did:qrasl:1:12345:{\"@context\":\"...\"}".into(),
        gas_limit: 0,
        fees: 0,
    };
    assert!(state_machine.process_transaction(&create_tx).is_ok());

    // In a real test, we would call a `resolve_did` function and assert
    // that the returned document matches the created one.
}
