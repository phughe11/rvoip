//! JWT validation module
//!
//! Provides JWT parsing, signature verification, and claims validation.

use crate::config::{JwtConfig, TrustedIssuerConfig};
use crate::error::{AuthError, Result};
use crate::types::UserContext;
use crate::jwks::{JwksCache, Jwk};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, warn};

/// Standard JWT claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Subject (user ID)
    pub sub: String,
    
    /// Issuer
    pub iss: Option<String>,
    
    /// Audience (can be string or array)
    #[serde(default)]
    pub aud: Audience,
    
    /// Expiration time (Unix timestamp)
    pub exp: Option<i64>,
    
    /// Not before time (Unix timestamp)
    pub nbf: Option<i64>,
    
    /// Issued at time (Unix timestamp)
    pub iat: Option<i64>,
    
    /// JWT ID
    pub jti: Option<String>,
    
    /// Email claim (common in OIDC)
    pub email: Option<String>,
    
    /// Preferred username
    pub preferred_username: Option<String>,
    
    /// Name
    pub name: Option<String>,
    
    /// Scope (space-separated string)
    pub scope: Option<String>,
    
    /// Roles (custom claim)
    #[serde(default)]
    pub roles: Vec<String>,
    
    /// Additional claims
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Audience can be a single string or array of strings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(untagged)]
pub enum Audience {
    #[default]
    None,
    Single(String),
    Multiple(Vec<String>),
}

impl Audience {
    pub fn contains(&self, aud: &str) -> bool {
        match self {
            Audience::None => false,
            Audience::Single(s) => s == aud,
            Audience::Multiple(v) => v.iter().any(|a| a == aud),
        }
    }
    
    pub fn is_empty(&self) -> bool {
        match self {
            Audience::None => true,
            Audience::Single(s) => s.is_empty(),
            Audience::Multiple(v) => v.is_empty(),
        }
    }
}

/// JWT Validator
pub struct JwtValidator {
    config: JwtConfig,
    jwks_cache: Arc<JwksCache>,
    trusted_issuers: HashMap<String, TrustedIssuerConfig>,
}

impl JwtValidator {
    /// Create a new JWT validator
    pub fn new(
        config: JwtConfig,
        jwks_cache: Arc<JwksCache>,
        trusted_issuers: HashMap<String, TrustedIssuerConfig>,
    ) -> Self {
        Self {
            config,
            jwks_cache,
            trusted_issuers,
        }
    }
    
