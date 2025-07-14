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
use tokio::task;

mod primitives;
mod chain;
mod miner;
mod network;
mod state;
mod beacon_chain;

async fn run_shard(
    shard_id: u64,
    is_miner: bool,
    beacon_chain: Arc<Mutex<BeaconChain>>,
) -> Result<(), Box<dyn Error>> {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let mut swarm = create_swarm(tx).await?;

    let topic = Topic::new(format!("shard-{}-new-blocks", shard_id));

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

    println!("Shard {}: Simulation Started!", shard_id);
    println!("------------------------------------");
    println!("Shard {}: Miner Address: {:?}", shard_id, miner_address);
    println!("Shard {}: Listener Address: {:?}", shard_id, listener_address);
    println!("Shard {}: Genesis Block: {:?}", shard_id, chain.blocks[0].header.hash());
    println!("Shard {}: Initial Balances: {:?}", shard_id, chain.state_machine.balances);
    println!("------------------------------------");

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

            println!("Shard {}: Mining new block...", shard_id);
            let new_block = mine_block(last_block, transactions, difficulty);
            let block_json = serde_json::to_string(&new_block).unwrap();

            if let Err(e) = swarm
                .behaviour_mut()
                .gossipsub
                .publish(topic.clone(), block_json.as_bytes())
            {
                eprintln!("Error publishing block: {:?}", e);
            }

            let mut beacon_chain_lock = beacon_chain.lock().unwrap();
            beacon_chain_lock.submit_shard_checkpoint(shard_id, new_block.header.hash());
        }

        tokio::select! {
            Some(msg) = rx.recv() => {
                let block: Block = serde_json::from_str(&msg)?;
                match chain.add_block(block) {
                    Ok(_) => {
                        let new_block_header = chain.blocks.last().unwrap().header.clone();
                        println!("Shard {}: Block #{} Added!", shard_id, chain.blocks.len() - 1);
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
                    println!("Shard {}: Listening on {}", shard_id, address);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let beacon_chain = Arc::new(Mutex::new(BeaconChain::new()));

    let args: Vec<String> = std::env::args().collect();
    let is_miner = args.len() > 1 && args[1] == "miner";
    let shard_id: u64 = if args.len() > 2 {
        args[2].parse().unwrap_or(0)
    } else {
        0
    };

    if shard_id == 0 {
        let beacon_chain_clone = beacon_chain.clone();
        task::spawn(async move {
            run_shard(0, is_miner, beacon_chain_clone).await.unwrap();
        });
    }

    if shard_id == 1 {
        let beacon_chain_clone = beacon_chain.clone();
        task::spawn(async move {
            run_shard(1, is_miner, beacon_chain_clone).await.unwrap();
        });
    }

    // Keep the main thread alive
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
