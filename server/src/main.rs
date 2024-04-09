mod router;
mod state;
mod apidoc;
mod handlers;
mod models;
mod configure;

use anyhow::Context;
use crate::router::create_api_router;
use crate::state::AppState;
use axum::Router;
use clap::Parser;
use http::HeaderName;
use log::info;
use sqlx::postgres::PgPoolOptions;
use tower_http::compression::CompressionLayer;
use tower_http::propagate_header::PropagateHeaderLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use kowalski_core::memory::memory_db::{EmbeddingMemory, EmbeddingMemoryQdrant};
use crate::configure::config::Config;

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

    // We create a single connection pool for SQLx that's shared across the whole application.
    // This saves us from opening a new connection for every API call, which is wasteful.
    let db = PgPoolOptions::new()
        // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
        // Since we're using the default superuser we don't have to worry about this too much,
        // although we should leave some connections available for manual access.
        //
        // If you're deploying your application with multiple replicas, then the total
        // across all replicas should not exceed the Postgres connection limit.
        .max_connections(50)
        .connect(&config.postgres_url)
        .await
        .context("could not connect to database_url")?;
    // sqlx::migrate!()
    //     .run(&postgres)
    //     .await
    //     .expect("Had some errors running migrations :(");

    let embedding_memory = EmbeddingMemoryQdrant::new(config.qdrant_url.as_str());
    
    // build our application with a single route
    let state = AppState {
        db,
        embedding_memory
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
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port)).await.unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
