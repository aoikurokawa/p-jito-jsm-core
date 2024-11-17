pub mod init_ncn_vault_ticket;
pub mod init_operator;
pub mod init_operator_vault_ticket;
pub mod list_ncn;
pub mod list_ncn_operator_state;
pub mod list_ncn_vault_slasher_ticket;
pub mod list_opeator_vault_ticket;
pub mod ncn;
pub mod operator_warmup_ncn;
pub mod restaking_config;
pub mod warmup_ncn_vault_ticket;

use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_restaking_client::instructions::{
    InitializeNcnOperatorStateBuilder, InitializeNcnVaultTicketBuilder, InitializeOperatorBuilder,
    InitializeOperatorVaultTicketBuilder, NcnWarmupOperatorBuilder, OperatorWarmupNcnBuilder,
    WarmupNcnVaultTicketBuilder, WarmupOperatorVaultTicketBuilder,
};
use jito_restaking_core::{
    ncn::Ncn, ncn_operator_state::NcnOperatorState, ncn_vault_ticket::NcnVaultTicket,
    operator::Operator, operator_vault_ticket::OperatorVaultTicket,
};
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair, signer::Signer,
    transaction::Transaction,
};

pub struct RestakingHandler<'a> {
    rpc_url: String,
    payer: &'a Keypair,
    restaking_program_id: Pubkey,
}

impl<'a> RestakingHandler<'a> {
    pub fn new(rpc_url: &str, payer: &'a Keypair, restaking_program_id: Pubkey) -> Self {
        Self {
            rpc_url: rpc_url.to_string(),
            payer,
            restaking_program_id,
        }
    }

    fn get_rpc_client(&self) -> RpcClient {
        RpcClient::new_with_commitment(self.rpc_url.clone(), CommitmentConfig::confirmed())
    }

