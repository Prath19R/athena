use worker::*;
use serde::{Deserialize, Serialize};
use crate::middleware::origin::add_cors_headers;

#[derive(Deserialize)]
pub struct ContactFormRequest {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct ContactResponse {
    pub success: bool,
    pub message: String,
}

pub async fn handle_contact(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let form: ContactFormRequest = match req.json().await {
        Ok(f) => f,
        Err(_) => {
            let mut err_resp = Response::error("Invalid contact form JSON", 400)?;
            add_cors_headers(&mut err_resp, &req)?;
            return Ok(err_resp);
        }
    };

    if form.email.trim().is_empty() || form.message.trim().is_empty() {
        let mut err_resp = Response::error("Email and message cannot be empty", 400)?;
        add_cors_headers(&mut err_resp, &req)?;
        return Ok(err_resp);
    }

    // Save contact submission in KV store if KV binding exists
    if let Ok(kv) = ctx.env.kv("ATHENA_KV") {
        let timestamp = worker::Date::now().to_string();
        let key = format!("contact:{}:{}", timestamp, form.email.replace('@', "_at_"));
        let val = serde_json::json!({
            "name": form.name,
            "email": form.email,
            "message": form.message,
            "submitted_at": timestamp,
        });

        let _ = kv.put(&key, val.to_string())?.execute().await;
    }

    let mut resp = Response::from_json(&ContactResponse {
        success: true,
        message: "Thank you for reaching out! Your message has been received.".into(),
    })?;
    add_cors_headers(&mut resp, &req)?;
    Ok(resp)
}
