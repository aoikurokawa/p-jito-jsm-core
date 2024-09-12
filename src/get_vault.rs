use std::path::PathBuf;

use clap::Parser;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
};

use crate::vault_handler::VaultHandler;

#[derive(Parser)]
#[command(about = "Initialize Vault account")]
pub struct InitVault {
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

    /// Supported token pubkey
    #[arg(short, long)]
    token_mint_pubkey: Pubkey,
}

pub async fn command_init_vault(args: InitVault) {
    let base = Keypair::new();
    let payer = read_keypair_file(args.keypair).expect("Failed to read keypair file");
    let handler = VaultHandler::new(
        &args.rpc_url,
        &payer,
        args.vault_program_id,
        args.restaking_program_id,
    );

    handler.initialize(&base, args.token_mint_pubkey).await;
}