    pub async fn list_account<T: Discriminator + AccountDeserialize + std::fmt::Debug>(&self) {
        let rpc_client = self.get_rpc_client();
        let accounts = rpc_client
            .get_program_accounts_with_config(
                &self.restaking_program_id,
                RpcProgramAccountsConfig {
                    filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new(
                        0,
                        MemcmpEncodedBytes::Bytes(vec![T::DISCRIMINATOR]),
                    ))]),
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        data_slice: None,
                        commitment: None,
                        min_context_slot: None,
                    },
                    with_context: None,
                },
            )
            .await
            .expect("Failed to get Account");

        let _ncns: Vec<(Pubkey, T)> = accounts
            .iter()
            .filter_map(|(pubkey, acc)| {
                println!("Pubkey: {}", pubkey);
                println!("Account data: {:?}", acc.data);
                let vault_ncn_ticket =
                    T::try_from_slice_unchecked(&acc.data).expect("Failed to deseriaze");
                println!("Account {:?}", vault_ncn_ticket);
                Some((*pubkey, *vault_ncn_ticket))
            })
            .collect();
    }

    pub async fn get_account<T: AccountDeserialize>(&self, acc_pubkey: Pubkey) -> T {
        let rpc_client = self.get_rpc_client();

        let acc_data = rpc_client
            .get_account_data(&acc_pubkey)
            .await
            .expect("Failed to get account");

        println!("Account data: {:?}", acc_data.len());

        *T::try_from_slice_unchecked(&acc_data).expect("Failed to deserialzie")
    }

    pub async fn get_config(&self) {
        let rpc_client = self.get_rpc_client();

        let config_address =
            jito_restaking_core::config::Config::find_program_address(&self.restaking_program_id).0;

        let config_acc = rpc_client
            .get_account_data(&config_address)
            .await
            .expect("Failed to get config account");

        let config = jito_restaking_core::config::Config::try_from_slice_unchecked(&config_acc)
            .expect("Failed to deserialize");

        println!("Config: {:?}", config);
    }

    pub async fn get_ncns(&self) {
        let rpc_client = self.get_rpc_client();
        let accounts = rpc_client
            .get_program_accounts_with_config(
                &self.restaking_program_id,
                RpcProgramAccountsConfig {
                    filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new(
                        0,
                        MemcmpEncodedBytes::Bytes(vec![Ncn::DISCRIMINATOR]),
                    ))]),
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        data_slice: None,
                        commitment: None,
                        min_context_slot: None,
                    },
                    with_context: None,
                },
            )
            .await
            .expect("Failed to get NCNs");

        let _ncns: Vec<(Pubkey, Ncn)> = accounts
            .iter()
            .filter_map(|(pubkey, acc)| {
                println!("Ncn pubkey: {}", pubkey);
                let ncn =
                    Ncn::try_from_slice_unchecked(&acc.data).expect("Failed to deseriaze NCN");
                Some((*pubkey, *ncn))
            })
            .collect();
    }

    pub async fn list_ncn_operator_state(&self) {
        let rpc_client = self.get_rpc_client();
        let accounts = rpc_client
            .get_program_accounts_with_config(
                &self.restaking_program_id,
                RpcProgramAccountsConfig {
                    filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new(
                        0,
                        MemcmpEncodedBytes::Bytes(vec![NcnOperatorState::DISCRIMINATOR]),
                    ))]),
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        data_slice: None,
                        commitment: None,
                        min_context_slot: None,
                    },
                    with_context: None,
                },
            )
            .await
            .expect("Failed to get NCNs");

        let _ncns: Vec<(Pubkey, NcnOperatorState)> = accounts
            .iter()
            .filter_map(|(pubkey, acc)| {
                let ncn_operator_state = NcnOperatorState::try_from_slice_unchecked(&acc.data)
                    .expect("Failed to deseriaze NCN");
                println!("NcnOperatorState pubkey: {}", pubkey);
                println!("NcnOperatorState acc data: {:?}", acc.data);
                println!("NcnOperatorState {:?}", ncn_operator_state);
                Some((*pubkey, *ncn_operator_state))
            })
            .collect();
    }

    pub async fn list_ncn_vault_ticket(&self) {
        let rpc_client = self.get_rpc_client();
        let accounts = rpc_client
            .get_program_accounts_with_config(
                &self.restaking_program_id,
                RpcProgramAccountsConfig {
                    filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new(
                        0,
                        MemcmpEncodedBytes::Bytes(vec![NcnVaultTicket::DISCRIMINATOR]),
                    ))]),
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        data_slice: None,
                        commitment: None,
                        min_context_slot: None,
                    },
                    with_context: None,
                },
            )
            .await
            .expect("Failed to get NCNs");

        let _ncns: Vec<(Pubkey, NcnVaultTicket)> = accounts
            .iter()
            .filter_map(|(pubkey, acc)| {
                let ncn_vault_ticket = NcnVaultTicket::try_from_slice_unchecked(&acc.data)
                    .expect("Failed to deseriaze NCN");
                println!("NcnVaultTicket pubkey: {}", pubkey);
                println!("NcnVaultTicket acc data: {:?}", acc.data);
                println!("NcnVaultTicket {:?}", ncn_vault_ticket);
                Some((*pubkey, *ncn_vault_ticket))
            })
            .collect();
    }

    pub async fn list_operator_vault_ticket(&self) {
        let rpc_client = self.get_rpc_client();
        let accounts = rpc_client
            .get_program_accounts_with_config(
                &self.restaking_program_id,
                RpcProgramAccountsConfig {
                    filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new(
                        0,
                        MemcmpEncodedBytes::Bytes(vec![OperatorVaultTicket::DISCRIMINATOR]),
                    ))]),
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        data_slice: None,
                        commitment: None,
                        min_context_slot: None,
                    },
                    with_context: None,
                },
            )
            .await
            .expect("Failed to get NCNs");

        let _ncns: Vec<(Pubkey, OperatorVaultTicket)> = accounts
            .iter()
            .filter_map(|(pubkey, acc)| {
                let operator_vault_ticket =
                    OperatorVaultTicket::try_from_slice_unchecked(&acc.data)
                        .expect("Failed to deseriaze NCN");
                println!("OperatorVaultTicket pubkey: {}", pubkey);
                println!("OperatorVaultTicket acc data: {:?}", acc.data);
                println!("OperatorVaultTicket {:?}", operator_vault_ticket);
                Some((*pubkey, *operator_vault_ticket))
            })
            .collect();
    }

    pub async fn initialize_operator(&self, base: &Keypair) {
        let rpc_client = self.get_rpc_client();

        let operator = Operator::find_program_address(&self.restaking_program_id, &base.pubkey()).0;

        let mut ix_builder = InitializeOperatorBuilder::new();
        ix_builder
            .config(
                jito_restaking_core::config::Config::find_program_address(
                    &self.restaking_program_id,
                )
                .0,
            )
            .operator(operator)
            .admin(self.payer.pubkey())
            .base(base.pubkey());
        let mut ix = ix_builder.instruction();
        ix.program_id = self.restaking_program_id;

        let blockhash = rpc_client.get_latest_blockhash().await.expect("");
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[self.payer, base],
            blockhash,
        );

        println!("Initialize Operator");
        let sig = rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .expect("");
        println!("Signature: {sig}");
    }

    pub async fn initialize_ncn_vault_ticket(&self, ncn: Pubkey, vault: Pubkey) {
        let rpc_client = self.get_rpc_client();

        let ncn_vault_ticket =
            NcnVaultTicket::find_program_address(&self.restaking_program_id, &ncn, &vault).0;

        let mut ix_builder = InitializeNcnVaultTicketBuilder::new();
        ix_builder
            .config(
                jito_restaking_core::config::Config::find_program_address(
                    &self.restaking_program_id,
                )
                .0,
            )
            .ncn(ncn)
            .vault(vault)
            .ncn_vault_ticket(ncn_vault_ticket)
            .admin(self.payer.pubkey())
            .payer(self.payer.pubkey());
        let mut ix = ix_builder.instruction();
        ix.program_id = self.restaking_program_id;

        let blockhash = rpc_client.get_latest_blockhash().await.expect("");
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[self.payer, self.payer],
            blockhash,
        );

        println!("Initialize NCN Vault Ticket");
        let sig = rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .expect("");
        println!("Signature: {sig}");
    }

    pub async fn initialize_ncn_operator_state(&self, ncn: Pubkey, operator: Pubkey) {
        let rpc_client = self.get_rpc_client();

        let ncn_operator_state =
            NcnOperatorState::find_program_address(&self.restaking_program_id, &ncn, &operator).0;

        let mut ix_builder = InitializeNcnOperatorStateBuilder::new();
        ix_builder
            .config(
                jito_restaking_core::config::Config::find_program_address(
                    &self.restaking_program_id,
                )
                .0,
            )
            .ncn(ncn)
            .operator(operator)
            .ncn_operator_state(ncn_operator_state)
            .admin(self.payer.pubkey())
            .payer(self.payer.pubkey());
        let mut ix = ix_builder.instruction();
        ix.program_id = self.restaking_program_id;

        let blockhash = rpc_client.get_latest_blockhash().await.expect("");
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[self.payer, self.payer],
            blockhash,
        );

        println!("Initialize NCN Operator State");
        let sig = rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .expect("");
        println!("Signature: {sig}");
    }

    pub async fn initialize_operator_vault_ticket(&self, operator: Pubkey, vault: Pubkey) {
        let rpc_client = self.get_rpc_client();

        let operator_vault_ticket = OperatorVaultTicket::find_program_address(
            &self.restaking_program_id,
            &operator,
            &vault,
        )
        .0;

        let mut ix_builder = InitializeOperatorVaultTicketBuilder::new();
        ix_builder
            .config(
                jito_restaking_core::config::Config::find_program_address(
                    &self.restaking_program_id,
                )
                .0,
            )
            .operator(operator)
            .vault(vault)
            .operator_vault_ticket(operator_vault_ticket)
            .admin(self.payer.pubkey())
            .payer(self.payer.pubkey());
        let mut ix = ix_builder.instruction();
        ix.program_id = self.restaking_program_id;

        let blockhash = rpc_client.get_latest_blockhash().await.expect("");
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[self.payer, self.payer],
            blockhash,
        );

        println!("Initialize Operator Vault Ticket");
        let sig = rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .expect("");
        println!("Signature: {sig}");
    }

    pub async fn warmup_ncn_vault_ticket(&self, ncn: Pubkey, vault: Pubkey) {
        let rpc_client = self.get_rpc_client();

        let ncn_vault_ticket =
            NcnVaultTicket::find_program_address(&self.restaking_program_id, &ncn, &vault).0;

        let mut ix_builder = WarmupNcnVaultTicketBuilder::new();
        ix_builder
            .config(
                jito_restaking_core::config::Config::find_program_address(
                    &self.restaking_program_id,
                )
                .0,
            )
            .ncn(ncn)
            .vault(vault)
            .ncn_vault_ticket(ncn_vault_ticket)
            .admin(self.payer.pubkey());
        let mut ix = ix_builder.instruction();
        ix.program_id = self.restaking_program_id;

        let blockhash = rpc_client.get_latest_blockhash().await.expect("");
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[self.payer, self.payer],
            blockhash,
        );

        println!("Warmup NCN Vault Ticket");
        let sig = rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .expect("");
        println!("Signature: {sig}");
    }

    pub async fn ncn_warmup_operator(&self, ncn: Pubkey, operator: Pubkey) {
        let rpc_client = self.get_rpc_client();

        let ncn_operator_state =
            NcnOperatorState::find_program_address(&self.restaking_program_id, &ncn, &operator).0;

        let mut ix_builder = NcnWarmupOperatorBuilder::new();
        ix_builder
            .config(
                jito_restaking_core::config::Config::find_program_address(
                    &self.restaking_program_id,
                )
                .0,
            )
            .ncn(ncn)
            .operator(operator)
            .ncn_operator_state(ncn_operator_state)
            .admin(self.payer.pubkey());
        let mut ix = ix_builder.instruction();
        ix.program_id = self.restaking_program_id;

        let blockhash = rpc_client.get_latest_blockhash().await.expect("");
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[self.payer, self.payer],
            blockhash,
        );

        println!("Warmup NCN Operator State");
        let sig = rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .expect("");
        println!("Signature: {sig}");
    }

    pub async fn operator_warmup_ncn(&self, ncn: Pubkey, operator: Pubkey) {
        let rpc_client = self.get_rpc_client();

        let ncn_operator_state =
            NcnOperatorState::find_program_address(&self.restaking_program_id, &ncn, &operator).0;

        let mut ix_builder = OperatorWarmupNcnBuilder::new();
        ix_builder
            .config(
                jito_restaking_core::config::Config::find_program_address(
                    &self.restaking_program_id,
                )
                .0,
            )
            .ncn(ncn)
            .operator(operator)
            .ncn_operator_state(ncn_operator_state)
            .admin(self.payer.pubkey());
        let mut ix = ix_builder.instruction();
        ix.program_id = self.restaking_program_id;

        let blockhash = rpc_client.get_latest_blockhash().await.expect("");
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[self.payer, self.payer],
            blockhash,
        );

        println!("Warmup NCN Operator State");
        let sig = rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .expect("");
        println!("Signature: {sig}");
    }

    pub async fn warmup_operator_vault_ticket(&self, operator: Pubkey, vault: Pubkey) {
        let rpc_client = self.get_rpc_client();

        let operator_vault_ticket = OperatorVaultTicket::find_program_address(
            &self.restaking_program_id,
            &operator,
            &vault,
        )
        .0;

        let mut ix_builder = WarmupOperatorVaultTicketBuilder::new();
        ix_builder
            .config(
                jito_restaking_core::config::Config::find_program_address(
                    &self.restaking_program_id,
                )
                .0,
            )
            .operator(operator)
            .vault(vault)
            .operator_vault_ticket(operator_vault_ticket)
            .admin(self.payer.pubkey());
        let mut ix = ix_builder.instruction();
        ix.program_id = self.restaking_program_id;

        let blockhash = rpc_client.get_latest_blockhash().await.expect("");
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[self.payer, self.payer],
            blockhash,
        );

        println!("Warmup Operator Vault Ticket");
        let sig = rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .expect("");
        println!("Signature: {sig}");
    }
}
