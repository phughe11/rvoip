//! Back-to-Back User Agent (B2BUA) Core Component
//!
//! # Status
//! 🚧 **Under Construction**
//!
//! This crate implements B2BUA functionality including:
//! - Dialog pair management
//! - Call interception
//! - Media bridging
//! - Application handlers
//!
//! # Architecture Note
//! B2BUA does NOT depend on SBC. Security processing (rate limiting, topology hiding)
//! can be injected via the `RequestProcessor` trait. This allows SBC to optionally
//! wrap or use B2BUA, maintaining proper layering.

pub mod call;
pub mod core;
pub mod state;
pub mod processor;

pub use core::B2buaEngine;
pub use call::B2buaCall;
pub use state::B2buaState;
pub use processor::RequestProcessor;
