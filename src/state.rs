use crate::primitives::{Address, Transaction};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StateMachine {
    pub balances: HashMap<Address, u128>,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }

    pub fn process_transaction(&mut self, tx: &Transaction) -> Result<(), &'static str> {
        let sender_balance = self.balances.get(&tx.sender).cloned().unwrap_or(0);
        if sender_balance < tx.value {
            return Err("Insufficient funds");
        }

        let recipient_balance = self.balances.get(&tx.recipient).cloned().unwrap_or(0);

        self.balances.insert(tx.sender, sender_balance - tx.value);
        self.balances
            .insert(tx.recipient, recipient_balance + tx.value);

        Ok(())
    }
}
