use worker::{kv::KvStore, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RateBucket {
    count: u32,
    window_start: u64,
}

pub async fn check_rate_limit(kv: &KvStore, ip: &str, max_req: u32, window_sec: u64) -> Result<bool> {
    let key = format!("rl:{ip}");
    let now = worker::Date::now().as_millis() / 1000;

    let bucket: Option<RateBucket> = kv.get(&key).json().await?;
    let mut current = bucket.unwrap_or(RateBucket { count: 0, window_start: now });

    if now.saturating_sub(current.window_start) >= window_sec {
        current = RateBucket { count: 0, window_start: now };
    }

    current.count += 1;
    if current.count > max_req {
        return Ok(false);
    }

    kv.put(&key, serde_json::to_string(&current)?)?
        .expiration_ttl(window_sec)
        .execute()
        .await?;

    Ok(true)
}
