use crate::primitives::Address;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NftId {
    pub collection_id: u64,
    pub token_id: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct NonFungibleToken {
    pub id: NftId,
    pub owner: Address,
    pub metadata_uri: String,
}
