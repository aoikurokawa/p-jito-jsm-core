use std::error;

use clap::{Parser, Subcommand};
use quick::{
    init_ncn::{command_init_ncn, InitNcn},
    init_ncn_operator_state::{command_init_ncn_operator_state, InitNcnOperatorState},
    init_ncn_vault_ticket::{command_init_ncn_vault_ticket, InitNcnVaultTicket},
    init_operator::{command_init_operator, InitOperator},
    init_restaking_config::{command_init_restaking_config, InitRestakingConfig},
    init_vault::{command_init_vault, InitVault},
    init_vault_config::{command_init_vault_config, InitConfig},
};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    InitVaultConfig(InitConfig),

    InitVault(InitVault),

    InitRestakingConfig(InitRestakingConfig),

    InitNcn(InitNcn),

    InitOperator(InitOperator),

    InitNcnVaultTicket(InitNcnVaultTicket),

    InitNcnOperatorState(InitNcnOperatorState),

    CreateTokenMetadata,
    GetConfig,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();

    match args.commands {
        Commands::InitVaultConfig(args) => command_init_vault_config(args).await,

        Commands::InitVault(args) => command_init_vault(args).await,

        Commands::InitRestakingConfig(args) => command_init_restaking_config(args).await,

        Commands::InitNcn(args) => command_init_ncn(args).await,

        Commands::InitOperator(args) => command_init_operator(args).await,

        Commands::InitNcnVaultTicket(args) => command_init_ncn_vault_ticket(args).await,

        Commands::InitNcnOperatorState(args) => command_init_ncn_operator_state(args).await,

        Commands::CreateTokenMetadata => {}

        Commands::GetConfig => {
            // let jito_vault_program_id = jito_vault_program_id();
            // let config_pubkey =
            //     jito_vault_core::config::Config::find_program_address(&jito_vault_program_id).0;
            // let res = client
            //     .get_account_data(&config_pubkey)
            //     .expect("Fail to fetch config account");
            // let config =
            //     jito_vault_core::config::Config::load_bytes(&res).expect("Fail to convert Config");

            // println!("config bump: {}", config.bump());
        }
    }
    Ok(())
}
