# Security Policy

## 🔒 Reporting a Vulnerability

**Please do NOT report security vulnerabilities through public GitHub issues.**

If you discover a security vulnerability, please report it privately to help us address it before public disclosure.

### Reporting Process

1. **Email**: Send details to the maintainers (contact info in GitHub profile)
2. **GitHub Security**: Use [GitHub's private vulnerability reporting](https://github.com/eisenzopf/rvoip/security/advisories/new)
3. **Include**: 
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if you have one)

### What to Expect

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 1 week
- **Status Updates**: Every 2 weeks until resolved
- **Resolution**: Depends on severity and complexity

## 🎯 Scope

### In Scope

Security vulnerabilities in:
- **sip-core** - SIP message parsing and handling
- **dialog-core** - Dialog state management
- **media-core** - Media processing
- **rtp-core** - RTP/RTCP and cryptographic implementations
- **session-core** (all versions) - Session management
- **auth-core** - Authentication mechanisms
- **registrar-core** - Registration handling
- **All other RVOIP crates**

### Out of Scope

- Vulnerabilities in dependencies (report to respective projects)
- DoS through resource exhaustion (best practices apply, but not considered critical)
- Issues requiring physical access to the server
- Social engineering attacks

## 🚨 Severity Levels

### Critical (P0)
- Remote code execution
- Authentication bypass
- Privilege escalation
- Cryptographic failures in security protocols

**Response Time**: Immediate (same day)

### High (P1)
- Information disclosure of sensitive data
- SIP message injection/manipulation
- Session hijacking
- Media stream interception

**Response Time**: Within 3 days

### Medium (P2)
- Denial of service (crash or hang)
- Partial information disclosure
- Logic errors in protocol handling

**Response Time**: Within 2 weeks

### Low (P3)
- Configuration issues
- Minor information leaks
- Timing attacks without practical exploit

**Response Time**: Next release

## 🛡️ Security Best Practices

### For Users

#### Transport Security
```rust
// Always use TLS for SIP signaling in production
let config = SipConfig {
    transport: Transport::TLS,
    cert_path: "/path/to/cert.pem",
    key_path: "/path/to/key.pem",
    // ...
};
```

#### Media Security
```rust
// Enable SRTP for media encryption
let media_config = MediaConfig {
    security: MediaSecurity::SRTP,
    crypto_suite: CryptoSuite::AES_CM_128_HMAC_SHA1_80,
    // ...
};
```

#### Authentication
```rust
// Use strong authentication
let auth_config = AuthConfig {
    method: AuthMethod::DigestMD5,
    realm: "secure.example.com",
    // Never hardcode credentials!
};
```

### For Developers

#### Input Validation
```rust
// ALWAYS validate input from untrusted sources
pub fn parse_uri(input: &str) -> Result<Uri> {
    // Validate length
    if input.len() > MAX_URI_LENGTH {
        return Err(Error::UriTooLong);
    }
    
    // Validate characters
    if !is_valid_uri_chars(input) {
        return Err(Error::InvalidUriChars);
    }
    
    // Parse and validate structure
    // ...
}
```

#### Cryptography
```rust
// Use standard, audited crypto libraries
use ring::aead::{Aes128Gcm, SealingKey};

// DON'T roll your own crypto!
// DON'T use deprecated algorithms (MD5, SHA1 for signing)
```

#### Error Handling
```rust
// Don't leak sensitive information in errors
match verify_password(password) {
    Ok(_) => { /* continue */ },
    Err(_) => {
        // Generic error message
        return Err(Error::AuthenticationFailed);
        // DON'T: "Password too short" or "User not found"
    }
}
```

## 🔐 Security Features

### Current Implementation

#### Transport Security
- ✅ TLS support for SIP signaling
- ✅ DTLS-SRTP for WebRTC
- ✅ Certificate validation

#### Media Security  
- ✅ SRTP/SRTCP (RFC 3711)
- ✅ DTLS-SRTP (RFC 5764)
- ✅ ZRTP for P2P (RFC 6189)
- ✅ MIKEY-PSK and MIKEY-PKE
- ✅ Multiple cipher suites (AES-CM, AES-GCM)

#### Authentication
- ✅ SIP Digest Authentication (RFC 2617)
- ✅ OAuth 2.0 Bearer tokens
- ✅ JWT support

#### Protocol Security
- ✅ Message validation and sanitization
- ✅ Rate limiting hooks
- ✅ Replay protection (SRTP)

### Planned Security Features

