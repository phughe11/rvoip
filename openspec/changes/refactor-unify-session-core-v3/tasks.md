## 1. Fix Architecture Dependencies ✅ COMPLETED
- [x] 1.1 Remove `rvoip-sbc-core` from b2bua-core Cargo.toml
- [x] 1.2 Update b2bua-core to work without sbc-core dependency
  - Created `RequestProcessor` trait for optional security policy injection
  - Added `NoOpProcessor` default implementation
  - B2buaEngine now accepts `Option<Arc<dyn RequestProcessor>>` via `with_processor()`
- [x] 1.3 Optionally add b2bua-core as dependency in sbc-core (for B2BUA mode)
  - Added `b2bua` feature flag
  - SbcEngine implements `RequestProcessor` trait when feature enabled
- [x] 1.4 Verify `cargo build` succeeds for both crates

## 2. Migrate call-engine to session-core-v3 🚧 DEFERRED
**Reason**: call-engine deeply uses SessionCoordinator, BridgeInfo, etc. from v1.
Full migration requires major refactor of CallCenterEngine to use UnifiedCoordinator.
Recommendation: Keep v1 for now, plan dedicated migration sprint.

- [x] 2.1 Update call-engine/Cargo.toml: added session-core-v3 alongside v1
- [ ] 2.2 Refactor orchestrator/core.rs to use UnifiedCoordinator
- [ ] 2.3 Update integration/mod.rs to use v3 event types
- [ ] 2.4 Update prelude exports for v3 types
- [ ] 2.5 Remove dual engine initialization in CallCenterEngine
- [ ] 2.6 Update bridge management for v3 event-driven model
- [ ] 2.7 Run call-engine tests and fix failures

## 3. Migrate client-core to session-core-v3 🚧 PENDING
- [ ] 3.1 Update client-core/Cargo.toml: replace session-core with session-core-v3
- [ ] 3.2 Refactor VoipClient to use SimplePeer API
- [ ] 3.3 Update event handling for v3 event model
- [ ] 3.4 Maintain backward-compatible public API where possible
- [ ] 3.5 Run client-core tests and fix failures

## 4. Migrate rvoip facade crate 🚧 PENDING
- [ ] 4.1 Update rvoip/Cargo.toml to use session-core-v3
- [ ] 4.2 Update lib.rs re-exports for v3
- [ ] 4.3 Add deprecation notices for v1 types
- [ ] 4.4 Run rvoip tests

## 5. Update dependent components 🚧 PENDING
- [ ] 5.1 Update audio-core to use session-core-v3
- [ ] 5.2 Update sip-client to use session-core-v3
- [ ] 5.3 Update examples to use v3 APIs
- [ ] 5.4 Update documentation (README, QUICKSTART, etc.)

## 6. Final validation 🚧 PENDING
- [ ] 6.1 Run full test suite: `cargo test --all`
- [ ] 6.2 Run clippy: `cargo clippy --all`
- [ ] 6.3 Run examples to verify functionality
- [ ] 6.4 Update PROJECT_DEEP_ANALYSIS.md with new status

## Summary
**Phase 1 (Architecture Fix)**: ✅ Complete
- b2bua-core no longer depends on sbc-core (proper layering)
- sbc-core can optionally use b2bua-core via feature flag
- RequestProcessor trait allows SBC to be injected into B2BUA

**Phase 2-5 (session-core-v3 Migration)**: 🚧 Partially started
- call-engine: Added v3 dependency, but v1 still primary (needs dedicated sprint)
- client-core, rvoip, others: Not started
