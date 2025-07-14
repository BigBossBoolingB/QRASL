use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use ed25519_dalek::{Keypair, Signer, Verifier};

pub type Hash = [u8; 32];
pub type Address = [u8; 32];
pub type Signature = [u8; 64];

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BlockHeader {
    pub parent_hash: Hash,
    pub state_root: Hash,
    pub transactions_root: Hash,
    pub timestamp: u64,
    pub shard_id: u32,
    pub nonce: Vec<u8>,
}

impl BlockHeader {
    pub fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        hasher.update(self.parent_hash);
        hasher.update(self.state_root);
        hasher.update(self.transactions_root);
        hasher.update(self.timestamp.to_be_bytes());
        hasher.update(self.shard_id.to_be_bytes());
        hasher.update(&self.nonce);
        hasher.finalize().into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub sender: Address,
    pub signature: Signature,
    pub recipient: Address,
    pub value: u128,
    pub payload: Vec<u8>,
    pub gas_limit: u64,
    pub fees: u64,
}

impl Transaction {
    pub fn sign(&mut self, keypair: &Keypair) {
        let message = self.hash();
        self.signature = keypair.sign(&message).to_bytes();
    }

    pub fn verify(&self) -> bool {
        let message = self.hash();
        let public_key = ed25519_dalek::PublicKey::from_bytes(&self.sender).unwrap();
        public_key.verify(&message, &ed25519_dalek::Signature::from_bytes(&self.signature).unwrap()).is_ok()
    }

    fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.recipient);
        hasher.update(self.value.to_be_bytes());
        hasher.update(&self.payload);
        hasher.update(self.gas_limit.to_be_bytes());
        hasher.update(self.fees.to_be_bytes());
        hasher.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::Keypair;
    use rand::rngs::OsRng;

    #[test]
    fn test_block_header_hash() {
        let header = BlockHeader {
            parent_hash: [0; 32],
            state_root: [1; 32],
            transactions_root: [2; 32],
            timestamp: 3,
            shard_id: 4,
            nonce: vec![5],
        };
        let hash = header.hash();
        assert_ne!(hash, [0; 32]);
    }

    #[test]
    fn test_transaction_signing_and_verification() {
        let mut csprng = OsRng{};
        let keypair: Keypair = Keypair::generate(&mut csprng);

        let mut tx = Transaction {
            sender: keypair.public.to_bytes(),
            signature: [0; 64],
            recipient: [1; 32],
            value: 100,
            payload: vec![2, 3, 4],
            gas_limit: 50000,
            fees: 10,
        };

        tx.sign(&keypair);
        assert!(tx.verify());

        // Tamper with the transaction
        tx.value = 200;
        assert!(!tx.verify());
    }
}
