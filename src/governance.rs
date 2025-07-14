use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    pub id: u64,
    pub description: String,
    pub aye_votes: u64,
    pub nay_votes: u64,
    pub executed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Vote {
    Aye,
    Nay,
    Abstain,
}
