//! The `cli` module provides the command-line interface for the QRASL node.

use crate::node::ShardNode;
use clap::{Parser, Subcommand};
use crate::crypto::{self};
use crate::core::types::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new PQC keypair
    GenerateKeypair,
    /// Wallet-related commands
    Wallet(WalletArgs),
    /// Transaction-related commands
    Transaction(TransactionArgs),
    /// Node-related commands
    Node(NodeArgs),
    /// Chain-related commands
    Chain(ChainArgs),
}

#[derive(Parser)]
struct WalletArgs {
    #[command(subcommand)]
    command: WalletCommands,
}

#[derive(Subcommand)]
enum WalletCommands {
    /// Get the balance of the genesis account
    GenesisBalance,
}

#[derive(Parser)]
struct TransactionArgs {
    #[command(subcommand)]
    command: TransactionCommands,
}

#[derive(Subcommand)]
enum TransactionCommands {
    /// Create a test transaction from the genesis account
    CreateTest,
}

#[derive(Parser)]
struct NodeArgs {
    #[command(subcommand)]
    command: NodeCommands,
}

#[derive(Subcommand)]
enum NodeCommands {
    /// Produce a new block from the mempool
    ProduceBlock,
}

#[derive(Parser)]
struct ChainArgs {
    #[command(subcommand)]
    command: ChainCommands,
}

#[derive(Subcommand)]
enum ChainCommands {
    /// View the current blockchain
    View,
}


pub fn run() {
    let cli = Cli::parse();
    let mut node = ShardNode::load();

    match &cli.command {
        Commands::GenerateKeypair => {
            let (pk, sk) = crypto::generate_keypair();
            println!("Public Key: {}", hex::encode(pk));
            println!("Secret Key: {}", hex::encode(sk));
            // No need to save the node state here
            return;
        }
        Commands::Wallet(args) => match &args.command {
            WalletCommands::GenesisBalance => {
                let (genesis_pk, _) = ShardNode::get_genesis_keypair();
                let balance = node.state.get_balance(&genesis_pk);
                println!("Genesis Account Balance: {}", balance);
            }
        },
        Commands::Transaction(args) => match &args.command {
            TransactionCommands::CreateTest => {
                let (sender_pk, sender_sk) = ShardNode::get_genesis_keypair();
                let (recipient_pk, _) = crypto::generate_keypair();

                let mut tx = Transaction {
                    sender: sender_pk,
                    recipient: recipient_pk,
                    amount: 100,
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                    signature: [0; crypto::SIGNATURE_LENGTH],
                };

                let signable_bytes = tx.to_signable_bytes();
                tx.signature = crypto::sign(&signable_bytes, &sender_sk);

                println!("Test transaction created and added to mempool.");
                node.mempool.add_transaction(tx);
            }
        },
        Commands::Node(args) => match &args.command {
            NodeCommands::ProduceBlock => {
                match node.produce_block() {
                    Ok(_) => println!("Successfully produced a new block."),
                    Err(e) => eprintln!("Error producing block: {}", e),
                }
            }
        },
        Commands::Chain(args) => match &args.command {
            ChainCommands::View => {
                println!("Current Blockchain:");
                for (i, block) in node.chain.iter().enumerate() {
                    println!("  Block {}:", i);
                    println!("    Hash: {}", hex::encode(block.block_hash));
                    println!("    Parent Hashes: {:?}", block.parent_hashes.iter().map(hex::encode).collect::<Vec<_>>());
                    println!("    Transactions: {}", block.transactions.len());
                }
            }
        }
    }

    node.save().expect("Failed to save node state");
}
