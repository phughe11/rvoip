# RVOIP Testing Strategy

**Version**: 1.0  
**Last Updated**: January 11, 2026  
**Status**: Living Document

---

## 🎯 Testing Goals

1. **Safety**: Prevent regressions and critical bugs
2. **Confidence**: Enable fearless refactoring
3. **Documentation**: Tests as executable specifications
4. **Coverage**: Target 80%+ for production code
5. **Performance**: Maintain benchmarks for critical paths

---

## 📊 Current Test Coverage Status

```
┌─────────────────────────────────────────────┐
│  Overall Coverage:  ████████░░░░░░░  55%    │
├─────────────────────────────────────────────┤
│  Unit Tests:        ███████████░░░░░  70%    │
│  Integration Tests: ████░░░░░░░░░░░░  30%    │
│  E2E Tests:         ██░░░░░░░░░░░░░░  15%    │
└─────────────────────────────────────────────┘
```

**Goal by v0.2.0**: 70% overall  
**Goal by v1.0.0**: 85% overall

### Coverage by Component

| Component | Unit | Integration | E2E | Overall |
|-----------|------|-------------|-----|---------|
| sip-core | 85% | 40% | 20% | 65% |
| dialog-core | 80% | 45% | 25% | 70% |
| media-core | 75% | 35% | 15% | 60% |
| rtp-core | 90% | 50% | 30% | 75% |
| session-core-v3 | 55% | 20% | 10% | 40% |
| codec-core | 80% | 30% | 5% | 55% |
| audio-core | 70% | 25% | 10% | 50% |

**Priority Areas**: session-core-v3, integration tests, E2E tests

---

## 🧪 Test Pyramid

```
           /\          
          /E2\         ← End-to-End (15%)
         /____\           Full scenarios
        /      \      
       /Integr.\      ← Integration (30%)
      /__ation__\        Component interactions
     /            \   
    /    Unit      \  ← Unit Tests (55%)
   /________________\    Pure functions
```

### Unit Tests (55% of total)
- **What**: Test individual functions, structs, methods
- **Why**: Fast feedback, easy to debug
- **Where**: `src/` alongside code or `tests/unit/`
- **Tools**: `#[cfg(test)]`, `cargo test`

### Integration Tests (30% of total)
- **What**: Test component interactions
- **Why**: Verify interfaces work together
- **Where**: `tests/integration/`
- **Tools**: Test harnesses, mock servers

### End-to-End Tests (15% of total)
- **What**: Test complete user scenarios
- **Why**: Verify real-world usage
- **Where**: `tests/e2e/`, `examples/`
- **Tools**: Test SIP endpoints, network simulation

---

## 📝 Test Categories

### 1. Unit Tests

**Purpose**: Test individual components in isolation.

**Guidelines**:
- Fast (< 1ms per test)
- No I/O (network, filesystem, database)
- Pure functions preferred
- Mock external dependencies
- Test edge cases

**Example**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_sip_uri() {
        let uri = "sip:alice@example.com";
        let result = SipUri::parse(uri);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().user, "alice");
    }

    #[test]
    fn test_parse_invalid_sip_uri() {
        let uri = "invalid";
        let result = SipUri::parse(uri);
        assert!(result.is_err());
    }
}
```

---

### 2. Integration Tests

**Purpose**: Test how components work together.

**Guidelines**:
- Moderate speed (< 100ms per test)
- Real dependencies where feasible
- Test realistic scenarios
- Verify contracts between components

**Example**:
```rust
// tests/integration/dialog_tests.rs
#[tokio::test]
async fn test_dialog_establishment() {
    let transport = create_test_transport().await;
    let dialog_manager = DialogManager::new(transport);
    
    // Simulate incoming INVITE
    let invite = create_test_invite();
    let dialog = dialog_manager.create_dialog(invite).await;
    
    assert_eq!(dialog.state(), DialogState::Early);
}
```

---

### 3. End-to-End Tests

**Purpose**: Test complete user flows.

**Guidelines**:
- Slow (seconds per test)
- Real components
- Minimal mocking
- Test happy paths and error cases
- Use in CI sparingly

**Example**:
```rust
// tests/e2e/call_flow_test.rs
#[tokio::test]
async fn test_basic_call_flow() {
    let alice = create_test_client("alice").await;
    let bob = create_test_client("bob").await;
    
    // Alice calls Bob
    let call = alice.call("sip:bob@example.com").await.unwrap();
    
    // Bob answers
    let incoming = bob.wait_for_call().await.unwrap();
    incoming.answer().await.unwrap();
    
    // Verify both in call
    assert_eq!(call.state(), CallState::Active);
    assert_eq!(incoming.state(), CallState::Active);
    
    // Hang up
    call.hangup().await.unwrap();
}
```

---

### 4. Property-Based Tests

**Purpose**: Test invariants with random inputs.

**Tools**: [proptest](https://github.com/proptest-rs/proptest), [quickcheck](https://github.com/BurntSushi/quickcheck)

**Example**:
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_sip_uri_roundtrip(user in "[a-z]{1,20}", host in "[a-z]{1,20}") {
        let original = format!("sip:{}@{}", user, host);
        let parsed = SipUri::parse(&original).unwrap();
        let serialized = parsed.to_string();
        prop_assert_eq!(original, serialized);
    }
}
```

---

### 5. Fuzz Tests

**Purpose**: Find crashes and panics with random inputs.

**Tools**: [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz)

**Example**:
```rust
// fuzz/fuzz_targets/sip_parser.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use sip_core::parser::parse_request;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = parse_request(s);
    }
});
```

---

### 6. Benchmark Tests

**Purpose**: Track performance over time.

