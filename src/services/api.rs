use gloo_net::http::Request;
use gloo_timers::future::TimeoutFuture;
use serde::{Deserialize, Serialize};

#[cfg(debug_assertions)]
const WORKER_URL: &str = "http://localhost:8787";

#[cfg(not(debug_assertions))]
const WORKER_URL: &str = "https://athena.prathmeshranadive19.workers.dev";

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

    let max_attempts = 3;

    for attempt in 1..=max_attempts {
        let req = match Request::post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
        {
            Ok(r) => r,
            Err(_) => return Ok("Athena encountered a formatting issue. Please try typing your question again.".into()),
        };

        match req.send().await {
            Ok(resp) => {
                if resp.ok() {
                    if let Ok(parsed) = resp.json::<ChatResponse>().await {
                        if !parsed.reply.trim().is_empty() {
                            return Ok(parsed.reply);
                        }
                    }
                }
            }
            Err(_) => {}
        }

        if attempt < max_attempts {
            TimeoutFuture::new(attempt * 1000).await;
        }
    }

    Ok("Athena is momentarily catching her breath due to high traffic. Please ask your question again in just a moment, or reach out to Prathmesh directly at prathmeshranadive@gmail.com or +91 8319057558.".into())
}
