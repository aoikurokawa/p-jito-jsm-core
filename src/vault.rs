pub mod create_token_metadata;
pub mod init_vault;
pub mod init_vault_config;
pub mod init_vault_ncn_ticket;
pub mod init_vault_operator_delegatin;
pub mod warmup_vault_ncn_ticket;

use jito_restaking_core::{
    ncn_vault_ticket::NcnVaultTicket, operator_vault_ticket::OperatorVaultTicket,
};
use jito_vault_client::instructions::{
    InitializeConfigBuilder, InitializeVaultBuilder, InitializeVaultNcnTicketBuilder,
    InitializeVaultOperatorDelegationBuilder, WarmupVaultNcnTicketBuilder,
};
use jito_vault_core::{
    config::Config, vault::Vault, vault_ncn_ticket::VaultNcnTicket,
    vault_operator_delegation::VaultOperatorDelegation,
};
use solana_client::nonblocking::rpc_client::RpcClient;
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

    pub async fn initialize_config(&self) {
        let rpc_client = self.get_rpc_client();

        let mut ix_builder = InitializeConfigBuilder::new();
        let config_address = Config::find_program_address(&self.vault_program_id).0;
        let ix_builder = ix_builder
            .config(config_address)
            .admin(self.payer.pubkey())
            .restaking_program(self.restaking_program_id);
        let mut ix = ix_builder.instruction();
        ix.program_id = self.vault_program_id;

        let blockhash = rpc_client.get_latest_blockhash().await.expect("");
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[&self.payer],
            blockhash,
        );

        println!("Initialize Vault Config");
        let sig = rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .expect("");
        println!("Signature: {sig}");
    }

    pub async fn initialize(&self, base: &Keypair, token_mint: Pubkey) {
        let rpc_client = self.get_rpc_client();

        let vault = Vault::find_program_address(&self.vault_program_id, &base.pubkey()).0;

        let vrt_mint = Keypair::new();

        let mut ix_builder = InitializeVaultBuilder::new();
        ix_builder
            .config(Config::find_program_address(&self.vault_program_id).0)
            .vault(vault)
            .vrt_mint(vrt_mint.pubkey())
            .token_mint(token_mint)
            .admin(self.payer.pubkey())
            .base(base.pubkey())
            .deposit_fee_bps(0)
            .withdrawal_fee_bps(0)
            .reward_fee_bps(0)
            .decimals(9);
        let mut ix = ix_builder.instruction();
        ix.program_id = self.vault_program_id;

        let blockhash = rpc_client.get_latest_blockhash().await.expect("");
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.payer.pubkey()),
            &[self.payer, base, &vrt_mint],
            blockhash,
        );

        println!("Initialize Vault");
        let sig = rpc_client
            .send_and_confirm_transaction(&tx)
            .await
            .expect("");
        println!("Signature {sig}");
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
