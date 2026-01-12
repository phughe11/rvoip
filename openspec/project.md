# Project Context

## Purpose
`rvoip` is a comprehensive, 100% pure Rust implementation of a SIP/VoIP stack designed to handle, route, and manage phone calls at scale.

**Goals:**
- Provide a robust, memory-safe, and performant foundation for VoIP applications (softphones, call centers, PBX).
- Clean separation of concerns through a modular architecture.
- Strict RFC compliance for interoperability with existing SIP systems (FreeSWITCH, Asterisk, managed providers).
- Enterprise-ready reliability and scalability.
- Alternative to open-source systems like FreeSWITCH/Asterisk and commercial systems like Avaya/Cisco.

**Current Version:** 0.1.26 (Alpha)

## Tech Stack

**Core Language:**
- Rust (Edition 2021, MSRV 1.70+)

**Build & Runtime:**
- **Build System:** Cargo (Workspace with 20+ crates)
- **Async Runtime:** `tokio` v1.36+ (full features)
- **HTTP/Web:** `axum` v0.7, `tower` v0.4

**Key Libraries:**
- **Parsing:** `nom` v7.1 (SIP message parsing)
- **Serialization:** `serde` v1.0, `serde_json` v1.0
- **Logging/Tracing:** `tracing` v0.1, `tracing-subscriber` v0.3, `log` v0.4
- **Crypto:** `ring` v0.17, `rustls` v0.23, `hmac` v0.12, `sha2` v0.10, `aes` v0.8 (SRTP/DTLS)
- **Error Handling:** `thiserror` v1.0, `anyhow` v1.0
- **Concurrency:** `dashmap` v5.5, `parking_lot` v0.12
- **Data:** `bytes` v1.5, `uuid` v1.7, `chrono` v0.4
- **Testing:** `proptest` v1.4, `criterion` v0.5, `serial_test` v3.1

## Workspace Architecture

### Foundation Layer (No Internal Dependencies)
- **infra-common**: Event system, configuration, infrastructure utilities
- **sip-core**: SIP message parsing, headers, URIs (RFC 3261)
- **codec-core**: Audio codecs (G.711, G.722, Opus, G.729)
- **users-core**: User management and authentication

### Transport Layer
- **rtp-core**: RTP/RTCP transport, SRTP/SRTCP encryption (depends on infra-common)
- **sip-transport**: Multi-protocol SIP transport (UDP/TCP/TLS/WS) (depends on sip-core, infra-common)

### Media Processing Layer
- **media-core**: Audio processing, codec transcoding, mixing (depends on rtp-core, codec-core)
- **audio-core**: Low-level audio utilities

### Dialog & Transaction Layer
- **dialog-core**: SIP dialog state machine, transactions (depends on sip-core, sip-transport)
- **registrar-core**: SIP registration service (depends on sip-core, infra-common)

### Session Management Layer
- **session-core**: Legacy session management (v1, maintenance mode)
- **session-core-v2**: Transitional, being phased out
- **session-core-v3**: **Recommended** - SimplePeer API, state-table driven (depends on dialog-core, media-core)

### Server Components
- **b2bua-core**: Back-to-back user agent (depends on dialog-core, NOT session-core)
- **proxy-core**: SIP proxy functionality (depends on dialog-core)
- **media-server-core**: Conference mixing, IVR, recording (depends on media-core)
- **sbc-core**: Session border controller, topology hiding (depends on b2bua-core, proxy-core)

### Application Layer
- **client-core**: High-level SIP client library (depends on session-core)
- **sip-client**: Simplified SIP client API
- **call-engine**: Call center orchestration with database (depends on dialog-core, b2bua-core)
- **rvoip**: Unified re-export crate for consumers

## Project Conventions

### Code Style
- **Formatting:** Standard `rustfmt` adherence.
- **Lints (workspace-level):**
  - `rust`: `warn` on warnings, unused_imports, unused_comparisons, deprecated
  - `rust`: `allow` on dead_code, unused_variables (during active development)
  - `clippy`: `warn` on correctness, suspicious; `allow` on style/pedantic/complexity
- **Safety:** Pure Rust only. Zero FFI dependencies.

### Architecture Patterns

**Design Principles:**
- **Event-Driven:** Heavy use of async/await and channels for state updates
- **Separation of Concerns:** Signaling (SIP) vs Media (RTP) vs Logic are in distinct crates
- **Builder Pattern:** Extensive use of builders (e.g., `ClientConfig`, `SessionManagerBuilder`)
- **State Machines:** Dialog and session states follow RFC state machines

**Key Architectural Rules:**
- **session-core-v3** is for SIP endpoints (clients, softphones) only
- **b2bua-core** builds directly on dialog-core, NOT session-core (server pattern)
- **media-server-core** runs as separate process, controlled via API
- **infra-common** provides shared event bus used by all libraries
- **media-core** is pure processing with no network I/O

