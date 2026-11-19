use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[cfg(debug_assertions)]
const WORKER_URL: &str = "http://localhost:8787";

#[cfg(not(debug_assertions))]
const WORKER_URL: &str = "https://athena-proxy.prathmeshranadive.workers.dev";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    messages: Vec<ChatMessage>,
}

#[derive(Deserialize)]
struct ChatResponse {
    reply: String,
}

pub async fn send_chat_message(messages: Vec<ChatMessage>) -> Result<String, String> {
    let body = ChatRequest { messages };

    let url = format!("{WORKER_URL}/api/chat");
    let resp = Request::post(&url)
        .header("Content-Type", "application/json")
        .json(&body)
        .map_err(|e| format!("Failed to serialize request: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Network request failed: {e}"))?;

    if !resp.ok() {
        return Err(format!("Worker error status: {}", resp.status()));
    }

    let parsed: ChatResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))?;

    Ok(parsed.reply)
}
