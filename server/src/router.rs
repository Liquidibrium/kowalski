use crate::analyze::analyze_handler;
use crate::state::AppState;
use axum::routing::{get, post};
use axum::Router;
use http::header::{ACCEPT, AUTHORIZATION, ORIGIN};
use http::Method;
use tower_http::cors::{AllowOrigin, CorsLayer};

pub fn create_api_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
    //     .allow_credentials(true)
    //     .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
    //     .allow_headers(vec![ORIGIN, AUTHORIZATION, ACCEPT])
        .allow_origin(AllowOrigin::any());

    Router::new()
        .route("/analyze", post(analyze_handler))
        .route("/health", get(health_check))
        .with_state(state)
        .layer(cors)
}

pub async fn health_check() -> &'static str {
    "status: OK"
}
