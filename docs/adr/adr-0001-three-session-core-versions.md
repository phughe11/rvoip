# ADR-0001: Three Session-Core Versions Strategy

**Status**: Accepted  
**Date**: 2026-01-11  
**Deciders**: RVOIP Core Team  
**Technical Story**: Resolves version confusion from parallel development

---

## Context

RVOIP currently has three parallel versions of session-core:

1. **session-core** (v1) - Original implementation (70% complete)
2. **session-core-v2** - Refactored version (80% complete) 
3. **session-core-v3** - Latest version (60% complete)

This has caused:
- **Developer confusion** about which version to use
- **Fragmented efforts** across three codebases
- **Maintenance burden** maintaining three APIs
- **User uncertainty** about stability and recommendations
- **Documentation gaps** with unclear migration paths

The team never explicitly decided to maintain three versions - they emerged through iterative rewrites without proper deprecation of older versions.

---

## Decision

We will adopt a **phased consolidation strategy**:

### Immediate (v0.2.0 - Q1 2026)

1. **Declare session-core-v3 as the recommended version**
   - Focus all new development here
   - Mark as "Transitional - Recommended for new projects"

2. **Mark session-core-v2 as transitional**
   - Maintain for existing users
   - Accept critical bug fixes only
   - No new features

3. **Mark session-core (v1) as maintenance mode**
   - Security fixes only
   - Encourage migration to v3
   - Deprecation warning in docs

### Near-term (v0.3.0 - Q2 2026)

4. **Complete session-core-v3 feature parity**
   - Registration support
   - Presence support  
   - All v2 capabilities
   - Comprehensive tests (70%+ coverage)

5. **Provide migration guides**
   - v1 → v3 migration path
   - v2 → v3 migration path
   - Code examples
   - API comparison table

### Long-term (v0.5.0 - Q3 2026)

6. **Deprecate v1 and v2**
   - Archive session-core (v1)
   - Archive session-core-v2
   - Provide migration support for 6 months

7. **Rename session-core-v3 → session-core**
   - Clean version history
   - Single source of truth

---

## Consequences

### Positive

- **Clarity**: Users know which version to use
- **Focus**: Development effort concentrated on v3
- **Quality**: Better testing and documentation for one version
- **Maintenance**: Reduced burden maintaining three APIs
- **Community**: Clear path forward for contributors

### Negative

- **Migration work**: Existing v1/v2 users must eventually migrate
- **Breaking changes**: v3 API differs from v1/v2
- **Documentation**: Need to write migration guides
- **Compatibility**: Existing examples may break
- **Timeline**: Will take 6+ months to fully consolidate

### Neutral

- **Learning curve**: New users only learn one API
- **Code churn**: Temporary disruption during migration
- **Risk**: v3 is less mature than v2 (mitigated by testing)

---

## Alternatives Considered

### Alternative 1: Maintain All Three Versions Indefinitely

**Description**: Continue developing all three versions in parallel.

**Rejected Because**:
- Unsustainable maintenance burden
- Confuses users and contributors
- Fragments limited development resources
- No clear path to stability
- All three would remain incomplete

### Alternative 2: Pick v2 as the Winner

**Description**: Standardize on session-core-v2 and abandon v1/v3.

**Rejected Because**:
- v3 has better architectural foundations
- v3 uses more modern patterns
- v2 has known design limitations
- Would waste v3 development effort
- v3 maintainers prefer v3 design

### Alternative 3: Start Over with v4

**Description**: Abandon all three and start fresh with lessons learned.

**Rejected Because**:
- Would extend timeline by 6+ months
- Much of v3 code is reusable
- Would further confuse users
- Risk of repeating same mistakes
- Team has limited resources

### Alternative 4: Immediate Deprecation of v1/v2

**Description**: Immediately remove v1/v2 support and force migration to v3.

**Rejected Because**:
- Breaking change for existing users
- v3 not yet feature-complete
- No migration path documented
- Would harm community trust
- Not following semantic versioning

---

## Implementation Plan

### Phase 1: Documentation (January 2026)

- [x] Create VERSION_STRATEGY.md
- [x] Update README.md with version warnings
- [x] Mark v1/v2 with deprecation notices
- [x] Document v3 as recommended
- [x] Create this ADR

### Phase 2: Feature Completion (Q1-Q2 2026)

- [ ] Complete v3 registration support
- [ ] Complete v3 presence support
- [ ] Achieve 70%+ test coverage in v3
- [ ] Write comprehensive v3 documentation
- [ ] Create migration guides

### Phase 3: Community Migration (Q2-Q3 2026)

- [ ] Announce deprecation timeline
- [ ] Provide migration support
- [ ] Update all examples to v3
- [ ] Offer consultation for complex migrations
- [ ] Monitor community feedback

### Phase 4: Consolidation (Q3 2026+)

- [ ] Archive v1 and v2 repositories
- [ ] Rename v3 to canonical session-core
- [ ] Clean up workspace structure
- [ ] Update all documentation
- [ ] Celebrate simplified architecture

---

## Success Metrics

- **User clarity**: No GitHub issues asking "which version?"
- **Contribution focus**: 90%+ PRs target v3
- **Migration rate**: 80%+ users on v3 by Q3 2026
- **Code quality**: v3 reaches 70%+ test coverage
- **Documentation**: Complete migration guides

---

## Related Decisions

- **ADR-0002**: Async-First Architecture (informed v3 design)
- **ADR-0003**: Memory Safety with Rust (applies to all versions)
- **Future ADR**: API stability guarantees for v1.0

---

## References

- [VERSION_STRATEGY.md](../../VERSION_STRATEGY.md) - Implementation details
- [session-core-v3 Cargo.toml](../../crates/session-core-v3/Cargo.toml) - v3 implementation
- [Semantic Versioning](https://semver.org/) - Version numbering
- [Rust API Evolution](https://rust-lang.github.io/api-guidelines/) - Breaking change guidelines

---

## Review History

| Date | Reviewer | Decision | Notes |
|------|----------|----------|-------|
| 2026-01-11 | System Analysis | Proposed | Created from audit findings |
| 2026-01-11 | RVOIP Core Team | Accepted | Approved strategy |

---

**Next Review**: April 2026 (after Q1 milestone)
