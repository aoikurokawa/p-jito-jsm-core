use std::error;

use clap::{Parser, Subcommand};
use quick::{
    restaking::{
        init_ncn::{command_init_ncn, InitNcn},
        init_ncn_operator_state::{command_init_ncn_operator_state, InitNcnOperatorState},
        init_ncn_vault_ticket::{command_init_ncn_vault_ticket, InitNcnVaultTicket},
        init_operator::{command_init_operator, InitOperator},
        init_operator_vault_ticket::{command_init_operator_vault_ticket, InitOperatorVaultTicket},
        init_restaking_config::{command_init_restaking_config, InitRestakingConfig},
        ncn_warmup_operator::{command_ncn_warmup_operator_state, NcnWarmupOperator},
        warmup_ncn_vault_ticket::{command_warmup_ncn_vault_ticket, WarmupNcnVaultTicket},
        warmup_opeator_vault_ticket::{
            command_warmup_operator_vault_ticket, WarmupOperatorVaultTicket,
        },
    },
    setup::{command_setup, Setup},
    vault::{
        init_vault::{command_init_vault, InitVault},
        init_vault_config::{command_init_vault_config, InitConfig},
        init_vault_ncn_ticket::{command_init_vault_ncn_ticket, InitVaultNcnTicket},
        init_vault_operator_delegatin::{
            command_init_vault_operator_delegation, InitVaultOperatorDelegation,
        },
        warmup_vault_ncn_ticket::{command_warmup_vault_ncn_ticket, WarmupVaultNcnTicket},
    },
};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Setup(Setup),

    // Restaking
    InitRestakingConfig(InitRestakingConfig),
    InitNcn(InitNcn),
    InitNcnOperatorState(InitNcnOperatorState),
    InitNcnVaultTicket(InitNcnVaultTicket),
    InitOperator(InitOperator),
    InitOperatorVaultTicket(InitOperatorVaultTicket),
    NcnWarmupOperator(NcnWarmupOperator),
    WarmupNcnVaultTicket(WarmupNcnVaultTicket),
    WarmupOperatorVaultTicket(WarmupOperatorVaultTicket),

    // Vault
    InitVaultConfig(InitConfig),
    InitVault(InitVault),
    InitVaultNcnTicket(InitVaultNcnTicket),
    InitVaultOperatorDelegation(InitVaultOperatorDelegation),
    WarmupVaultNcnTicket(WarmupVaultNcnTicket),
    CreateTokenMetadata,
    GetConfig,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();

    match args.commands {
        Commands::Setup(args) => command_setup(args).await,

        // Restaking
        Commands::InitRestakingConfig(args) => command_init_restaking_config(args).await,
        Commands::InitNcn(args) => command_init_ncn(args).await,
        Commands::InitNcnOperatorState(args) => command_init_ncn_operator_state(args).await,
        Commands::InitNcnVaultTicket(args) => command_init_ncn_vault_ticket(args).await,
        Commands::InitOperator(args) => command_init_operator(args).await,
        Commands::InitOperatorVaultTicket(args) => command_init_operator_vault_ticket(args).await,
        Commands::NcnWarmupOperator(args) => command_ncn_warmup_operator_state(args).await,
        Commands::WarmupNcnVaultTicket(args) => command_warmup_ncn_vault_ticket(args).await,
        Commands::WarmupOperatorVaultTicket(args) => {
            command_warmup_operator_vault_ticket(args).await
        }

        // Vault
        Commands::InitVaultConfig(args) => command_init_vault_config(args).await,
        Commands::InitVault(args) => command_init_vault(args).await,
        Commands::InitVaultNcnTicket(args) => command_init_vault_ncn_ticket(args).await,
        Commands::InitVaultOperatorDelegation(args) => {
            command_init_vault_operator_delegation(args).await
        }
        Commands::WarmupVaultNcnTicket(args) => command_warmup_vault_ncn_ticket(args).await,
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
