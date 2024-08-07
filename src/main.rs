use std::{str::FromStr, sync::Arc};

use dotenv::dotenv;
use jito_jsm_api::{router, RouterState};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let rpc_url = std::env::var("RPC_URL").expect("RPC_URL must be set.");
    let program_id = std::env::var("PROGRAM_ID").expect("PROGRAM_ID must be set.");

    let rpc_client = RpcClient::new(rpc_url);

    let state = Arc::new(RouterState {
        program_id: Pubkey::from_str(&program_id).expect("Fail to read program_id"),
        rpc_client,
    });

    let app = router::get_routes(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
