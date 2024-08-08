use std::{str::FromStr, sync::Arc};

use dotenv::dotenv;
use jito_jsm_api::{router, RouterState, JITO_RESTAKING_PROGRAM_ID, JITO_VAULT_PROGRAM_ID};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let rpc_url = std::env::var("RPC_URL").expect("RPC_URL must be set.");

    let rpc_client = RpcClient::new(rpc_url);

    let state = Arc::new(RouterState {
        jito_vault_program_id: Pubkey::from_str(&JITO_VAULT_PROGRAM_ID)
            .expect("Fail to read jito vault program_id"),
        jito_restaking_program_id: Pubkey::from_str(&JITO_RESTAKING_PROGRAM_ID)
            .expect("Fail to read jito restaking program_id"),
        admin: read_keypair_file("~/.config/solana/id.json").expect("Fail to read admin pubkey"),
        rpc_client,
    });

    let app = router::get_routes(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
