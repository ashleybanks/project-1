use std::sync::Arc;

use rand::{rngs::OsRng, RngCore};
use tracing::info;
use uaparser::UserAgentParser;

pub struct AppState {
    pub queue_auth_token: String,
    pub salt_store: SaltStore,
    pub ua_parser: UserAgentParser,
    pub clickhouse: clickhouse::Client,
}

pub struct SaltStore {
    current: tokio::sync::RwLock<[u8; 32]>,
}

impl SaltStore {
    pub fn new() -> Self {
        Self {
            current: tokio::sync::RwLock::new(generate_salt()),
        }
    }

    pub async fn get(&self) -> [u8; 32] {
        *self.current.read().await
    }

    pub async fn rotate(&self) {
        *self.current.write().await = generate_salt();
    }
}

pub fn generate_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);
    salt
}

pub async fn run_salt_rotation(state: Arc<AppState>) {
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
        info!("Daily salt rotated");
    }
}
