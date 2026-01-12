use std::time::{Duration, Instant};
use dashmap::DashMap;
use std::net::IpAddr;
use std::sync::Arc;

/// Rate Limiter for SIP requests
///
/// Uses a generic Fixed Window algorithm.
/// Tracks requests per IP per second.
#[derive(Clone)]
pub struct RateLimiter {
    // Stores (count, window_start_time) per IP
    trackers: Arc<DashMap<IpAddr, (u32, Instant)>>,
    limit_per_second: u32,
}

impl RateLimiter {
    pub fn new(limit_per_second: u32) -> Self {
        Self {
            trackers: Arc::new(DashMap::new()),
            limit_per_second,
        }
    }

    /// Check if an IP has exceeded the rate limit
    /// Returns true if allowed, false if blocked
    pub fn check(&self, ip: IpAddr) -> bool {
        if self.limit_per_second == 0 {
            return true; // Unlimited
        }

        let mut entry = self.trackers.entry(ip).or_insert((0, Instant::now()));
        let (count, start_time) = entry.value_mut();

        if start_time.elapsed() >= Duration::from_secs(1) {
            // New window
            *count = 1;
            *start_time = Instant::now();
            true
        } else {
            // Existing window
            if *count < self.limit_per_second {
                *count += 1;
                true
            } else {
                false // Limit exceeded
            }
        }
    }

    /// Clean up old entries to prevent memory leaks (optional, can be run periodically)
    pub fn cleanup(&self) {
        let now = Instant::now();
        self.trackers.retain(|_, (_, time)| now.duration_since(*time) < Duration::from_secs(60));
    }
}
