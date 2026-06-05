use std::sync::Arc;

use axum::{Router, routing::{get, post}};
use dotenvy::dotenv;
use uaparser::UserAgentParser;

mod db;
mod handlers;
mod models;
mod processing;
mod state;

use processing::UA_REGEXES;
use state::{AppState, SaltStore, run_salt_rotation};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let ua_parser = UserAgentParser::from_bytes(UA_REGEXES)
        .expect("Failed to load UA parser regexes");

    let clickhouse = clickhouse::Client::default()
        .with_url(dotenvy::var("CLICKHOUSE_URL").expect("CLICKHOUSE_URL must be set"))
        .with_database(dotenvy::var("CLICKHOUSE_DB").unwrap_or_else(|_| "analytics".into()))
        .with_user(dotenvy::var("CLICKHOUSE_USER").unwrap_or_else(|_| "analytics".into()))
        .with_password(dotenvy::var("CLICKHOUSE_PASSWORD").unwrap_or_else(|_| "analytics".into()));

    let state = Arc::new(AppState {
        queue_auth_token: dotenvy::var("QUEUE_AUTH_TOKEN")
            .expect("QUEUE_AUTH_TOKEN env var must be set"),
        salt_store: SaltStore::new(),
        ua_parser,
        clickhouse,
    });

    tokio::spawn(run_salt_rotation(Arc::clone(&state)));

    let app = Router::new()
        .route("/health", get(handlers::health))
        .route("/queue/events", post(handlers::events))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::info!("analytics-service listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
