use anyhow::{Context, Result};
use config::Config;
use crate::beacon_chain::BeaconChain;
use crate::chain::Chain;
use crate::network::create_swarm;
use crate::primitives::{Address, Block, CrossShardMessage, Transaction};
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

async fn run_shard(
    shard_id: u64,
    is_validator: bool,
    beacon_chain: Arc<Mutex<BeaconChain>>,
) -> Result<()> {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let mut swarm = create_swarm(tx).await?;

    let topic = Topic::new(format!("shard-{}-new-blocks", shard_id));

    let mut chain = Chain::new(shard_id)?;

    // Create some keypairs for simulation
    let mut csprng = OsRng {};
    let validator_keypair = Keypair::generate(&mut csprng);
    let nominator_keypair = Keypair::generate(&mut csprng);
    let validator_address: Address = validator_keypair.public.to_bytes();
    let nominator_address: Address = nominator_keypair.public.to_bytes();

    // Give them some initial funds
    chain.db.insert(&bincode::serialize(&validator_address)?, bincode::serialize(&1000u128)?)?;
    chain.db.insert(&bincode::serialize(&nominator_address)?, bincode::serialize(&500u128)?)?;

    println!("Shard {}: Simulation Started!", shard_id);
    println!("------------------------------------");
    println!("Shard {}: Validator Address: {:?}", shard_id, validator_address);
    println!("Shard {}: Nominator Address: {:?}", shard_id, nominator_address);
    println!("------------------------------------");

    let mut slot = 0;
    let mut nft_minted = false;
    let mut nft_transferred = false;

    loop {
        let active_validators = beacon_chain.lock().unwrap().active_validators.clone();
        if is_validator && !active_validators.is_empty() {
            let current_validator = active_validators[slot % active_validators.len()];
            if current_validator == validator_address {
                let last_block_bytes = chain.db.get(b"tip")?.context("Failed to get last block")?;
                let last_block: Block = bincode::deserialize(&last_block_bytes)?;
                let mut transactions = vec![];
                let cross_shard_messages_to_send = vec![];

                if shard_id == 0 {
                    if !nft_minted {
                        let mut tx = Transaction {
                            sender: validator_address,
                            signature: [0; 64],
                            recipient: [4; 32], // NFT contract address
                            value: 0,
                            payload: "MINT_NFT:1:1:http://example.com/proto-critter".into(),
                            gas_limit: 0,
                            fees: 0,
                        };
                        tx.sign(&validator_keypair);
                        transactions.push(tx);
                        nft_minted = true;
                    } else if !nft_transferred {
                        let mut tx = Transaction {
                            sender: validator_address,
                            signature: [0; 64],
                            recipient: [4; 32], // NFT contract address
                            value: 0,
                            payload: format!(
                                "TRANSFER_NFT:1:1:{}",
                                serde_json::to_string(&nominator_address)?
                            )
                            .into(),
                            gas_limit: 0,
                            fees: 0,
                        };
                        tx.sign(&validator_keypair);
                        transactions.push(tx);
                        nft_transferred = true;
                    }
                }

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
                println!("Shard {}: Validator {:?} produced block #{}", shard_id, validator_address, slot);
            }
            slot += 1;
        }

        tokio::select! {
            Some(msg) = rx.recv() => {
                let block: Block = serde_json::from_str(&msg)?;
                let active_validators = beacon_chain.lock().unwrap().active_validators.clone();
                if let Err(e) = chain.add_block(block, &active_validators) {
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

    let is_validator_node = settings.get_bool("node.is_validator").unwrap_or(false);
    let shard_id = settings.get_int("node.shard_id").unwrap_or(0) as u64;

    // Simulate staking
    let mut csprng = OsRng {};
    let validator1_keypair = Keypair::generate(&mut csprng);
    let validator2_keypair = Keypair::generate(&mut csprng);
    let nominator1_keypair = Keypair::generate(&mut csprng);
    let validator1_address = validator1_keypair.public.to_bytes();
    let validator2_address = validator2_keypair.public.to_bytes();
    let nominator1_address = nominator1_keypair.public.to_bytes();

    {
        let mut bc_lock = beacon_chain.lock().unwrap();
        staking::stake(&mut bc_lock.validators, validator1_address, 100);
        staking::stake(&mut bc_lock.validators, validator2_address, 150);
        staking::nominate(&mut bc_lock.validators, nominator1_address, validator2_address, 50);
    }

    let beacon_chain_clone = beacon_chain.clone();
    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10));
        loop {
            interval.tick().await;
            beacon_chain_clone.lock().unwrap().run_election();
        }
    });

    if shard_id == 0 {
        let beacon_chain_clone = beacon_chain.clone();
        task::spawn(async move {
            if let Err(e) = run_shard(0, is_validator_node, beacon_chain_clone).await {
                eprintln!("Shard 0 failed: {}", e);
            }
        });
    }

    // Keep the main thread alive
    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
