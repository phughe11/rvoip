# RVOIP Session Core v3

> **✅ RECOMMENDED**: This is session-core v3, the recommended version for all new projects. See [VERSION_STRATEGY.md](../../VERSION_STRATEGY.md) for details.

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

## Status

- **Version**: 0.1.0 (Recommended)
- **Stability**: Alpha → Beta (improving rapidly)
- **Recommendation**: ✅ Use for all new projects
- **Support**: Active development

## Overview

The `session-core-v3` library provides state table-based session coordination with the simplified SimplePeer API for the rvoip VoIP stack. This is the latest and recommended version, featuring modern Rust patterns and an intuitive developer experience.

## Quick Start

```rust
use rvoip_session_core_v3::SimplePeer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a peer
    let mut alice = SimplePeer::new("alice").await?;
    alice.start().await?;
    
    // Make a call
    let call_id = alice.make_call("sip:bob@example.com").await?;
    
    // Wait for answer
    alice.wait_for_answer().await?;
    
    // Exchange audio
    alice.exchange_audio(|audio_data| {
        // Process incoming audio
        println!("Received {} bytes of audio", audio_data.len());
    }).await?;
    
    Ok(())
}
```

## Why v3?

### ✅ SimplePeer API
Intuitive high-level API that handles complexity for you:
- `make_call()` - Initiate calls
- `accept()` - Answer incoming calls  
- `wait_for_*()` - Simple async waiting
- `exchange_audio()` - Handle media streams

### ✅ Complete Feature Set
- Basic call flows (UAC/UAS)
- Hold/Resume
- Blind transfer (both sides)
- Audio streaming
- Event-driven architecture

### ✅ Modern Architecture
- State table-driven (YAML configuration)
- Clean adapter pattern
- Excellent test coverage
- Well-documented

### ✅ Active Development
- New features added regularly
- Bug fixes prioritized
- Community feedback incorporated

## Features

### Completed (Phase 1) ✅
- [x] Basic call flow (make call, accept, hangup)
- [x] Audio streaming and exchange
- [x] Hold/Resume operations
- [x] Blind transfer initiation
- [x] Blind transfer reception
- [x] SimplePeer API

### In Progress (Phase 2) 🚧
- [ ] Registration support
- [ ] Presence/Subscription
- [ ] Enhanced NOTIFY handling
- [ ] Attended transfer

### Planned (Phase 3) 📋
- [ ] Conference support
- [ ] Advanced error recovery
- [ ] Performance optimizations

## Examples

See the [examples/](examples/) directory:
- `api_peer_audio` - Basic peer-to-peer audio call
- `blind_transfer` - Three-party transfer scenario
- `thread_benchmark` - Performance testing

## Architecture

```
SimplePeer API
      ↓
UnifiedCoordinator
      ↓
State Machine (YAML-driven)
      ↓
Adapters (Dialog, Media, Event)
      ↓
dialog-core + media-core
```

## Documentation

- [Architecture Overview](docs/ARCHITECTURE_OVERVIEW.md)
- [SimplePeer Completion Plan](SIMPLEPEER_COMPLETION_PLAN.md)
- [Phase 1 Summary](PHASE1_IMPLEMENTATION_SUMMARY.md)
- [Transfer Implementation](docs/BLIND_TRANSFER_COMPLETE_IMPLEMENTATION.md)

## Migration from v1 or v2

See [VERSION_STRATEGY.md](../../VERSION_STRATEGY.md) for:
- Detailed migration steps
- API comparison
- Code examples
- Timeline

## Contributing

We welcome contributions! v3 is under active development.

**Priority Areas**:
- Registration support (Phase 2)
- Presence implementation
- Additional examples
- Documentation improvements

## Support

**Bug Reports**: [GitHub Issues](https://github.com/eisenzopf/rvoip/issues)  
**Questions**: [GitHub Discussions](https://github.com/eisenzopf/rvoip/discussions)  
**Documentation**: This README and docs/ directory

## Roadmap

- **Q1 2026**: Phase 2 completion (Registration, Presence)
- **Q2 2026**: Phase 3 (Conferences, Advanced features)
- **Q3 2026**: Beta release, production testing
- **Q4 2026**: 1.0 release (becomes official session-core)

---

**Questions?** See [VERSION_STRATEGY.md](../../VERSION_STRATEGY.md) or open a GitHub issue.
