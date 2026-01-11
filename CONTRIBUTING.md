# Contributing to RVOIP

First off, thank you for considering contributing to RVOIP! It's people like you that make RVOIP such a great tool for the Rust VoIP community.

## 🎯 Where to Start

### Quick Links
- [Project Health Dashboard](PROJECT_HEALTH.md) - See current status
- [Version Strategy](VERSION_STRATEGY.md) - Understand version policy
- [Missing Components](MISSING_COMPONENTS.md) - Find implementation opportunities
- [System Audit Report](SYSTEM_AUDIT_REPORT.md) - Deep dive into architecture

### Ways to Contribute

1. **Report Bugs** - Found a bug? Let us know!
2. **Suggest Features** - Have an idea? We'd love to hear it!
3. **Write Code** - Implement new features or fix bugs
4. **Improve Documentation** - Help others understand the codebase
5. **Write Tests** - Increase code coverage
6. **Review Pull Requests** - Share your expertise

## 🐛 Reporting Bugs

Before creating a bug report, please check existing issues to avoid duplicates.

**Use the bug report template** when creating an issue. Include:
- Clear title and description
- Steps to reproduce
- Expected vs actual behavior
- System information (OS, Rust version)
- Minimal code example if possible

## 💡 Suggesting Features

Feature suggestions are welcome! Before submitting:

1. Check if it's already planned in [MISSING_COMPONENTS.md](MISSING_COMPONENTS.md)
2. Check existing feature requests
3. Explain the use case and value
4. Consider implementation complexity

**Use the feature request template** when creating an issue.

## 🔧 Development Setup

### Prerequisites
```bash
# Install Rust (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install development tools
rustup component add rustfmt clippy
cargo install cargo-audit cargo-watch
```

### Clone and Build
```bash
# Clone the repository
git clone https://github.com/eisenzopf/rvoip.git
cd rvoip

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Run specific crate tests
cargo test -p rvoip-sip-core
```

### IDE Setup

**VS Code** (Recommended):
```bash
# Install Rust Analyzer extension
code --install-extension rust-lang.rust-analyzer
```

**Other IDEs**: IntelliJ IDEA, Vim, Emacs all have good Rust support.

## 📝 Code Contribution Process

### 1. Fork and Branch

```bash
# Fork on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/rvoip.git
cd rvoip

# Add upstream remote
git remote add upstream https://github.com/eisenzopf/rvoip.git

# Create a feature branch
git checkout -b feature/my-new-feature
```

### 2. Make Your Changes

**Follow these guidelines**:

#### Code Style
```bash
# Format code before committing
cargo fmt --all

# Check for common mistakes
cargo clippy --workspace --all-features -- -D warnings

# Run tests
cargo test --workspace
```

#### Commit Messages
Use conventional commits format:
```
type(scope): brief description

Longer explanation if needed.

Fixes #123
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Examples:
```
feat(sip-core): add support for REFER method
fix(dialog-core): correct dialog state transition
docs(readme): update installation instructions
test(media-core): add tests for AEC processing
```

#### Code Quality Checklist
- [ ] Code formatted with `cargo fmt`
- [ ] No clippy warnings
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] Examples work (if applicable)
- [ ] CHANGELOG.md updated (for significant changes)

### 3. Submit Pull Request

```bash
# Push to your fork
git push origin feature/my-new-feature

