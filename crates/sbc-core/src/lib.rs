//! Session Border Controller (SBC) Core Component
//!
//! # Status
//! 🚧 **Under Construction**
//!
//! This crate implements SBC functionality:
//! - Topology Hiding
//! - Security / Rate Limiting
//! - NAT Traversal / Media Anchoring
//!
//! # Architecture Note
//! SBC can optionally use B2BUA (with the `b2bua` feature) for B2BUA-mode operation.
//! SBC also implements `RequestProcessor` trait from b2bua-core, allowing it to be
//! injected into B2BUA for security processing.

use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use tracing::{info, debug};
use rvoip_sip_core::{Request, Response, Header, HeaderName};
use std::net::IpAddr;

mod rate_limit;
use rate_limit::RateLimiter;

#[cfg(feature = "b2bua")]
pub use rvoip_b2bua_core::{B2buaEngine, RequestProcessor};

/// SBC Engine for security and topology hiding
pub struct SbcEngine {
    config: SbcConfig,
    rate_limiter: RateLimiter,
}

#[derive(Debug, Clone)]
pub struct SbcConfig {
    pub hide_topology: bool,
    pub strip_server_header: bool,
    pub strip_user_agent: bool,
    pub max_requests_per_second: u32,
}

impl Default for SbcConfig {
    fn default() -> Self {
        Self {
            hide_topology: true,
            strip_server_header: true,
            strip_user_agent: true,
            max_requests_per_second: 50,
        }
    }
}

impl SbcEngine {
    /// Create a new SBC Engine
    pub fn new(config: SbcConfig) -> Self {
        let rate_limiter = RateLimiter::new(config.max_requests_per_second);
        Self { config, rate_limiter }
    }
    
    /// Default configuration
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self::new(SbcConfig {
            hide_topology: true,
            strip_server_header: true,
            strip_user_agent: true,
            max_requests_per_second: 50,
        })
    }

    /// Process an incoming request for topology hiding and security (sync version)
    pub fn process_request_sync(&self, request: &mut Request, source_ip: IpAddr) -> Result<()> {
        // 1. Rate Limiting
        if !self.rate_limiter.check(source_ip) {
            info!("SBC: Rate limit exceeded for {}", source_ip);
            anyhow::bail!("Rate limit exceeded");
        }

        // 2. Topology Hiding
        if self.config.hide_topology {
            self.sanitize_headers(request);
        }
        
        Ok(())
    }
    
    /// Sanitize headers to hide internal topology or privacy
    fn sanitize_headers(&self, request: &mut Request) {
        // Strip sensitive headers based on config
        // Note: In a B2BUA, the outgoing leg is usually generated fresh, so this 
        // mainly affects what is stored/logged or forwarded if acting as Proxy.
        
        request.headers.retain(|h| {
            let name = h.name();
            
            if self.config.strip_server_header && name == HeaderName::Server {
                debug!("Stripped Server header");
                return false;
            }
            
            if self.config.strip_user_agent && name == HeaderName::UserAgent {
                debug!("Stripped User-Agent header");
                return false;
            }
            
            // Should also strip X-Internal-*, Record-Route (if hiding topology), etc.
            
            true
        });
        
        // TODO: Advanced Topology Hiding (Via/Contact rewriting)
        // This usually requires stateful mapping which is handled by the B2BUA logic itself
        // rather than just header stripping.
    }
}

// Implement RequestProcessor trait when b2bua feature is enabled
#[cfg(feature = "b2bua")]
#[async_trait]
impl rvoip_b2bua_core::RequestProcessor for SbcEngine {
    async fn process_request(&self, request: &mut Request, source_ip: IpAddr) -> Result<()> {
        // Delegate to sync implementation
        self.process_request_sync(request, source_ip)
    }
}

// Also provide the trait impl even without b2bua feature for standalone use
// This allows SbcEngine to be used as a processor when b2bua-core is available
#[cfg(not(feature = "b2bua"))]
impl SbcEngine {
    /// Process an incoming request for topology hiding and security
    #[deprecated(since = "0.1.26", note = "Use process_request_sync instead")]
    pub fn process_request(&self, request: &mut Request, source_ip: IpAddr) -> Result<()> {
        self.process_request_sync(request, source_ip)
    }
}
