# 🏛️ Athena — Autonomous AI Portfolio Guide

> Conversational AI guide and systems portfolio companion for **Prathmesh Ranadive**.  
> **100% Pure Rust Architecture**: Leptos 0.7+ CSR frontend + Cloudflare Workers edge gateway in `workers-rs`.

- **Live Standalone:** [prath19r.github.io/athena](https://prath19r.github.io/athena/)
- **Embedded Mode:** [prath19r.github.io/athena/?embed=true](https://prath19r.github.io/athena/?embed=true)

---

## 🔐 Zero-Secret Architecture

1. **Frontend (Rust / Leptos WASM):** Pure presentation layer. Contains zero API keys, zero encryption secrets, and zero system prompts.
2. **Edge Proxy (`worker/` in Rust / `workers-rs`):** Runs on Cloudflare's edge V8 isolates. 
   - Origin verification (`prath19r.github.io` whitelist)
   - Cloudflare KV-backed rate limiter (10 req/min/IP)
   - Prompt-injection filter & sanitization
   - Injects server-side `NVIDIA_API_KEY` to query `nvidia/nemotron-3.5-lightning-30b-a3b`

---

## 🛠️ Project Structure

```
athena/
├── worker/                       # Cloudflare Worker (Pure Rust cdylib)
│   ├── src/
│   │   ├── lib.rs                # Router & fetch event handler
│   │   ├── routes/               # /api/chat, /api/contact, /api/resume
│   │   ├── middleware/           # Rate limit, origin guard, sanitizer
│   │   └── knowledge/            # Resume context, project specs, system prompt
│   ├── Cargo.toml                # worker = "0.5"
│   └── wrangler.toml             # worker-build config
├── src/                          # Frontend (Leptos Rust CSR)
│   ├── components/               # ChatWindow, MessageBubble, InputBar, EmbedLayout
│   ├── services/                 # API client (gloo-net)
│   ├── app.rs                    # Root component with embed detection
│   └── main.rs
├── assets/                       # Static files (resume.pdf)
├── Trunk.toml                    # Trunk config
└── Cargo.toml                    # Leptos CSR dependencies
```

---

## 🚀 Local Development

### 1. Run Cloudflare Worker Locally
```bash
cd worker
npx wrangler dev --port 8787
```

### 2. Run Leptos Frontend
```bash
npm install
trunk serve --port 8081
```
Open [http://localhost:8081](http://localhost:8081) in your browser.
