use worker::*;
use serde::{Deserialize, Serialize};
use crate::knowledge;
use crate::middleware::{sanitize::sanitize_user_input, origin::add_cors_headers};

const NVIDIA_URL: &str = "https://integrate.api.nvidia.com/v1/chat/completions";
const MODEL_NAME: &str = "nvidia/nemotron-3.5-lightning-30b-a3b";

#[derive(Deserialize)]
pub struct IncomingChatRequest {
    pub messages: Vec<ClientMessage>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ClientMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
struct NvidiaPayload {
    model: String,
    messages: Vec<ClientMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct NvidiaResponse {
    choices: Vec<NvidiaChoice>,
}

#[derive(Deserialize)]
struct NvidiaChoice {
    message: ClientMessage,
}

#[derive(Serialize)]
pub struct ChatReply {
    pub reply: String,
}

pub async fn handle_chat(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let api_key = match ctx.secret("NVIDIA_API_KEY") {
        Ok(k) => k.to_string(),
        Err(_) => {
            let mut err_resp = Response::error("API key secret is not configured on Worker", 500)?;
            add_cors_headers(&mut err_resp, &req)?;
            return Ok(err_resp);
        }
    };

    let body: IncomingChatRequest = match req.json().await {
        Ok(b) => b,
        Err(_) => {
            let mut err_resp = Response::error("Invalid JSON body", 400)?;
            add_cors_headers(&mut err_resp, &req)?;
            return Ok(err_resp);
        }
    };

    let mut messages: Vec<ClientMessage> = Vec::new();
    messages.push(ClientMessage {
        role: "system".into(),
        content: knowledge::system_prompt::get_system_prompt(),
    });

    for m in body.messages.into_iter().take(12) {
        if m.role == "user" || m.role == "assistant" {
            messages.push(ClientMessage {
                role: m.role,
                content: sanitize_user_input(&m.content),
            });
        }
    }

    let payload = NvidiaPayload {
        model: MODEL_NAME.into(),
        messages,
        temperature: 0.7,
        max_tokens: 1024,
    };

    let headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    headers.set("Authorization", &format!("Bearer {api_key}"))?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(headers)
        .with_body(Some(serde_json::to_string(&payload)?.into()));

    let nvidia_req = Request::new_with_init(NVIDIA_URL, &init)?;
    let mut nvidia_resp = match Fetch::Request(nvidia_req).send().await {
        Ok(res) => res,
        Err(e) => {
            let mut err_resp = Response::error(format!("NVIDIA NIM fetch error: {e}"), 502)?;
            add_cors_headers(&mut err_resp, &req)?;
            return Ok(err_resp);
        }
    };

    if nvidia_resp.status_code() != 200 {
        let text = nvidia_resp.text().await.unwrap_or_default();
        let mut err_resp = Response::error(format!("NVIDIA API upstream error: {text}"), 502)?;
        add_cors_headers(&mut err_resp, &req)?;
        return Ok(err_resp);
    }

    let nvidia_data: NvidiaResponse = nvidia_resp.json().await?;
    let reply = nvidia_data
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .unwrap_or_else(|| "Athena is momentarily busy. Please ask again.".into());

    let mut resp = Response::from_json(&ChatReply { reply })?;
    add_cors_headers(&mut resp, &req)?;
    Ok(resp)
}
