use crate::primitives::{Address, CrossShardMessage, Transaction};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StateMachine {
    pub balances: HashMap<Address, u128>,
    pub contract_codes: HashMap<Address, Vec<u8>>,
    pub contract_storage: HashMap<Address, HashMap<[u8; 32], [u8; 32]>>,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
            contract_codes: HashMap::new(),
            contract_storage: HashMap::new(),
        }
    }

    pub fn process_transaction(&mut self, tx: &Transaction) -> Result<(), &'static str> {
        // Contract deployment
        if !tx.payload.is_empty() && tx.value == 0 {
            self.contract_codes.insert(tx.recipient, tx.payload.clone());
            println!("Deployed contract at address {:?}", tx.recipient);
            return Ok(());
        }

        if self.contract_codes.contains_key(&tx.recipient) {
            return self.process_contract_call(tx);
        }

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

    pub fn process_contract_call(&mut self, tx: &Transaction) -> Result<(), &'static str> {
        crate::vm::execute_contract(self, tx)
    }

    pub fn process_cross_shard_message(&mut self, msg: &CrossShardMessage) -> Result<(), &'static str> {
        // In a real implementation, the payload would be a more complex enum
        // For now, we'll assume it's a simple "recipient_address:amount" string
        let payload_str = String::from_utf8(msg.payload.clone()).unwrap();
        let parts: Vec<&str> = payload_str.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid cross-shard message payload");
        }

        let recipient_str = parts[0];
        let amount_str = parts[1];

        let recipient: Address = serde_json::from_str(recipient_str).unwrap();
        let amount: u128 = amount_str.parse().unwrap();

        let recipient_balance = self.balances.get(&recipient).cloned().unwrap_or(0);
        self.balances
            .insert(recipient, recipient_balance + amount);

        println!(
            "Processed cross-shard message: Credited {:?} with {}",
            recipient, amount
        );

        Ok(())
    }
}
