use std::path::PathBuf;

use clap::Parser;
use jito_vault_core::vault_operator_delegation::VaultOperatorDelegation;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};

use super::VaultHandler;

#[derive(Parser)]
#[command(about = "Fetch Vault Operator Delegation account")]
pub struct ListVaultOperatorDelegation {
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

    /// Restaking program ID (Pubkey as base58 string)
    #[arg(
        long,
        env,
        default_value = "RestkWeAVL8fRGgzhfeoqFhsqKRchg6aa1XrcH96z4Q"
    )]
    restaking_program_id: Pubkey,
}

pub async fn command_list_vault_operator_delegation(args: ListVaultOperatorDelegation) {
    let payer = read_keypair_file(args.keypair).expect("Failed to read keypair file");
    let handler = VaultHandler::new(
        &args.rpc_url,
        &payer,
        args.vault_program_id,
        args.restaking_program_id,
    );

    handler.list_accounts::<VaultOperatorDelegation>().await
}