# Create Pull Request on GitHub
```

**In your PR description**:
- Explain what and why (not just how)
- Reference related issues
- List testing performed
- Note any breaking changes

## 🎯 Good First Issues

New to the project? Look for issues labeled:
- `good first issue` - Easy tasks for newcomers
- `help wanted` - We need community help
- `documentation` - Improve docs (no code required)

### Beginner-Friendly Tasks

1. **Add Tests**
   - Find components with `⚠️ Basic` test coverage in [PROJECT_HEALTH.md](PROJECT_HEALTH.md)
   - Add unit tests for untested functions
   - Target: Get coverage to 70%+

2. **Improve Documentation**
   - Add doc comments to public APIs
   - Write code examples
   - Improve README files

3. **Fix Compiler Warnings**
   - Run `cargo clippy --workspace`
   - Fix warnings one by one
   - Easy wins for code quality

4. **Add Examples**
   - Create examples for common use cases
   - Improve existing example documentation
   - Add README files to example directories

## 🏗️ Architecture Guidelines

### Crate Organization

Follow the established pattern:
```
crate-name/
├── Cargo.toml
├── README.md
├── TODO.md (optional)
├── src/
│   ├── lib.rs          # Public API
│   ├── errors/         # Error types
│   ├── api/            # High-level API
│   └── internal/       # Implementation details
├── tests/              # Integration tests
└── examples/           # Usage examples
```

### Dependency Rules

1. **Foundation crates** (no internal deps): `sip-core`, `codec-core`, `infra-common`
2. **Middle layer** depends on foundation: `dialog-core`, `media-core`, `rtp-core`
3. **Top layer** depends on middle: `session-core-v3`, `call-engine`

**Never create circular dependencies!**

### Which Version to Use?

- **New features**: Add to `session-core-v3` only
- **Bug fixes**: Fix in all affected versions if critical
- **Breaking changes**: Discuss in an issue first

See [VERSION_STRATEGY.md](VERSION_STRATEGY.md) for details.

## 🧪 Testing Guidelines

### Test Levels

1. **Unit Tests** (`src/` co-located with code)
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_parse_uri() {
           let uri = parse("sip:alice@example.com").unwrap();
           assert_eq!(uri.user, Some("alice".to_string()));
       }
   }
   ```

2. **Integration Tests** (`tests/` directory)
   ```rust
   #[tokio::test]
   async fn test_full_call_flow() {
       // Test complete scenarios
   }
   ```

3. **Doc Tests** (in doc comments)
   ```rust
   /// Parse a SIP URI
   /// 
   /// # Example
   /// ```
   /// use rvoip_sip_core::parse_uri;
   /// let uri = parse_uri("sip:alice@example.com").unwrap();
   /// ```
   pub fn parse_uri(input: &str) -> Result<Uri> {
       // ...
   }
   ```

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p rvoip-dialog-core

# With logging
RUST_LOG=debug cargo test

# Single test
cargo test test_parse_uri -- --nocapture
```

## 📚 Documentation Guidelines

### Doc Comments

Every public item should have documentation:

```rust
/// Brief one-line description.
///
/// More detailed explanation if needed. Can span multiple
/// paragraphs and include examples.
///
/// # Examples
///
/// ```
/// use rvoip_sip_core::Message;
/// let msg = Message::new();
/// ```
///
/// # Errors
///
/// Returns `Error::InvalidUri` if the URI is malformed.
///
/// # Panics
///
/// This function never panics (or document when it does).
pub fn create_message() -> Result<Message> {
    // ...
}
```

### README Files

Each crate should have a README with:
- Overview of purpose
- Feature list (✅ completed, 🚧 in progress, ❌ not started)
- Quick example
- Link to docs.rs
- Architecture diagram (if applicable)

## 🚀 Advanced Contributions

### Implementing Missing Components

Want to implement a major component? Great!

1. **Read the design document** (see [MISSING_COMPONENTS.md](MISSING_COMPONENTS.md))
2. **Open a discussion** - Claim the component
3. **Start with API design** - Get feedback early
4. **Write tests first** - TDD approach
5. **Implement incrementally** - Small, reviewable PRs
6. **Document thoroughly** - README, doc comments, examples

### Priority Order

See [MISSING_COMPONENTS.md](MISSING_COMPONENTS.md) for priorities:

🔴 **Critical** (blocking production):
- b2bua-core
- media-server-core
- proxy-core

🟡 **Medium** (important but workarounds exist):
- sbc-core
- Enhanced registrar-core

🟢 **Low** (nice to have):
- Performance optimizations
- Additional examples

## 🤝 Code Review Process

### What We Look For

**Code Quality**:
- ✅ Follows Rust idioms
- ✅ No unnecessary allocations
- ✅ Proper error handling
- ✅ Clear variable names

**Testing**:
- ✅ Tests for new code
- ✅ Edge cases covered
- ✅ Integration tests if needed

**Documentation**:
- ✅ Public APIs documented
- ✅ Complex logic explained
- ✅ Examples provided

**Backwards Compatibility**:
- ✅ No breaking changes without discussion
- ✅ Deprecation warnings if needed
- ✅ Migration guide for breaking changes

### Review Timeline

- **Initial feedback**: 2-3 days
- **Follow-up reviews**: 1-2 days
- **Merge decision**: After approval from 1+ maintainers

### Addressing Review Comments

```bash
# Make requested changes
git add .
git commit -m "Address review comments"

