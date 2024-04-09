mod router;
mod state;
mod apidoc;
mod handlers;
mod models;

use crate::router::create_api_router;
use crate::state::AppState;
use axum::Router;
use http::HeaderName;
use log::info;
use tower_http::compression::CompressionLayer;
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    // sqlx::migrate!()
    //     .run(&postgres)
    //     .await
    //     .expect("Had some errors running migrations :(");

    // build our application with a single route
    let state = AppState {};

    let api_router = create_api_router(state);
        // .layer(PropagateHeaderLayer::new(HeaderName::from_static(
        //     "x-request-id",
        // )));

    let router = Router::new()
        .nest("/api", api_router)
        .merge(apidoc::router())
        .layer(TraceLayer::new_for_http())
        // .nest_service(
        //     "/",
        //     ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),
        // )
        .layer(CompressionLayer::new());

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    info!("Listening on port {}", port);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
