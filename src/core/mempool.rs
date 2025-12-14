//! The `mempool` module contains the `Mempool` struct for storing pending transactions.

use crate::core::types::Transaction;
use std::collections::VecDeque;

/// A simple in-memory mempool for pending transactions.
#[derive(Debug, Default)]
pub struct Mempool {
    transactions: VecDeque<Transaction>,
}

impl Mempool {
    /// Creates a new, empty mempool.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a transaction to the mempool.
    /// In a real implementation, this would involve validation.
    pub fn add_transaction(&mut self, tx: Transaction) {
        self.transactions.push_back(tx);
    }

    /// Retrieves up to `max_count` transactions from the mempool.
    pub fn get_transactions(&mut self, max_count: usize) -> Vec<Transaction> {
        self.transactions.drain(..std::cmp::min(max_count, self.transactions.len())).collect()
    }

    /// Returns the number of transactions in the mempool.
    pub fn len(&self) -> usize {
        self.transactions.len()
    }
}
