use serde::{Deserialize, Serialize};

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
