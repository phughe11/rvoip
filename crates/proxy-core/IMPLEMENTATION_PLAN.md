# Proxy Core Implementation Plan

## Goal
Implement a basic SIP Proxy capability in `rvoip-proxy-core`.
Initially targeting **Stateless Proxy** logic for high throughput packet forwarding, but effectively we might use **Transaction Stateful** if we reuse `TransactionManager`.

## Dependencies
- `rvoip-sip-core`: Parsing.
- `rvoip-sip-transport`: Network IO.
- `rvoip-dialog-core`: Optional, for transaction state if needed.

## Architecture

### `ProxyConfig`
Configuration for:
- Listen address/port.
- Record-Route enforcement.
- Domain rules (aliases).

### `ProxyEngine`
The main loop that:
1. Receives SIP messages from `SipTransport`.
2. Validates the message (Max-Forwards, Loop Detection).
3. Applies routing logic (Location Service lookup - *Future*).
4. Adds `Via` header.
5. Forwards to destination.

## Phase 1: Stateless Forwarding
- We will use `SipTransport` to listen.
- `TransactionManager` might be overkill or overhead if we want pure stateless.
- **Decision**: We will use `TransactionManager` for *Consumer* convenience (it handles retransmissions), effectively making this a **Stateful Proxy**. This aligns with `dialog-core` dependency.
- Implementing "Stateless" in Rust with `TransactionManager` is contradictory. I will implement **Stateful Proxy** as it's the standard for RVOIP components.

## Implementation Steps
1.  **Transport Setup**: Initialize `TransactionManager` bound to a port.
2.  **Request Processing**:
    - `handle_request`:
        - Decrement Max-Forwards.
        - Add Record-Route (if configured).
        - Determine Target (User A -> Proxy -> User B).
        - **Critical**: For now, simple logic: "Forward to URI in Request-URI".
3.  **ClientTransaction Creation**:
    - Create a new ClientTransaction to the target.
    - Forward the request.

## Verification
- Start Proxy.
- Send INVITE to Proxy with Request-URI = Target.
- Verify Target receives INVITE.
