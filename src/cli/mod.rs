//! The `cli` module provides the command-line interface for the QRASL node.

use clap::{Parser, Subcommand};
use crate::crypto;

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
}

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::GenerateKeypair => {
            let (pk, sk) = crypto::generate_keypair();
            println!("Public Key: {}", hex::encode(pk));
            println!("Secret Key: {}", hex::encode(sk));
        }
    }
}
