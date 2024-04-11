use crate::state::AppState;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;

use crate::handlers::analyze::analyze_handler;
use crate::handlers::auth::{login_handler, register_handler};
use crate::handlers::health::health_check;
use tower_http::cors::{AllowOrigin, CorsLayer};

pub fn create_api_router(state: AppState) -> Router {
    let cors = CorsLayer::permissive();
        //     .allow_credentials(true)
        //     .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        //     .allow_headers(vec![ORIGIN, AUTHORIZATION, ACCEPT])
        // .allow_origin(AllowOrigin::any());

    Router::new()
        .route("/analyze", post(analyze_handler))
        .route("/health", get(health_check))
        .nest("/auth", create_auth_router())
        .with_state(Arc::new(state))
        .layer(cors)
}

fn create_auth_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/register", post(register_handler))
    // .route("/active", post(active_handler))
}
