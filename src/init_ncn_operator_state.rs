use std::path::PathBuf;

use clap::Parser;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};

use crate::restaking_handler::RestakingHandler;

#[derive(Parser)]
#[command(about = "Initialize NCN Operator State account")]
pub struct InitNcnOperatorState {
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
        default_value = "5b2dHDz9DLhXnwQDG612bgtBGJD62Riw9s9eYuDT3Zma"
    )]
    restaking_program_id: Pubkey,

    /// NCN pubkey
    #[arg(long)]
    ncn: Pubkey,

    /// Operator Pubkey
    #[arg(long)]
    operator: Pubkey,
}

pub async fn command_init_ncn_operator_state(args: InitNcnOperatorState) {
    let payer = read_keypair_file(args.keypair).expect("");
    let handler = RestakingHandler::new(&args.rpc_url, &payer, args.restaking_program_id);
    handler
        .initialize_ncn_operator_state(args.ncn, args.operator)
        .await;
}
