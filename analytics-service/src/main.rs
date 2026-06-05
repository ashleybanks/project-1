use std::sync::Arc;

use axum::{
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dotenvy::dotenv;
use rand::{rngs::OsRng, RngCore};
use serde::Deserialize;
use tracing::info;

use blake3::Hasher;

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
    salt_store: SaltStore,
}

struct SaltStore {
    current: tokio::sync::RwLock<[u8; 32]>,
}

impl SaltStore {
    fn new() -> Self {
        Self {
            current: tokio::sync::RwLock::new(generate_salt()),
        }
    }
    async fn get(&self) -> [u8; 32] {
        *self.current.read().await
    }
    async fn rotate(&self) {
        *self.current.write().await = generate_salt();
    }
}

fn generate_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);
    salt
}

async fn run_salt_rotation(state: Arc<AppState>) {
    loop {
        let now = chrono::Utc::now();
        let next_midnight = (now + chrono::Duration::days(1))
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();
        let duration = (next_midnight - now).to_std().unwrap();
        tokio::time::sleep(duration).await;
        state.salt_store.rotate().await;
        info!("Daily salt rotated")
    }
}

fn generate_hash(salt: [u8; 32], input: &EventPayload) -> String {
    let utc_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let mut hasher = Hasher::new_keyed(&salt);
    hasher.update(input.site_id.as_bytes());
    hasher.update(input.ip.as_bytes());
    hasher.update(input.user_agent.as_bytes());
    hasher.update(utc_date.as_bytes());
    hasher.finalize().to_string()
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let state = Arc::new(AppState {
        queue_auth_token: dotenvy::var("QUEUE_AUTH_TOKEN")
            .expect("QUEUE_AUTH_TOKEN env var must be set"),
        salt_store: SaltStore::new(),
    });
    tokio::spawn(run_salt_rotation(Arc::clone(&state)));
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .route("/health", get(health))
        .route("/queue/events", post(events))
        .with_state(state);

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

async fn events(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(e): Json<QueuedMessage>,
) -> impl IntoResponse {
    let expected_token =
        HeaderValue::try_from(format!("Bearer {}", state.queue_auth_token)).unwrap();
    let supplied_token = headers
        .get("Authorization")
        .unwrap_or(&HeaderValue::from_static("Not found"))
        .clone();
    if expected_token != supplied_token {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    for m in e.messages.iter() {
        let _hash = generate_hash(state.salt_store.get().await, &m.body);
        // write to clickhouse
        info!("{} written to clickhouse", m.id)
    }
    StatusCode::OK.into_response()
}
