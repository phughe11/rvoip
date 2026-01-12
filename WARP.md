# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Core commands

All commands are intended to run from the repository root unless noted otherwise.

### Build
- Full workspace build (recommended while developing):
  - `cargo build --all`
- Faster default-member build (protocol/media/core crates only):
  - `cargo build`

### Tests
- Run all tests across the workspace:
  - `cargo test --all`
- Run tests for a single crate (example: SIP core):
  - `cargo test -p rvoip-sip-core`
- Run a specific test within a crate:
  - `cargo test -p rvoip-sip-core name_of_test`  
    or
  - `cargo test -p rvoip-sip-core module_path::name_of_test`
- Run only doctests for a crate:
  - `cargo test -p rvoip-sip-core --doc`
- Full curated test sweep with per-crate summaries (unit, integration, docs where present):
  - `./scripts/test_all.sh`

For testing strategy, coverage and targets, prefer `TESTING_STRATEGY.md` over inventing new conventions.

### Linting, formatting, docs
- Lint (Clippy) over the whole workspace:
  - `cargo clippy --all`
- Format:
  - `cargo fmt --all`
- Local API/docs build:
  - `cargo doc --no-deps --open`

### Examples and demos
Top-level examples are the fastest way to see end‑to‑end behavior:
- Peer-to-peer calling:
  - `cargo run --example peer_receiver`  
  - `cargo run --example peer_caller`
- Audio streaming:
  - `cargo run --example audio_player -- input.wav`  
  - `cargo run --example audio_recorder -- output.wav`
- Call center playground:
  - `cargo run --example call_center`  
  - `cargo run --example call_center_client`

Crate-specific examples (not exhaustive):
- Session v3 SimplePeer flows:
  - `cargo run -p rvoip-session-core-v3 --example api_peer_audio`
- Call engine / call center flows:
  - `cargo run -p rvoip-call-engine --example call_center_server`

### Logging and debugging
- Enable verbose logging for any binary or example:
  - `RUST_LOG=debug cargo run --example peer_caller`
- When debugging tests, it is often useful to add `-- --nocapture` to see test output:
  - `cargo test -p rvoip-sip-core -- --nocapture`

## High-level architecture

### Workspace overview
The repository is a Rust workspace (`Cargo.toml` at root) containing the full RVOIP VoIP stack. Crates are grouped roughly as:
- **Infrastructure & shared building blocks**: `infra-common`, `users-core`, `auth-core`
- **Protocol & transport**: `sip-core`, `sip-transport`, `dialog-core`, `registrar-core`
- **Media & RTP**: `codec-core`, `rtp-core`, `media-core`
- **Session and endpoints**:
  - Legacy / internal: `session-core`, `session-core-v2`
  - **Recommended for new work**: `session-core-v3` (state-table based SimplePeer API)
  - Client-side: `client-core`, `sip-client`
- **Server & control-plane**: `b2bua-core`, `proxy-core`, `media-server-core`, `sbc-core`, `registrar-core`
- **Application / integration layer**: `call-engine` (call center orchestration), `api-server`, and the unified `rvoip` crate which re-exports the stack as a single dependency.

Use the root `README.md` and `LIBRARY_DESIGN_ARCHITECTURE.md` when you need a deeper view of how these crates relate; don’t re-derive the dependency graph from scratch unless you are validating it.

### Layered VoIP stack (conceptual)
At a high level, the system is structured in layers that align with a typical SIP/VoIP stack:
- **Application layer**
  - Call centers, custom VoIP apps, APIs
  - Crates: `call-engine`, `api-server`, top-level `examples/`, and consumer apps depending on `rvoip`
- **Session & coordination layer**
  - Controls call/session lifecycle and high-level call flows
  - Crates: `session-core-v3` (for endpoint/peer APIs), legacy `session-core`/`session-core-v2` for older flows
- **Protocol & signaling layer**
  - SIP dialogs, transactions, registration, routing
  - Crates: `sip-core`, `dialog-core`, `sip-transport`, `registrar-core`, client-facing pieces in `client-core`
