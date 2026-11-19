use worker::*;

mod routes;
mod middleware;
mod knowledge;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // 1. Handle CORS preflight
    if req.method() == Method::Options {
        return middleware::origin::cors_preflight_response(&req);
    }

    // 2. Validate Origin
    if !middleware::origin::is_allowed_origin(&req) {
        let mut err_resp = Response::error("Forbidden: Unauthorized origin", 403)?;
        let _ = middleware::origin::add_cors_headers(&mut err_resp, &req);
        return Ok(err_resp);
    }

    // 3. Rate limiting (10 requests per minute per IP via Cloudflare KV)
    let ip = req.headers().get("CF-Connecting-IP")?.unwrap_or_else(|| "127.0.0.1".into());
    if let Ok(kv) = env.kv("ATHENA_KV") {
        if let Ok(allowed) = middleware::rate_limit::check_rate_limit(&kv, &ip, 10, 60).await {
            if !allowed {
                let mut err_resp = Response::error("Rate limit exceeded. Please wait a minute.", 429)?;
                let _ = middleware::origin::add_cors_headers(&mut err_resp, &req);
                return Ok(err_resp);
            }
        }
    }

    // 4. Router dispatch
    let router = Router::new();
    router
        .post_async("/api/chat", |req, ctx| routes::chat::handle_chat(req, ctx))
        .post_async("/api/contact", |req, ctx| routes::contact::handle_contact(req, ctx))
        .get_async("/api/resume", |req, ctx| routes::resume::handle_resume(req, ctx))
        .run(req, env)
        .await
}
