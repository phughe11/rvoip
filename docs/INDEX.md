# RVOIP Documentation Index

**Version**: 0.1.26 (Alpha)  
**Last Updated**: January 11, 2026

Welcome to the RVOIP documentation! This index helps you find the right documentation for your needs.

---

## 🚀 Getting Started

| Document | Description | Audience |
|----------|-------------|----------|
| [README.md](../README.md) | Project overview and introduction | Everyone |
| [QUICKSTART.md](../QUICKSTART.md) | 5-minute guide to your first call | New users |
| [Examples Guide](../examples/README.md) | Ready-to-run code examples | Developers |

**Start here**: [QUICKSTART.md](../QUICKSTART.md)

---

## 📚 Core Documentation

### Architecture & Design

| Document | Description | Audience |
|----------|-------------|----------|
| [LIBRARY_DESIGN_ARCHITECTURE.md](../LIBRARY_DESIGN_ARCHITECTURE.md) | System architecture and layering | Architects, contributors |
| [ROADMAP.md](../ROADMAP.md) | Development roadmap and milestones | Everyone |
| [VERSION_STRATEGY.md](../VERSION_STRATEGY.md) | Version policy for session-core | Users, contributors |
| [PROJECT_HEALTH.md](../PROJECT_HEALTH.md) | Component status dashboard | Maintainers |

### Project Governance

| Document | Description | Audience |
|----------|-------------|----------|
| [CONTRIBUTING.md](../CONTRIBUTING.md) | Contribution guidelines | Contributors |
| [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md) | Community standards | Everyone |
| [SECURITY.md](../SECURITY.md) | Security policy and reporting | Security researchers |
| [CHANGELOG.md](../CHANGELOG.md) | Version history | Users |
| [LICENSE](../LICENSE) | Apache 2.0 license | Legal |

### Development

| Document | Description | Audience |
|----------|-------------|----------|
| [TESTING_STRATEGY.md](../TESTING_STRATEGY.md) | Testing approach and coverage | Contributors |
| [Architecture Decision Records](adr/) | Design decisions and rationale | Architects |
| [GitHub Templates]../.github/) | Issue and PR templates | Contributors |

---

## 🧩 Component Documentation

### Core Components (Production-Ready)

| Component | README | Purpose | Completeness |
|-----------|--------|---------|--------------|
| **sip-core** | [README](../crates/sip-core/README.md) | SIP protocol (RFC 3261) | 95% |
| **dialog-core** | [README](../crates/dialog-core/README.md) | Dialog state machines | 90% |
| **media-core** | [README](../crates/media-core/README.md) | Media processing | 90% |
| **rtp-core** | [README](../crates/rtp-core/README.md) | RTP/SRTP streaming | 95% |

### Session Management (Choose Version)

| Component | README | Status | Recommendation |
|-----------|--------|--------|----------------|
| **session-core-v3** | [README](../crates/session-core-v3/README.md) | Transitional | ✅ **Use This** |
| session-core-v2 | [README](../crates/session-core-v2/README.md) | Maintenance | ⚠️ Migrate to v3 |
| session-core | [README](../crates/session-core/README.md) | Legacy | ⛔ Deprecated |

See [VERSION_STRATEGY.md](../VERSION_STRATEGY.md) for details.

### Supporting Components

| Component | README | Purpose |
|-----------|--------|---------|
| **audio-core** | [README](../crates/audio-core/README.md) | Audio processing |
| **codec-core** | [README](../crates/codec-core/README.md) | Audio codecs |
| **auth-core** | [README](../crates/auth-core/README.md) | Authentication |
| **registrar-core** | [README](../crates/registrar-core/README.md) | Registration server |
| **call-engine** | [README](../crates/call-engine/README.md) | Call center logic |
| **client-core** | [README](../crates/client-core/README.md) | SIP client library |
| **infra-common** | [README](../crates/infra-common/README.md) | Shared infrastructure |

### Planned Components

| Component | Status | Documentation | Timeline |
|-----------|--------|---------------|----------|
| **b2bua-core** | Planned | [IMPLEMENTATION_PLAN.md](../B2BUA_IMPLEMENTATION_PLAN.md) | Q2-Q3 2026 |
| **media-server-core** | Planned | [MEDIA_SERVER_PLAN.md](../MEDIA_SERVER_PLAN.md) | Q2 2026 |
| **proxy-core** | Planned | [MISSING_COMPONENTS.md](../MISSING_COMPONENTS.md) | Q3 2026 |
| **sbc-core** | Planned | [MISSING_COMPONENTS.md](../MISSING_COMPONENTS.md) | Q3-Q4 2026 |

See [MISSING_COMPONENTS.md](../MISSING_COMPONENTS.md) for full details.

---

## 📖 Feature Documentation

### Call Features

