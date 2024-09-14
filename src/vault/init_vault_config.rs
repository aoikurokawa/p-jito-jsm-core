use std::path::PathBuf;

use clap::Parser;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};

use super::VaultHandler;

#[derive(Parser)]
#[command(about = "Initialize Vault Config account")]
pub struct InitConfig {
    /// RPC URL for the cluster
    #[arg(short, long, env, default_value = "https://api.devnet.solana.com")]
    rpc_url: String,

    /// Path to keypair used to pay
    #[arg(long, env, default_value = "~/.config/solana/id.json")]
    keypair: PathBuf,

    /// Vault program ID (Pubkey as base58 string)
    #[arg(
        long,
        env,
        default_value = "BLCDL7LqxaYWxSEkayc4VYjs3iCNJJw8SQzsvEL2uVT"
    )]
    vault_program_id: Pubkey,

    /// Vault program ID (Pubkey as base58 string)
    #[arg(
        long,
        env,
        default_value = "78J8YzXGGNynLRpn85MH77PVLBZsWyLCHZAXRvKaB6Ng"
    )]
    restaking_program_id: Pubkey,
}

pub async fn command_init_vault_config(args: InitConfig) {
    let payer = read_keypair_file(args.keypair).expect("Failed to read keypair file");
    let handler = VaultHandler::new(
        &args.rpc_url,
        &payer,
        args.vault_program_id,
        args.restaking_program_id,
    );
    handler.initialize_config().await;
}
