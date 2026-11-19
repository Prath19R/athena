use worker::*;

pub fn is_allowed_origin(req: &Request) -> bool {
    let origin = req.headers().get("Origin").unwrap_or_default().unwrap_or_default();
    if origin.is_empty() {
        return true; // allow direct browser navigations or tools without Origin header
    }
    matches!(
        origin.as_str(),
        "https://prath19r.github.io"
            | "http://localhost:8080"
            | "http://localhost:8081"
            | "http://127.0.0.1:8080"
            | "http://127.0.0.1:8081"
    )
}

pub fn cors_preflight_response(req: &Request) -> Result<Response> {
    let origin = req.headers().get("Origin")?.unwrap_or_else(|| "*".into());
    let mut resp = Response::empty()?;
    let h = resp.headers_mut();
    h.set("Access-Control-Allow-Origin", &origin)?;
    h.set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
    h.set("Access-Control-Allow-Headers", "Content-Type, Authorization")?;
    h.set("Access-Control-Max-Age", "86400")?;
    Ok(resp)
}

pub fn add_cors_headers(resp: &mut Response, req: &Request) -> Result<()> {
    let origin = req.headers().get("Origin")?.unwrap_or_else(|| "*".into());
    let h = resp.headers_mut();
    h.set("Access-Control-Allow-Origin", &origin)?;
    h.set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
    h.set("Access-Control-Allow-Headers", "Content-Type, Authorization")?;
    Ok(())
}
