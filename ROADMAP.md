# RVOIP Development Roadmap

**Version**: 1.0  
**Last Updated**: January 11, 2026  
**Status**: Active Planning Document

---

## 🎯 Vision

Build a production-ready, 100% Rust VoIP stack that can serve as an alternative to FreeSWITCH and Asterisk for modern deployments, with a focus on:
- Memory safety and security
- Modern async architecture
- Clean, maintainable code
- Comprehensive RFC compliance
- Enterprise-grade features

---

## 📊 Current Status (v0.1.26)

```
┌─────────────────────────────────────────────┐
│  Overall Progress: ████████░░░░░░░░░  40%   │
├─────────────────────────────────────────────┤
│  Core Protocol:    ████████████████░░  80%   │
│  Session Mgmt:     ████████████░░░░░░  60%   │
│  Enterprise:       ██░░░░░░░░░░░░░░░░  10%   │
│  Production Ready: ███░░░░░░░░░░░░░░░  15%   │
└─────────────────────────────────────────────┘
```

**Release Stage**: Alpha  
**Recommended for**: Experiments, prototypes, P2P applications  
**Not Ready for**: Production call centers, enterprise PBX

---

## 🗺️ Release Roadmap

### Q1 2026 (Current) - Foundation Solidification

**Target**: v0.2.0 (February 2026)

**Goals**:
- ✅ Establish project governance and documentation
- ✅ Version strategy for session-core
- ✅ CI/CD infrastructure
- 🚧 Stabilize session-core-v3
- 🚧 Complete transfer functionality
- 🚧 Improve test coverage (→ 70%)

**Deliverables**:
- [x] VERSION_STRATEGY.md
- [x] CONTRIBUTING.md
- [x] CI/CD templates
- [ ] session-core-v3 stable release
- [ ] 70%+ test coverage
- [ ] Performance benchmarks

**Status**: 60% complete

---

### Q2 2026 - Media Infrastructure

**Target**: v0.3.0 (May 2026)

**Goals**:
- Implement media-server-core
- Improve media processing
- Add advanced audio features
- Registration and presence completion

**Deliverables**:
- [ ] media-server-core (MVP)
  - Endpoint pool management
  - Basic mixing engine
  - Recorder/Player engines
  - REST/gRPC control API
- [ ] session-core-v3 registration support
- [ ] session-core-v3 presence support
- [ ] Enhanced media quality monitoring
- [ ] Media server examples

**Estimated Effort**: 2-3 months (1-2 developers)

---

### Q2-Q3 2026 - B2BUA Implementation

**Target**: v0.4.0 (August 2026)

**Goals**:
- Implement b2bua-core
- Enable call center use cases
- IVR foundation
- Call queuing

**Deliverables**:
- [ ] b2bua-core complete
  - Dialog pair management
  - Media server integration
  - Call interception/recording
  - Application handler framework
- [ ] Basic IVR support
- [ ] Call queue enhancements
- [ ] B2BUA examples
- [ ] Integration tests

**Estimated Effort**: 2-3 months (1-2 developers)

---

### Q3 2026 - Proxy and Routing

**Target**: v0.5.0 (October 2026)

**Goals**:
- Implement proxy-core
- Advanced routing
- Load balancing
- Failover support

**Deliverables**:
- [ ] proxy-core implementation
  - Stateless proxy mode
  - Transaction-stateful mode
  - Request routing engine
  - Load balancing algorithms
- [ ] Routing rules engine
- [ ] Parallel/serial forking
- [ ] Proxy examples
- [ ] Performance testing

**Estimated Effort**: 1-2 months (1-2 developers)

---

### Q3-Q4 2026 - Security and SBC

**Target**: v0.6.0 (December 2026)

**Goals**:
- Implement sbc-core
- Enhanced security
- NAT traversal
- Production hardening

**Deliverables**:
- [ ] sbc-core implementation
  - Topology hiding
  - NAT traversal (STUN/TURN/ICE)
  - Rate limiting and DDoS protection
  - TLS termination
  - Header manipulation
