# RVOIP Version Strategy

**Last Updated**: January 11, 2026  
**Status**: Active Decision Document

---

## Current Situation

RVOIP currently has **three parallel implementations** of session-core:

| Version | Status | Completion | Recommendation |
|---------|--------|------------|----------------|
| **session-core** (v1) | 🟡 Maintenance | 85% | Legacy - Use only for existing projects |
| **session-core-v2** | 🟡 Active Development | 80% | Transitional - Being phased out |
| **session-core-v3** | 🟢 Recommended | 75% | **Recommended for new projects** |

---

## Official Recommendation

### ✅ For New Projects: Use `session-core-v3`

**Rationale**:
- Modern state table-driven architecture
- SimplePeer API for ease of use
- Active development and bug fixes
- Will become the official `session-core` in 1.0 release

**Example**:
```toml
[dependencies]
rvoip-session-core-v3 = { path = "crates/session-core-v3" }
```

### 🔄 For Existing Projects

**If using session-core (v1)**:
- Continue with current version for stability
- Plan migration to v3 within 6 months
- See [MIGRATION_GUIDE.md](MIGRATION_GUIDE.md) (coming soon)

**If using session-core-v2**:
- Upgrade to v3 is recommended
- v2 will be deprecated in next release
- API differences are minimal

---

## Deprecation Timeline

| Date | Event |
|------|-------|
| **January 2026** | v3 becomes official recommendation |
| **March 2026** | v2 marked as deprecated |
| **June 2026** | v1 marked as deprecated |
| **September 2026** | v1 and v2 archived (still available but no updates) |
| **1.0 Release** | Only v3 remains (renamed to `session-core`) |

---

## Key Differences

### session-core (v1) - Imperative Pattern
```rust
// Imperative coordination
let session = coordinator.create_outgoing_call(from, to).await?;
coordinator.wait_for_answer(session_id).await?;
```

**Pros**: Rich features, comprehensive APIs  
**Cons**: Complex (100+ files), harder to maintain

### session-core-v2 - State Table Driven
```rust
// State machine driven
let coordinator = UnifiedCoordinator::new().await?;
coordinator.handle_event(event).await?;
```

**Pros**: Declarative YAML configuration, cleaner architecture  
**Cons**: Less mature, some features incomplete

### session-core-v3 - SimplePeer API
```rust
// High-level simplified API
let mut peer = SimplePeer::new("alice").await?;
let call_id = peer.make_call("bob").await?;
peer.wait_for_answer().await?;
```

**Pros**: Easiest to use, modern patterns, actively developed  
**Cons**: Still alpha, some features in progress

---

## Feature Comparison

| Feature | v1 | v2 | v3 |
|---------|----|----|-----|
| Basic Calls | ✅ | ✅ | ✅ |
| Hold/Resume | ✅ | ✅ | ✅ |
| Blind Transfer (Initiator) | ✅ | ✅ | ✅ |
| Blind Transfer (Recipient) | ✅ | ⚠️ Partial | ✅ |
| Registration | ✅ | 🚧 In Progress | 🚧 In Progress |
| Presence | ✅ | 🚧 In Progress | 🚧 In Progress |
| Conferences | ✅ | ❌ | ❌ |
| SimplePeer API | ❌ | ⚠️ Basic | ✅ Full |
| State Table Config | ❌ | ✅ | ✅ |

---

## Migration Path

### From v1 to v3

**Step 1**: Update dependencies
```toml
# Old
rvoip-session-core = { path = "crates/session-core" }

# New
rvoip-session-core-v3 = { path = "crates/session-core-v3" }
```

**Step 2**: Adopt SimplePeer API
```rust
// Old v1 API
let coordinator = SessionCoordinator::new(config).await?;
let session = coordinator.create_outgoing_call(from, to).await?;

// New v3 API
let mut peer = SimplePeer::new("alice").await?;
let call_id = peer.make_call("bob").await?;
```

**Step 3**: Update event handling
- v3 uses simplified event enums
- Event subscription is built into SimplePeer

### From v2 to v3

Minimal changes required:
- Update import paths
- Adopt SimplePeer API if desired (optional)
- Update state table YAML if customized

---

## Development Status

### session-core-v3 Roadmap

**✅ Completed (Phase 1)**:
- Basic call flow (UAC/UAS)
- Audio streaming
- Hold/Resume
- Blind transfer initiation
- Blind transfer recipient (NEW)

**🚧 In Progress (Phase 2)**:
- Registration support
- Presence/Subscription
- NOTIFY handling improvements

**📋 Planned (Phase 3)**:
- Attended transfer
- Conference support
- Advanced error recovery

---

## Frequently Asked Questions

### Q: Can I use v1 in production?
**A**: Yes, v1 is stable but will be deprecated. Plan migration to v3.

### Q: Should I wait for v3 to stabilize?
**A**: No, v3 is ready for new projects. Core features are solid.

### Q: Will v1/v2 receive bug fixes?
**A**: Critical bugs only. New features only in v3.

### Q: When is 1.0 release?
**A**: Estimated Q3 2026, with v3 as the single session-core.

### Q: What about b2bua-core, proxy-core?
**A**: These new components will use v3 architecture from the start.

---

## Getting Help

- **Documentation**: See respective crate README files
- **Migration Issues**: Open GitHub issue with `migration` label
- **Questions**: GitHub Discussions

---

## Contributing

If working on session-core code:
- **All new features** go into v3
- **Bug fixes** may be backported to v2 if critical
- **v1** receives security fixes only

---

**Decision Authority**: RVOIP Core Team  
**Review Date**: March 2026  
**Status**: Official Policy
