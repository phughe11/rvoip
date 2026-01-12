//! Storage trait for Registrar persistence

use async_trait::async_trait;
use crate::error::Result;
use crate::types::{UserRegistration, ContactInfo};

/// Abstract storage interface for user registrations
/// Implement this trait for different backends (Redis, SQL, Memory, etc.)
#[async_trait]
pub trait Storage: Send + Sync {
    /// Save or update a user registration
    async fn save_registration(&self, user_id: &str, registration: &UserRegistration) -> Result<()>;

    /// Retrieve a user registration
    async fn get_registration(&self, user_id: &str) -> Result<Option<UserRegistration>>;

    /// Delete a user registration
    async fn delete_registration(&self, user_id: &str) -> Result<()>;

    /// Add a contact to an existing user
    async fn add_contact(&self, user_id: &str, contact: &ContactInfo) -> Result<()>;

    /// Remove a contact from a user
    async fn remove_contact(&self, user_id: &str, contact_uri: &str) -> Result<()>;
}

/// A simple in-memory storage implementation (default)
pub struct InMemoryStorage {
    // Placeholder implementation
}
