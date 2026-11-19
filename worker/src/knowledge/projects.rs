pub fn get_projects_context() -> &'static str {
    r#"
================================================================================
COMPANY & ENTERPRISE PROJECTS (HIGHLIGHTED — PRODUCTION ENTERPRISE PLATFORMS)
================================================================================

1. Enterprise ACDHub Gateway & Envoy WASM Plugins (Company: NICE CXone · Enterprise R&D)
   - Category: Company Enterprise Project
   - Stack: Rust, WebAssembly (WASM), Envoy Proxy, AWS MSK (Kafka), Redis Streams, Go, Java, AWS CDK
   - Scale & Impact: Greenfield real-time contact-center data ingestion platform. Reduced real-time streaming latency by 50%+ (p99 < 99 ms) and historical latency by 67% (30s to 10s). Developed custom Rust/WASM Envoy plugins for Amazon Connect agent-state payloads and Salesforce TTI event ingestion, mapping streams to Universal Event Schema v1.1 and publishing to Redis Streams and AWS MSK (Kafka).

2. WFM Forecaster — Platform Modernization & Performance (Company: NICE CXone)
   - Category: Company Enterprise Project
   - Stack: Java, Spring Boot 3, Hibernate, AWS Batch, ECS, RDS, CloudWatch, Gatling, Docker
   - Scale & Impact: Enterprise platform spanning 15+ microservices for contact-center staffing forecasts. Cut production batch cleanup job runtime from 2 hours to under 30 minutes (-75%), resolved calculation anomalies, built Gatling PSR load simulations, and led Black Duck & Veracode security compliance across 2 enterprise releases.

3. Yash Training Management System (YTMS) (Company: YASH Technologies)
   - Category: Company Enterprise Project
   - Stack: Java 8, Spring Boot, Angular, TypeScript, MySQL, Spring Security, Gemini Generative AI, React
   - Scale & Impact: Internal enterprise platform managing employee training tracks. Designed the dynamic calendar scheduling engine, RBAC authentication, and integrated Gemini Generative AI in Angular to assist trainees with course comprehension.

4. Trainee Task Management Platform (Company: YASH Technologies)
   - Category: Company Enterprise Project
   - Stack: Java 8, Servlets, JSP, MySQL, HTML, CSS, JavaScript
   - Scale & Impact: Enterprise workflow platform for sprint task assignment, lifecycle tracking, state-machine transitions, and RBAC authorization.

================================================================================
PERSONAL & OPEN-SOURCE SYSTEMS PROJECTS
================================================================================

5. Athena — Autonomous AI Portfolio Guide & Edge Gateway
   - Category: Personal Project
   - Stack: 100% Rust (Leptos 0.7+ CSR WASM frontend + Cloudflare Workers `workers-rs` proxy)
   - Highlights: Zero-client-secret architecture. Edge proxy handles origin validation, Cloudflare KV token-bucket rate limiting (10 req/min/IP), and prompt-injection sanitization. Connects to NVIDIA NIM (Nemotron 3.5 30B reasoning). Embedded as an interactive thought bubble companion in Akashic Records.

6. Akashic Records — Personal Systems Engineering Portfolio
   - Category: Personal Project
   - Stack: Rust, Leptos 0.7+ (CSR mode), WebAssembly, Trunk, Tailwind CSS
   - Highlights: Minimalist personal engineering portfolio with zero Node runtime dependencies in production. Features an interactive pure Rust state-machine chibi companion with real-time thought bubble chat.
"#
}
