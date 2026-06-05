use std::sync::Arc;

use axum::{
    Json, Router, extract::State, http::{HeaderMap, HeaderValue, StatusCode}, response::IntoResponse, routing::{get, post}
};
use dotenvy::dotenv;
use tracing::info;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QueuedMessage {
    account_id: String,
    queue: String,
    messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
struct Message {
    id: String,
    timestamp: String,
    body: EventPayload,
    attempts: u8,
}

#[derive(Debug, Deserialize)]
struct EventPayload {
    site_id: String,
    timestamp: String,
    ip: String,
    user_agent: String,
    country: String,
    asn: String,
    referrer: Option<String>,
    request_path: Option<String>,
}

struct AppState {
    queue_auth_token: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let state = Arc::new(AppState {
        queue_auth_token: dotenvy::var("QUEUE_AUTH_TOKEN").expect("QUEUE_AUTH_TOKEN env var must be set")
    });
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .route("/health", get(health))
        .route("/queue/events", post(events)).with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::info!(
        "analytics-service listening on {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "ok"
}

async fn events(State(state): State<Arc<AppState>>, headers: HeaderMap, Json(e): Json<QueuedMessage>) -> impl IntoResponse {
    let expected_token = HeaderValue::try_from(format!("Bearer {}", state.queue_auth_token)).unwrap();
    let supplied_token = headers.get("Authorization").unwrap_or(&HeaderValue::from_static("Not found")).clone();
    if expected_token != supplied_token {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    for m in e.messages.iter(){
        info!("{}", m.id)
    }
    StatusCode::OK.into_response()
}
