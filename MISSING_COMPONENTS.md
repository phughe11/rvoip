# Missing Components Status and Implementation Plan

**Last Updated**: January 11, 2026  
**Purpose**: Track status of planned but not yet implemented components

---

## Overview

Based on the architecture design documents, the following critical components are planned but not yet implemented. This document serves as a tracking mechanism and prevents confusion about what's available vs. what's documented.

---

## 🔴 Critical Priority Components

### 1. b2bua-core ❌ NOT IMPLEMENTED

**Status**: Not Started  
**Priority**: 🔴 Critical  
**Blocking**: Call center deployments, IVR systems  
**Design Document**: [B2BUA_IMPLEMENTATION_PLAN.md](B2BUA_IMPLEMENTATION_PLAN.md) (744 lines)

**Planned Features**:
- Back-to-back User Agent core functionality
- Dialog pair management (leg-a, leg-b)
- Call interception and recording
- Application handlers (IVR, Queue, Conference)
- Media server client integration

**Why Not Implemented**:
- Requires completion of session-core-v3 stabilization
- Needs media-server-core for media processing
- Complex state management requires careful design

**Estimated Effort**: 4-6 weeks (1 developer)

**Dependencies**:
```rust
// Planned dependencies (not yet in Cargo.toml)
// dialog-core ✅ (ready)
// media-server-core ❌ (not implemented)
// infra-common ✅ (ready)
```

**Status Indicators**:
```
📁 crates/b2bua-core/          ❌ Directory does not exist
📄 Cargo.toml member           ❌ Not in workspace
🧪 Tests                       ❌ None
📖 Implementation code         ❌ 0%
```

---

### 2. proxy-core ❌ NOT IMPLEMENTED

**Status**: Not Started  
**Priority**: 🔴 Critical  
**Blocking**: Enterprise SIP deployments, load balancing  
**Design Document**: Referenced in [CALL_CENTER_REFERENCE_ARCHITECTURE.md](CALL_CENTER_REFERENCE_ARCHITECTURE.md)

**Planned Features**:
- Stateless SIP proxy functionality
- Request routing based on rules
- Load balancing across servers
- Parallel/serial forking
- Route advance on failure

**Why Not Implemented**:
- Lower priority than B2BUA for initial use cases
- Can be partially replaced by external proxies (Kamailio, OpenSIPS)
- Requires mature dialog-core and transaction handling

**Estimated Effort**: 3-4 weeks (1 developer)

**Dependencies**:
```rust
// Planned dependencies
// dialog-core ✅ (ready)
// sip-core ✅ (ready)
// infra-common ✅ (ready)
```

**Status Indicators**:
```
📁 crates/proxy-core/          ❌ Directory does not exist
📄 Cargo.toml member           ❌ Not in workspace
🧪 Tests                       ❌ None
📖 Implementation code         ❌ 0%
```

---

### 3. media-server-core ❌ NOT IMPLEMENTED

**Status**: Not Started  
**Priority**: 🔴 Critical  
**Blocking**: B2BUA media processing, conferences, IVR  
**Design Document**: [MEDIA_SERVER_PLAN.md](MEDIA_SERVER_PLAN.md) (1051 lines)

**Planned Features**:
- Standalone media server (separate from signaling)
- Endpoint pool management
- Mixer engine (N-way audio mixing)
- Recorder engine
- Player engine (IVR playback)
- DTMF detector
- REST/gRPC control API

**Why Not Implemented**:
- Complex media processing architecture
- Requires careful RTP handling and performance optimization
- Needs clear separation from media-core (which is local processing)

**Estimated Effort**: 6-8 weeks (1 developer)

**Dependencies**:
```rust
// Planned dependencies
// media-core ✅ (ready for local processing)
// rtp-core ✅ (ready)
// codec-core ✅ (ready)
// infra-common ✅ (ready)
```

**Architecture Note**:
```
b2bua-core → (API) → media-server-core → rtp-core
              ↑
              └─ REST/gRPC control interface
```

**Status Indicators**:
```
📁 crates/media-server-core/   ❌ Directory does not exist
📄 Cargo.toml member           ❌ Not in workspace
🧪 Tests                       ❌ None
📖 Implementation code         ❌ 0%
```

---

## 🟡 Medium Priority Components

### 4. sbc-core ❌ NOT IMPLEMENTED

**Status**: Not Started  
**Priority**: 🟡 Medium  
**Blocking**: Internet-facing deployments, security  
**Design Document**: Referenced in architecture docs

**Planned Features**:
- Session Border Controller functionality
- Topology hiding (remove internal IPs)
- NAT traversal (Far-end NAT support)
- Protocol normalization
- Rate limiting and DDoS protection
- TLS termination
- Header manipulation

