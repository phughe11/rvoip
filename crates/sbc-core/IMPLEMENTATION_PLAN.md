# SBC Core Implementation Plan

## Goal
Implement functionality for a Session Border Controller (SBC) to secure the VoIP network.
Primary focus: **Topology Hiding** and **Security Policies**.

## Architecture
`sbc-core` will provide a set of **Processors** that transform SIP messages.
It is designed to be used by `proxy-core` or `b2bua-core` when they act as an edge node.

### Components
1.  **SbcEngine**: Configurable engine applying rules.
2.  **TopologyHider**:
    - `anonymize_via(message)`: Replace internal IPs in Via.
    - `anonymize_contact(message)`: Rewrite Contact header.
    - `strip_headers(message)`: Remove sensitive headers (Server, User-Agent, X-Internal).
3.  **SecurityGuard**:
    - `check_rate_limit(source_ip)`: implementation of rate limiting.
    - `validate_message(message)`: Malformed packet checks.

## Phase 1: Topology Hiding
- Implement `SbcEngine` struct.
- Implement `sanitize_request` method.
- Use `sip-core` to manipulate headers.

## Phase 2: Security (Future)
- Connect to Redis for dynamic rate limiting.
- Blacklist checking.

## Usage Example
```rust
let sbc = SbcEngine::new(config);
let sanitized_req = sbc.process_incoming_request(request);
if let Err(e) = sanitized_req {
    // Block attack
}
```
