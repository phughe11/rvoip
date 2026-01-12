# Auth-Core Implementation Tasks

## 1. Core Framework
- [ ] 1.1 Define AuthCore struct and builder pattern
- [ ] 1.2 Define TokenValidationService trait
- [ ] 1.3 Define TrustedIssuer configuration
- [ ] 1.4 Implement configuration loading from TOML

## 2. JWT Validation
- [ ] 2.1 Implement JWT parsing and claims extraction
- [ ] 2.2 Implement JWKS client for public key fetching
- [ ] 2.3 Implement RS256/RS384/RS512 signature verification
- [ ] 2.4 Implement issuer and audience validation
- [ ] 2.5 Add users-core public key integration

## 3. OAuth2 Support
- [ ] 3.1 Implement OAuth2 client with reqwest
- [ ] 3.2 Implement authorization code flow
- [ ] 3.3 Implement token refresh flow
- [ ] 3.4 Add PKCE support
- [ ] 3.5 Implement OpenID Connect discovery

## 4. Provider Adapters
- [ ] 4.1 Implement generic OpenID Connect adapter
- [ ] 4.2 Implement Google OAuth2 adapter
- [ ] 4.3 Implement Keycloak adapter
- [ ] 4.4 Implement opaque token introspection

## 5. Token Cache
- [ ] 5.1 Implement Moka-based cache
- [ ] 5.2 Add positive/negative TTL configuration
- [ ] 5.3 Implement cache invalidation
- [ ] 5.4 Add cache metrics

## 6. SIP Integration
- [ ] 6.1 Implement Bearer token extraction from SIP headers
- [ ] 6.2 Add SIP Digest authentication support
- [ ] 6.3 Create integration helpers for session-core

## 7. Testing
- [ ] 7.1 Unit tests for JWT validation
- [ ] 7.2 Unit tests for OAuth2 flows
- [ ] 7.3 Integration tests with mock providers
- [ ] 7.4 Performance benchmarks
