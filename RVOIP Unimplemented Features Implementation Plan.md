# RVOIP Unimplemented Features Implementation Plan
## Overview
Implement all unimplemented features identified in the codebase gap analysis using OpenSpec-guided development.
## Implementation Priority
### Phase 1: High Priority - Core Infrastructure (Blocking Production)
**1.1 auth-core - Token Validation & OAuth2 (Days 1-4)**
Current: ~82 lines (only type definitions)
* TokenValidationService trait implementation
* JWT validation with JWKS support
* OAuth2 provider integration (Google, Keycloak)
* Token caching with TTL
* Users-core integration
**1.2 proxy-core - SIP Proxy Enhancement (Days 5-7)**
Current: ~312 lines (basic framework)
* Complete DNS SRV resolution (RFC 3263)
* Proper Via header handling
* Record-Route support
* Loop detection
* Failover and load balancing
**1.3 sbc-core - Session Border Controller (Days 8-10)**
Current: ~213 lines (basic framework)
* Complete topology hiding (Via/Contact rewriting)
* Media anchoring
* NAT traversal integration
* Security policies
* Blacklist/whitelist
**1.4 media-server-core - Media Server (Days 11-14)**
Current: ~351 lines (basic framework)
* Complete IVR system with menu navigation
* Recording system (to WAV/raw)
* Text-to-Speech integration hooks
* DTMF generation (RFC 2833)
* Complete conference mixing
### Phase 2: Medium Priority - Feature Enhancement (Days 15-25)
**2.1 session-core-v3 - SimplePeer Completion**
* Phase 2: Registration flows
* Phase 3: Presence support
* Attended transfer
**2.2 call-engine - Call Center Features**
* IVR integration
* Call recording
* Skills-based routing
* Real-time statistics
**2.3 sip-client - Client Features**
* Multi-call handling
* Call conferencing
* MWI (Message Waiting Indicator)
* BLF (Busy Lamp Field)
**2.4 media-core - Media Processing**
* Recording framework completion
* DTMF detection/generation
* Announcement engine
### Phase 3: Low Priority - Optimization (Days 26-30)
**3.1 Performance Optimizations**
* sip-core: Zero-copy parsing, SIMD
* rtp-core: FEC, hardware acceleration
* sip-transport: Connection pooling, RFC 3263
## OpenSpec Workflow Per Feature
1. Create change directory: `openspec/changes/add-<feature>/`
2. Write `proposal.md` (why, what, impact)
3. Create `specs/<capability>/spec.md` with ADDED Requirements
4. Write `tasks.md` (implementation checklist)
5. Validate: `openspec validate <change-id> --strict`
6. Implement after approval
7. Archive when complete
## Starting Point
Begin with **auth-core** as it's the foundation for secure authentication across the entire stack.