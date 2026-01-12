# Change: Unify System to session-core-v3 and Fix Architecture Dependencies

## Why
The RVOIP project currently has architectural inconsistencies that block production readiness:
1. Multiple crates still depend on legacy `session-core` (v1) instead of recommended `session-core-v3`
2. `b2bua-core` incorrectly depends on `sbc-core` (should be the inverse)
3. `CallCenterEngine` has a "dual engine" problem, running both legacy SessionCoordinator and new B2buaEngine simultaneously

Per VERSION_STRATEGY.md, session-core-v3 is the official recommendation and v1/v2 are deprecated.

## What Changes

### Phase 1: Fix Architecture Dependencies
- **BREAKING**: Remove `rvoip-sbc-core` dependency from `b2bua-core`
- Update `sbc-core` to optionally depend on `b2bua-core` (for B2BUA mode)
- Ensure layering follows LIBRARY_DESIGN_ARCHITECTURE.md

### Phase 2: Migrate call-engine to session-core-v3
- Replace `rvoip-session-core` dependency with `rvoip-session-core-v3`
- Refactor `CallCenterEngine` to use `UnifiedCoordinator`/`SimplePeer` API
- Remove dual engine initialization (deprecate legacy SessionCoordinator usage)
- Update bridge management to use v3's event-driven model

### Phase 3: Migrate client-core to session-core-v3
- Replace `rvoip-session-core` with `rvoip-session-core-v3`
- Update client APIs to use SimplePeer pattern
- Maintain backward-compatible public API where possible

### Phase 4: Migrate rvoip facade crate
- Update re-exports to use session-core-v3
- Deprecate v1 re-exports (with warnings)
- Update documentation and examples

### Phase 5: Update dependent components
- `audio-core`: Update session integration
- `sip-client`: Update to v3 APIs
- Examples: Update all examples to use v3

## Impact
- Affected specs: session-management, call-engine, client-core
- Affected code: 
  - `crates/call-engine/` (major refactor)
  - `crates/client-core/` (medium refactor)
  - `crates/b2bua-core/Cargo.toml` (dependency fix)
  - `crates/sbc-core/Cargo.toml` (dependency fix)
  - `crates/rvoip/` (re-export updates)
  - `crates/audio-core/` (minor updates)
  - `crates/sip-client/` (minor updates)
  - `examples/` (updates throughout)

## Risks
- **High**: call-engine refactor is substantial; integration tests critical
- **Medium**: API changes may break downstream users
- **Low**: Architecture dependency fix is straightforward

## Success Criteria
1. `cargo build --all` succeeds with no session-core v1 dependencies in main crates
2. All existing tests pass
3. Examples work with v3 APIs
4. Documentation updated
