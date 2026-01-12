//! Request Processor Trait
//!
//! This module defines the `RequestProcessor` trait which allows external
//! components (like SBC) to inject security and topology hiding logic
//! into the B2BUA without creating a dependency from B2BUA to SBC.

use anyhow::Result;
use async_trait::async_trait;
use rvoip_sip_core::Request;
use std::net::IpAddr;

/// Trait for processing incoming SIP requests before B2BUA handling.
///
/// This allows external components (e.g., SBC) to inject security policies,
/// rate limiting, and topology hiding without B2BUA depending on them.
///
/// # Example
///
/// ```ignore
/// use rvoip_b2bua_core::RequestProcessor;
///
/// struct MySecurityProcessor {
///     max_requests_per_sec: u32,
/// }
///
/// #[async_trait::async_trait]
/// impl RequestProcessor for MySecurityProcessor {
///     async fn process_request(&self, request: &mut Request, source_ip: IpAddr) -> Result<()> {
///         // Rate limiting logic
///         // Header sanitization
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait RequestProcessor: Send + Sync {
    /// Process an incoming request for security and topology hiding.
    ///
    /// # Arguments
    /// * `request` - Mutable reference to the SIP request to process
    /// * `source_ip` - The source IP address of the request
    ///
    /// # Returns
    /// * `Ok(())` if the request should be processed
    /// * `Err(...)` if the request should be rejected (e.g., rate limited)
    async fn process_request(&self, request: &mut Request, source_ip: IpAddr) -> Result<()>;
}

/// A no-op processor that allows all requests through without modification.
/// Used as default when no security processing is needed.
#[derive(Debug, Clone, Default)]
pub struct NoOpProcessor;

#[async_trait]
impl RequestProcessor for NoOpProcessor {
    async fn process_request(&self, _request: &mut Request, _source_ip: IpAddr) -> Result<()> {
        Ok(())
    }
}
