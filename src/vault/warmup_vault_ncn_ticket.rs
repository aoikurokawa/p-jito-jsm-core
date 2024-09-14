use std::path::PathBuf;

use clap::Parser;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};

use super::VaultHandler;

#[derive(Parser)]
#[command(about = "Warmup Vault NCN Ticket account")]
pub struct WarmupVaultNcnTicket {
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

    /// Vault pubkey
    #[arg(long)]
    vault: Pubkey,

    /// NCN Pubkey
    #[arg(long)]
    ncn: Pubkey,
}

pub async fn command_warmup_vault_ncn_ticket(args: WarmupVaultNcnTicket) {
    let payer = read_keypair_file(args.keypair).expect("Failed to read keypair file");
    let handler = VaultHandler::new(
        &args.rpc_url,
        &payer,
        args.vault_program_id,
        args.restaking_program_id,
    );

    handler.warmup_vault_ncn_ticket(args.vault, args.ncn).await;
}
