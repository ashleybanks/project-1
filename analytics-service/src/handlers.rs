use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use tracing::info;

use crate::{
    db::{PageviewRow, insert_pageview},
    models::QueuedMessage,
    processing::{generate_hash, normalise_referrer, parse_ua},
    state::AppState,
};

pub async fn health() -> &'static str {
    "ok"
}

pub async fn events(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(e): Json<QueuedMessage>,
) -> impl IntoResponse {
    let expected_token =
        HeaderValue::try_from(format!("Bearer {}", state.queue_auth_token)).unwrap();
    let supplied_token = headers
        .get("Authorization")
        .unwrap_or(&HeaderValue::from_static(""))
        .clone();

    if expected_token != supplied_token {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    for m in e.messages.iter() {
        let (browser_family, os_family, device_type) =
            parse_ua(&state.ua_parser, &m.body.user_agent);
        let visitor_hash = generate_hash(state.salt_store.get().await, &m.body);
        let referrer_domain = normalise_referrer(m.body.referrer.as_deref());

        let row = PageviewRow {
            site_id: m.body.site_id.clone(),
            timestamp: m.body.timestamp.parse().unwrap_or(0),
            page_url: m.body.request_path.clone(),
            referrer_domain,
            country: m.body.country.clone(),
            browser_family,
            os_family,
            device_type: device_type.to_string(),
            visitor_hash,
            asn: m.body.asn.clone()
        };

        match insert_pageview(&state.clickhouse, row).await {
            Ok(_) => info!("event {} written to clickhouse", m.id),
            Err(e) => tracing::error!("failed to write event {}: {}", m.id, e),
        }
    }

    StatusCode::OK.into_response()
}
