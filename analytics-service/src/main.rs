use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    tracing::info!("analytics-service listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "ok"
}
