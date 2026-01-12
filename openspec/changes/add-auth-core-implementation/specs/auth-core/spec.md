# Auth-Core Capability Specification

## ADDED Requirements

### Requirement: Token Validation Service
The system SHALL provide a unified token validation service that validates tokens from multiple sources and returns a standardized UserContext.

#### Scenario: Validate JWT from users-core
- **WHEN** a JWT token issued by users-core is presented
- **THEN** the system SHALL validate the signature using the cached public key
- **AND** extract user claims into UserContext
- **AND** cache the validation result

#### Scenario: Validate expired token
- **WHEN** a token with expired `exp` claim is presented
- **THEN** the system SHALL return AuthError::TokenExpired
- **AND** cache the negative result briefly

#### Scenario: Validate token from untrusted issuer
- **WHEN** a JWT with an unknown issuer is presented
- **THEN** the system SHALL return AuthError::UntrustedIssuer

### Requirement: JWT Validation
The system SHALL validate JWT tokens using standard algorithms (RS256, RS384, RS512) and JWKS-based public key discovery.

#### Scenario: Fetch JWKS from issuer
- **WHEN** a JWT is received from a configured issuer
- **THEN** the system SHALL fetch and cache the JWKS from the configured endpoint
- **AND** use the appropriate key based on the `kid` header

#### Scenario: Validate required claims
- **WHEN** validating a JWT
- **THEN** the system SHALL verify `iss`, `aud`, `exp`, and `sub` claims are present and valid

### Requirement: OAuth2 Authorization Code Flow
The system SHALL support OAuth2 authorization code flow with PKCE for web and mobile clients.

#### Scenario: Exchange authorization code
- **WHEN** a valid authorization code is presented
- **THEN** the system SHALL exchange it for access and refresh tokens
- **AND** validate the returned tokens

#### Scenario: Refresh expired token
- **WHEN** an access token is expired but refresh token is valid
- **THEN** the system SHALL obtain new tokens using the refresh flow

### Requirement: Token Caching
The system SHALL cache token validation results to minimize latency and reduce load on token providers.

#### Scenario: Cache hit for valid token
- **WHEN** a previously validated token is presented
- **AND** the cache entry has not expired
- **THEN** the system SHALL return the cached UserContext without re-validation

#### Scenario: Cache invalidation
- **WHEN** a token is revoked
- **THEN** the system SHALL remove the cache entry

### Requirement: OAuth2 Provider Adapters
The system SHALL support multiple OAuth2/OIDC providers through pluggable adapters.

#### Scenario: Google OAuth2 authentication
- **WHEN** configuring Google as an OAuth2 provider
- **THEN** the system SHALL support Google's OAuth2 endpoints and user info API

#### Scenario: OpenID Connect discovery
- **WHEN** an OIDC provider is configured with a discovery URL
- **THEN** the system SHALL automatically fetch endpoints from the well-known configuration

### Requirement: SIP Bearer Token Authentication
The system SHALL extract and validate Bearer tokens from SIP Authorization headers.

#### Scenario: Extract Bearer token from REGISTER
- **WHEN** a SIP REGISTER request contains an Authorization header with scheme "Bearer"
- **THEN** the system SHALL extract the token and validate it

#### Scenario: Challenge unauthenticated request
- **WHEN** a SIP request requires authentication but lacks a valid token
- **THEN** the system SHALL respond with 401 Unauthorized and WWW-Authenticate header