- 🚧 Advanced rate limiting and DDoS protection
- 🚧 Security event logging
- 🚧 Automated security scanning in CI
- 🚧 Fuzzing for protocol parsers

## ⚠️ Known Security Considerations

### Current Limitations

1. **No Built-in Rate Limiting**
   - Application must implement rate limiting
   - See examples for reference implementations

2. **IPv6 Support**
   - IPv6 support is basic
   - Some edge cases may not be handled

3. **NAT Traversal**
   - STUN/TURN support is basic
   - Complex NAT scenarios may fail

4. **Denial of Service**
   - Large message handling needs resource limits
   - Application must implement timeouts

### Mitigation Strategies

#### Rate Limiting
```rust
// Implement at application level
use governor::{Quota, RateLimiter};

let limiter = RateLimiter::direct(
    Quota::per_second(nonzero!(10u32))
);

if limiter.check().is_err() {
    return Err(Error::RateLimitExceeded);
}
```

#### Resource Limits
```rust
// Set reasonable limits
const MAX_MESSAGE_SIZE: usize = 65536;
const MAX_CONCURRENT_CALLS: usize = 1000;
const CALL_TIMEOUT: Duration = Duration::from_secs(300);
```

## 🔍 Security Audits

### Status
- **Last Audit**: None (Alpha software)
- **Next Planned**: Before 1.0 release
- **Third-party Review**: Seeking sponsors

### Request an Audit

Interested in sponsoring a security audit? Contact the maintainers.

## 📚 Security Resources

### Specifications
- [RFC 3261 - SIP Security](https://www.rfc-editor.org/rfc/rfc3261#section-26)
- [RFC 3711 - SRTP](https://www.rfc-editor.org/rfc/rfc3711)
- [RFC 5764 - DTLS-SRTP](https://www.rfc-editor.org/rfc/rfc5764)
- [RFC 6189 - ZRTP](https://www.rfc-editor.org/rfc/rfc6189)

### Tools
- [cargo-audit](https://github.com/RustSec/rustsec) - Dependency vulnerability scanning
- [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) - Dependency policy enforcement
- [cargo-geiger](https://github.com/rust-secure-code/cargo-geiger) - Unsafe code detection

### Best Practices
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [OWASP VoIP Security](https://owasp.org/www-community/vulnerabilities/VoIP_Security)

## 🏆 Security Hall of Fame

We recognize and thank security researchers who responsibly disclose vulnerabilities:

<!-- Will be updated as reports are received and resolved -->

*No vulnerabilities reported yet (project in alpha)*

## 📋 Disclosure Policy

### Our Commitment

- We will acknowledge your report within 48 hours
- We will not pursue legal action against researchers who:
  - Report vulnerabilities responsibly
  - Do not exploit vulnerabilities beyond proof of concept
  - Do not access/modify user data
  - Do not perform DoS attacks

### Coordinated Disclosure

- We prefer a 90-day disclosure window
- We will work with you on the disclosure timeline
- We will credit researchers (unless they prefer anonymity)
- We will notify affected users before public disclosure

## 🔄 Security Updates

### Notification Channels

- **GitHub Security Advisories** - Automatic for dependents
- **CHANGELOG.md** - All security fixes documented
- **Release Notes** - Security fixes highlighted

### Update Frequency

- **Critical**: Immediate patch release
- **High**: Within 1 week
- **Medium**: Next scheduled release
- **Low**: Bundled with feature releases

## ✅ Compliance

### Standards

RVOIP aims to comply with:
- RFC 3261 (SIP) security requirements
- RFC 3711 (SRTP) specifications
- NIST guidelines for cryptographic protocols

### Certifications

*None yet - project in alpha*

Target certifications for 1.0:
- SOC 2 compliance (for enterprise)
- GDPR compliance (for EU deployments)

## 🤝 Contributing to Security

### Security-Related Contributions Welcome

- Improve input validation
- Add security tests
- Enhance cryptographic implementations
- Improve documentation
- Add security examples

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Security Review Checklist

When reviewing security-related PRs:
- [ ] Input validation is thorough
- [ ] Errors don't leak sensitive info
- [ ] Crypto uses standard libraries
- [ ] No hardcoded secrets
- [ ] Rate limiting considered
- [ ] Tests include security cases
- [ ] Documentation updated

## 📞 Contact

- **Security Issues**: Use GitHub Security Advisories
- **General Questions**: GitHub Discussions
- **Email**: Check maintainer profiles for contact info

---

**Last Updated**: January 11, 2026  
**Next Review**: March 2026 or after significant changes

Thank you for helping keep RVOIP secure! 🔒
