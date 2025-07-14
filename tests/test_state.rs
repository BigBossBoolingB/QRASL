use qrasl::primitives::{Address, Transaction};
use qrasl::state::StateMachine;
use std::collections::HashMap;

#[test]
fn test_process_transaction_valid() {
    let mut state_machine = StateMachine::new();
    let sender: Address = [0; 32];
    let recipient: Address = [1; 32];
    state_machine.balances.insert(sender, 100);

    let tx = Transaction {
        sender,
        signature: [0; 64],
        recipient,
        value: 50,
        payload: vec![],
        gas_limit: 0,
        fees: 0,
    };

    assert!(state_machine.process_transaction(&tx).is_ok());
    assert_eq!(state_machine.balances.get(&sender), Some(&50));
    assert_eq!(state_machine.balances.get(&recipient), Some(&50));
}

#[test]
fn test_process_transaction_insufficient_funds() {
    let mut state_machine = StateMachine::new();
    let sender: Address = [0; 32];
    let recipient: Address = [1; 32];
    state_machine.balances.insert(sender, 20);

    let tx = Transaction {
        sender,
        signature: [0; 64],
        recipient,
        value: 50,
        payload: vec![],
        gas_limit: 0,
        fees: 0,
    };

    assert_eq!(
        state_machine.process_transaction(&tx),
        Err("Insufficient funds")
    );
    assert_eq!(state_machine.balances.get(&sender), Some(&20));
    assert_eq!(state_machine.balances.get(&recipient), None);
}
