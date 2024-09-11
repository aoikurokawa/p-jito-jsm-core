use std::path::PathBuf;

use clap::Parser;
use jito_restaking_core::{ncn::Ncn, operator::Operator};
use jito_vault_core::vault::Vault;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
    signer::Signer,
};

use crate::{restaking_handler::RestakingHandler, vault_handler::VaultHandler};

#[derive(Parser)]
#[command(about = "Setup Restaking Registration and Vault")]
pub struct Setup {
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

    /// Restaking program ID (Pubkey as base58 string)
    #[arg(
        long,
        env,
        default_value = "5b2dHDz9DLhXnwQDG612bgtBGJD62Riw9s9eYuDT3Zma"
    )]
    restaking_program_id: Pubkey,

    /// Supported token pubkey
    #[arg(short, long)]
    token_mint_pubkey: Pubkey,
}

pub async fn command_setup(args: Setup) {
    let base = Keypair::new();
    let payer = read_keypair_file(args.keypair).expect("Failed to read keypair file");
    let restaking_handler = RestakingHandler::new(&args.rpc_url, &payer, args.restaking_program_id);
    let vault_handler = VaultHandler::new(&args.rpc_url, &payer, args.vault_program_id);

    vault_handler
        .initialize(&base, args.token_mint_pubkey)
        .await;

    restaking_handler.initialize_ncn(&base).await;
    restaking_handler.initialize_operator(&base).await;

    let ncn = Ncn::find_program_address(&args.restaking_program_id, &base.pubkey()).0;
    let vault = Vault::find_program_address(&args.vault_program_id, &base.pubkey()).0;
    restaking_handler
        .initialize_ncn_vault_ticket(ncn, vault)
        .await;

    let operator = Operator::find_program_address(&args.restaking_program_id, &base.pubkey()).0;
    restaking_handler
        .initialize_ncn_operator_state(ncn, operator)
        .await;
}
