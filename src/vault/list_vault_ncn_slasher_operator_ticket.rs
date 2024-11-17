use std::path::PathBuf;

use clap::Parser;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};

use super::VaultHandler;

#[derive(Parser)]
#[command(about = "Initialize Vault account")]
pub struct ListVaultNcnSlasherOperatorTicket {
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
        default_value = "Vau1t6sLNxnzB7ZDsef8TLbPLfyZMYXH8WTNqUdm9g8"
    )]
    vault_program_id: Pubkey,

    /// Vault program ID (Pubkey as base58 string)
    #[arg(
        long,
        env,
        default_value = "RestkWeAVL8fRGgzhfeoqFhsqKRchg6aa1XrcH96z4Q"
    )]
    restaking_program_id: Pubkey,
}

pub async fn command_list_vault_ncn_slasher_operator_ticket(args: ListVaultNcnSlasherOperatorTicket) {
    let payer = read_keypair_file(args.keypair).expect("Failed to read keypair file");
    let handler = VaultHandler::new(
        &args.rpc_url,
        &payer,
        args.vault_program_id,
        args.restaking_program_id,
    );

    handler.list_vault_ncn_slasher_operator_ticket().await;
}
