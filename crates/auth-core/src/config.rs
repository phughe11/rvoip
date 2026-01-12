//! Configuration for auth-core
//!
//! Provides configuration structures for token validation, OAuth2 providers,
//! and caching behavior.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Main configuration for AuthCore
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// JWT validation settings
    #[serde(default)]
    pub jwt: JwtConfig,
    
    /// Token cache settings
    #[serde(default)]
    pub cache: CacheConfig,
    
    /// Trusted JWT issuers (including users-core)
    #[serde(default)]
    pub trusted_issuers: HashMap<String, TrustedIssuerConfig>,
    
    /// OAuth2/OIDC providers
    #[serde(default)]
    pub providers: HashMap<String, ProviderConfig>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt: JwtConfig::default(),
            cache: CacheConfig::default(),
            trusted_issuers: HashMap::new(),
            providers: HashMap::new(),
        }
    }
}

/// JWT validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// Whether to validate expiration claim
    #[serde(default = "default_true")]
    pub validate_exp: bool,
    
    /// Whether to validate not-before claim
    #[serde(default = "default_true")]
    pub validate_nbf: bool,
    
    /// Leeway for time-based claims (seconds)
    #[serde(default = "default_leeway")]
    pub leeway_seconds: u64,
    
    /// Required claims
    #[serde(default = "default_required_claims")]
    pub required_claims: Vec<String>,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            validate_exp: true,
            validate_nbf: true,
            leeway_seconds: 60,
            required_claims: vec!["sub".to_string(), "iss".to_string()],
        }
    }
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Maximum number of cached tokens
    #[serde(default = "default_max_entries")]
    pub max_entries: u64,
    
    /// TTL for successful validation results (seconds)
    #[serde(default = "default_positive_ttl")]
    pub positive_ttl_seconds: u64,
    
    /// TTL for failed validation results (seconds)
    #[serde(default = "default_negative_ttl")]
    pub negative_ttl_seconds: u64,
    
    /// TTL for JWKS cache (seconds)
    #[serde(default = "default_jwks_ttl")]
    pub jwks_ttl_seconds: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 10000,
            positive_ttl_seconds: 300,
            negative_ttl_seconds: 60,
            jwks_ttl_seconds: 3600,
        }
    }
}

impl CacheConfig {
    pub fn positive_ttl(&self) -> Duration {
        Duration::from_secs(self.positive_ttl_seconds)
    }
    
    pub fn negative_ttl(&self) -> Duration {
        Duration::from_secs(self.negative_ttl_seconds)
    }
    
    pub fn jwks_ttl(&self) -> Duration {
        Duration::from_secs(self.jwks_ttl_seconds)
    }
}

/// Configuration for a trusted JWT issuer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedIssuerConfig {
    /// Issuer identifier (must match `iss` claim)
    pub issuer: String,
    
    /// JWKS endpoint URL (optional if public_key is provided)
    pub jwks_uri: Option<String>,
    
    /// Direct public key in PEM or JWK format (optional if jwks_uri is provided)
    pub public_key: Option<String>,
    
    /// Expected audiences
    #[serde(default)]
    pub audiences: Vec<String>,
    
    /// Allowed algorithms
    #[serde(default = "default_algorithms")]
    pub algorithms: Vec<String>,
}

/// Configuration for an OAuth2/OIDC provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Provider type
    #[serde(rename = "type")]
    pub provider_type: ProviderType,
    
    /// OAuth2 client ID
    pub client_id: String,
    
    /// OAuth2 client secret (can use env var placeholder like ${SECRET_NAME})
    #[serde(default)]
    pub client_secret: Option<String>,
    
    /// OpenID Connect discovery URL (optional, auto-configures endpoints)
    pub discovery_url: Option<String>,
    
    /// Authorization endpoint (required if no discovery_url)
    pub auth_url: Option<String>,
    
    /// Token endpoint (required if no discovery_url)
    pub token_url: Option<String>,
    
    /// User info endpoint
    pub userinfo_url: Option<String>,
    
    /// Introspection endpoint (for opaque tokens)
    pub introspection_url: Option<String>,
    
    /// Revocation endpoint
    pub revocation_url: Option<String>,
    
    /// JWKS endpoint (if different from discovery)
    pub jwks_uri: Option<String>,
    
    /// OAuth2 scopes to request
    #[serde(default = "default_scopes")]
    pub scopes: Vec<String>,
    
    /// Issuer identifier (for JWT validation)
    pub issuer: Option<String>,
    
    /// Whether to use PKCE
    #[serde(default = "default_true")]
    pub use_pkce: bool,
}

/// Provider type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    /// Standard OAuth2 provider
    OAuth2,
    /// OpenID Connect provider (supports discovery)
    Oidc,
    /// Google-specific configuration
    Google,
    /// Keycloak-specific configuration
    Keycloak,
}

// Default value helpers
fn default_true() -> bool { true }
fn default_leeway() -> u64 { 60 }
fn default_required_claims() -> Vec<String> { vec!["sub".to_string(), "iss".to_string()] }
fn default_max_entries() -> u64 { 10000 }
fn default_positive_ttl() -> u64 { 300 }
fn default_negative_ttl() -> u64 { 60 }
fn default_jwks_ttl() -> u64 { 3600 }
fn default_algorithms() -> Vec<String> { vec!["RS256".to_string()] }
fn default_scopes() -> Vec<String> { vec!["openid".to_string(), "profile".to_string(), "email".to_string()] }

/// Builder for AuthConfig
pub struct AuthConfigBuilder {
    config: AuthConfig,
}

impl AuthConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: AuthConfig::default(),
        }
    }
    
    /// Add a trusted issuer (e.g., users-core)
    pub fn add_trusted_issuer(mut self, name: &str, config: TrustedIssuerConfig) -> Self {
        self.config.trusted_issuers.insert(name.to_string(), config);
        self
    }
    
    /// Add an OAuth2/OIDC provider
    pub fn add_provider(mut self, name: &str, config: ProviderConfig) -> Self {
        self.config.providers.insert(name.to_string(), config);
        self
    }
    
    /// Set JWT configuration
    pub fn jwt_config(mut self, config: JwtConfig) -> Self {
        self.config.jwt = config;
        self
    }
    
    /// Set cache configuration
    pub fn cache_config(mut self, config: CacheConfig) -> Self {
        self.config.cache = config;
        self
    }
    
    /// Build the configuration
    pub fn build(self) -> AuthConfig {
        self.config
    }
}

impl Default for AuthConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
