//! Token Validation Service Implementation

use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{info, warn, error};

use crate::error::{AuthError, Result};
use crate::types::UserContext;
use crate::jwt::JwtValidator;
use crate::config::{AuthConfig, JwtConfig, TrustedIssuerConfig};
use crate::jwks::JwksCache;

/// Core service for token validation
#[async_trait]
pub trait TokenValidationService: Send + Sync {
    /// Validate any token and return user context
    async fn validate_token(&self, token: &str) -> Result<UserContext>;
    
    /// Invalidate a specific token (if cached)
    async fn invalidate_token(&self, token: &str);
}

/// Standard implementation of TokenValidationService
pub struct StandardTokenService {
    config: AuthConfig,
    jwt_validator: JwtValidator,
    token_cache: Arc<DashMap<String, CachedToken>>,
}

struct CachedToken {
    user_context: UserContext,
    expires_at: Instant,
}

impl StandardTokenService {
    pub fn new(config: AuthConfig) -> Self {
        // Initialize JWKS cache
        let jwks_cache = Arc::new(JwksCache::new());
        
        // Initialize JWT validator
        // Assuming config has a default JWT config for now or we extract it
        let jwt_config = JwtConfig::default(); // Should come from AuthConfig
        
        // Convert trusted issuers map
        let trusted_issuers = config.trusted_issuers.clone();
        
        let jwt_validator = JwtValidator::new(
            jwt_config,
            jwks_cache,
            trusted_issuers,
        );
        
        Self {
            config,
            jwt_validator,
            token_cache: Arc::new(DashMap::new()),
        }
    }
    
    // Background task to clean up expired cache entries
    pub async fn start_cleanup_task(&self) {
        let cache = self.token_cache.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                let now = Instant::now();
                // Remove expired entries
                // DashMap::retain safely locks shards during iteration
                cache.retain(|_, v| v.expires_at > now);
                tracing::debug!("Token cache cleanup completed");
            }
        });
    }
}

#[async_trait]
impl TokenValidationService for StandardTokenService {
    async fn validate_token(&self, token: &str) -> Result<UserContext> {
        // 1. Check cache
        if let Some(entry) = self.token_cache.get(token) {
            if entry.expires_at > Instant::now() {
                return Ok(entry.user_context.clone());
            } else {
                // Expired in cache
                drop(entry);
                self.token_cache.remove(token);
            }
        }
        
        // 2. Determine token type and validate
        // For MVP, we assume JWT. Later add opaque token support.
        let user_context = if crate::jwt::is_jwt(token) {
            self.jwt_validator.validate(token).await?
        } else {
            // Treat as opaque token - unimplemented for now
            // In full system, check with OAuth2 provider introspection endpoint
            return Err(AuthError::InvalidToken("Opaque tokens not supported yet".to_string()));
        };
        
        // 3. Cache result
        // Use token expiry or default TTL
        let ttl = if let Some(exp) = user_context.expires_at {
             // Calculate duration until exp
             let now_unix = chrono::Utc::now().timestamp();
             if exp > now_unix {
                 Duration::from_secs((exp - now_unix) as u64)
             } else {
                 Duration::from_secs(0) // Already expired
             }
        } else {
            Duration::from_secs(300) // Default 5 min
        };
        
        if ttl.as_secs() > 0 {
            self.token_cache.insert(token.to_string(), CachedToken {
                user_context: user_context.clone(),
                expires_at: Instant::now() + ttl,
            });
        }
        
        Ok(user_context)
    }
    
    async fn invalidate_token(&self, token: &str) {
        self.token_cache.remove(token);
    }
}
