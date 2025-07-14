use crate::governance::{Proposal, Vote};
use crate::marketplace::Listing;
use crate::nft::{NftId, NonFungibleToken};
use crate::primitives::{Address, CrossShardMessage, Transaction};
use sled::Db;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StateMachine {
    db: Db,
}

impl StateMachine {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub fn process_transaction(&mut self, tx: &Transaction) -> Result<(), &'static str> {
        let payload_str = String::from_utf8(tx.payload.clone()).unwrap_or_default();
        if payload_str.starts_with("CREATE_COLLECTION") {
            return self.handle_create_collection(tx);
        } else if payload_str.starts_with("MINT_NFT") {
            return self.handle_mint_nft(tx);
        } else if payload_str.starts_with("TRANSFER_NFT") {
            return self.handle_transfer_nft(tx);
        } else if payload_str.starts_with("LIST_NFT") {
            return self.handle_list_nft(tx);
        } else if payload_str.starts_with("CANCEL_LISTING") {
            return self.handle_cancel_listing(tx);
        } else if payload_str.starts_with("BUY_NFT") {
            return self.handle_buy_nft(tx);
        }

        // Contract deployment
        if !tx.payload.is_empty() && tx.value == 0 {
            self.db.insert(b"contract_codes", bincode::serialize(&tx.recipient).unwrap()).unwrap();
            println!("Deployed contract at address {:?}", tx.recipient);
            return Ok(());
        }

        if self.db.contains_key(b"contract_codes").unwrap() {
            return self.process_contract_call(tx);
        }

        let sender_balance: u128 = self
            .db
            .get(&bincode::serialize(&tx.sender).unwrap())
            .unwrap()
            .map(|v| bincode::deserialize(&v).unwrap())
            .unwrap_or(0);

        if sender_balance < tx.value {
            return Err("Insufficient funds");
        }

        let recipient_balance: u128 = self
            .db
            .get(&bincode::serialize(&tx.recipient).unwrap())
            .unwrap()
            .map(|v| bincode::deserialize(&v).unwrap())
            .unwrap_or(0);

        self.db
            .insert(
                &bincode::serialize(&tx.sender).unwrap(),
                bincode::serialize(&(sender_balance - tx.value)).unwrap(),
            )
            .unwrap();
        self.db
            .insert(
                &bincode::serialize(&tx.recipient).unwrap(),
                bincode::serialize(&(recipient_balance + tx.value)).unwrap(),
            )
            .unwrap();

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