### Session-Core Version Strategy
- **v1 (session-core)**: Legacy, security fixes only. Deprecation: June 2026.
- **v2 (session-core-v2)**: Transitional. Deprecation: March 2026.
- **v3 (session-core-v3)**: **Recommended for all new work.** SimplePeer API.
- At 1.0 release, v3 becomes the sole `session-core`.

### Testing Strategy
- **Unit Tests:** Co-located in crates (`#[cfg(test)]`), target 80%+ coverage
- **Integration Tests:** Use `sipp` for SIP flow validation, test harnesses
- **E2E Tests:** Full call flows between test endpoints
- **Compliance:** RFC 4475 torture tests for parser robustness
- **Performance:** `criterion` benchmarks for parsing, media mixing, critical paths
- **Property Testing:** `proptest` for parser roundtrip invariants
- **Current Coverage:** ~55% overall (target 85% by v1.0)

### Git Workflow
- Feature Branch -> Pull Request workflow
- Semantic Versioning (workspace-managed: `workspace.package.version`)
- Commit co-authorship: `Co-Authored-By: Warp <agent@warp.dev>` for AI-assisted commits
- OpenSpec workflow for major changes (see `openspec/AGENTS.md`)

## Domain Context

### SIP Protocol (RFC 3261)
- **Methods:** INVITE, ACK, BYE, CANCEL, REGISTER, OPTIONS, SUBSCRIBE, NOTIFY, MESSAGE, UPDATE, INFO, PRACK, REFER, PUBLISH
- **Authentication:** Digest (MD5, SHA-256, SHA-512-256), QoP (auth, auth-int)
- **Session Timers:** RFC 4028 keep-alive
- **Reliable Provisionals:** RFC 3262 PRACK

### VoIP Entities
- **User Agent (UA):** Client (UAC) and Server (UAS) roles
- **Proxy:** Stateless/stateful request routing
- **Registrar:** Location service for user bindings
- **B2BUA:** Back-to-back user agent for call centers, IVR
- **SBC:** Session border controller for topology hiding, security

### Media
- **SDP:** Session Description Protocol (RFC 8866), WebRTC extensions
- **RTP/RTCP:** Real-time transport (RFC 3550), quality feedback (RFC 4585)
- **Codecs:** G.711 (PCMU/PCMA), G.722, Opus, G.729
- **Security:** SRTP/SRTCP (RFC 3711), DTLS-SRTP (RFC 5763), ZRTP (RFC 6189), SDES (RFC 4568)
- **Audio Processing:** AEC (echo cancellation), AGC (gain control), VAD (voice activity), noise suppression

### NAT Traversal
- **STUN:** RFC 5389 binding discovery (complete)
- **TURN:** RFC 5766 relay (partial)
- **ICE:** RFC 8445 connectivity establishment (partial)
- **Symmetric RTP:** RFC 4961 (complete)

## Important Constraints

- **Performance:** Critical. <10ms latency for media, high throughput for signaling.
- **Binary Size:** Profiles: `release` (opt-level=s), `release-small` (opt-level=z), `release-fast` (opt-level=3)
- **Reliability:** `panic = "abort"` in release for fail-fast behavior.
- **Compatibility:** Strict RFC compliance for interop with FreeSWITCH, Asterisk, commercial systems.
- **Pure Rust:** No FFI, no C dependencies.

## External Dependencies

- **Runtime:** None (standalone binaries)
- **Dev Dependencies:**
  - `sipp` - SIP traffic generator for integration testing
  - `cargo-nextest` - Faster test runner (recommended)
  - `cargo-tarpaulin` or `cargo-llvm-cov` - Coverage reporting
  - `cargo-fuzz` - Fuzz testing

## Key Commands

```bash
# Build
cargo build --all                    # Full workspace
cargo build                          # Default members only

# Test
cargo test --all                     # All tests
cargo test -p rvoip-sip-core         # Single crate
./scripts/test_all.sh                # Curated test sweep

# Lint & Format
cargo clippy --all
cargo fmt --all

# Documentation
cargo doc --no-deps --open

# Examples
cargo run --example peer_caller
cargo run --example call_center
RUST_LOG=debug cargo run --example peer_caller  # With logging
```

## Key Documentation

- `README.md` - Project overview and quick start
- `LIBRARY_DESIGN_ARCHITECTURE.md` - Crate dependency graph and layering
- `VERSION_STRATEGY.md` - Session-core version guidance
- `TESTING_STRATEGY.md` - Test pyramid and coverage targets
- `PROJECT_HEALTH.md` - Current status dashboard
- `WARP.md` - Agent/IDE guidance
- `openspec/AGENTS.md` - OpenSpec workflow for changes
