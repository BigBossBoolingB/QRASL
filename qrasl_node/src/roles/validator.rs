//! Validator Role
//!
//! Responsible for participating in consensus, proposing blocks,
//! and validating transactions and state transitions.

// --- Core Data Structures ---

/// A simplified representation of a transaction.
#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
}

/// The header of a block, containing metadata.
#[derive(Debug, Clone)]
pub struct BlockHeader {
    pub block_number: u64,
    pub previous_hash: [u8; 32],
    pub transactions_root: [u8; 32], // A Merkle root of the transactions
    pub timestamp: u64,
}

/// A full block in the QRASL chain.
#[derive(Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub signature: Option<[u8; 64]>, // Signed by the proposing validator
}

// --- Validator Implementation ---

/// Represents a Validator node in the QRASL network.
pub struct Validator {
    pub id: u64,
    stake: u128,
    // In a real implementation, this would be a secure private key.
    // Here, we just use a simple identifier for conceptual signing.
    secret_key: String,
}

/// The trait defining the core functionalities of a Validator.
pub trait ValidatorActions {
    /// Proposes a new block for a specific shard.
    fn propose_block(&self, previous_hash: [u8; 32], block_number: u64, transactions: Vec<Transaction>) -> Block;

    /// Validates a received block.
    fn validate_block(&self, block: &Block) -> bool;

    /// Conceptually "signs" data with the validator's key.
    fn sign_data(&self, data: &[u8]) -> [u8; 64];
}

impl ValidatorActions for Validator {
    /// Conceptually creates and signs a new block.
    fn propose_block(&self, previous_hash: [u8; 32], block_number: u64, transactions: Vec<Transaction>) -> Block {
        println!("Validator {}: Proposing new block #{}...", self.id, block_number);

        let header = BlockHeader {
            block_number,
            previous_hash,
            transactions_root: [0; 32], // Placeholder Merkle root
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        };

        let mut block = Block {
            header: header.clone(),
            transactions,
            signature: None,
        };

        // Conceptually sign the block header
        let header_bytes = bincode::serialize(&header).unwrap();
        block.signature = Some(self.sign_data(&header_bytes));

        block
    }

    /// Validates the integrity and signature of a block.
    fn validate_block(&self, block: &Block) -> bool {
        println!("Validator {}: Validating block #{}...", self.id, block.header.block_number);
        // In a real implementation, this would also check state transitions, proofs, etc.

        // For this conceptual test, we only check if the signature exists.
        // A real implementation would perform a full cryptographic verification.
        block.signature.is_some()
    }

    /// A conceptual representation of signing.
    fn sign_data(&self, data: &[u8]) -> [u8; 64] {
        // This is not a real cryptographic signature.
        // It's a placeholder to represent the signing action.
        let mut signature = [0u8; 64];
        let key_bytes = self.secret_key.as_bytes();
        for i in 0..data.len() {
            signature[i % 64] = signature[i % 64].wrapping_add(data[i].wrapping_add(key_bytes[i % key_bytes.len()]));
        }
        signature
    }
}

// --- Unit Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_validator() -> Validator {
        Validator {
            id: 1,
            stake: 1000,
            secret_key: "my_secret_key".to_string(),
        }
    }

    #[test]
    fn test_validator_creation() {
        let validator = get_test_validator();
        assert_eq!(validator.id, 1);
        assert_eq!(validator.stake, 1000);
    }

    #[test]
    fn test_block_proposal() {
        let validator = get_test_validator();
        let transactions = vec![Transaction { from: "a".to_string(), to: "b".to_string(), amount: 10 }];
        let block = validator.propose_block([0; 32], 1, transactions);

        assert_eq!(block.header.block_number, 1);
        assert_eq!(block.transactions.len(), 1);
        assert!(block.signature.is_some(), "Block should be signed after proposal");
    }

    #[test]
    fn test_block_validation_success() {
        let validator = get_test_validator();
        let mut block = Block {
            header: BlockHeader { block_number: 1, previous_hash: [0;32], transactions_root: [0;32], timestamp: 0 },
            transactions: vec![],
            signature: None,
        };

        // Sign the block to make it valid for this conceptual test
        let header_bytes = bincode::serialize(&block.header).unwrap();
        block.signature = Some(validator.sign_data(&header_bytes));

        assert!(validator.validate_block(&block), "A signed block should be valid");
    }

    #[test]
    fn test_block_validation_failure_no_signature() {
        let validator = get_test_validator();
        let block = Block {
            header: BlockHeader { block_number: 1, previous_hash: [0;32], transactions_root: [0;32], timestamp: 0 },
            transactions: vec![],
            signature: None, // No signature
        };

        // In our conceptual test, validation fails if there's no signature.
        // A real implementation would have validate_block return a Result type with specific errors.
        // For now, we expect 'false'.
        assert!(!validator.validate_block(&block), "A block without a signature should be invalid");
    }
}
