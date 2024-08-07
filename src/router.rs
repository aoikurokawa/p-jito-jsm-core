use std::sync::Arc;

use axum::{routing::get, Router};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;

pub struct RouterState {
    pub program_id: Pubkey,
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

    let router = Router::new().route("/", get(health_check));
    // .route("/users", get(get_users))
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
