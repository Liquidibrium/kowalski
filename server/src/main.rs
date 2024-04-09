mod apidoc;
mod configure;
mod db;
mod entities;
mod handlers;
mod models;
mod router;
mod state;

use crate::router::create_api_router;
use crate::state::AppState;

use axum::Router;
use clap::Parser;

use log::info;

use tower_http::compression::CompressionLayer;

use crate::configure::config::Config;
use crate::db::create_db_pool;
use kowalski_core::memory::memory_db::{EmbeddingMemory, EmbeddingMemoryQdrant};
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let config = Config::parse();

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

    let db = create_db_pool(&config.database_url).await?;

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Errors occurred while running migration");

    let embedding_memory = EmbeddingMemoryQdrant::new(config.qdrant_url.as_str());

    // build our application with a single route
    let state = AppState {
        db,
        embedding_memory,
    };

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

    info!("Listening on port {}", config.port);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
