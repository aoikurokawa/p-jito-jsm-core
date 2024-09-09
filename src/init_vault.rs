use std::path::PathBuf;

use clap::Parser;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};

use crate::vault_handler::VaultHandler;

#[derive(Parser)]
#[command(about = "Initialize vault account")]
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

    /// Path to keypair for vault admin
    #[arg(short, long)]
    token_mint_pubkey: Pubkey,
}

pub async fn command_init_vault(args: InitVault) {
    let payer = read_keypair_file(args.keypair).expect("");
    let handler = VaultHandler::new(&args.rpc_url, &payer, args.vault_program_id);

    handler.initialize(args.token_mint_pubkey).await;
}
