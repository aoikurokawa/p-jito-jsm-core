pub mod list_vault_ncn_slasher_operator_ticket;
pub mod list_vault_ncn_slasher_ticket;
pub mod list_vault_ncn_ticket;
pub mod list_vault_operator_delegation;
pub mod list_vault_staker_withdrawal_ticket;

use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_restaking_core::{
    ncn_vault_ticket::NcnVaultTicket, operator_vault_ticket::OperatorVaultTicket,
};
use jito_vault_client::instructions::{
    InitializeVaultNcnTicketBuilder, InitializeVaultOperatorDelegationBuilder,
    WarmupVaultNcnTicketBuilder,
};
use jito_vault_core::{
    config::Config, vault_ncn_slasher_operator_ticket::VaultNcnSlasherOperatorTicket,
    vault_ncn_slasher_ticket::VaultNcnSlasherTicket, vault_ncn_ticket::VaultNcnTicket,
    vault_operator_delegation::VaultOperatorDelegation,
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

pub struct VaultHandler<'a> {
    rpc_url: String,
    payer: &'a Keypair,
    vault_program_id: Pubkey,
    restaking_program_id: Pubkey,
}

impl<'a> VaultHandler<'a> {
    pub fn new(
        rpc_url: &str,
        payer: &'a Keypair,
        vault_program_id: Pubkey,
        restaking_program_id: Pubkey,
    ) -> Self {
        Self {
            rpc_url: rpc_url.to_string(),
            payer,
            vault_program_id,
            restaking_program_id,
        }
    }

    fn get_rpc_client(&self) -> RpcClient {
        RpcClient::new_with_commitment(self.rpc_url.clone(), CommitmentConfig::confirmed())
    }