    /// Validate a JWT token and return user context
    pub async fn validate(&self, token: &str) -> Result<UserContext> {
        // 1. Decode header to get algorithm and key ID
        let header = decode_header(token)
            .map_err(|e| AuthError::InvalidToken(format!("Invalid JWT header: {}", e)))?;
        
        debug!("JWT header: alg={:?}, kid={:?}", header.alg, header.kid);
        
        // 2. Peek at claims to get issuer (without verification)
        let claims = self.peek_claims(token)?;
        let issuer = claims.iss.as_ref()
            .ok_or_else(|| AuthError::InvalidToken("Missing issuer claim".to_string()))?;
        
        // 3. Find trusted issuer configuration
        let issuer_config = self.find_issuer_config(issuer)?;
        
        // 4. Get decoding key
        let decoding_key = self.get_decoding_key(&header, issuer_config).await?;
        
        // 5. Build validation parameters
        let validation = self.build_validation(&header.alg, issuer_config)?;
        
        // 6. Decode and validate token
        let token_data: TokenData<JwtClaims> = decode(token, &decoding_key, &validation)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                jsonwebtoken::errors::ErrorKind::InvalidAudience => {
                    AuthError::InvalidToken("Invalid audience".to_string())
                }
                jsonwebtoken::errors::ErrorKind::InvalidIssuer => {
                    AuthError::InvalidToken("Invalid issuer".to_string())
                }
                _ => AuthError::InvalidToken(format!("Token validation failed: {}", e)),
            })?;
        
        // 7. Additional validation for required claims
        self.validate_required_claims(&token_data.claims)?;
        
        // 8. Convert to UserContext
        Ok(self.claims_to_context(token_data.claims))
    }
    
    /// Peek at claims without verifying signature
    fn peek_claims(&self, token: &str) -> Result<JwtClaims> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(AuthError::InvalidToken("Invalid JWT format".to_string()));
        }
        
        let payload = URL_SAFE_NO_PAD
            .decode(parts[1])
            .map_err(|e| AuthError::InvalidToken(format!("Invalid payload encoding: {}", e)))?;
        
        serde_json::from_slice(&payload)
            .map_err(|e| AuthError::InvalidToken(format!("Invalid claims JSON: {}", e)))
    }
    
    /// Find the issuer configuration
    fn find_issuer_config(&self, issuer: &str) -> Result<&TrustedIssuerConfig> {
        for config in self.trusted_issuers.values() {
            if config.issuer == issuer {
                return Ok(config);
            }
        }
        Err(AuthError::UntrustedIssuer(issuer.to_string()))
    }
    
    /// Get the decoding key for signature verification
    async fn get_decoding_key(
        &self,
        header: &jsonwebtoken::Header,
        issuer_config: &TrustedIssuerConfig,
    ) -> Result<DecodingKey> {
        // Try JWKS first if configured
        if let Some(jwks_uri) = &issuer_config.jwks_uri {
            let jwk = self.jwks_cache.get_key(jwks_uri, header.kid.as_deref()).await?;
            return jwk_to_decoding_key(&jwk);
        }
        
        // Try direct public key
        if let Some(public_key) = &issuer_config.public_key {
            return DecodingKey::from_rsa_pem(public_key.as_bytes())
                .map_err(|e| AuthError::ConfigError(format!("Invalid public key: {}", e)));
        }
        
        Err(AuthError::ConfigError(
            "No JWKS URI or public key configured for issuer".to_string(),
        ))
    }
    
    /// Build validation parameters
    fn build_validation(
        &self,
        algorithm: &Algorithm,
        issuer_config: &TrustedIssuerConfig,
    ) -> Result<Validation> {
        // Check if algorithm is allowed
        let alg_str = format!("{:?}", algorithm);
        if !issuer_config.algorithms.is_empty() 
            && !issuer_config.algorithms.iter().any(|a| a == &alg_str) {
            return Err(AuthError::InvalidToken(format!(
                "Algorithm {} not allowed for issuer",
                alg_str
            )));
        }
        
        let mut validation = Validation::new(*algorithm);
        
        // Set issuer
        validation.set_issuer(&[&issuer_config.issuer]);
        
        // Set audiences if configured
        if !issuer_config.audiences.is_empty() {
            validation.set_audience(&issuer_config.audiences);
        } else {
            validation.validate_aud = false;
        }
        
        // Time validation settings
        validation.validate_exp = self.config.validate_exp;
        validation.validate_nbf = self.config.validate_nbf;
        validation.leeway = self.config.leeway_seconds;
        
        // Required claims
        validation.set_required_spec_claims(&self.config.required_claims);
        
        Ok(validation)
    }
    
    /// Validate required claims
    fn validate_required_claims(&self, claims: &JwtClaims) -> Result<()> {
        for claim in &self.config.required_claims {
            match claim.as_str() {
                "sub" if claims.sub.is_empty() => {
                    return Err(AuthError::InvalidToken("Missing required claim: sub".to_string()));
                }
                "iss" if claims.iss.is_none() => {
                    return Err(AuthError::InvalidToken("Missing required claim: iss".to_string()));
                }
                "aud" if claims.aud.is_empty() => {
                    return Err(AuthError::InvalidToken("Missing required claim: aud".to_string()));
                }
                "exp" if claims.exp.is_none() => {
                    return Err(AuthError::InvalidToken("Missing required claim: exp".to_string()));
                }
                _ => {}
            }
        }
        Ok(())
    }
    
    /// Convert JWT claims to UserContext
    fn claims_to_context(&self, claims: JwtClaims) -> UserContext {
        let username = claims.preferred_username
            .or(claims.email.clone())
            .or(claims.name.clone())
            .unwrap_or_else(|| claims.sub.clone());
        
        let scopes = claims.scope
            .map(|s| s.split_whitespace().map(String::from).collect())
            .unwrap_or_default();
        
        UserContext {
            user_id: claims.sub,
            username,
            roles: claims.roles,
            claims: claims.extra,
            expires_at: claims.exp,
            scopes,
        }
    }
}

/// Convert JWK to DecodingKey
fn jwk_to_decoding_key(jwk: &Jwk) -> Result<DecodingKey> {
    match jwk.kty.as_str() {
        "RSA" => {
            let n = jwk.n.as_ref()
                .ok_or_else(|| AuthError::ConfigError("JWK missing 'n' parameter".to_string()))?;
            let e = jwk.e.as_ref()
                .ok_or_else(|| AuthError::ConfigError("JWK missing 'e' parameter".to_string()))?;
            
            DecodingKey::from_rsa_components(n, e)
                .map_err(|e| AuthError::ConfigError(format!("Invalid RSA key: {}", e)))
        }
        "EC" => {
            // For EC keys, we need the x and y coordinates
            let x = jwk.x.as_ref()
                .ok_or_else(|| AuthError::ConfigError("JWK missing 'x' parameter".to_string()))?;
            let y = jwk.y.as_ref()
                .ok_or_else(|| AuthError::ConfigError("JWK missing 'y' parameter".to_string()))?;
            
            DecodingKey::from_ec_components(x, y)
                .map_err(|e| AuthError::ConfigError(format!("Invalid EC key: {}", e)))
        }
        _ => Err(AuthError::ConfigError(format!(
            "Unsupported key type: {}",
            jwk.kty
        ))),
    }
}

/// Check if a string looks like a JWT
pub fn is_jwt(token: &str) -> bool {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    
    // Check if parts are base64url encoded
    parts.iter().all(|p| {
        p.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_jwt() {
        assert!(is_jwt("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.signature"));
        assert!(!is_jwt("not-a-jwt"));
        assert!(!is_jwt("only.two.parts.here.nope"));
    }
    
    #[test]
    fn test_audience_contains() {
        let single = Audience::Single("test".to_string());
        assert!(single.contains("test"));
        assert!(!single.contains("other"));
        
        let multi = Audience::Multiple(vec!["a".to_string(), "b".to_string()]);
        assert!(multi.contains("a"));
        assert!(multi.contains("b"));
        assert!(!multi.contains("c"));
    }
}
