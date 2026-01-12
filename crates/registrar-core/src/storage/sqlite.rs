//! SQLite Storage implementation for Registrar

use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use crate::error::{Result, RegistrarError};
use crate::types::{UserRegistration, ContactInfo, Transport};
use crate::storage::Storage;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};

pub struct SqliteStorage {
    pool: SqlitePool,
}

impl SqliteStorage {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn initialize(&self) -> Result<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS registrations (
                user_id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                domain TEXT NOT NULL,
                contact_uri TEXT NOT NULL,
                transport TEXT NOT NULL,
                expires_at INTEGER NOT NULL,
                last_updated INTEGER NOT NULL
            )"
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RegistrarError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn save_registration(&self, user_id: &str, reg: &UserRegistration) -> Result<()> {
        let expires = reg.expires.timestamp();
        let now = Utc::now().timestamp();
        
        // Take the first contact for MVP storage
        if let Some(contact) = reg.contacts.first() {
            let transport = format!("{:?}", contact.transport);
            
            // Derive domain from user_id if possible, or use placeholder
            let domain = user_id.split('@').nth(1).unwrap_or("localhost");
            let username = user_id.split('@').next().unwrap_or(user_id);

            sqlx::query(
                "INSERT OR REPLACE INTO registrations 
                (user_id, username, domain, contact_uri, transport, expires_at, last_updated)
                VALUES (?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(user_id)
            .bind(username)
            .bind(domain)
            .bind(&contact.uri)
            .bind(transport)
            .bind(expires)
            .bind(now)
            .execute(&self.pool)
            .await
            .map_err(|e| RegistrarError::DatabaseError(e.to_string()))?;
        }

        Ok(())
    }

    async fn get_registration(&self, user_id: &str) -> Result<Option<UserRegistration>> {
        let row = sqlx::query("SELECT * FROM registrations WHERE user_id = ?")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RegistrarError::DatabaseError(e.to_string()))?;

        if let Some(row) = row {
            let expires_at_secs: i64 = row.get("expires_at");
            let expires = DateTime::<Utc>::from(UNIX_EPOCH + std::time::Duration::from_secs(expires_at_secs as u64));
            
            let transport_str: String = row.get("transport");
            let transport = match transport_str.as_str() {
                "Udp" => Transport::Udp,
                "Tcp" => Transport::Tcp,
                "Tls" => Transport::Tls,
                "Ws" => Transport::Ws,
                "Wss" => Transport::Wss,
                _ => Transport::Udp, // Default
            };

            let contact = ContactInfo {
                uri: row.get("contact_uri"),
                transport,
                expires: expires, 
                q_value: 1.0,
                instance_id: "".to_string(), // Default
                user_agent: "Unknown".to_string(), // Default
                received: None,
                path: vec![],
                methods: vec![],
            };

            let last_updated_secs: i64 = row.get("last_updated");
            let registered_at = DateTime::<Utc>::from(UNIX_EPOCH + std::time::Duration::from_secs(last_updated_secs as u64));

            Ok(Some(UserRegistration {
                user_id: row.get("user_id"),
                contacts: vec![contact], // Single contact for MVP
                expires,
                presence_enabled: true, // Default
                capabilities: vec![],
                registered_at,
                attributes: std::collections::HashMap::new(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn delete_registration(&self, user_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM registrations WHERE user_id = ?")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| RegistrarError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn add_contact(&self, _user_id: &str, _contact: &ContactInfo) -> Result<()> {
        // Multi-device support to be implemented
        // For MVP, save_registration overwrites
        Ok(())
    }

    async fn remove_contact(&self, user_id: &str, _contact_uri: &str) -> Result<()> {
        // For MVP, just delete the user record
        self.delete_registration(user_id).await
    }
}
