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
    chat_template_kwargs: ChatTemplateKwargs,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize)]
struct ChatTemplateKwargs {
    enable_thinking: bool,
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
        chat_template_kwargs: ChatTemplateKwargs {
            enable_thinking: false,
        },
        temperature: 0.5,
        max_tokens: 1024,
    };

    let payload_json = match serde_json::to_string(&payload) {
        Ok(j) => j,
        Err(_) => {
            let mut resp = Response::from_json(&ChatReply {
                reply: "Athena could not format the payload. Please try again.".into(),
            })?;
            add_cors_headers(&mut resp, &req)?;
            return Ok(resp);
        }
    };

    let mut nvidia_text = String::new();
    let mut success = false;

    for _ in 0..2 {
        let headers = Headers::new();
        let _ = headers.set("Content-Type", "application/json");
        let _ = headers.set("Authorization", &format!("Bearer {api_key}"));

        let mut init = RequestInit::new();
        init.with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(payload_json.clone().into()));

        if let Ok(nvidia_req) = Request::new_with_init(NVIDIA_URL, &init) {
            if let Ok(mut resp) = Fetch::Request(nvidia_req).send().await {
                if resp.status_code() == 200 {
                    if let Ok(txt) = resp.text().await {
                        nvidia_text = txt;
                        success = true;
                        break;
                    }
                }
            }
        }
    }

    let raw_reply = if success {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&nvidia_text) {
            val.get("choices")
                .and_then(|c| c.as_array())
                .and_then(|arr| arr.first())
                .and_then(|choice| choice.get("message"))
                .and_then(|msg| msg.get("content"))
                .and_then(|c| c.as_str())
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.to_string())
        } else {
            None
        }
    } else {
        None
    };

    // Clean up any stray thinking or analysis preamble and awkward delimiter artifacts
    let clean_reply = raw_reply.map(|text| {
        let trimmed = text.trim();
        let content = if let Some(end) = trimmed.find("</think>") {
            trimmed[end + 8..].trim()
        } else if let Some(formulate) = trimmed.find("**Formulate Response:**") {
            trimmed[formulate + 23..].trim()
        } else {
            trimmed
        };

        content
            .replace(":** - **", ": **")
            .replace(":** - ", ": ")
            .replace("** - **", " — ")
            .replace("-  **", "- **")
            .to_string()
    });

    let final_reply = clean_reply.unwrap_or_else(|| {
        "Athena is momentarily catching her breath due to high traffic. Please try asking your question again in just a moment.".into()
    });

    let mut resp = Response::from_json(&ChatReply { reply: final_reply })?;
    add_cors_headers(&mut resp, &req)?;
    Ok(resp)
}