**Tools**: [criterion](https://github.com/bheisler/criterion.rs)

**Example**:
```rust
// benches/sip_parser_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_parse_invite(c: &mut Criterion) {
    let invite = "INVITE sip:bob@example.com SIP/2.0\r\n...";
    
    c.bench_function("parse_invite", |b| {
        b.iter(|| parse_request(black_box(invite)))
    });
}

criterion_group!(benches, benchmark_parse_invite);
criterion_main!(benches);
```

---

## 🛠️ Test Infrastructure

### Test Utilities

Location: `tests/common/` or `test-utils/`

**Provide**:
- Mock SIP endpoints
- Test message builders
- Network simulators
- Time controllers
- Assertion helpers

**Example**:
```rust
// tests/common/mod.rs
pub fn create_test_invite() -> SipRequest {
    SipRequest::builder()
        .method(Method::INVITE)
        .uri("sip:bob@example.com")
        .from("Alice", "sip:alice@example.com")
        .to("Bob", "sip:bob@example.com")
        .build()
}
```

---

### Continuous Integration

**Run on every PR**:
- Unit tests (all)
- Integration tests (fast subset)
- Linting (`cargo clippy`)
- Formatting (`cargo fmt --check`)
- Documentation (`cargo doc`)

**Run nightly**:
- All integration tests
- E2E tests
- Fuzz tests (short runs)
- Benchmarks
- Coverage reports

---

### Coverage Reporting

**Tools**:
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)
- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)

**Commands**:
```bash
# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# Upload to codecov (CI)
bash <(curl -s https://codecov.io/bash)
```

**Track**:
- Overall coverage percentage
- Per-component coverage
- Coverage trends over time
- Uncovered critical paths

---

## ✅ Test Checklist

### Before Every Commit
- [ ] All existing tests pass
- [ ] New code has tests
- [ ] Tests are meaningful (not just for coverage)
- [ ] Tests are fast (optimize or mark as ignored)
- [ ] No `.skip()` or `ignore` without comments

### Before Every PR
- [ ] CI passes
- [ ] Coverage doesn't decrease
- [ ] Integration tests pass
- [ ] Documentation tests pass
- [ ] No flaky tests introduced

### Before Every Release
- [ ] All tests pass (including E2E)
- [ ] Coverage meets target
- [ ] Benchmarks reviewed
- [ ] No disabled tests without tracking issues

---

## 🎯 Coverage Targets

### By Release

| Release | Unit | Integration | E2E | Overall |
|---------|------|-------------|-----|---------|
| v0.2.0 | 75% | 35% | 20% | 60% |
| v0.3.0 | 80% | 40% | 25% | 65% |
| v0.5.0 | 85% | 50% | 30% | 70% |
| v1.0.0 | 90% | 60% | 40% | 80% |

### Critical Components

Must reach 80%+ before v1.0:
- sip-core (SIP parsing and state machines)
- dialog-core (Dialog management)
- session-core-v3 (Session logic)
- media-core (Media processing)
- rtp-core (RTP/SRTP)

---

## 🚫 Common Anti-Patterns

### ❌ Don't Do This

**1. Testing Implementation Details**
```rust
// Bad: Testing private methods
#[test]
fn test_internal_cache_update() {
    let obj = MyObject::new();
    obj.update_cache(); // Don't test private methods
}
```

**2. Brittle Tests**
```rust
// Bad: Overly specific assertions
assert_eq!(log_output, "INFO: Starting at 2026-01-11 10:30:15");
```

**3. Slow Unit Tests**
```rust
// Bad: I/O in unit tests
#[test]
fn test_load_config() {
    let config = load_from_file("config.yaml"); // Use mocks!
}
```

**4. Flaky Tests**
```rust
// Bad: Timing-dependent
#[test]
fn test_timeout() {
    std::thread::sleep(Duration::from_millis(100));
    assert!(is_complete()); // May fail randomly
}
```

### ✅ Do This Instead

**1. Test Public API**
```rust
// Good: Test behavior through public API
#[test]
fn test_cache_updates_on_set() {
    let mut obj = MyObject::new();
    obj.set_value(42);
    assert_eq!(obj.get_value(), 42); // Test observable behavior
}
```

**2. Flexible Assertions**
```rust
// Good: Test essential properties
assert!(log_output.starts_with("INFO: Starting"));
```

**3. Mock I/O**
```rust
// Good: Dependency injection
#[test]
fn test_load_config() {
    let mock_reader = MockConfigReader::new("test: value");
    let config = load_config(mock_reader);
}
```

**4. Deterministic Tests**
```rust
// Good: Control time
#[test]
fn test_timeout() {
    let mut clock = MockClock::new();
    clock.advance(Duration::from_millis(100));
    assert!(is_complete(&clock));
}
```

---

## 📚 Resources

### Internal
- [Test examples](../examples/)
- [Test utilities](../tests/common/)
- [Benchmarks](../benches/)

### External
- [Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust API Guidelines - Testing](https://rust-lang.github.io/api-guidelines/testing.html)
- [Test Pyramid](https://martinfowler.com/articles/practical-test-pyramid.html)
- [Property-Based Testing](https://hypothesis.works/articles/what-is-property-based-testing/)

---

## 🔄 Continuous Improvement

### Monthly Reviews
- Review coverage trends
- Identify untested areas
- Remove obsolete tests
- Update this strategy

### Quarterly Goals
- Increase coverage by 5%
- Add missing integration tests
- Improve test performance
- Update test infrastructure

---

## ❓ Questions?

- Ask in [GitHub Discussions](https://github.com/eisenzopf/rvoip/discussions)
- See [CONTRIBUTING.md](../CONTRIBUTING.md) for general contribution guidelines
- Check component-specific test docs in each crate's README

---

**Next Review**: April 2026  
**Maintained By**: RVOIP Core Team