- [ ] Security audit
- [ ] Production deployment guides
- [ ] SBC examples

**Estimated Effort**: 2-3 months (1-2 developers)

---

### Q4 2026 - Beta and Testing

**Target**: v0.9.0 (Beta - December 2026)

**Goals**:
- Complete feature freeze
- Comprehensive testing
- Beta deployments
- Documentation completion

**Activities**:
- [ ] Feature freeze (no new major features)
- [ ] Beta testing program
  - Small production deployments
  - Performance testing
  - Load testing
  - Security testing
- [ ] Documentation completion
  - API documentation
  - Deployment guides
  - Best practices
  - Migration guides
- [ ] Bug fixing and stabilization
- [ ] Performance optimization

**Success Criteria**:
- 3+ beta deployments running
- 80%+ test coverage
- No critical bugs
- All documentation complete
- Performance benchmarks published

---

### Q1 2027 - Production Release

**Target**: v1.0.0 (March 2027)

**Goals**:
- Production-ready release
- Long-term support commitment
- Enterprise features

**Deliverables**:
- [ ] v1.0.0 release
- [ ] LTS (Long Term Support) commitment
- [ ] Enterprise support options
- [ ] Professional services
- [ ] Training materials
- [ ] Certification program

**Success Criteria**:
- 10+ production deployments
- Zero critical security issues
- 85%+ test coverage
- Complete API documentation
- 3rd party security audit completed
- Performance comparable to FreeSWITCH

---

## 🎯 Feature Roadmap

### Core Protocol Stack

| Feature | Q1 | Q2 | Q3 | Q4 | Status |
|---------|----|----|----|----|--------|
| SIP Core (RFC 3261) | ✅ | - | - | - | Complete |
| Dialog Management | ✅ | 🚧 | - | - | 90% |
| Media Processing | ✅ | 🚧 | - | - | 90% |
| RTP/SRTP | ✅ | - | - | - | Complete |
| DTLS-SRTP | ✅ | - | - | - | Complete |

### Session Management

| Feature | Q1 | Q2 | Q3 | Q4 | Status |
|---------|----|----|----|----|--------|
| Basic Calls | ✅ | - | - | - | Complete |
| Hold/Resume | ✅ | - | - | - | Complete |
| Blind Transfer | 🚧 | - | - | - | 90% |
| Attended Transfer | - | 🚧 | - | - | 30% |
| Registration | 🚧 | ✅ | - | - | 70% |
| Presence | - | ✅ | - | - | 60% |

### Enterprise Features

| Feature | Q1 | Q2 | Q3 | Q4 | Status |
|---------|----|----|----|----|--------|
| B2BUA | - | - | ✅ | - | Planned |
| Media Server | - | ✅ | - | - | Planned |
| Proxy | - | - | ✅ | - | Planned |
| SBC | - | - | - | ✅ | Planned |
| IVR | - | - | 🚧 | - | Planned |
| Call Recording | - | - | 🚧 | - | Planned |
| Conferences | - | - | - | 🚧 | Planned |

### Operations

| Feature | Q1 | Q2 | Q3 | Q4 | Status |
|---------|----|----|----|----|--------|
| Monitoring | - | 🚧 | ✅ | - | Planned |
| Metrics | - | 🚧 | ✅ | - | Planned |
| Health Checks | - | - | ✅ | - | Planned |
| Admin API | - | - | 🚧 | ✅ | Planned |
| Web UI | - | - | - | 🚧 | Future |

**Legend**: ✅ Complete | 🚧 In Progress | - Not Started

---

## 🔢 Version Numbering

