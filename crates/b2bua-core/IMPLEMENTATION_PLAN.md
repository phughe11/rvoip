# B2BUA Core Implementation Plan

## Goal
Implement the foundational `b2bua-core` library as described in `B2BUA_IMPLEMENTATION_PLAN.md`.
This implies bypassing `session-core` and working directly with `dialog-core` and `sip-core`.

## Phase 1: Core Structures (Current Task)
1.  **State Logic**: define `B2buaState` enum.
2.  **Core Engine**: `B2buaEngine` struct managing a `DashMap` of calls.
3.  **Dialog Abstraction**: wrappers around `dialog-core` concepts for Leg A and Leg B.

## Proposed Code Structure

### `crates/b2bua-core/src/state.rs`
```rust
pub enum B2buaState {
    Idle,
    IncomingCall,
    Bridged,
    Terminated
}
```

### `crates/b2bua-core/src/core.rs`
```rust
pub struct B2buaEngine {
    // Dialog manager handle
    // Event bus
    // Active calls map
}
```

### `crates/b2bua-core/src/call.rs`
```rust
pub struct B2buaCall {
    pub id: String,
    pub leg_a: DialogHandle,
    pub leg_b: Option<DialogHandle>,
    pub state: B2buaState,
}
```

## Dependencies
Need to ensure `b2bua-core` depends on:
- `rvoip-dialog-core` (for `Dialog`, `DialogManager`)
- `rvoip-sip-core` (for SIP parsing)
- `rvoip-infra-common` (for `EventBus`)
- `dashmap`
- `tokio`

## Verification
- Compile check.
- Basic test where an `Engine` struct can be instantiated.
