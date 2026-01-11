# RVOIP Session Core v2

> **⚠️ Transitional Version**: This is session-core v2, being phased out in favor of v3. For new projects, please use **session-core-v3**. See [VERSION_STRATEGY.md](../../VERSION_STRATEGY.md) for details.

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

## Status

- **Version**: 0.1.0 (Transitional)
- **Stability**: Beta
- **Recommendation**: Use session-core-v3 for new projects
- **Support**: Critical bug fixes only

## Overview

The `session-core-v2` library provides state table-based session coordination for the rvoip VoIP stack. This version introduced the declarative YAML-based state machine architecture that is further refined in v3.

## Why Use v3 Instead?

Session-core-v3 offers:
- ✅ Simplified SimplePeer API
- ✅ Better event handling
- ✅ More complete transfer support
- ✅ Active development and new features
- ✅ Improved documentation

## Migration Path

If you're using v2:

```toml
# Old
[dependencies]
rvoip-session-core-v2 = { path = "crates/session-core-v2" }

# New  
[dependencies]
rvoip-session-core-v3 = { path = "crates/session-core-v3" }
```

See [VERSION_STRATEGY.md](../../VERSION_STRATEGY.md) for detailed migration guide.

## Features (Still Available)

- State table-driven architecture
- YAML configuration
- Dialog and media coordination
- Basic call flows
- Event-driven design

## Documentation

For v2-specific documentation, see:
- [Architecture Overview](docs/ARCHITECTURE_OVERVIEW.md)
- [State Tables](docs/STATE_TABLE_CONFIGURATION.md)
- [Examples](examples/)

## Support

**Bug Reports**: Critical issues only  
**New Features**: Will not be added to v2  
**Questions**: Check v3 documentation first

## Deprecation Timeline

- **Now**: Transitional status
- **March 2026**: Officially deprecated
- **September 2026**: Archived (no updates)

---

**For new projects, use [session-core-v3](../session-core-v3/README.md)**