Following [Semantic Versioning](https://semver.org/):

- **0.x.y** - Pre-release (current)
  - Breaking changes allowed
  - APIs will evolve
  - Not production-ready

- **1.0.0** - First stable release
  - API stability guarantee
  - Production-ready
  - LTS support

- **1.x.y** - Stable releases
  - Patch: Bug fixes only
  - Minor: New features, backwards compatible
  - Major: Breaking changes

---

## 📋 Milestone Definitions

### Alpha (Current - v0.1.x to v0.6.x)
- Core functionality implemented
- APIs evolving
- For experimentation and prototypes
- Breaking changes expected

### Beta (v0.9.x)
- All planned features implemented
- API mostly stable
- For testing and feedback
- Minor breaking changes possible

### Release Candidate (v0.9.5+)
- Feature complete
- No known critical bugs
- API frozen
- Only bug fixes

### Stable (v1.0.0+)
- Production-ready
- API stable
- Long-term support
- Semantic versioning enforced

---

## 🎓 Learning from Competition

### FreeSWITCH Lessons
- ✅ Modular architecture (apply)
- ✅ Comprehensive docs (apply)
- ⚠️ C complexity (avoid with Rust)
- ⚠️ Memory management issues (Rust solves)

### Asterisk Lessons
- ✅ Extensive features (target over time)
- ✅ Community support (build community)
- ⚠️ Legacy code burden (start fresh)
- ⚠️ Configuration complexity (simplify)

### Modern Systems (Janus, Kamailio)
- ✅ Focused scope (learn from)
- ✅ Good performance (match or exceed)
- ✅ WebRTC first (support from start)

---

## 🤝 Community Growth

### Q1 2026
- [ ] Establish contributing guidelines ✅
- [ ] Set up CI/CD
- [ ] Regular releases (monthly)
- [ ] Active issue triage

### Q2-Q3 2026
- [ ] Community calls (monthly)
- [ ] Tutorial series
- [ ] Conference talks
- [ ] Blog posts

### Q4 2026+
- [ ] User conference
- [ ] Training programs
- [ ] Certification
- [ ] Commercial support options

---

## 💼 Enterprise Readiness

### Technical Requirements
- [x] Memory safety (Rust provides)
- [x] Core protocol compliance
- [ ] High availability (Q3 2026)
- [ ] Clustering (Q3 2026)
- [ ] Monitoring integration (Q3 2026)
- [ ] Security audit (Q4 2026)

### Business Requirements
- [ ] LTS commitment (v1.0)
- [ ] Commercial support (v1.0)
- [ ] SLA options (v1.0)
- [ ] Training programs (v1.0)
- [ ] Professional services (v1.0)

---

## 🔄 Feedback Loops

### User Feedback
- GitHub Issues and Discussions
- Community calls
- Beta testing program
- Production deployment case studies

### Technical Feedback
- Performance benchmarks
- Security audits
- Code reviews
- Competitor analysis

### Market Feedback
- User surveys
- Feature requests
- Use case analysis
- Industry trends

---

## 🚀 Success Metrics

### v0.2.0 (Q1)
- [ ] 70% test coverage
- [ ] CI/CD operational
- [ ] 5+ external contributors
- [ ] 100+ GitHub stars

### v0.5.0 (Q3)
- [ ] 80% test coverage
- [ ] All critical components implemented
- [ ] 3+ beta deployments
- [ ] 500+ GitHub stars

### v1.0.0 (Q1 2027)
- [ ] 85% test coverage
- [ ] 10+ production deployments
- [ ] 1000+ GitHub stars
- [ ] Community self-sustaining

---

## ❓ Risk Management

### Technical Risks
- **Complex implementations** → Start simple, iterate
- **Performance issues** → Early benchmarking
- **Security vulnerabilities** → Regular audits

### Resource Risks
- **Limited contributors** → Active community building
- **Time constraints** → Prioritize ruthlessly
- **Scope creep** → Strict milestone discipline

### Market Risks
- **Low adoption** → Focus on documentation and examples
- **Competition** → Differentiate with Rust advantages
- **Changing requirements** → Flexible architecture

---

## 📞 Questions?

- Discuss the roadmap in [GitHub Discussions](https://github.com/eisenzopf/rvoip/discussions)
- Propose changes via Issues or Pull Requests
- See [CONTRIBUTING.md](CONTRIBUTING.md) for how to help

---

**This roadmap is a living document and will be updated quarterly.**

**Last Review**: January 11, 2026  
**Next Review**: April 2026  
**Maintained By**: RVOIP Core Team
