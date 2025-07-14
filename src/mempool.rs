use crate::primitives::{Address, Transaction};
use crate::state::StateMachine;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Mempool {
    transactions: HashMap<[u8; 32], Transaction>,
    state_machine: Arc<Mutex<StateMachine>>,
}

impl Mempool {
    pub fn new(state_machine: Arc<Mutex<StateMachine>>) -> Self {
        Self {
            transactions: HashMap::new(),
            state_machine,
        }
    }

    pub fn add_transaction(&mut self, tx: Transaction) -> Result<()> {
        if !tx.verify() {
            return Err(anyhow::anyhow!("Transaction signature is invalid"));
        }

        let mut state_machine = self.state_machine.lock().unwrap();
        let sender_balance: u128 = state_machine
            .db
            .get(&bincode::serialize(&tx.sender)?)?
            .map(|v| bincode::deserialize(&v).unwrap_or(0))
            .unwrap_or(0);

        if sender_balance < tx.value {
            return Err(anyhow::anyhow!("Insufficient funds for transaction"));
        }

        self.transactions.insert(tx.hash(), tx);
        Ok(())
    }

    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.transactions.values().cloned().collect()
    }

    pub fn clear(&mut self) {
        self.transactions.clear();
    }
}
