use crate::beacon_chain::BeaconChain;
use crate::chain::Chain;
use crate::miner::mine_block;
use crate::network::create_swarm;
use crate::primitives::{Address, Block, Transaction};
use ed25519_dalek::Keypair;
use libp2p::gossipsub::IdentTopic as Topic;
use libp2p::swarm::SwarmEvent;
use libp2p::Swarm;
use rand::rngs::OsRng;
use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

mod primitives;
mod chain;
mod miner;
mod network;
mod state;
mod beacon_chain;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let beacon_chain = Arc::new(Mutex::new(BeaconChain::new()));

    let (tx, mut rx) = mpsc::unbounded_channel();
    let mut swarm = create_swarm(tx).await?;

    let topic = Topic::new("new-blocks");

    let mut chain = Chain::new();
    let difficulty = 12;

    // Create some keypairs for simulation
    let mut csprng = OsRng {};
    let miner_keypair = Keypair::generate(&mut csprng);
    let listener_keypair = Keypair::generate(&mut csprng);
    let miner_address: Address = miner_keypair.public.to_bytes();
    let listener_address: Address = listener_keypair.public.to_bytes();

    // Give the miner some initial funds in the genesis state
    chain.state_machine.balances.insert(miner_address, 1000);

    println!("QRASL Blockchain Simulation Started!");
    println!("------------------------------------");
    println!("Miner Address: {:?}", miner_address);
    println!("Listener Address: {:?}", listener_address);
    println!("Genesis Block: {:?}", chain.blocks[0].header.hash());
    println!("Initial Balances: {:?}", chain.state_machine.balances);
    println!("------------------------------------");

    let mut is_miner = false;
    if let Some(arg) = std::env::args().nth(1) {
        if arg == "miner" {
            is_miner = true;
        }
    }

    let beacon_chain_clone = beacon_chain.clone();

    loop {
        if is_miner {
            let last_block = chain.blocks.last().unwrap();
            let mut tx = Transaction {
                sender: miner_address,
                signature: [0; 64],
                recipient: listener_address,
                value: 10,
                payload: vec![],
                gas_limit: 0,
                fees: 0,
            };
            tx.sign(&miner_keypair);
            let transactions = vec![tx];

            println!("Shard 0: Mining new block...");
            let new_block = mine_block(last_block, transactions, difficulty);
            let block_json = serde_json::to_string(&new_block).unwrap();

            if let Err(e) = swarm
                .behaviour_mut()
                .gossipsub
                .publish(topic.clone(), block_json.as_bytes())
            {
                eprintln!("Error publishing block: {:?}", e);
            }

            let mut beacon_chain_lock = beacon_chain_clone.lock().unwrap();
            beacon_chain_lock.submit_shard_checkpoint(0, new_block.header.hash());
        }

        tokio::select! {
            Some(msg) = rx.recv() => {
                let block: Block = serde_json::from_str(&msg)?;
                match chain.add_block(block) {
                    Ok(_) => {
                        let new_block_header = chain.blocks.last().unwrap().header.clone();
                        println!("Shard 0: Block #{} Added!", chain.blocks.len() - 1);
                        println!("  Hash: {:?}", new_block_header.hash());
                        println!("  Balances: {:?}", chain.state_machine.balances);
                        println!("------------------------------------");
                    }
                    Err(e) => {
                        eprintln!("Error adding block: {}", e);
                    }
                }
            }
            event = swarm.select_next_some() => {
                if let SwarmEvent::NewListenAddr { address, .. } = event {
                    println!("Listening on {}", address);
                }
            }
        }
    }
}
