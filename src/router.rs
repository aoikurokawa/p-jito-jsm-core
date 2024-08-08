use std::sync::Arc;

use axum::{extract::State, routing::get, Json, Router};
use serde_json::{json, Value};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::{message::legacy, pubkey::Pubkey};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

pub struct RouterState {
    pub jito_vault_program_id: Pubkey,
    pub jito_restaking_program_id: Pubkey,
    pub admin: Keypair,
    pub rpc_client: RpcClient,
}

pub fn get_routes(state: Arc<RouterState>) -> Router {
    // let middleware = ServiceBuilder::new()
    //     .layer(HandleErrorLayer::new(error::handle_error))
    //     .layer(BufferLayer::new(1000))
    //     .layer(RateLimitLayer::new(10000, Duration::from_secs(1)))
    //     .layer(TimeoutLayer::new(Duration::from_secs(20)))
    //     .layer(LoadShedLayer::new())
    //     .layer(
    //         TraceLayer::new_for_http()
    //             .on_request(|request: &Request<Body>, _span: &Span| {
    //                 info!("started {} {}", request.method(), request.uri().path())
    //             })
    //             .on_response(
    //                 DefaultOnResponse::new()
    //                     .level(tracing_core::Level::INFO)
    //                     .latency_unit(LatencyUnit::Millis),
    //             ),
    //     );

    let router = Router::new()
        .route("/", get(health_check))
        .route("/initialize_config", get(initialize_config))
        .route("/initialize_vault", get(initialize_vault));
    // .route("/distributor", get(get_distributor))
    // .route("/status/:user_pubkey", get(get_status))
    // .route("/version", get(get_version))

    // don't enable until airdrop starts
    // if enable_proof_endpoint {
    //     router = router.route("/proof/:user_pubkey", get(get_proof));
    // }

    // router.layer(middleware).with_state(state)
    router.with_state(state)
}

async fn health_check() {}

async fn initialize_config(State(state): State<Arc<RouterState>>) -> Json<Value> {
    let config_pubkey =
        jito_vault_core::config::Config::find_program_address(&state.jito_vault_program_id).0;

    let instruction = jito_vault_sdk::initialize_config(
        &state.jito_vault_program_id,
        &config_pubkey,
        &state.admin.pubkey(),
        &state.jito_restaking_program_id,
    );

    let message = legacy::Message::new(&[instruction], Some(&state.admin.pubkey()));

    let blockhash = state.rpc_client.get_latest_blockhash().await.expect("");
    let tx = Transaction::new(&[&state.admin], message, blockhash);

    let sig = state
        .rpc_client
        .send_and_confirm_transaction(&tx)
        .await
        .expect("Fail to send transaction");

    Json(json!({"sig": sig.to_string()}))
}

async fn initialize_vault(State(state): State<Arc<RouterState>>) {
    //     let instruction = VaultInstruction::InitializeVault {
    //         deposit_fee_bps: (),
    //         withdrawal_fee_bps: (),
    //     };
}
