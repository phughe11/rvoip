//! # Auth-Core - Authentication and Authorization for RVoIP
//! 
//! This crate provides OAuth2 and token-based authentication services
//! for the RVoIP ecosystem, supporting multiple authentication flows
//! and token validation strategies.

pub mod config;

pub mod error;

pub mod jwt;

pub mod types;

pub mod jwks; // Adding this because it's referenced in jwt.rs

pub mod service;



pub use error::{AuthError, Result};

pub use types::UserContext;

pub use service::{TokenValidationService, StandardTokenService};
