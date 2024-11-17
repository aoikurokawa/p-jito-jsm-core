use std::path::PathBuf;

use clap::Parser;
use jito_restaking_core::ncn::Ncn as NcnAccount;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};

use super::RestakingHandler;

#[derive(Parser)]
#[command(about = "Get NCN account")]
pub struct Ncn {
    /// RPC URL for the cluster
    #[arg(short, long, env, default_value = "https://api.devnet.solana.com")]
    rpc_url: String,

    /// Path to keypair used to pay
    #[arg(long, env, default_value = "~/.config/solana/id.json")]
    keypair: PathBuf,

    /// Restaking Program ID (Pubkey as base58 string)
    #[arg(
        long,
        env,
        default_value = "5b2dHDz9DLhXnwQDG612bgtBGJD62Riw9s9eYuDT3Zma"
    )]
    restaking_program_id: Pubkey,

    /// NCN Pubkey
    #[arg(long, env)]
    ncn_pubkey: Pubkey,
}

pub async fn command_ncn(args: Ncn) {
    let payer = read_keypair_file(args.keypair).expect("Failed to read keypair file");
    let handler = RestakingHandler::new(&args.rpc_url, &payer, args.restaking_program_id);

    let ncn: NcnAccount = handler.get_account(args.ncn_pubkey).await;

    println!("{:?}", ncn);
}
