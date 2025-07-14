use crate::chain::Chain;
use crate::miner::mine_block;
use crate::primitives::Transaction;
use std::thread;
use std::time::Duration;

mod primitives;
mod chain;
mod miner;

fn main() {
    let mut chain = Chain::new();
    let difficulty = 12;

    println!("QRASL Blockchain Simulation Started!");
    println!("------------------------------------");
    println!("Genesis Block: {:?}", chain.blocks[0].header.hash());
    println!("------------------------------------");


    loop {
        let last_block = chain.blocks.last().unwrap();
        let transactions = vec![
            // In a real implementation, transactions would be sourced from a mempool
            Transaction {
                sender: [0; 32],
                signature: [0; 64],
                recipient: [1; 32],
                value: 10,
                payload: vec![],
                gas_limit: 0,
                fees: 0,
            }
        ];

        println!("Mining new block...");
        let new_block = mine_block(last_block, transactions, difficulty);

        match chain.add_block(new_block) {
            Ok(_) => {
                let new_block_header = chain.blocks.last().unwrap().header.clone();
                println!("Block #{} Mined!", chain.blocks.len() - 1);
                println!("  Hash: {:?}", new_block_header.hash());
                println!("  Parent Hash: {:?}", new_block_header.parent_hash);
                println!("  Nonce: {:?}", String::from_utf8_lossy(&new_block_header.nonce));
                println!("------------------------------------");
            }
            Err(e) => {
                eprintln!("Error adding block: {}", e);
            }
        }

        thread::sleep(Duration::from_secs(2));
    }
}