# Update your PR
git push origin feature/my-new-feature
```

Don't force-push unless specifically requested!

## 📋 Project Governance

### Maintainers

Core maintainers have merge rights and guide project direction:
- Review and merge pull requests
- Triage issues
- Set roadmap priorities
- Enforce code of conduct

### Decision Making

- **Minor changes**: Maintainer approval
- **Major changes**: Discussion → RFC → Implementation
- **Breaking changes**: Require RFC and consensus

### Becoming a Maintainer

Regular contributors may be invited to become maintainers based on:
- Quality and quantity of contributions
- Understanding of codebase
- Community involvement
- Alignment with project values

## 🎓 Learning Resources

### Rust Resources
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Async Book](https://rust-lang.github.io/async-book/)

### VoIP/SIP Resources
- [RFC 3261 - SIP](https://www.rfc-editor.org/rfc/rfc3261)
- [RFC 3550 - RTP](https://www.rfc-editor.org/rfc/rfc3550)
- [RFC 3711 - SRTP](https://www.rfc-editor.org/rfc/rfc3711)

### Project-Specific
- [Architecture Guide](LIBRARY_DESIGN_ARCHITECTURE.md)
- [Session Core Comparison](SESSION_CORE_COMPARISON.md)
- [Examples Directory](examples/)

## 💬 Communication

### GitHub
- **Issues** - Bug reports, feature requests
- **Discussions** - Questions, ideas, announcements
- **Pull Requests** - Code contributions

### Best Practices
- Be respectful and constructive
- Search before posting
- Provide context and examples
- Follow up on your issues/PRs

## 📜 Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive environment for everyone.

### Expected Behavior
- Be respectful and considerate
- Give and receive constructive feedback
- Focus on what's best for the community
- Show empathy towards others

### Unacceptable Behavior
- Harassment or discrimination
- Trolling or inflammatory comments
- Personal or political attacks
- Publishing others' private information

### Enforcement

Violations may result in:
1. Warning
2. Temporary ban
3. Permanent ban

Report issues to project maintainers.

## ❓ FAQ

### Q: I found a security vulnerability. What should I do?
**A**: Do NOT open a public issue. Email maintainers directly or use GitHub's private vulnerability reporting.

### Q: How long does it take to review my PR?
**A**: Usually 2-3 days for initial feedback. Complex PRs may take longer.

### Q: Can I work on multiple issues at once?
**A**: Yes, but focus on completing PRs rather than starting many incomplete ones.

### Q: My PR was closed without merging. Why?
**A**: Common reasons: doesn't fit project goals, needs too much work, or superseded by another solution. Check the closing comment for details.

### Q: I want to add a dependency. Is that okay?
**A**: Discuss in an issue first. We prefer minimal dependencies and avoid large/unmaintained crates.

### Q: How can I become a maintainer?
**A**: Contribute regularly (code, reviews, documentation) and express interest. We'll reach out when ready.

## 🙏 Recognition

Contributors are recognized in:
- GitHub contributors graph
- CHANGELOG.md entries
- Release notes
- Special mentions for major contributions

Thank you for contributing to RVOIP! 🚀

---

**Questions?** Open a discussion or ask in your PR/issue.

**Happy Coding!** 🦀