- **Media & transport layer**
  - RTP, codecs, audio processing
  - Crates: `rtp-core`, `codec-core`, `media-core`
- **Infrastructure layer**
  - Shared config, event bus, utilities, common types
  - Crate: `infra-common` (plus auth/user crates for authentication concerns)

This layering is enforced both in the workspace configuration and in `LIBRARY_DESIGN_ARCHITECTURE.md`; when introducing new dependencies, align with those diagrams and avoid inverting the layering (e.g., media depending on application crates).

### Endpoints vs server-side components
There is a deliberate split between **endpoint/client** functionality and **server-side control-plane** functionality:
- **Endpoints / softphones / peers**
  - Use `session-core-v3` + `client-core`.
  - Typical flow: SimplePeer-style API manages dialogs and media via `dialog-core` and `media-core` under the hood.
- **Servers (proxy, B2BUA, media server, SBC)**
  - Use `b2bua-core`, `proxy-core`, `media-server-core`, `sbc-core` on top of `dialog-core`, `sip-transport`, and `media-core`.
  - Media servers are treated as separate processes controlled over an API; media processing happens in `media-core`/`rtp-core`, not in the B2BUA itself.

When adding behavior, choose the right side of this split: don’t push server-specific concerns into endpoint crates, and don’t couple media implementation details into pure signaling crates.

### Call center stack
The call center implementation is layered on top of the core stack:
- **Business logic / orchestration**: `call-engine`
  - Handles agent registration & status, queueing, routing, and B2BUA bridging.
  - Persists state in a SQL database (SQLite by default).
- **Signaling & sessions**: `session-core`/`session-core-v3`, `dialog-core`, `sip-transport`
- **Media**: `media-core` + `rtp-core`

The call center examples and e2e tests live under `crates/call-engine/examples/` and top-level `examples/call-center`. Before modifying call center behavior, skim the `call-engine` crate README and its `examples/` to understand the existing routing and queueing model.

### Key documents worth reading
When you need more context, prefer these documents (in roughly this order):
- `QUICKSTART.md` — concrete commands and example flows for getting something running quickly.
- `README.md` — high-level marketing/architecture overview and feature tables.
- `LIBRARY_DESIGN_ARCHITECTURE.md` — authoritative view of crate layering and responsibilities.
- `VERSION_STRATEGY.md` — which session-core variant to target (v3 is recommended) and migration notes.
- `TESTING_STRATEGY.md` — how tests are structured across unit/integration/e2e, and coverage expectations.
- `PROJECT_DEEP_ANALYSIS.md` — current high-level gaps and known inconsistencies between docs and code.

## Specs and OpenSpec workflow

This repository uses **OpenSpec** for spec-driven development under `openspec/`.
- For any change involving new capabilities, breaking changes, architecture shifts, or major performance/security work, consult `openspec/AGENTS.md` and follow the three-stage workflow (proposal → implementation → archive).
- Specs under `openspec/specs/` are treated as the source of truth for behavior; `openspec/changes/` describes proposed deltas.
- Before planning or implementing large features, use the `openspec` CLI to discover existing capabilities and active changes (see the command reference in `openspec/AGENTS.md`).

For small bug fixes, typo/formatting changes, configuration tweaks, or tests that simply encode existing behavior, you generally do **not** need to create a new OpenSpec change; follow the guidance in `openspec/AGENTS.md` about when proposals are required.

## How future agents should approach work here

- Prefer **session-core-v3** for any new endpoint/peer-level session work unless a document explicitly calls for another version.
- Keep layering intact: infrastructure → protocol/transport → media → session → application. Question any new dependency that points "up" the stack.
- When touching multi-crate behavior (e.g., call-engine plus media or SIP changes), look for an existing OpenSpec change or create one rather than scattering ad hoc changes.
- Use the existing scripts and documented commands (Quickstart, `scripts/test_all.sh`, crate READMEs) instead of inventing new workflows for building, testing, or running demos.