    pub async fn list_vault_ncn_slasher_operator_ticket(&self) {
        let rpc_client = self.get_rpc_client();
        let accounts = rpc_client
            .get_program_accounts_with_config(
                &self.vault_program_id,
                RpcProgramAccountsConfig {
                    filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new(
                        0,
                        MemcmpEncodedBytes::Bytes(vec![
                            VaultNcnSlasherOperatorTicket::DISCRIMINATOR,
                        ]),
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

        let _ncns: Vec<(Pubkey, VaultNcnSlasherOperatorTicket)> = accounts
            .iter()
            .filter_map(|(pubkey, acc)| {
                println!("VaultNcnSlasherOperatorTicket pubkey: {}", pubkey);
                println!("VaultNcnSlasherOperatorTicket acc data: {:?}", acc.data);
                let vault_ncn_slasher_operator_ticket =
                    VaultNcnSlasherOperatorTicket::try_from_slice_unchecked(&acc.data)
                        .expect("Failed to deseriaze VaultNcnSlasherOperatorTicket");
                println!(
                    "VaultNcnSlasherOperatorTicket {:?}",
                    vault_ncn_slasher_operator_ticket
                );
                Some((*pubkey, *vault_ncn_slasher_operator_ticket))
            })
            .collect();
    }

    pub async fn list_vault_ncn_slasher_ticket(&self) {
        let rpc_client = self.get_rpc_client();
        let accounts = rpc_client
            .get_program_accounts_with_config(
                &self.vault_program_id,
                RpcProgramAccountsConfig {
                    filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new(
                        0,
                        MemcmpEncodedBytes::Bytes(vec![VaultNcnSlasherTicket::DISCRIMINATOR]),
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
            .expect("Failed to get VaultNcnSlasherTicket");

        let _ncns: Vec<(Pubkey, VaultNcnSlasherTicket)> = accounts
            .iter()
            .filter_map(|(pubkey, acc)| {
                println!("VaultNcnSlasherTicket pubkey: {}", pubkey);
                println!("VaultNcnSlasherTicket acc data: {:?}", acc.data);
                let vault_ncn_slasher_ticket =
                    VaultNcnSlasherTicket::try_from_slice_unchecked(&acc.data)
                        .expect("Failed to deseriaze VaultNcnSlasherTicket");
                println!("VaultNcnSlasherTicket {:?}", vault_ncn_slasher_ticket);
                Some((*pubkey, *vault_ncn_slasher_ticket))
            })
            .collect();
    }

    pub async fn list_vault_ncn_ticket(&self) {
        let rpc_client = self.get_rpc_client();
        let accounts = rpc_client
            .get_program_accounts_with_config(
                &self.vault_program_id,
                RpcProgramAccountsConfig {
                    filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new(
                        0,
                        MemcmpEncodedBytes::Bytes(vec![VaultNcnTicket::DISCRIMINATOR]),
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
            .expect("Failed to get VaultNcnTicket");

        let _ncns: Vec<(Pubkey, VaultNcnTicket)> = accounts
            .iter()
            .filter_map(|(pubkey, acc)| {
                println!("VaultNcnTicket pubkey: {}", pubkey);
                println!("VaultNcnTicket acc data: {:?}", acc.data);
                let vault_ncn_ticket = VaultNcnTicket::try_from_slice_unchecked(&acc.data)
                    .expect("Failed to deseriaze VaultNcnTicket");
                println!("VaultNcnTicket {:?}", vault_ncn_ticket);
                Some((*pubkey, *vault_ncn_ticket))
            })
            .collect();
    }

    pub async fn list_accounts<T: Discriminator + AccountDeserialize + std::fmt::Debug>(&self) {
        let rpc_client = self.get_rpc_client();
        let accounts = rpc_client
            .get_program_accounts_with_config(
                &self.vault_program_id,
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
            .expect("Failed to get VaultNcnTicket");

        let _ncns: Vec<(Pubkey, T)> = accounts
            .iter()
            .filter_map(|(pubkey, acc)| {
                println!("VaultNcnTicket pubkey: {}", pubkey);
                println!("VaultNcnTicket acc data: {:?}", acc.data);
                let vault_ncn_ticket = T::try_from_slice_unchecked(&acc.data)
                    .expect("Failed to deseriaze VaultNcnTicket");
                println!("VaultNcnTicket {:?}", vault_ncn_ticket);
                Some((*pubkey, *vault_ncn_ticket))
            })
            .collect();
    }

    pub async fn initialize_vault_operator_delegation(&self, vault: Pubkey, operator: Pubkey) {
        let rpc_client = self.get_rpc_client();

        let operator_vault_ticket = OperatorVaultTicket::find_program_address(
            &self.restaking_program_id,
            &operator,
            &vault,
        )
        .0;
        let vault_operator_delegation = VaultOperatorDelegation::find_program_address(
            &self.vault_program_id,
            &vault,
            &operator,
        )
        .0;

        let mut ix_builder = InitializeVaultOperatorDelegationBuilder::new();
        ix_builder
            .config(Config::find_program_address(&self.vault_program_id).0)
            .vault(vault)
            .operator(operator)
            .operator_vault_ticket(operator_vault_ticket)
            .vault_operator_delegation(vault_operator_delegation)
            .admin(self.payer.pubkey())
            .payer(self.payer.pubkey());
        let mut ix = ix_builder.instruction();
        ix.program_id = self.vault_program_id;

        let blockhash = rpc_client.get_latest_blockhash().await.expect("");
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[self.payer, self.payer],
            blockhash,
        );

        println!("Initialize Vault Operator Delegation");
        let sig = rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .expect("");
        println!("Signature {sig}");
    }

    pub async fn initialize_vault_ncn_ticket(&self, vault: Pubkey, ncn: Pubkey) {
        let rpc_client = self.get_rpc_client();

        let ncn_vault_ticket =
            NcnVaultTicket::find_program_address(&self.restaking_program_id, &ncn, &vault).0;
        let vault_ncn_ticket =
            VaultNcnTicket::find_program_address(&self.vault_program_id, &vault, &ncn).0;

        let mut ix_builder = InitializeVaultNcnTicketBuilder::new();
        ix_builder
            .config(Config::find_program_address(&self.vault_program_id).0)
            .vault(vault)
            .ncn(ncn)
            .ncn_vault_ticket(ncn_vault_ticket)
            .vault_ncn_ticket(vault_ncn_ticket)
            .admin(self.payer.pubkey())
            .payer(self.payer.pubkey());
        let mut ix = ix_builder.instruction();
        ix.program_id = self.vault_program_id;

        let blockhash = rpc_client.get_latest_blockhash().await.expect("");
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[self.payer, self.payer],
            blockhash,
        );

        println!("Initialize Vault NCN Ticket");
        let sig = rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .expect("");
        println!("Signature {sig}");
    }

    pub async fn warmup_vault_ncn_ticket(&self, vault: Pubkey, ncn: Pubkey) {
        let rpc_client = self.get_rpc_client();

        let vault_ncn_ticket =
            VaultNcnTicket::find_program_address(&self.vault_program_id, &vault, &ncn).0;

        let mut ix_builder = WarmupVaultNcnTicketBuilder::new();
        ix_builder
            .config(Config::find_program_address(&self.vault_program_id).0)
            .vault(vault)
            .ncn(ncn)
            .vault_ncn_ticket(vault_ncn_ticket)
            .admin(self.payer.pubkey());
        let mut ix = ix_builder.instruction();
        ix.program_id = self.vault_program_id;

        let blockhash = rpc_client.get_latest_blockhash().await.expect("");
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[self.payer, self.payer],
            blockhash,
        );

        println!("Warmup Vault NCN Ticket");
        let sig = rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .expect("");
        println!("Signature {sig}");
    }
}