    pub fn handle_create_collection(&mut self, tx: &Transaction) -> Result<(), &'static str> {
        // In a real implementation, we would have a more robust way of managing collections
        println!("Collection created by {:?}", tx.sender);
        Ok(())
    }

    pub fn handle_mint_nft(&mut self, tx: &Transaction) -> Result<(), &'static str> {
        let payload_str = String::from_utf8(tx.payload.clone()).unwrap();
        let parts: Vec<&str> = payload_str.split(':').collect();
        let collection_id: u64 = parts[0].parse().unwrap();
        let token_id: u64 = parts[1].parse().unwrap();
        let metadata_uri = parts[2].to_string();

        let nft_id = NftId {
            collection_id,
            token_id,
        };
        let nft = NonFungibleToken {
            id: nft_id.clone(),
            owner: tx.sender,
            metadata_uri,
        };

        self.db
            .insert(
                &bincode::serialize(&nft_id).unwrap(),
                bincode::serialize(&nft).unwrap(),
            )
            .unwrap();
        println!("NFT {:?} minted by {:?}", nft_id, tx.sender);
        Ok(())
    }

    pub fn handle_transfer_nft(&mut self, tx: &Transaction) -> Result<(), &'static str> {
        let payload_str = String::from_utf8(tx.payload.clone()).unwrap();
        let parts: Vec<&str> = payload_str.split(':').collect();
        let collection_id: u64 = parts[0].parse().unwrap();
        let token_id: u64 = parts[1].parse().unwrap();
        let recipient: Address = serde_json::from_str(parts[2]).unwrap();

        let nft_id = NftId {
            collection_id,
            token_id,
        };

        if let Some(nft_bytes) = self.db.get(&bincode::serialize(&nft_id).unwrap()).unwrap() {
            let mut nft: NonFungibleToken = bincode::deserialize(&nft_bytes).unwrap();
            if nft.owner != tx.sender {
                return Err("Transaction sender is not the owner of the NFT");
            }
            nft.owner = recipient;
            self.db
                .insert(
                    &bincode::serialize(&nft_id).unwrap(),
                    bincode::serialize(&nft).unwrap(),
                )
                .unwrap();
            println!("NFT {:?} transferred to {:?}", nft_id, recipient);
            Ok(())
        } else {
            Err("NFT not found")
        }
    }

    pub fn handle_list_nft(&mut self, tx: &Transaction) -> Result<(), &'static str> {
        let payload_str = String::from_utf8(tx.payload.clone()).unwrap();
        let parts: Vec<&str> = payload_str.split(':').collect();
        let collection_id: u64 = parts[1].parse().unwrap();
        let token_id: u64 = parts[2].parse().unwrap();
        let price: u128 = parts[3].parse().unwrap();

        let nft_id = NftId {
            collection_id,
            token_id,
        };

        if let Some(nft_bytes) = self.db.get(&bincode::serialize(&nft_id).unwrap()).unwrap() {
            let mut nft: NonFungibleToken = bincode::deserialize(&nft_bytes).unwrap();
            if nft.owner != tx.sender {
                return Err("Transaction sender is not the owner of the NFT");
            }

            // Escrow NFT
            let marketplace_address: Address = [0; 32]; // Special address for the marketplace
            nft.owner = marketplace_address;
            self.db
                .insert(
                    &bincode::serialize(&nft_id).unwrap(),
                    bincode::serialize(&nft).unwrap(),
                )
                .unwrap();

            let listing = Listing {
                nft_id: nft_id.clone(),
                seller: tx.sender,
                price,
            };
            self.db
                .insert(
                    &bincode::serialize(&nft_id).unwrap(),
                    bincode::serialize(&listing).unwrap(),
                )
                .unwrap();
            println!("NFT {:?} listed for sale by {:?}", nft_id, tx.sender);
            Ok(())
        } else {
            Err("NFT not found")
        }
    }

    pub fn handle_cancel_listing(&mut self, tx: &Transaction) -> Result<(), &'static str> {
        // ... implementation for cancelling a listing
        Ok(())
    }

    pub fn handle_buy_nft(&mut self, tx: &Transaction) -> Result<(), &'static str> {
        let payload_str = String::from_utf8(tx.payload.clone()).unwrap();
        let parts: Vec<&str> = payload_str.split(':').collect();
        let collection_id: u64 = parts[1].parse().unwrap();
        let token_id: u64 = parts[2].parse().unwrap();

        let nft_id = NftId {
            collection_id,
            token_id,
        };

        if let Some(listing_bytes) = self.db.get(&bincode::serialize(&nft_id).unwrap()).unwrap() {
            let listing: Listing = bincode::deserialize(&listing_bytes).unwrap();
            let buyer_balance: u128 = self
                .db
                .get(&bincode::serialize(&tx.sender).unwrap())
                .unwrap()
                .map(|v| bincode::deserialize(&v).unwrap())
                .unwrap_or(0);

            if buyer_balance < listing.price {
                return Err("Insufficient funds");
            }

            // Atomic swap
            let seller_balance: u128 = self
                .db
                .get(&bincode::serialize(&listing.seller).unwrap())
                .unwrap()
                .map(|v| bincode::deserialize(&v).unwrap())
                .unwrap_or(0);

            self.db
                .insert(
                    &bincode::serialize(&tx.sender).unwrap(),
                    bincode::serialize(&(buyer_balance - listing.price)).unwrap(),
                )
                .unwrap();
            self.db
                .insert(
                    &bincode::serialize(&listing.seller).unwrap(),
                    bincode::serialize(&(seller_balance + listing.price)).unwrap(),
                )
                .unwrap();

            if let Some(nft_bytes) = self.db.get(&bincode::serialize(&nft_id).unwrap()).unwrap() {
                let mut nft: NonFungibleToken = bincode::deserialize(&nft_bytes).unwrap();
                nft.owner = tx.sender;
                self.db
                    .insert(
                        &bincode::serialize(&nft_id).unwrap(),
                        bincode::serialize(&nft).unwrap(),
                    )
                    .unwrap();
                println!("NFT {:?} sold to {:?}", nft_id, tx.sender);
                Ok(())
            } else {
                Err("NFT not found after purchase, this should not happen")
            }
        } else {
            Err("Listing not found")
        }
    }
}
