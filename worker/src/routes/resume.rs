use worker::*;
use crate::middleware::origin::add_cors_headers;

pub async fn handle_resume(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Record download event in KV
    if let Ok(kv) = ctx.env.kv("ATHENA_KV") {
        let ip = req.headers().get("CF-Connecting-IP")?.unwrap_or_else(|| "unknown".into());
        let ua = req.headers().get("User-Agent")?.unwrap_or_else(|| "unknown".into());
        let timestamp = worker::Date::now().to_string();
        let key = format!("resume_dl:{}:{}", timestamp, ip);
        let val = serde_json::json!({
            "ip": ip,
            "user_agent": ua,
            "timestamp": timestamp,
        });
        let _ = kv.put(&key, val.to_string())?.execute().await;
    }

    // Redirect to the hosted resume PDF on GitHub Pages
    let target = "https://prath19r.github.io/akashic-records/assets/resume.pdf";
    let mut resp = Response::redirect(Url::parse(target)?)?;
    add_cors_headers(&mut resp, &req)?;
    Ok(resp)
}
