use anyhow::{Context, Result};
use config::Config;
use crate::beacon_chain::BeaconChain;
use crate::chain::Chain;
use crate::network::create_swarm;
use crate::primitives::{Address, Block, CrossShardMessage, Transaction};
use crate::rpc;
use crate::staking;
use ed25519_dalek::Keypair;
use libp2p::gossipsub::IdentTopic as Topic;
use libp2p::swarm::SwarmEvent;
use libp2p::Swarm;
use rand::rngs::OsRng;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::{self, Duration};

mod primitives;
mod chain;
mod network;
mod state;
mod beacon_chain;
mod vm;
mod governance;
mod staking;
mod nft;
mod marketplace;
mod rpc;

async fn run_shard(
    shard_id: u64,
    is_validator: bool,
    beacon_chain: Arc<Mutex<BeaconChain>>,
) -> Result<()> {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let mut swarm = create_swarm(tx).await?;

    let topic = Topic::new(format!("shard-{}-new-blocks", shard_id));

    let chain = Arc::new(Mutex::new(Chain::new(shard_id)?));

    // Create some keypairs for simulation
    let mut csprng = OsRng {};
    let validator_keypair = Keypair::generate(&mut csprng);
    let nominator_keypair = Keypair::generate(&mut csprng);
    let validator_address: Address = validator_keypair.public.to_bytes();
    let nominator_address: Address = nominator_keypair.public.to_bytes();

    // Give them some initial funds
    chain.lock().unwrap().db.insert(&bincode::serialize(&validator_address)?, bincode::serialize(&1000u128)?)?;
    chain.lock().unwrap().db.insert(&bincode::serialize(&nominator_address)?, bincode::serialize(&500u128)?)?;

    let rpc_chain = chain.clone();
    task::spawn(async move {
        rpc::run_rpc_server(rpc_chain).await.unwrap();
    });

    println!("Shard {}: Simulation Started!", shard_id);
    println!("------------------------------------");
    println!("Shard {}: Validator Address: {:?}", shard_id, validator_address);
    println!("Shard {}: Nominator Address: {:?}", shard_id, nominator_address);
    println!("------------------------------------");

    let mut slot = 0;
    let mut nft_listed = false;
    let mut nft_bought = false;

    loop {
        let (active_validators, epoch) = {
            let bc_lock = beacon_chain.lock().unwrap();
            (bc_lock.active_validators.clone(), bc_lock.epoch)
        };

        if is_validator && !active_validators.is_empty() {
            let current_validator = active_validators[slot % active_validators.len()];
            if current_validator == validator_address {
                let last_block_bytes = chain.db.get(b"tip")?.context("Failed to get last block")?;
                let last_block: Block = bincode::deserialize(&last_block_bytes)?;
                let mut transactions = vec![];
                let cross_shard_messages_to_send = vec![];

                // ... (transaction creation logic remains the same)

                let new_block = Block::new(
                    last_block.header.hash(),
                    [0; 32],
                    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
                    shard_id as u32,
                    validator_address.to_vec(),
                    transactions,
                    cross_shard_messages_to_send,
                );

                let block_json = serde_json::to_string(&new_block)?;
                if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic.clone(), block_json.as_bytes()) {
                    eprintln!("Error publishing block: {:?}", e);
                }

                let mut beacon_chain_lock = beacon_chain.lock().unwrap();
                beacon_chain_lock.submit_shard_checkpoint(shard_id, new_block.header.hash());
                println!("Epoch {}: Shard {}: Validator {:?} produced block #{}", epoch, shard_id, validator_address, slot);
            }
            slot += 1;
        }

        tokio::select! {
            Some(msg) = rx.recv() => {
                let block: Block = serde_json::from_str(&msg)?;
                if let Err(e) = chain.lock().unwrap().add_block(block, &active_validators) {
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
        .add_source(config::File::with_name("config.toml"))
        .build()?;

    let node_count = settings.get_int("simulation.node_count").unwrap_or(1) as usize;
    let validator_count = settings.get_int("simulation.validator_count").unwrap_or(1) as usize;

    let mut csprng = OsRng {};
    let keypairs: Vec<Keypair> = (0..node_count).map(|_| Keypair::generate(&mut csprng)).collect();

    {
        let mut bc_lock = beacon_chain.lock().unwrap();
        for (i, keypair) in keypairs.iter().enumerate() {
            if i < validator_count {
                staking::stake(&mut bc_lock.validators, keypair.public.to_bytes(), 100);
            } else {
                let validator_to_nominate = keypairs[i % validator_count].public.to_bytes();
                staking::nominate(
                    &mut bc_lock.validators,
                    keypair.public.to_bytes(),
                    validator_to_nominate,
                    50,
                );
            }
        }
    }

    let beacon_chain_clone = beacon_chain.clone();
    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            beacon_chain_clone.lock().unwrap().run_election();
        }
    });

    for (i, keypair) in keypairs.into_iter().enumerate() {
        let is_validator = i < validator_count;
        let beacon_chain_clone = beacon_chain.clone();
        task::spawn(async move {
            if let Err(e) = run_shard(0, is_validator, keypair, beacon_chain_clone).await {
                eprintln!("Shard 0 node failed: {}", e);
            }
        });
    }

    // Keep the main thread alive
    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