**Why Not Implemented**:
- Can use external SBCs initially (Kamailio with rtpengine, FreeSWITCH)
- Complex NAT traversal and media anchoring
- Security features require careful implementation

**Estimated Effort**: 4-6 weeks (1 developer)

**Dependencies**:
```rust
// Planned dependencies
// b2bua-core ❌ (optional, for B2BUA mode)
// proxy-core ❌ (optional, for proxy mode)
// media-core ✅ (for RTP anchoring)
// sip-transport ✅ (ready)
```

**Status Indicators**:
```
📁 crates/sbc-core/            ❌ Directory does not exist
📄 Cargo.toml member           ❌ Not in workspace
🧪 Tests                       ❌ None
📖 Implementation code         ❌ 0%
```

---

## 🟢 Lower Priority Components

### 5. intermediary-core ⚠️ PLACEHOLDER ONLY

**Status**: Placeholder exists but minimal implementation  
**Priority**: 🟢 Low  
**Design Document**: [crates/intermediary-core/IMPLEMENTATION_PLAN.md](crates/intermediary-core/IMPLEMENTATION_PLAN.md)

**Current State**:
```
📁 crates/intermediary-core/   ✅ Directory exists
📄 Cargo.toml                  ⚠️ Basic structure only
📖 Implementation code         ⚠️ ~10% (skeleton only)
```

**Note**: This may be merged into proxy-core or sbc-core when implemented.

---

## Implementation Roadmap

### Phase 1: Foundation (Current - Q1 2026)
- ✅ Stabilize session-core-v3
- ✅ Complete dialog-core enhancements
- ✅ Finalize media-core and rtp-core

### Phase 2: Media Infrastructure (Q2 2026)
- 🎯 **Implement media-server-core** (6-8 weeks)
  - Endpoint management
  - Basic mixing
  - Recorder/Player engines
  - Control API

### Phase 3: B2BUA (Q2-Q3 2026)
- 🎯 **Implement b2bua-core** (4-6 weeks)
  - Dialog pair management
  - Media server integration
  - Basic IVR support
  - Call queuing foundation

### Phase 4: Proxy and SBC (Q3-Q4 2026)
- 🎯 **Implement proxy-core** (3-4 weeks)
  - Stateless proxy
  - Load balancing
  - Routing engine
- 🎯 **Implement sbc-core** (4-6 weeks)
  - Topology hiding
  - NAT traversal
  - Security features

---

## Workarounds Until Implementation

### For B2BUA functionality:
1. Use call-engine as a basic proof-of-concept
2. Consider external B2BUA (FreeSWITCH, Asterisk) temporarily
3. Implement custom coordination using session-core-v3

### For Proxy functionality:
1. Use external SIP proxy (Kamailio, OpenSIPS, OpenSIPS)
2. Implement basic routing in application layer
3. Use DNS SRV for basic load balancing

### For Media Server:
1. Use peer-to-peer media (direct RTP between endpoints)
2. Consider external media server (Janus, Kurento)
3. Use media-core for local processing (limited to 1-to-1)

### For SBC:
1. Use external SBC (Kamailio + RTPEngine, SEMS)
2. Deploy behind firewall initially
3. Use STUN/TURN services for NAT traversal

---

## How to Track Progress

### GitHub Issues
Each component should have a tracking issue:
- [ ] Issue #XXX: Implement b2bua-core
- [ ] Issue #XXX: Implement media-server-core  
- [ ] Issue #XXX: Implement proxy-core
- [ ] Issue #XXX: Implement sbc-core

### Milestone Tracking
- Milestone: v0.5.0 - Media Infrastructure (media-server-core)
- Milestone: v0.6.0 - B2BUA Implementation (b2bua-core)
- Milestone: v0.7.0 - Proxy and Routing (proxy-core)
- Milestone: v0.8.0 - Security and SBC (sbc-core)
- Milestone: v1.0.0 - Production Ready (all components)

---

## Contributing

Want to help implement these components?

1. **Check the design document** - Each has detailed plans
2. **Discuss on GitHub** - Open an issue to claim a component
3. **Start small** - Begin with the API and tests
4. **Follow patterns** - Use existing crates as reference

### Implementation Guidelines
- Follow the existing crate structure (see dialog-core, media-core)
- Write tests first (TDD approach)
- Document as you go (doc comments)
- Keep dependencies minimal
- Use infra-common for event bus

---

## Questions?

- **"Why so many planned components?"** - Enterprise VoIP requires these building blocks
- **"Can I use RVOIP without these?"** - Yes, for simple P2P or client applications
- **"When will these be done?"** - See roadmap above; contributions welcome!
- **"Are the designs final?"** - Designs are solid but may evolve during implementation

---

**Maintained By**: RVOIP Core Team  
**Last Review**: January 11, 2026  
**Next Review**: March 2026
