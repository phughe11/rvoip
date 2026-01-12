# Change: Implement Auth-Core Token Validation and OAuth2 Support

## Why
Auth-core is currently only a stub with type definitions (~82 lines). The RVOIP stack requires a unified token validation service that can:
- Validate JWT tokens from users-core
- Support OAuth2 providers (Google, Azure AD, Keycloak)
- Provide high-performance token caching
- Enable secure SIP authentication via Bearer tokens

Without this implementation, the entire authentication infrastructure is non-functional, blocking production readiness.

## What Changes
- **TokenValidationService**: Core trait and implementation for token validation
- **JWT Validator**: JWKS-based JWT signature verification
- **OAuth2 Client**: Authorization code flow, token refresh, PKCE support
- **Provider Adapters**: Google, Keycloak, generic OpenID Connect
- **Token Cache**: Moka-based in-memory cache with TTL
- **SIP Integration**: Bearer token extraction and validation helpers
- **Configuration**: TOML-based provider configuration

## Impact
- Affected specs: `auth-core` (new capability)
- Affected code:
  - `crates/auth-core/src/` - Full implementation
  - `crates/session-core/` - Integration for SIP authentication
  - `crates/registrar-core/` - Token-based registration support
