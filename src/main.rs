use crate::chain::Chain;
use crate::miner::mine_block;
use crate::network::create_swarm;
use crate::primitives::{Block, Transaction};
use libp2p::gossipsub::IdentTopic as Topic;
use libp2p::swarm::SwarmEvent;
use libp2p::Swarm;
use std::error::Error;
use tokio::sync::mpsc;

mod primitives;
mod chain;
mod miner;
mod network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let mut swarm = create_swarm(tx).await?;

    let topic = Topic::new("new-blocks");

    let mut chain = Chain::new();
    let difficulty = 12;

    println!("QRASL Blockchain Simulation Started!");
    println!("------------------------------------");
    println!("Genesis Block: {:?}", chain.blocks[0].header.hash());
    println!("------------------------------------");

    let mut is_miner = false;
    if let Some(arg) = std::env::args().nth(1) {
        if arg == "miner" {
            is_miner = true;
        }
    }

    loop {
        if is_miner {
            let last_block = chain.blocks.last().unwrap();
            let transactions = vec![Transaction {
                sender: [0; 32],
                signature: [0; 64],
                recipient: [1; 32],
                value: 10,
                payload: vec![],
                gas_limit: 0,
                fees: 0,
            }];

            println!("Mining new block...");
            let new_block = mine_block(last_block, transactions, difficulty);
            let block_json = serde_json::to_string(&new_block).unwrap();

            if let Err(e) = swarm
                .behaviour_mut()
                .gossipsub
                .publish(topic.clone(), block_json.as_bytes())
            {
                eprintln!("Error publishing block: {:?}", e);
            }
        }

        tokio::select! {
            Some(msg) = rx.recv() => {
                let block: Block = serde_json::from_str(&msg)?;
                match chain.add_block(block) {
                    Ok(_) => {
                        let new_block_header = chain.blocks.last().unwrap().header.clone();
                        println!("Block #{} Added!", chain.blocks.len() - 1);
                        println!("  Hash: {:?}", new_block_header.hash());
                        println!("  Parent Hash: {:?}", new_block_header.parent_hash);
                        println!("  Nonce: {:?}", String::from_utf8_lossy(&new_block_header.nonce));
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