| Feature | Documentation | Status |
|---------|---------------|--------|
| Basic Calls | [session-core-v3 README](../crates/session-core-v3/README.md) | ✅ Complete |
| Hold/Resume | [session-core-v3 README](../crates/session-core-v3/README.md) | ✅ Complete |
| Blind Transfer | [call_transfer_implementation_plan_v2.md](../call_transfer_implementation_plan_v2.md) | 🚧 90% |
| Attended Transfer | [call_transfer_implementation_plan_v2.md](../call_transfer_implementation_plan_v2.md) | 🚧 30% |
| Registration | [session_core_v2_register_presence_implementation_plan.md](../crates/session_core_v2_register_presence_implementation_plan.md) | 🚧 70% |
| Presence | [session_core_v2_register_presence_implementation_plan.md](../crates/session_core_v2_register_presence_implementation_plan.md) | 🚧 60% |

### Media Features

| Feature | Documentation | Status |
|---------|---------------|--------|
| RTP | [rtp-core README](../crates/rtp-core/README.md) | ✅ Complete |
| SRTP | [rtp-core README](../crates/rtp-core/README.md) | ✅ Complete |
| DTLS-SRTP | [rtp-core README](../crates/rtp-core/README.md) | ✅ Complete |
| Audio Codecs | [codec-core README](../crates/codec-core/README.md) | ✅ Complete |
| Media Server | [MEDIA_SERVER_PLAN.md](../MEDIA_SERVER_PLAN.md) | 📋 Planned |

### Enterprise Features

| Feature | Documentation | Status |
|---------|---------------|--------|
| B2BUA | [B2BUA_IMPLEMENTATION_PLAN.md](../B2BUA_IMPLEMENTATION_PLAN.md) | 📋 Planned |
| Proxy | [MISSING_COMPONENTS.md](../MISSING_COMPONENTS.md) | 📋 Planned |
| SBC | [MISSING_COMPONENTS.md](../MISSING_COMPONENTS.md) | 📋 Planned |
| Call Center | [CALL_CENTER_REFERENCE_ARCHITECTURE.md](../CALL_CENTER_REFERENCE_ARCHITECTURE.md) | 🚧 In Progress |

---

## 🎓 Tutorials & Guides

### Official Tutorial Book

Location: [tutorial/](../tutorial/)

**Topics**:
1. SIP Basics
2. Making Your First Call
3. Handling Incoming Calls
4. Media Processing
5. Advanced Features

**Build the book**:
```bash
cd tutorial
mdbook serve
```

### Example Use Cases

| Use Case | Documentation | Code |
|----------|---------------|------|
| Peer-to-Peer Calls | [examples/peer-to-peer/](../examples/peer-to-peer/) | Examples |
| Audio Streaming | [examples/audio-streaming/](../examples/audio-streaming/) | Examples |
| Call Center | [examples/call-center/](../examples/call-center/) | Examples |
| SIP Client | [examples/sip-client-p2p/](../examples/sip-client-p2p/) | Examples |

See [examples/USE_CASES.md](../examples/USE_CASES.md) for full list.

---

## 🔧 API Documentation

### Generated API Docs

```bash
# Build and open API documentation
cargo doc --no-deps --open
```

### Key API Guides

| Component | API Guide | Level |
|-----------|-----------|-------|
| session-core-v3 | [API_GUIDE.md](../crates/session-core-v3/API_GUIDE.md) | Beginner |
| session-core-v3 | [API_DESIGN.md](../crates/session-core-v3/API_DESIGN.md) | Advanced |
| session-core-v3 | [COOKBOOK.md](../crates/session-core-v3/COOKBOOK.md) | Practical |
| dialog-core | [README.md](../crates/dialog-core/README.md) | Intermediate |

---

## 📋 Planning Documents

### Implementation Plans

| Plan | Description | Status |
|------|-------------|--------|
| [B2BUA_IMPLEMENTATION_PLAN.md](../B2BUA_IMPLEMENTATION_PLAN.md) | B2BUA architecture | Planned |
| [MEDIA_SERVER_PLAN.md](../MEDIA_SERVER_PLAN.md) | Media server design | Planned |
| [call_transfer_implementation_plan_v2.md](../call_transfer_implementation_plan_v2.md) | Call transfer features | In Progress |
| [configuration_screen_implementation_plan.md](../configuration_screen_implementation_plan.md) | Configuration UI | Future |
| [SESSION_CORE_CLEANUP_PLAN.md](../SESSION_CORE_CLEANUP_PLAN.md) | Code cleanup | Ongoing |

### Reference Architecture

| Document | Description | Audience |
|----------|-------------|----------|
| [CALL_CENTER_REFERENCE_ARCHITECTURE.md](../CALL_CENTER_REFERENCE_ARCHITECTURE.md) | Call center design patterns | System architects |
| [LIBRARY_DESIGN_ARCHITECTURE.md](../LIBRARY_DESIGN_ARCHITECTURE.md) | Overall library structure | Developers |

### Comparison & Analysis

| Document | Description |
|----------|-------------|
| [SESSION_CORE_COMPARISON.md](../SESSION_CORE_COMPARISON.md) | Comparing v1/v2/v3 |
| [sip_message_compatibility_report.md](../sip_message_compatibility_report.md) | SIP compatibility |

---

## 🛠️ Development Workflows

### Setting Up

