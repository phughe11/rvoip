//! Load Balancer for Proxy Core
//!
//! Provides strategies for distributing requests across multiple destinations.

use async_trait::async_trait;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Trait for load balancing strategies
#[async_trait]
pub trait LoadBalancer: Send + Sync {
    /// Select a destination from a list of candidates
    async fn select_destination(&self, candidates: &[SocketAddr]) -> Option<SocketAddr>;
}

/// Round-robin load balancer
pub struct RoundRobinLoadBalancer {
    counter: AtomicUsize,
}

impl RoundRobinLoadBalancer {
    pub fn new() -> Self {
        Self {
            counter: AtomicUsize::new(0),
        }
    }
}

#[async_trait]
impl LoadBalancer for RoundRobinLoadBalancer {
    async fn select_destination(&self, candidates: &[SocketAddr]) -> Option<SocketAddr> {
        if candidates.is_empty() {
            return None;
        }
        
        let count = self.counter.fetch_add(1, Ordering::Relaxed);
        let index = count % candidates.len();
        
        Some(candidates[index])
    }
}
