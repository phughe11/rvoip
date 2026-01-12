# Change: Implement Audit Fixes (Auth, Registrar, Session, Transport)

## Why
A comprehensive audit revealed critical gaps in the implementation that prevent the system from being production-ready:
1. **Auth-Core**: Code exists but is not exported in `lib.rs`, rendering the crate unusable.
2. **Registrar-Core**: Persistence is missing (only in-memory stub), limiting utility for production.
3. **Session-Core-v3**: Registration logic is missing, blocking its use as a full SIP client replacement.
4. **SIP-Transport**: TLS functionality is documented but unimplemented (stubbed), leading to runtime errors if configured.

## What Changes

### 1. Fix Auth-Core Exports
- Update `crates/auth-core/src/lib.rs` to publicly export `jwt`, `config`, and other modules.
- Ensure the crate is usable by `session-core` and other components.

### 2. Implement Registrar Persistence
- Implement `SqliteStorage` in `crates/registrar-core/src/storage/sqlite.rs`.
- Use `sqlx` for database operations.
- Wire up the storage implementation in `RegistrarService`.

### 3. Add Registration to Session-Core-v3
- Implement `RegistrationManager` in `crates/session-core-v3`.
- Add `register()` method to `SimplePeer` API.
- Handle `REGISTER` requests and authentication challenges.

### 4. Clarify SIP-Transport TLS Status
- Add a clear warning/error log in `TlsTransport` placeholder methods.
- Update documentation to reflect its current "Stub/MVP" status to avoid misleading users.

## Impact
- **Auth-Core**: Becomes a functional component.
- **Registrar**: Gains persistence capability.
- **Session-Core-v3**: Gains feature parity with v1 for client registration.
- **SIP-Transport**: Manages user expectations regarding TLS support.
