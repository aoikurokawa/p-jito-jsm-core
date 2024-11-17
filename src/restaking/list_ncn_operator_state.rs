use std::path::PathBuf;

use clap::Parser;
use jito_restaking_core::ncn_vault_slasher_ticket::NcnVaultSlasherTicket;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};

use super::RestakingHandler;

#[derive(Parser)]
#[command(about = "List NCN Vault Slasher Ticket account")]
pub struct ListNcnVaultSlasherTicket {
    /// RPC URL for the cluster
    #[arg(short, long, env, default_value = "https://api.devnet.solana.com")]
    rpc_url: String,

    /// Path to keypair used to pay
    #[arg(long, env, default_value = "~/.config/solana/id.json")]
    keypair: PathBuf,

    /// Validator history program ID (Pubkey as base58 string)
    #[arg(
        long,
        env,
        default_value = "RestkWeAVL8fRGgzhfeoqFhsqKRchg6aa1XrcH96z4Q"
    )]
    restaking_program_id: Pubkey,
}

pub async fn command_list_ncn_vault_slasher_ticket(args: ListNcnVaultSlasherTicket) {
    let payer = read_keypair_file(args.keypair).expect("Failed to read keypair file");
    let handler = RestakingHandler::new(&args.rpc_url, &payer, args.restaking_program_id);

    handler.list_account::<NcnVaultSlasherTicket>().await;
}
