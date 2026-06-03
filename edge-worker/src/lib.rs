use serde::Serialize;
use worker::*;

#[derive(Serialize)]
struct Payload {
    site_id: String,
    timestamp: String,
    ip: String,
    user_agent: String,
    country: String,
    asn: String,
    referrer: Option<String>,
    request_path: Option<String>,
}

const TRANSPARENT_GIF: &[u8] = &[
    0x47, 0x49, 0x46, 0x38, 0x39, 0x61, 0x01, 0x00, 0x01, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xff, 0xff, 0xff, 0x21, 0xf9, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x01, 0x00, 0x00, 0x02, 0x02, 0x4c, 0x01, 0x00, 0x3b,
];

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    Router::with_data(ctx)
        .get_async(
            "/p",
            |req, ctx| async move { handle_pixel(req, ctx).await },
        )
        .run(req, env)
        .await
}

async fn handle_pixel(req: Request, ctx: RouteContext<Context>) -> Result<Response> {
    let request_headers = req.headers();
    let cf = req.cf();
    let url = req.url()?;
    let query: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();

    // Validate site token via KV
    if let Some(site_id) = query.get("s") {
        let kv = ctx.env.kv("SITE_TOKENS")?;
        let queue = ctx.env.queue("EVENT_QUEUE")?;

        if kv.get(site_id).text().await?.is_some() {
            // Extract headers and enqueue event
            let payload = Payload {
                site_id: site_id.clone(),
                timestamp: Date::now().as_millis().to_string(),
                ip: request_headers
                .get("CF-Connecting-IP")?
                .unwrap_or_else(|| "127.0.0.1".to_string()),
                user_agent: request_headers.get("User-Agent")?.unwrap_or_default(),
                country: cf.as_ref().and_then(|cf| cf.country()).unwrap_or_else(|| "unknown".to_string()),
                asn: cf.as_ref().and_then(|cf| cf.asn()).map(|n| n.to_string()).unwrap_or_default(),
                referrer: request_headers.get("Referer")?,
                request_path: query.get("u").map(|u| u.to_string())
            };
            ctx.data.wait_until(async move {
                let _ = queue.send(&payload).await;
            });
        }
    }

    // Respond with 1x1 GIF
    let response_headers = Headers::new();
    response_headers.set("Content-Type", "image/gif")?;
    response_headers.set("Cache-Control", "no-store, no-cache")?;
    response_headers.set("Pragma", "no-cache")?;
    if request_headers.get("Origin")?.is_some() {
        response_headers.set("Access-Control-Allow-Origin", "*")?;
    }
    Ok(Response::from_bytes(TRANSPARENT_GIF.to_vec())?.with_headers(response_headers))
}
