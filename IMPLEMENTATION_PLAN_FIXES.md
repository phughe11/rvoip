# Audit Fix Implementation Plan

## Goal
Address the critical issues identified in the system audit:
1.  **Session Core Versioning**: Clarify confusion by marking v1/v2 as legacy and v3 as recommended.
2.  **Missing Components**: Scaffold `b2bua-core`, `proxy-core`, and `media-server-core` to enable development.
3.  **CI/CD**: Establish a basic build pipeline.
4.  **Registrar Persistence**: Define the storage interface.

## User Review Required
> [!IMPORTANT]
> I will NOT delete `session-core` (v1) or `session-core-v2` code to avoid "deleting multiple lines" as per your rule. Instead, I will mark them as **DEPRECATED** in their documentation and crate metadata.

## Proposed Changes

### 1. Unified Session Strategy
Update documentation to explicitly guide developers to `session-core-v3`.
- `crates/session-core/README.md`: Add Deprecation Warning.
- `crates/session-core-v2/README.md`: Add Deprecation Warning.
- `crates/session-core-v3/README.md`: Mark as Recommended.
- `VERSION_STRATEGY.md`: Update or Create to formalize this.

### 2. Scaffold Missing Components
Create the basic structure for missing critical components so they are part of the workspace.
- **[NEW] `crates/b2bua-core/`**:
    - `Cargo.toml`: Basic dependencies.
    - `src/lib.rs`: Module structure.
- **[NEW] `crates/proxy-core/`**:
    - `Cargo.toml`: Basic dependencies.
    - `src/lib.rs`: Module structure.
- **[NEW] `crates/media-server-core/`**:
    - `Cargo.toml`: Basic dependencies.
    - `src/lib.rs`: Module structure.
- **[MODIFY] `Cargo.toml`**: Add new crates to workspace members.

### 3. Registrar Persistence Interface
Add a persistence abstraction to `registrar-core` to solve the "missing persistence" architectural issue.
- **[MODIFY] `crates/registrar-core/src/lib.rs`**: Export new storage module.
- **[NEW] `crates/registrar-core/src/storage.rs`**: API definition for user storage.

### 4. CI/CD Pipeline
- **[NEW] `.github/workflows/ci.yml`**: Basic `cargo test` and `cargo clippy` workflow.

## Verification Plan
1.  **Build**: Run `cargo build --workspace` to ensure new crates link correctly.
2.  **Check**: Verify `Cargo.toml` members list.
3.  **Documentation**: Verify README changes.