1. [QUICKSTART.md](../QUICKSTART.md) - Initial setup
2. [CONTRIBUTING.md](../CONTRIBUTING.md) - Development environment
3. [TESTING_STRATEGY.md](../TESTING_STRATEGY.md) - Running tests

### Making Changes

1. Read [CONTRIBUTING.md](../CONTRIBUTING.md) - Contribution process
2. Check [ROADMAP.md](../ROADMAP.md) - Alignment with plans
3. Review [ADRs](adr/) - Understanding decisions
4. Follow [TESTING_STRATEGY.md](../TESTING_STRATEGY.md) - Adding tests

### Reviewing Code

1. [GitHub PR Template](../.github/PULL_REQUEST_TEMPLATE.md) - Checklist
2. [TESTING_STRATEGY.md](../TESTING_STRATEGY.md) - Test requirements
3. Component README - Specific guidelines

---

## 🐛 Issue Templates

Located in [.github/ISSUE_TEMPLATE/](../.github/ISSUE_TEMPLATE/):

- [Bug Report](../.github/ISSUE_TEMPLATE/bug_report.md)
- [Feature Request](../.github/ISSUE_TEMPLATE/feature_request.md)
- [Question](../.github/ISSUE_TEMPLATE/question.md)

---

## 📊 Status & Health

### Project Health

- [PROJECT_HEALTH.md](../PROJECT_HEALTH.md) - Current status dashboard
- [ROADMAP.md](../ROADMAP.md) - Future plans
- [CHANGELOG.md](../CHANGELOG.md) - What's changed
- [MISSING_COMPONENTS.md](../MISSING_COMPONENTS.md) - What's missing

### Quality Metrics

- Test Coverage: See [TESTING_STRATEGY.md](../TESTING_STRATEGY.md)
- Component Completeness: See [PROJECT_HEALTH.md](../PROJECT_HEALTH.md)
- Code Quality: Run `cargo clippy --all`

---

## 🔍 Finding Documentation

### By Topic

**Want to understand SIP?**
→ [sip-core README](../crates/sip-core/README.md)

**Want to make calls?**
→ [QUICKSTART.md](../QUICKSTART.md) + [session-core-v3 README](../crates/session-core-v3/README.md)

**Want to process media?**
→ [media-core README](../crates/media-core/README.md) + [rtp-core README](../crates/rtp-core/README.md)

**Want to build a call center?**
→ [CALL_CENTER_REFERENCE_ARCHITECTURE.md](../CALL_CENTER_REFERENCE_ARCHITECTURE.md)

**Want to contribute?**
→ [CONTRIBUTING.md](../CONTRIBUTING.md) + [ROADMAP.md](../ROADMAP.md)

### By Role

**User** (using RVOIP in your app):
1. [QUICKSTART.md](../QUICKSTART.md)
2. [examples/](../examples/)
3. [session-core-v3 README](../crates/session-core-v3/README.md)
4. Component READMEs

**Contributor** (fixing bugs, adding features):
1. [CONTRIBUTING.md](../CONTRIBUTING.md)
2. [TESTING_STRATEGY.md](../TESTING_STRATEGY.md)
3. [ROADMAP.md](../ROADMAP.md)
4. [ADRs](adr/)

**Maintainer** (project management):
1. [PROJECT_HEALTH.md](../PROJECT_HEALTH.md)
2. [ROADMAP.md](../ROADMAP.md)
3. [CHANGELOG.md](../CHANGELOG.md)
4. [GitHub Templates](../.github/)

**Architect** (system design):
1. [LIBRARY_DESIGN_ARCHITECTURE.md](../LIBRARY_DESIGN_ARCHITECTURE.md)
2. [ADRs](adr/)
3. [Reference architectures](../CALL_CENTER_REFERENCE_ARCHITECTURE.md)
4. Component designs

---

## 🆘 Getting Help

### Documentation Issues

- **Missing docs?** [Open a documentation issue](https://github.com/eisenzopf/rvoip/issues/new?template=documentation.md)
- **Unclear docs?** [Ask in Discussions](https://github.com/eisenzopf/rvoip/discussions)
- **Found errors?** [Submit a PR](../CONTRIBUTING.md)

### Code Issues

- **Bug?** [Bug report template](../.github/ISSUE_TEMPLATE/bug_report.md)
- **Feature request?** [Feature request template](../.github/ISSUE_TEMPLATE/feature_request.md)
- **Question?** [Question template](../.github/ISSUE_TEMPLATE/question.md)

---

## 📝 Contributing to Docs

Documentation improvements are always welcome! See:
- [CONTRIBUTING.md](../CONTRIBUTING.md) - General guidelines
- Documentation issues on GitHub
- This index for structure

**Common doc improvements**:
- Fix typos and errors
- Add examples
- Improve clarity
- Update outdated info
- Add missing docs

---

## 🔄 Keeping Docs Updated

This index is maintained by the RVOIP core team and reviewed quarterly.

**Last Review**: January 11, 2026  
**Next Review**: April 2026

Found something outdated? [Let us know!](https://github.com/eisenzopf/rvoip/issues)

---

**Happy reading!** 📚

For questions about documentation, ask in [GitHub Discussions](https://github.com/eisenzopf/rvoip/discussions).
