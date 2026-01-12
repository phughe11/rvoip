# Project Context

## Purpose
`rvoip` is a comprehensive, 100% pure Rust implementation of a SIP/VoIP stack designed to handle, route, and manage phone calls at scale.
**Goals:**
- Provide a robust, memory-safe, and performant foundation for VoIP applications (softphones, call centers, PBX).
- Clean separation of concerns through a modular architecture.
- Strict RFC compliance for interoperability with existing SIP systems (FreeSWITCH, Asterisk, managed providers).
- Enterprise-ready reliability and scalability.

## Tech Stack
**Core Language:**
- Rust (Edition 2021)

**Build & Runtime:**
- **Build System:** Cargo (Workspace with multiple crates)
- **Async Runtime:** `tokio` (v1.x)
- **HTTP/Web:** `axum`, `tower`

**Key Libraries:**
- **Parsing:** `nom` (SIP message parsing)
- **Serialization:** `serde`, `serde_json`
- **Logging/Tracing:** `tracing`, `tracing-subscriber`, `log`
- **Crypto:** `ring`, `rustls`, `hmac`, `sha2`, `aes` (SRTP/DTLS implementation)
- **Error Handling:** `thiserror`, `anyhow`

## Project Conventions

### Code Style
- **Formatting:** Standard `rustfmt` adherence.
- **Lints:**
  - `rust`: `warn` on warnings, unused imports, unused comparisons, etc.
  - `clippy`: Strict correctness (`warn`), but lenient on style/pedantic/complexity (`allow`).
- **Safety:** Pure Rust preferred. No FFI dependencies aimed for.

### Architecture Patterns
**Modular Monorepo (Workspace):**
- **Foundation:** `sip-core` (Parsing), `sip-transport` (IO).
- **Protocol:** `transaction-core` (Reliability), `dialog-core` (State).
- **Media:** `rtp-core` (Transport), `media-core` (Processing/Codecs), `codec-core`.
- **Session:** `session-core` (High-level orchestration).
- **Application:** `client-core` (UA logic), `call-engine` (Call Center logic).

**Design Principles:**
- **Event-Driven:** Heavy use of async/await and channels/events for state updates.
- **Separation of Concerns:** Distinct crates for Signaling (SIP) vs Media (RTP) vs Logic.
- **Builder Pattern:** Extensive use of builders for configuration (e.g., `ClientConfig`, `SessionManagerBuilder`).

### Testing Strategy
- **Unit Tests:** Co-located in crates (`#[test]`).
- **Integration Tests:** Use of `sipp` for end-to-end flow validation.
- **Compliance:** Torture tests based on RFC 4475.
- **Performance:** Benchmarks using `criterion` for critical paths (parsing, media mixing).

### Git Workflow
- Standard Feature Branch -> Pull Request workflow.
- Semantic Versioning (managed via workspace version).

## Domain Context
- **SIP (RFC 3261):** Understanding of INVITE, ACK, BYE, CANCEL, REGISTER, OPTIONS methods.
- **VoIP Entities:** User Agent (UA) Client/Server, Proxy, Registrar, B2BUA (Back-to-Back User Agent).
- **Media:** SDP (Session Description Protocol), RTP (Real-time Transport), Codecs (G.711, Opus, G.722).
- **NAT Traversal:** STUN, TURN, ICE, Symmetric RTP.

## Important Constraints
- **Performance:** Critical. Low latency for media, high throughput for signaling.
- **Binary Size:** Optimization profiles exist (`release-small`, `release-fast`) to control stripping and LTO.
- **Reliability:** Panic behavior set to `abort` in release to ensure process stability/predictability (fail-fast without unwinding overhead).
- **Compatibility:** Must maintain strict RFC compliance to work with other vendors.

## External Dependencies
- **Runtime:** None (Standalone binary).
- **Dev Deps:** `sipp` (for integration testing), `cargo-nextest` (recommended for faster test runs).
