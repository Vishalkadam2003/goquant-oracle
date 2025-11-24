mod config;
mod models;
mod oracle_manager;
mod pyth_client;
mod switchboard_client;
mod price_aggregator;
mod cache;
mod db;
mod health_monitor;
mod utils;

mod routes {
    pub mod rest;
    pub mod ws;
}

use std::sync::Arc;
use axum::Router;
use tracing::info;

use crate::cache::Cache;
use crate::db::Database;
use crate::config::AppConfig;
use crate::oracle_manager::OracleManager;

use axum::serve;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
    pub cache: Cache,
    pub db: Database,
}

pub type SharedState = Arc<AppState>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let cfg = AppConfig::from_env();
    info!("Starting Oracle Backend Service with symbols: {:?}", cfg.symbols);

    // Initialize Cache + DB layers
    let cache = Cache::new(&cfg.redis_url);
    let db = Database::new(&cfg.postgres_url);

    let state = Arc::new(AppState { cache, db });

    // Spawn OracleManager loop
    let om_state = state.clone();
    let om_cfg = cfg.clone();
    tokio::spawn(async move {
        let manager = OracleManager::new(&om_cfg, om_state);
        manager.run().await;
    });

    // Build API router
    let app = Router::new()
        .merge(routes::rest::router(state.clone()))
        .merge(routes::ws::router());

    let addr = "0.0.0.0:8080".parse::<std::net::SocketAddr>().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();

    info!("HTTP server listening on {}", addr);

    serve(listener, app).await.unwrap();
}
