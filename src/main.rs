use anyhow::{Context, Result};
use config::Config;
use crate::beacon_chain::BeaconChain;
use crate::chain::Chain;
use crate::miner::mine_block;
use crate::network::create_swarm;
use crate::primitives::{Address, Block, CrossShardMessage, Transaction};
use ed25519_dalek::Keypair;
use libp2p::gossipsub::IdentTopic as Topic;
use libp2p::swarm::SwarmEvent;
use libp2p::Swarm;
use rand::rngs::OsRng;
use std::fs;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task;

mod primitives;
mod chain;
mod miner;
mod network;
mod state;
mod beacon_chain;
mod vm;
mod governance;

async fn run_shard(
    shard_id: u64,
    is_miner: bool,
    difficulty: u32,
    beacon_chain: Arc<Mutex<BeaconChain>>,
) -> Result<()> {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let mut swarm = create_swarm(tx).await?;

    let topic = Topic::new(format!("shard-{}-new-blocks", shard_id));

    let mut chain = Chain::new(shard_id)?;

    // Create some keypairs for simulation
    let mut csprng = OsRng {};
    let miner_keypair = Keypair::generate(&mut csprng);
    let listener_keypair = Keypair::generate(&mut csprng);
    let miner_address: Address = miner_keypair.public.to_bytes();
    let listener_address: Address = listener_keypair.public.to_bytes();

    // Give the miner some initial funds in the genesis state
    chain.db.insert(
        &bincode::serialize(&miner_address)?,
        bincode::serialize(&1000u128)?,
    )?;

    println!("Shard {}: Simulation Started!", shard_id);
    println!("------------------------------------");
    println!("Shard {}: Miner Address: {:?}", shard_id, miner_address);
    println!("Shard {}: Listener Address: {:?}", shard_id, listener_address);
    println!("------------------------------------");

    let mut contract_deployed = false;

    loop {
        if is_miner {
            let last_block_bytes = chain.db.get(b"tip")?.context("Failed to get last block")?;
            let last_block: Block = bincode::deserialize(&last_block_bytes)?;
            let mut transactions = vec![];
            let mut cross_shard_messages_to_send = vec![];

            if shard_id == 0 {
                // On Shard 0, create a cross-shard message to propose something on Shard 1
                let proposal_message = CrossShardMessage {
                    source_shard_id: 0,
                    target_shard_id: 1,
                    payload: "PROPOSE:Increase block reward".into(),
                };
                cross_shard_messages_to_send.push(proposal_message);
                println!("Shard 0: Sending governance proposal to Shard 1");
            } else if shard_id == 1 {
                // On Shard 1, vote on the proposal
                if let Some(proposal_bytes) = chain.db.get(b"proposals")? {
                    let proposals: std::collections::HashMap<u64, crate::governance::Proposal> =
                        bincode::deserialize(&proposal_bytes)?;
                    if let Some(proposal) = proposals.values().next() {
                        let mut tx = Transaction {
                            sender: miner_address,
                            signature: [0; 64],
                            recipient: [3; 32], // Governance contract address
                            value: 0,
                            payload: format!("VOTE:{}:AYE", proposal.id).into(),
                            gas_limit: 0,
                            fees: 0,
                        };
                        tx.sign(&miner_keypair);
                        transactions.push(tx);
                        println!("Shard 1: Voting on proposal {}", proposal.id);
                    }
                }
            }

            // Poll for incoming messages from the Beacon Chain
            let incoming_messages = beacon_chain.lock().unwrap().get_messages_for_shard(shard_id);

            println!("Shard {}: Mining new block...", shard_id);
            let new_block = mine_block(
                &last_block,
                transactions,
                incoming_messages,
                difficulty,
            );
            let block_json = serde_json::to_string(&new_block)?;

            if let Err(e) = swarm
                .behaviour_mut()
                .gossipsub
                .publish(topic.clone(), block_json.as_bytes())
            {
                eprintln!("Error publishing block: {:?}", e);
            }

            let mut beacon_chain_lock = beacon_chain.lock().unwrap();
            beacon_chain_lock.submit_shard_checkpoint(shard_id, new_block.header.hash());
            for msg in cross_shard_messages_to_send {
                beacon_chain_lock.submit_cross_shard_message(msg);
            }
        }

        tokio::select! {
            Some(msg) = rx.recv() => {
                let block: Block = serde_json::from_str(&msg)?;
                if let Err(e) = chain.add_block(block) {
                    eprintln!("Error adding block: {}", e);
                } else {
                    println!("------------------------------------");
                }
            }
            event = swarm.select_next_some() => {
                if let SwarmEvent::NewListenAddr { address, .. } = event {
                    println!("Shard {}: Listening on {}", shard_id, address);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let beacon_chain = Arc::new(Mutex::new(BeaconChain::new()));

    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;

    let is_miner = settings.get_bool("node.is_miner")?;
    let shard_id = settings.get_int("node.shard_id")? as u64;
    let difficulty = settings.get_int("network.difficulty")? as u32;

    if shard_id == 0 {
        let beacon_chain_clone = beacon_chain.clone();
        task::spawn(async move {
            if let Err(e) = run_shard(0, is_miner, difficulty, beacon_chain_clone).await {
                eprintln!("Shard 0 failed: {}", e);
            }
        });
    }

    if shard_id == 1 {
        let beacon_chain_clone = beacon_chain.clone();
        task::spawn(async move {
            if let Err(e) = run_shard(1, is_miner, difficulty, beacon_chain_clone).await {
                eprintln!("Shard 1 failed: {}", e);
            }
        });
    }

    // Keep the main thread alive
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
