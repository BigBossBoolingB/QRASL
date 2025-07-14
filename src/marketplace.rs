use crate::nft::NftId;
use crate::primitives::Address;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Listing {
    pub nft_id: NftId,
    pub seller: Address,
    pub price: u128,
}
