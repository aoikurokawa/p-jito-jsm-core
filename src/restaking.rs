pub mod init_ncn;
pub mod init_ncn_operator_state;
pub mod init_ncn_vault_ticket;
pub mod init_operator;
pub mod init_operator_vault_ticket;
pub mod init_restaking_config;
pub mod ncn_warmup_operator;
pub mod operator_warmup_ncn;
pub mod warmup_ncn_vault_ticket;
pub mod warmup_opeator_vault_ticket;

use jito_restaking_client::instructions::{
    InitializeNcnBuilder, InitializeNcnOperatorStateBuilder, InitializeNcnVaultTicketBuilder,
    InitializeOperatorBuilder, InitializeOperatorVaultTicketBuilder, NcnWarmupOperatorBuilder,
    OperatorWarmupNcnBuilder, WarmupNcnVaultTicketBuilder, WarmupOperatorVaultTicketBuilder,
};
use jito_restaking_core::{
    ncn::Ncn, ncn_operator_state::NcnOperatorState, ncn_vault_ticket::NcnVaultTicket,
    operator::Operator, operator_vault_ticket::OperatorVaultTicket,
};
use solana_client::nonblocking::rpc_client::RpcClient;
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

    pub async fn initialize_config(&self, vault_program_id: Pubkey) {
        let rpc_client = self.get_rpc_client();

        let mut ix_builder = jito_restaking_client::instructions::InitializeConfigBuilder::new();
        let config_address =
            jito_restaking_core::config::Config::find_program_address(&self.restaking_program_id).0;
        ix_builder
            .config(config_address)
            .admin(self.payer.pubkey())
            .vault_program(vault_program_id);
        let mut ix = ix_builder.instruction();
        ix.program_id = self.restaking_program_id;

        let blockhash = rpc_client.get_latest_blockhash().await.expect("");
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[&self.payer],
            blockhash,
        );

        println!("Initialize Restaking Config");
        let sig = rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .expect("");
        println!("Signature: {sig}");
    }

    pub async fn initialize_ncn(&self, base: &Keypair) {
        let rpc_client = self.get_rpc_client();

        let ncn = Ncn::find_program_address(&self.restaking_program_id, &base.pubkey()).0;

        let mut ix_builder = InitializeNcnBuilder::new();
        ix_builder
            .config(
                jito_restaking_core::config::Config::find_program_address(
                    &self.restaking_program_id,
                )
                .0,
            )
            .ncn(ncn)
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

        println!("Initialize NCN");
        let sig = rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .expect("");
        println!("Signature: {sig}");
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
