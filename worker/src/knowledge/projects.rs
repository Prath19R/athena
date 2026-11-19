pub fn get_projects_context() -> &'static str {
    r#"
FEATURED PROJECTS:

1. Athena — AI Portfolio Guide & Edge Architecture
   - Stack: 100% Rust (Leptos 0.7+ CSR WASM frontend + Cloudflare Workers `workers-rs` proxy)
   - LLM: NVIDIA NIM API (nvidia/nemotron-3.5-lightning-30b-a3b)
   - Highlights: Zero-client-secret architecture. Edge proxy handles origin validation, Cloudflare KV token-bucket rate limiting (10 req/min/IP), and input prompt-injection sanitization. Operates both as a standalone web application and an embedded responsive iframe widget in the main Akashic Records portfolio.

2. Autonomous Quantitative Trading Agent
   - Stack: Python, C (Native Shared Library Fast Engine), NVIDIA NIM (Nemotron 30B Reasoning), Docker
   - Market: Indian Equities (NSE NIFTY 100)
   - Highlights: Features a high-speed C shared engine for multi-factor technical screening (Momentum, Volatility, Volume, Mean-Reversion), dynamic rotating AI batching, opportunity-cost capital rotation that protects profitable winners while rotating stagnant capital, and rigorous risk guards (drawdown ceilings, stop losses, trailing stops, crash detectors).

3. Enterprise ACDHub Gateway & Envoy WASM Plugins (NICE CXone R&D)
   - Stack: Rust, WebAssembly (WASM), Envoy Proxy, AWS MSK (Kafka), Redis Streams, Go, Java
   - Highlights: Low-latency real-time data streaming (<99 ms p99) processing Amazon Connect and Cisco UCCE contact-center event streams, custom Envoy WASM filter plugins written in Rust, and AWS CDK infrastructure.

4. Akashic Records — Personal Systems Engineering Portfolio
   - Stack: Rust, Leptos 0.7+ (CSR mode), WebAssembly, Trunk, Tailwind CSS
   - Highlights: Dark/Zinc minimalistic aesthetic, ultra-low bundle footprint, full client-side reactivity without Node runtime dependencies in production, continuous deployment to GitHub Pages via GitHub Actions.
"#
}
