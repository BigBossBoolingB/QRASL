use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

// --- Module Imports ---
mod beacon_chain;
mod chain;
mod governance;
mod mempool;
mod network;
mod nft;
mod marketplace;
mod primitives;
mod rpc;
mod staking;
mod state;
mod vm;

// --- Crate Imports ---
use crate::beacon_chain::BeaconChain;
use crate::chain::Chain;
use crate::primitives::{Block, Transaction, Nft, NftCollection, Listing, CrossShardMessage, Proposal, Vote};
use crate::state::StateMachine;
use crate::network::{Node, Event};
use crate::staking::{Validator, Nominator};

// --- Main Simulation Logic ---

async fn run_shard(
    shard_id: u64,
    mut node: Node,
    state_machine: Arc<Mutex<StateMachine>>,
    chain: Arc<Mutex<Chain>>,
    beacon_chain: Arc<Mutex<BeaconChain>>,
    keypair: primitives::Keypair, // Each node/shard runner needs its own identity
) -> Result<()> {
    println!("[Shard {}] Node starting up with PeerId: {}", shard_id, node.peer_id);

    loop {
        tokio::select! {
            // Handle incoming network events
            event = node.next_event() => {
                if let Some(event) = event {
                    match event {
                        Event::Block(block) => {
                            let mut chain_lock = chain.lock().unwrap();
                            let mut sm_lock = state_machine.lock().unwrap();
                            println!("[Shard {}] Received new block via gossip: {}", shard_id, block.header.height);
                            if chain_lock.add_block(block, &mut sm_lock).is_ok() {
                                println!("[Shard {}] Successfully added gossiped block to chain.", shard_id);
                            } else {
                                eprintln!("[Shard {}] Failed to add gossiped block to chain.", shard_id);
                            }
                        },
                        // Handle other event types like transactions, etc.
                        _ => {}
                    }
                }
            },
            // Block production logic for elected validators
            _ = sleep(Duration::from_secs(5)) => {
                let is_our_turn = {
                    let bc_lock = beacon_chain.lock().unwrap();
                    bc_lock.is_validator_turn(shard_id, &keypair.public)
                };

                if is_our_turn {
                    println!("\n[Shard {}] It's our turn to produce a block!", shard_id);
                    let mut chain_lock = chain.lock().unwrap();
                    let mut sm_lock = state_machine.lock().unwrap();

                    let last_block = chain_lock.get_last_block().unwrap().unwrap(); // Should exist after genesis

                    // --- CREATE MEANINGFUL TRANSACTIONS ---
                    let mut transactions = Vec::new();

                    // ** THE FIRST CITIZEN: DID DEPLOYMENT & CREATION **
                    // On the first turn of the first validator of the governance shard, deploy the DID contract.
                    if shard_id == 1 && last_block.header.height == 0 {
                        println!("[Shard 1] Deploying DID Registry Contract...");
                        let contract_bytes = std::fs::read("./contracts/did_registry_contract.wasm")?;
                        let deploy_tx = Transaction::new_contract_deploy(keypair.public, contract_bytes);
                        transactions.push(deploy_tx);
                    }
                    // In a subsequent block, create a DID for one of the Shard 0 nodes.
                    else if shard_id == 1 && last_block.header.height == 1 {
                        println!("[Shard 1] Creating first DID...");
                        // This is a simplified payload. A real one would be more complex.
                        let did_payload = b"create_did:did:qrasl:1:node_shard_0".to_vec();
                        let call_tx = Transaction::new_contract_call(keypair.public, state::DID_REGISTRY_ADDRESS.into(), did_payload);
                        transactions.push(call_tx);
                    }


                    let new_block = Block::new(transactions, last_block.header.hash()?);

                    if chain_lock.add_block(new_block.clone(), &mut sm_lock).is_ok() {
                        println!("[Shard {}] Successfully produced and added new block: {}", shard_id, new_block.header.height);
                        node.gossip_block(new_block.clone()).await?;

                        // Submit checkpoint to Beacon Chain
                        let mut bc_lock = beacon_chain.lock().unwrap();
                        bc_lock.submit_shard_checkpoint(shard_id, new_block.header.hash()?);
                    } else {
                        eprintln!("[Shard {}] Produced a block that was rejected by our own chain.", shard_id);
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // --- Initialization ---
    let beacon_chain = Arc::new(Mutex::new(BeaconChain::new()));

    // Create identities for our nodes
    let node_keys: Vec<_> = (0..3).map(|_| primitives::Keypair::new()).collect();
    let node_pubkeys: Vec<_> = node_keys.iter().map(|k| k.public).collect();

    // Setup initial staking and nominations
    {
        let mut bc_lock = beacon_chain.lock().unwrap();
        // Node 0 and 1 stake to become validators
        bc_lock.staking_system.stake(node_pubkeys[0], 1000);
        bc_lock.staking_system.stake(node_pubkeys[1], 1000);
        // Node 2 nominates Node 0
        bc_lock.staking_system.nominate(node_pubkeys[2], node_pubkeys[0], 500);
    }

    // --- Launch Shards ---
    for i in 0..2 { // Launching Shard 0 and Shard 1
        let db_path = format!("./db/shard_{}", i);
        let db = Arc::new(sled::open(db_path)?);

        let state_machine = Arc::new(Mutex::new(StateMachine::new(db.clone())));
        let chain = Arc::new(Mutex::new(Chain::new(db.clone())?));

        println!("Launching nodes for Shard {}...", i);
        for (j, keypair) in node_keys.iter().enumerate() {
            let node = Node::new().await?;
            let sm_clone = state_machine.clone();
            let chain_clone = chain.clone();
            let bc_clone = beacon_chain.clone();
            let keypair_clone = keypair.clone();

            println!("[Shard {}] Node {} starting...", i, j);
            tokio::spawn(async move {
                if let Err(e) = run_shard(i, node, sm_clone, chain_clone, bc_clone, keypair_clone).await {
                    eprintln!("[Shard {}] Node {} crashed: {}", i, j, e);
                }
            });
        }
    }

    // --- Beacon Chain Main Loop ---
    let mut interval = tokio::time::interval(Duration::from_secs(15));
    loop {
        interval.tick().await;
        let mut bc_lock = beacon_chain.lock().unwrap();

        // Run election cycle
        println!("\n[BeaconChain] Epoch ended. Running election cycle...");
        bc_lock.run_election();
        let active_set = bc_lock.get_active_validators();
        println!("[BeaconChain] New Active Validator Set: {:?}", active_set.iter().map(|v| hex::encode(v.id)).collect::<Vec<_>>());
    }
}
