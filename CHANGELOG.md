# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive project documentation and governance
  - VERSION_STRATEGY.md for version management guidance
  - MISSING_COMPONENTS.md to track unimplemented features
  - PROJECT_HEALTH.md dashboard for project status
  - CONTRIBUTING.md with detailed contribution guidelines
  - GitHub issue templates (bug report, feature request, question)
  - Pull request template with comprehensive checklist
  - CI/CD workflow template for automated testing

### Changed
- Cargo.toml lint configuration to enable more warnings for better code quality
- README.md with version status warning and important documentation links
- session-core crate READMEs with clear version status indicators
  - session-core (v1): Marked as maintenance mode
  - session-core-v2: Marked as transitional
  - session-core-v3: Marked as recommended version

### Documentation
- SYSTEM_AUDIT_REPORT.md - Comprehensive code review and analysis
- FIXES_SUMMARY.md - Summary of initial improvements

## [0.1.26] - 2025-01-11

### Project Status
- **Alpha Release** - APIs subject to change
- Core components production-ready: sip-core, dialog-core, media-core, rtp-core
- Session management in active development (v3 recommended)
- Missing critical components: b2bua-core, proxy-core, media-server-core

---

## Version Policy

### session-core Versions

Starting January 2026, the project has three session-core implementations with clear guidance:

- **session-core-v3** (RECOMMENDED)
  - Latest version with SimplePeer API
  - Target for all new features
  - Will become official session-core in 1.0

- **session-core-v2** (TRANSITIONAL)
  - State table-based architecture
  - Critical bug fixes only
  - Deprecated March 2026

- **session-core-v1** (MAINTENANCE)
  - Original imperative pattern
  - Security fixes only
  - Deprecated June 2026

See [VERSION_STRATEGY.md](VERSION_STRATEGY.md) for details.

---

## Component Status

### ✅ Production Ready
- **sip-core** (0.1.0) - RFC 3261 compliant SIP implementation
- **dialog-core** (0.1.0) - SIP dialog management
- **media-core** (0.1.0) - Audio processing with AEC, AGC, VAD
- **rtp-core** (0.1.0) - RTP/RTCP with full security stack
- **codec-core** (0.1.0) - Audio codec support
- **sip-transport** (0.1.0) - SIP transport layer

### 🚧 Beta/Active Development
- **session-core-v3** (0.1.0) - Recommended session management
- **registrar-core** (0.1.0) - Registration and presence
- **call-engine** (0.1.0) - Call center proof of concept
- **client-core** (0.1.0) - Client API layer

### 🟡 Basic/Stable
- **users-core** (0.1.0) - User management
- **auth-core** (0.1.0) - Authentication

### ❌ Not Implemented (Planned)
- **b2bua-core** - Back-to-back user agent
- **proxy-core** - SIP proxy server
- **media-server-core** - Standalone media server
- **sbc-core** - Session border controller

See [MISSING_COMPONENTS.md](MISSING_COMPONENTS.md) for roadmap.

---

## Migration Guide

### From session-core-v1 to v3
```toml
# Before
rvoip-session-core = { path = "crates/session-core" }

# After
rvoip-session-core-v3 = { path = "crates/session-core-v3" }
```

See [VERSION_STRATEGY.md](VERSION_STRATEGY.md) for detailed migration steps.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to contribute to this project.

---

## Links

- [Homepage](https://github.com/eisenzopf/rvoip)
- [Documentation](https://docs.rs/rvoip)
- [Issue Tracker](https://github.com/eisenzopf/rvoip/issues)
- [Project Health](PROJECT_HEALTH.md)

---

**Note**: This CHANGELOG was established on January 11, 2026. Previous changes were not formally tracked. Historical information may be incomplete.
