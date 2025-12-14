//! The `state` module contains the `ShardState` struct, which manages the state of a shard.

use crate::core::types::Transaction;
use crate::crypto::PublicKeyBytes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the state of a single shard.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ShardState {
    #[serde(with = "crate::core::serde_helpers::hashmap_as_vec")]
    balances: HashMap<PublicKeyBytes, u64>,
}

impl ShardState {
    /// Creates a new, empty shard state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the balance of a given public key.
    pub fn get_balance(&self, pk: &PublicKeyBytes) -> u64 {
        self.balances.get(pk).copied().unwrap_or(0)
    }

    /// Sets the balance for a given public key. (Used for testing/genesis)
    pub fn set_balance(&mut self, pk: PublicKeyBytes, balance: u64) {
        self.balances.insert(pk, balance);
    }

    /// Applies a transaction to the shard state.
    /// This is a simplified application that does not include validation.
    fn apply_transaction(&mut self, tx: &Transaction) {
        // Decrement sender's balance
        let sender_balance = self.get_balance(&tx.sender);
        self.balances.insert(tx.sender, sender_balance - tx.amount);

        // Increment recipient's balance
        let recipient_balance = self.get_balance(&tx.recipient);
        self.balances.insert(tx.recipient, recipient_balance + tx.amount);
    }

    /// Validates a transaction and applies it to the shard state if valid.
    pub fn validate_and_apply_transaction(&mut self, tx: &Transaction) -> Result<(), &'static str> {
        // Check signature
        if !crate::crypto::verify(&tx.signature, &tx.to_signable_bytes(), &tx.sender) {
            return Err("Invalid signature");
        }

        // Check for sufficient funds
        let sender_balance = self.get_balance(&tx.sender);
        if sender_balance < tx.amount {
            return Err("Insufficient funds");
        }

        self.apply_transaction(tx);
        Ok(())
    }
}
