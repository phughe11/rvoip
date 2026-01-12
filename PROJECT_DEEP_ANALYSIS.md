# Project Deep Analysis & Gap Assessment

## 1. Overview
This document provides a comprehensive audit of the **RVOIP** project state as of `2026-01-12`.
The analysis compares the actual codebase against the documentation (`README.md`, `IMPLEMENTATION_PROGRESS.md`) and the stated architectural goals.

> **Note**: Updated January 12, 2026 - Architecture dependency fix completed (B2BUA ↔ SBC layering corrected).

## 2. Component Analysis

### 2.1 Server Layer (The "Big Three" + SBC)

| Component | Status | Reality Check | Remaining Gaps |
| :--- | :--- | :--- | :--- |
| **B2BUA Core** | ✅ **Functional** (~60%) | Implements Transparent 2-Way Bridging, Answer Handling, Hangup Propagation. Uses `UnifiedDialogManager`. | Advanced transfer, error recovery, application handlers. |
| **Proxy Core** | ⚠️ **Basic** (~40%) | Implements `ProxyServer`, Event Loop, `LocationService` (In-Memory). | Load balancing, persistent storage, SRV lookup. |
| **Media Server** | ✅ **Functional** (~50%) | Implements `ConferenceManager`, `DTMF`, WAV playback. Uses `media-core::AudioMixer`. APIs exposed in Call Engine. | Recording, REST API, endpoint pool. |
| **SBC Core** | ✅ **Integrated** (~45%) | **Architecture Fixed!** SBC no longer a dependency of B2BUA. Instead, SBC implements `RequestProcessor` trait and can be injected into B2BUA via `B2buaEngine::with_processor()`. SBC has optional `b2bua` feature flag. | Via/Contact rewriting, NAT traversal, TLS termination. |

### 2.2 Core Infrastructure

| Component | Status | Reality Check |
| :--- | :--- | :--- |
| **Dialog Core** | ✅ **Mature** | "Unified" version active. Handles Transactions, State, SDP persistence (Just added). |
| **SIP Core** | ✅ **Mature** | Robust parser/generator. |
| **Media Core** | ✅ **Mature** | Contains DSP logic (`AudioMixer`, `AEC`, `VAD`). |
| **Session Core v1** | ⚠️ **Legacy** | Still used by `CallCenterEngine`, `client-core`, `rvoip`. Migration to v3 in progress. |
| **Session Core v3** | ✅ **Recommended** | SimplePeer API, state-table driven. Ready for new projects. call-engine has v3 dependency added for gradual migration. |

### 2.3 Application Layer

| Component | Status | Reality Check |
| :--- | :--- | :--- |
| **Call Engine** | ⚠️ **Hybrid** | Orchestrator initializes `MediaServer` and `B2BUA`. <br> **Issue**: Tries to run Legacy (`SessionCoordinator`) and Modern (`B2buaEngine`) side-by-side. |
| **rvoip (Crate)** | ⚠️ **Library Only** | `src/lib.rs` exists. **Missing Binary (`main.rs`)**. Cannot "run" the server outside of tests. |

## 3. Documentation vs. Reality

| Document | Claim | Reality | Verdict |
| :--- | :--- | :--- | :--- |
| **README.md** | "Modern Architecture" | Architecture is correct, but wiring is incomplete (SBC). | **Mostly Accurate** |
| **PROGRESS.md** | "SBC Core: Complete" | Logic written, but effective protection is 0% (unplugged). | **Misleading** |
| **PROGRESS.md** | "Application Integration" | `CallCenterEngine` has fields, but running it requires solving the Port Conflict. | **Partial** |

## 4. Remaining Gaps & Risks

1.  **The "Dual Engine" Consideration**:
    - `CallCenterEngine` may initialize both `SessionCoordinator` (Legacy) AND `B2buaEngine` (New).
    - **Mitigation**: Use different ports or share `TransactionManager` instance.
    - **Status**: ⚠️ Needs careful configuration in production.

2.  ~~**SBC is a Ghost**~~ ✅ **RESOLVED**:
    - **Fixed**: `B2buaEngine::process_invite()` now calls `sbc.process_request()` before creating dialogs.
    - SBC provides rate limiting and header sanitization.
    - **Remaining**: Via/Contact rewriting for full topology hiding.

3.  **No Standalone Executable**:
    - User cannot `cargo run` the main rvoip crate directly.
    - **Workaround**: Use examples (`cargo run --example call_center`).

## 5. Roadmap Recommendations

1.  **Solve Port Conflict**: Pass `Arc<TransactionManager>` from `CallCenterEngine` to both subsystems, OR disable `SessionCoordinator`.
2.  ~~**Wire SBC**~~ ✅ **DONE**: SBC integrated via `RequestProcessor` trait injection. Use `B2buaEngine::with_processor(Some(Arc::new(sbc_engine)))`.
3.  **Create Binary**: Add `crates/rvoip/src/main.rs`.
4.  **Migrate to Session-Core-v3**: 
    - call-engine: Major refactor needed (SessionCoordinator → UnifiedCoordinator)
    - client-core, rvoip: Smaller scope, prioritize next
    - See `openspec/changes/refactor-unify-session-core-v3/` for tracking
