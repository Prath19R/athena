pub fn get_resume_context() -> &'static str {
    r#"
PRATHMESH RANADIVE
Software Engineer | Backend & Distributed Systems — Java · Rust · Go · Envoy · AWS · Kafka
Pune, India · +91-8319057558 · prathmeshranadive@gmail.com · linkedin.com/in/prathmesh-ranadive

SUMMARY:
Software Engineer with 3 years in enterprise R&D at NICE CXone on a greenfield contact-center data gateway (ACDHub) and legacy WFM modernization. Cut real-time data latency by 50%+ (p99 under 99 ms), built Rust/WASM Envoy plugins for Amazon Connect real-time and Salesforce TTI event ingestion, provisioned Amazon MSK (Kafka) via CDK, and owned epics spanning Redis cluster migration, observability, and security compliance across 15+ microservices.

TECHNICAL SKILLS:
- Languages: Java, Rust, Go, Python, SQL, TypeScript/JavaScript
- Backend & Platform: Spring Boot 3, Hibernate, Envoy, WASM plugins, Kafka, Angular, Nginx, HTTP/2, TLS, JSON Schema
- Cloud & Infrastructure: AWS (ECS, MSK, RDS, Lambda, CloudWatch, X-Ray, Batch, CDK), Docker, Redis (Cluster), Valkey, Git
- Testing & Quality: Playwright, Cucumber, Gatling, Xray, ReportPortal, JUnit, SonarQube, Veracode, Black Duck
- AI-Assisted Development: GitHub Copilot, Claude Code, MCP

CAREER CHRONOLOGY & WORK EXPERIENCE:
[CRITICAL ORDER: Began career at YASH Technologies in 2023. Joined NICE CXone in Jul 2024 and STILL ACTIVELY WORKS at NICE CXone today!]

1. CURRENT POSITION (Jul 2024 – Present):
   Associate Software Engineer — R&D
   Company: NICE (CXone) — ACDHub Gateway Platform & WFM Forecaster (Pune, India · Hybrid)
   [EMPLOYMENT STATUS: CURRENTLY EMPLOYED HERE — STILL WORKING AT NICE CXone]
   • Reduced real-time data latency from 200 ms to under 99 ms at p99 and historical data latency from 30 s to 10 s (–67%) on ACDHub, a greenfield gateway ingesting contact-center data from Cisco UCCE and Amazon Connect.
   • Built the Rust/WASM Envoy Normalizer plugin for real-time Amazon Connect agent-state events — validates and maps payloads to the ACDHub model and writes to Redis Streams, verified end-to-end through the Web Adapter; also built the Discovery Service with in-memory config caching and 30-minute refresh.
   • Built Salesforce TTI event ingestion end-to-end: JWT-secured Web Adapter endpoint with per-kind JSON Schema validation, Rust/WASM plugin mapping events to the Universal Event Schema v1.1 (deterministic UUIDv5 IDs), and at-least-once Kafka publish (acks=all); authored a 48-case Xray test plan.
   • Delivered Kafka (Amazon MSK) provisioning across 3 repos: CDK stack for cluster and dynamic topics, SSM Parameter Store topic config, and topic-lifecycle endpoints; validated IAM producer/consumer policies on AWS.
   • Owned the Cisco UCCE historical integration: shipped a polling-adapter AWS Lambda with Cisco file parsing and Redis-based deduplication, plus IDP Lambda and SFTP-server observability — adapter delivered end-to-end in a single sprint.
   • Drove the Redis Cluster migration epic (sharding & high availability) including CDK changes for Valkey cluster support, and implemented auto-scaling policies for ASM, Egress Manager, and RTA Router services.
   • Led security compliance (Black Duck & Veracode) across 15+ microservices and libraries of the WFM Forecaster platform over two release cycles; contributed to the Spring Boot 3 + Hibernate migration.
   • Built performance engineering (PSR) tooling: Gatling simulations for historical reports and agent-state load, data-generation pipelines, automated secrets provisioning, and real-time/historical traffic simulators.
   • Strengthened test automation: Go and Java integration suites with HTTP/2 + TLS, ReportPortal integration, and E2E automation per ACD type; created AWS X-Ray dashboards and CloudWatch alarms (new repo + CI pipeline).
   • Resolved production bugs in forecasting and staffing workflows and optimized a production cleanup job from 2 hours to under 30 minutes; debugged incidents across CloudWatch, ECS, RDS, AWS Batch, and Nginx.

2. PREVIOUS POSITION (Dec 2023 – Jul 2024):
   Trainee Programmer
   Company: YASH Technologies (Indore, India)
   • Developed the internal Yash Training Management System — authentication, task tracking, calendar scheduling, and responsive UI — using Java, Spring Boot, Angular, and React/Redux, following intensive full-stack training (Core Java, Java 8, JUnit, Servlets/JSP).

3. EARLIEST POSITION / INTERNSHIP (Sep 2023 – Dec 2023):
   Software Development Intern
   Company: YASH Technologies (Indore, India)
   • Built task-management features (user authentication, task creation/assignment, status tracking) and contributed to Book Store and Expense Management applications.

EDUCATION:
B.Tech, Computer Science — Medi-Caps University (Aug 2019 – Jun 2023)
Grade: 88% (Indore, India)
"#
}
