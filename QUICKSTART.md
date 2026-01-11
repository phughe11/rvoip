# Quick Start Guide

Get started with RVOIP in 5 minutes! 🚀

---

## Prerequisites

- **Rust**: 1.75 or later ([install](https://rustup.rs/))
- **OS**: Linux, macOS, or Windows
- **Optional**: Docker (for examples)

---

## Installation

### Option 1: Add to Existing Project

```bash
# Add to Cargo.toml
cargo add rvoip
```

Or manually:
```toml
[dependencies]
rvoip = "0.1"
```

### Option 2: Clone and Explore

```bash
git clone https://github.com/eisenzopf/rvoip.git
cd rvoip
cargo build
```

---

## Your First SIP Call (5 minutes)

### 1. Simple P2P Call

Create `main.rs`:

```rust
use rvoip::session_core_v3::SessionManager;
use rvoip::sip_core::Uri;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create session manager
    let manager = SessionManager::new("127.0.0.1:5060").await?;
    
    // Make a call
    let session = manager.invite("sip:bob@127.0.0.1:5061").await?;
    
    println!("Calling Bob...");
    
    // Wait for answer
    session.wait_for_established().await?;
    println!("Call connected!");
    
    // Keep call active for 30 seconds
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    
    // Hang up
    session.bye().await?;
    println!("Call ended.");
    
    Ok(())
}
```

Run it:
```bash
cargo run
```

### 2. Answer Incoming Calls

```rust
use rvoip::session_core_v3::SessionManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = SessionManager::new("127.0.0.1:5061").await?;
    
    println!("Waiting for calls on port 5061...");
    
    loop {
        // Wait for incoming call
        let session = manager.wait_for_invite().await?;
        
        println!("Incoming call from: {}", session.remote_uri());
        
        // Answer automatically
        session.accept().await?;
        println!("Call answered!");
        
        // Handle call...
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        
        session.bye().await?;
    }
}
```

---

## Running Examples

RVOIP comes with ready-to-run examples:

### Peer-to-Peer Call

```bash
# Terminal 1: Start Bob (receiver)
cargo run --example peer_receiver

# Terminal 2: Start Alice (caller)
cargo run --example peer_caller
```

### Audio Streaming

```bash
# Play audio file
cargo run --example audio_player -- input.wav

# Record audio
cargo run --example audio_recorder -- output.wav
```

### Call Center Simulation

```bash
# Start call center
cargo run --example call_center

# Make test call
cargo run --example call_center_client
```

### Explore All Examples

```bash
ls examples/
cat examples/README.md
```

---

## Component Tour

### 🧩 Core Components

**Already have a specific need?**

```rust
// SIP parsing and messages
use rvoip::sip_core::{Request, Response, Header};

// Dialog management
use rvoip::dialog_core::Dialog;

// Media processing
use rvoip::media_core::{MediaSession, Codec};

// RTP streaming
use rvoip::rtp_core::{RtpSession, RtpPacket};

// Session management (recommended: v3)
use rvoip::session_core_v3::SessionManager;
```

### 📚 Quick References

| Component | Purpose | Documentation |
|-----------|---------|---------------|
| `sip-core` | SIP protocol (RFC 3261) | [README](crates/sip-core/README.md) |
| `dialog-core` | Dialog state machines | [README](crates/dialog-core/README.md) |
| `media-core` | Media processing | [README](crates/media-core/README.md) |
| `rtp-core` | RTP/SRTP | [README](crates/rtp-core/README.md) |
| `session-core-v3` | High-level API (use this!) | [README](crates/session-core-v3/README.md) |

---

## Common Patterns

### Pattern 1: Basic Call

```rust
// 1. Create manager
let manager = SessionManager::new("0.0.0.0:5060").await?;

// 2. Make call
let session = manager.invite("sip:user@remote.com").await?;

// 3. Wait for answer
session.wait_for_established().await?;

// 4. Hang up when done
session.bye().await?;
```

### Pattern 2: Receive Calls

```rust
let manager = SessionManager::new("0.0.0.0:5060").await?;

loop {
    let session = manager.wait_for_invite().await?;
    
    // Accept or reject
    if should_accept(&session) {
        session.accept().await?;
    } else {
        session.reject(486).await?; // Busy
    }
}
```

### Pattern 3: Call with Media

```rust
let session = manager.invite("sip:user@remote.com").await?;

// Get media session
let media = session.media_session();

// Send audio
media.send_audio(&audio_data).await?;

// Receive audio
let received = media.receive_audio().await?;
```

### Pattern 4: Call Transfer

```rust
// Blind transfer
session.blind_transfer("sip:target@example.com").await?;

// Attended transfer
let consult_session = manager.invite("sip:target@example.com").await?;
consult_session.wait_for_established().await?;
session.attended_transfer(consult_session).await?;
```

---

## Development Setup

### Build Everything

```bash
cargo build --all
```

### Run Tests

```bash
# All tests
cargo test --all

# Specific component
cargo test -p sip-core

# With output
cargo test -- --nocapture
```

### Run with Logging

```bash
RUST_LOG=debug cargo run --example peer_caller
```

### Check Code Quality

```bash
# Linting
cargo clippy --all

# Formatting
cargo fmt --all

# Documentation
cargo doc --no-deps --open
```

---

## Configuration

### Basic Configuration

```rust
use rvoip::session_core_v3::SessionConfig;

let config = SessionConfig::builder()
    .bind_address("0.0.0.0:5060")
    .public_address("203.0.113.1:5060") // Your public IP
    .user_agent("MyApp/1.0")
    .build();

let manager = SessionManager::with_config(config).await?;
```

### Advanced Configuration

```rust
let config = SessionConfig::builder()
    .bind_address("0.0.0.0:5060")
    .public_address("203.0.113.1:5060")
    .user_agent("MyApp/1.0")
    .enable_srtp(true)
    .enable_ice(true)
    .session_timeout(3600) // 1 hour
    .codecs(vec![Codec::PCMU, Codec::Opus])
    .build();
```

---

## Troubleshooting

### Issue: "Address already in use"

```bash
# Check what's using port 5060
lsof -i :5060

# Use a different port
let manager = SessionManager::new("0.0.0.0:5061").await?;
```

### Issue: "Connection timeout"

- Check firewall settings
- Verify network connectivity
- Use correct IP address (not 127.0.0.1 for remote)
- Enable debug logging: `RUST_LOG=debug`

### Issue: "No audio"

- Verify codec support
- Check NAT/firewall for RTP ports
- Ensure media session is properly established
- Use RTP debug: `RUST_LOG=rtp_core=debug`

### Issue: "Build errors"

```bash
# Update Rust
rustup update

# Clean build
cargo clean
cargo build

# Check version
rustc --version  # Should be 1.75+
```

---

## Next Steps

### Learning Path

1. ✅ **You are here**: Quick Start
2. 📖 **Read**: [Architecture Overview](LIBRARY_DESIGN_ARCHITECTURE.md)
3. 🎯 **Try**: Examples in `examples/`
4. 📚 **Study**: Component READMEs in `crates/*/README.md`
5. 🧪 **Experiment**: Build your own application
6. 🤝 **Contribute**: See [CONTRIBUTING.md](CONTRIBUTING.md)

### Recommended Reading

**For beginners**:
- [SIP Basics](docs/tutorials/sip-basics.md) - Understanding SIP protocol
- [Session Management](crates/session-core-v3/README.md) - High-level API guide
- [Examples Guide](examples/README.md) - Example walkthroughs

**For advanced users**:
- [Architecture](LIBRARY_DESIGN_ARCHITECTURE.md) - System design
- [API Guide](crates/session-core-v3/API_GUIDE.md) - Detailed API docs
- [RTP Processing](crates/rtp-core/README.md) - Media streaming

**For contributors**:
- [Contributing Guide](CONTRIBUTING.md) - How to contribute
- [Testing Strategy](TESTING_STRATEGY.md) - Testing guidelines
- [Roadmap](ROADMAP.md) - Future plans

---

## Resources

### Documentation
- [Project README](README.md) - Overview
- [Component Docs](crates/) - Per-component documentation
- API Reference: `cargo doc --open`

### Examples
- [Examples Directory](examples/)
- [Use Cases](examples/USE_CASES.md)
- [Tutorial Book](tutorial/) - Step-by-step guides

### Community
- [GitHub Issues](https://github.com/eisenzopf/rvoip/issues) - Bug reports and features
- [Discussions](https://github.com/eisenzopf/rvoip/discussions) - Questions and ideas
- [Contributing](CONTRIBUTING.md) - How to help

### External
- [SIP RFC 3261](https://datatracker.ietf.org/doc/html/rfc3261)
- [RTP RFC 3550](https://datatracker.ietf.org/doc/html/rfc3550)
- [Rust Async Book](https://rust-lang.github.io/async-book/)

---

## Getting Help

### Documentation Issues?

If something in this guide is unclear or incorrect:
- [Open an issue](https://github.com/eisenzopf/rvoip/issues/new?template=documentation.md)
- [Ask in Discussions](https://github.com/eisenzopf/rvoip/discussions)

### Code Issues?

- Check [existing issues](https://github.com/eisenzopf/rvoip/issues)
- Enable debug logging: `RUST_LOG=debug`
- Provide minimal reproduction
- Include version info: `cargo --version`, `rustc --version`

### Want to Contribute?

We'd love your help! See [CONTRIBUTING.md](CONTRIBUTING.md) for:
- How to set up development environment
- Coding standards
- PR process
- Where help is needed

---

**Welcome to RVOIP!** 🎉

Start building your VoIP application today. If you get stuck, we're here to help in [Discussions](https://github.com/eisenzopf/rvoip/discussions).

Happy coding! 📞
