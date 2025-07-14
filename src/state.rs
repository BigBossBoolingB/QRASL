use crate::governance::{Proposal, Vote};
use crate::primitives::{Address, CrossShardMessage, Transaction};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StateMachine {
    pub balances: HashMap<Address, u128>,
    pub contract_codes: HashMap<Address, Vec<u8>>,
    pub contract_storage: HashMap<Address, HashMap<[u8; 32], [u8; 32]>>,
    pub proposals: HashMap<u64, Proposal>,
    next_proposal_id: u64,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
            contract_codes: HashMap::new(),
            contract_storage: HashMap::new(),
            proposals: HashMap::new(),
            next_proposal_id: 0,
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
        let payload_str = String::from_utf8(msg.payload.clone()).unwrap();
        let parts: Vec<&str> = payload_str.split(':').collect();

        match parts[0] {
            "PROPOSE" => {
                let description = parts[1].to_string();
                let proposal = Proposal {
                    id: self.next_proposal_id,
                    description,
                    aye_votes: 0,
                    nay_votes: 0,
                    executed: false,
                };
                self.proposals.insert(self.next_proposal_id, proposal);
                self.next_proposal_id += 1;
                println!("New proposal created via cross-shard message with ID {}", self.next_proposal_id - 1);
            }
            _ => {
                // Handle other cross-shard message types, e.g., balance transfers
                if parts.len() != 2 {
                    return Err("Invalid cross-shard message payload");
                }
                let recipient_str = parts[0];
                let amount_str = parts[1];
                let recipient: Address = serde_json::from_str(recipient_str).unwrap();
                let amount: u128 = amount_str.parse().unwrap();
                let recipient_balance = self.balances.get(&recipient).cloned().unwrap_or(0);
                self.balances.insert(recipient, recipient_balance + amount);
                println!("Processed cross-shard message: Credited {:?} with {}", recipient, amount);
            }
        }
        Ok(())
    }

    pub fn process_governance_tx(
        &mut self,
        tx: &Transaction,
        shard_id: u64,
    ) -> Result<(), &'static str> {
        if shard_id != 1 {
            return Err("Governance transactions can only be processed on Shard 1");
        }

        // Simplified: payload determines action
        // "PROPOSE:description"
        // "VOTE:proposal_id:AYE/NAY"
        let payload_str = String::from_utf8(tx.payload.clone()).unwrap();
        let parts: Vec<&str> = payload_str.split(':').collect();

        match parts[0] {
            "PROPOSE" => {
                let description = parts[1].to_string();
                let proposal = Proposal {
                    id: self.next_proposal_id,
                    description,
                    aye_votes: 0,
                    nay_votes: 0,
                    executed: false,
                };
                self.proposals.insert(self.next_proposal_id, proposal);
                self.next_proposal_id += 1;
                println!("New proposal created with ID {}", self.next_proposal_id - 1);
            }
            "VOTE" => {
                let proposal_id: u64 = parts[1].parse().unwrap();
                let vote_str = parts[2];
                let vote = match vote_str {
                    "AYE" => Vote::Aye,
                    "NAY" => Vote::Nay,
                    _ => Vote::Abstain,
                };

                if let Some(proposal) = self.proposals.get_mut(&proposal_id) {
                    match vote {
                        Vote::Aye => proposal.aye_votes += 1,
                        Vote::Nay => proposal.nay_votes += 1,
                        _ => {}
                    }
                    println!("Vote tallied for proposal {}", proposal_id);
                }
            }
            _ => return Err("Unknown governance action"),
        }

        Ok(())
    }
}
