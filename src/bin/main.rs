use std::error;

use clap::{Parser, Subcommand};
use quick::{
    restaking::{
        init_ncn_vault_ticket::{command_init_ncn_vault_ticket, InitNcnVaultTicket},
        init_operator::{command_init_operator, InitOperator},
        init_operator_vault_ticket::{command_init_operator_vault_ticket, InitOperatorVaultTicket},
        list_ncn::{command_list_ncn, ListNcn},
        list_ncn_vault_slasher_ticket::{
            command_list_ncn_vault_slasher_ticket, ListNcnVaultSlasherTicket,
        },
        list_opeator_vault_ticket::{command_list_operator_vault_ticket, ListOperatorVaultTicket},
        ncn::{command_ncn, Ncn},
        restaking_config::{command_get_restaking_config, RestakingConfig},
        warmup_ncn_vault_ticket::{command_list_ncn_vault_ticket, ListNcnVaultTicket},
    },
    vault::{
        list_vault_ncn_slasher_operator_ticket::{
            command_list_vault_ncn_slasher_operator_ticket, ListVaultNcnSlasherOperatorTicket,
        },
        list_vault_ncn_slasher_ticket::{
            command_list_vault_ncn_slasher_ticket, ListVaultNcnSlasherTicket,
        },
        list_vault_ncn_ticket::{command_list_vault_ncn_ticket, ListVaultNcnTicket},
        list_vault_operator_delegation::{
            command_list_vault_operator_delegation, ListVaultOperatorDelegation,
        },
        list_vault_staker_withdrawal_ticket::{
            command_list_vault_staker_withdrawal_ticket, ListVaultStakerWithdrawalTicket,
        },
    },
};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Restaking
    RestakingConfig(RestakingConfig),
    ListNcn(ListNcn),
    Ncn(Ncn),
    ListNcnVaultTicket(ListNcnVaultTicket),
    ListOperatorVaultTicket(ListOperatorVaultTicket),
    ListNcnVaultSlasherTicket(ListNcnVaultSlasherTicket),
    InitNcnVaultTicket(InitNcnVaultTicket),
    InitOperator(InitOperator),
    InitOperatorVaultTicket(InitOperatorVaultTicket),

    // Vault
    ListVaultNcnSlasherOperatorTicket(ListVaultNcnSlasherOperatorTicket),
    ListVaultNcnSlasherTicket(ListVaultNcnSlasherTicket),
    ListVaultNcnTicket(ListVaultNcnTicket),
    ListVaultOperatorDelegation(ListVaultOperatorDelegation),
    ListVaultStakerWithdrawalTicket(ListVaultStakerWithdrawalTicket),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();

    match args.commands {
        // Restaking
        Commands::RestakingConfig(args) => command_get_restaking_config(args).await,
        Commands::ListNcn(args) => command_list_ncn(args).await,
        Commands::Ncn(args) => command_ncn(args).await,
        Commands::ListNcnVaultSlasherTicket(args) => {
            command_list_ncn_vault_slasher_ticket(args).await
        }
        Commands::ListNcnVaultTicket(args) => command_list_ncn_vault_ticket(args).await,
        Commands::ListOperatorVaultTicket(args) => command_list_operator_vault_ticket(args).await,
        Commands::InitNcnVaultTicket(args) => command_init_ncn_vault_ticket(args).await,
        Commands::InitOperator(args) => command_init_operator(args).await,
        Commands::InitOperatorVaultTicket(args) => command_init_operator_vault_ticket(args).await,

        // Vault
        Commands::ListVaultNcnSlasherOperatorTicket(args) => {
            command_list_vault_ncn_slasher_operator_ticket(args).await
        }
        Commands::ListVaultNcnSlasherTicket(args) => {
            command_list_vault_ncn_slasher_ticket(args).await
        }
        Commands::ListVaultNcnTicket(args) => command_list_vault_ncn_ticket(args).await,
        Commands::ListVaultOperatorDelegation(args) => {
            command_list_vault_operator_delegation(args).await
        }
        Commands::ListVaultStakerWithdrawalTicket(args) => {
            command_list_vault_staker_withdrawal_ticket(args).await
        }
    }
    Ok(())
}
