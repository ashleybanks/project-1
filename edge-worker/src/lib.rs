use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get_async("/p", |req, ctx| async move {
            handle_pixel(req, ctx).await
        })
        .run(req, env)
        .await
}

async fn handle_pixel(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    // TODO: validate site token via KV
    // TODO: extract headers and enqueue event
    // TODO: respond with 1x1 GIF
    Response::ok("stub")
}
