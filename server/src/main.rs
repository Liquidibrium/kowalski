mod analyze;
mod router;
mod state;

use crate::router::create_api_router;
use crate::state::AppState;
use axum::Router;
use http::HeaderName;
use tower_http::compression::CompressionLayer;
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // sqlx::migrate!()
    //     .run(&postgres)
    //     .await
    //     .expect("Had some errors running migrations :(");

    // build our application with a single route
    let state = AppState {};

    let api_router = create_api_router(state)
        .layer(TraceLayer::new_for_http())
        .layer(PropagateHeaderLayer::new(HeaderName::from_static(
            "x-request-id",
        )));

    let router = Router::new()
        .nest("/api", api_router)
        .nest_service(
            "/",
            ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),
        )
        .layer(CompressionLayer::new());

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
