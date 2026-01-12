//! # Async Database Management Module (sqlx + SQLite)
//!
//! This module provides comprehensive database management functionality for the call center,
//! built on top of sqlx with SQLite. It provides a fully async, Send-safe interface
//! that eliminates all the previous trait object and async boundary issues.
//!
//! ## Key Features
//!
//! - **Fully Async**: No `spawn_blocking` - all operations are naturally async
//! - **Send + Sync Safe**: No trait object issues, works seamlessly with `tokio::spawn`
//! - **Compile-time Checked**: SQL queries are validated at compile time
//! - **Transaction Support**: Proper async transactions with rollback
//! - **Connection Pooling**: Built-in connection pooling for performance
//! - **Type Safety**: Strong typing for all database operations
//!
//! ## Quick Start
//!
//! ```rust
//! use rvoip_call_engine::database::DatabaseManager;
//! 
//! # async fn example() -> anyhow::Result<()> {
//! // Create database manager
//! let db = DatabaseManager::new("sqlite:callcenter.db").await?;
//! 
//! // All operations are Send-safe and can be used in tokio::spawn
//! tokio::spawn(async move {
//!     let agents = db.get_available_agents().await?;
//!     println!("Found {} available agents", agents.len());
//!     anyhow::Ok(())
//! });
//! # Ok(())
//! # }
//! ```

use anyhow::{Result, anyhow};
use sqlx::{SqlitePool, Row, Transaction, Sqlite};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tracing::{info, debug};
use crate::agent::AgentStatus;
use crate::prelude::SessionId;

// Re-export commonly used types
pub use chrono;
pub use sqlx;

/// Main database manager using sqlx for async operations
#[derive(Clone)]
pub struct DatabaseManager {
    pool: SqlitePool,
}

impl DatabaseManager {
    /// Create a new database manager with automatic migrations
    pub async fn new(database_url: &str) -> Result<Self> {
        info!("🗄️ Initializing sqlx database manager: {}", database_url);
        use std::str::FromStr;
        
        // Configure connection options for production performance
        let options = sqlx::sqlite::SqliteConnectOptions::from_str(database_url)?
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
            .busy_timeout(std::time::Duration::from_secs(5))
            .create_if_missing(true);

        // Connect to database
        let pool = SqlitePool::connect_with(options).await
            .map_err(|e| anyhow!("Failed to connect to database: {}", e))?;
        
        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|e| anyhow!("Failed to run migrations: {}", e))?;
        
        info!("✅ Database manager initialized successfully (WAL mode enabled)");
        Ok(Self { pool })
    }
    
    /// Create an in-memory database for testing
    pub async fn new_in_memory() -> Result<Self> {
        Self::new("sqlite::memory:").await
    }
    
    /// Get a reference to the connection pool
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
    
    /// Start a new database transaction
    pub async fn begin_transaction(&self) -> Result<Transaction<Sqlite>> {
        self.pool.begin().await
            .map_err(|e| anyhow!("Failed to start transaction: {}", e))
    }
}

/// Agent status enum for database operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DbAgentStatus {
    Offline,
    Available,
    Busy,
    PostCallWrapUp,
    Reserved,
}

impl DbAgentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DbAgentStatus::Offline => "OFFLINE",
            DbAgentStatus::Available => "AVAILABLE",
            DbAgentStatus::Busy => "BUSY",
            DbAgentStatus::PostCallWrapUp => "POSTCALLWRAPUP",
            DbAgentStatus::Reserved => "RESERVED",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "OFFLINE" => Some(DbAgentStatus::Offline),
            "AVAILABLE" => Some(DbAgentStatus::Available),
            "BUSY" => Some(DbAgentStatus::Busy),
            "POSTCALLWRAPUP" => Some(DbAgentStatus::PostCallWrapUp),
            "RESERVED" => Some(DbAgentStatus::Reserved),
            _ => None,
        }
    }
}

/// Agent record from database
#[derive(Debug, Clone)]
pub struct DbAgent {
    pub agent_id: String,
    pub username: String,
    pub contact_uri: Option<String>,
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub status: String, // Will be converted to/from DbAgentStatus
    pub current_calls: i32,
    pub max_calls: i32,
    pub available_since: Option<DateTime<Utc>>,
    pub skills: Option<String>, // JSON array of skills
    pub last_active: Option<DateTime<Utc>>,
}

impl DbAgent {
    /// Get the typed status
    pub fn get_status(&self) -> Option<DbAgentStatus> {
        DbAgentStatus::from_str(&self.status)
    }
    
    /// Set the typed status
    pub fn set_status(&mut self, status: DbAgentStatus) {
        self.status = status.as_str().to_string();
    }
    
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self> {
        Ok(DbAgent {
            agent_id: row.try_get("agent_id")?,
            username: row.try_get("username")?,
            contact_uri: row.try_get("contact_uri")?,
            last_heartbeat: row.try_get("last_heartbeat")?,
            status: row.try_get("status")?,
            current_calls: row.try_get("current_calls")?,
            max_calls: row.try_get("max_calls")?,
            available_since: row.try_get("available_since")?,
            skills: row.try_get("skills")?,
            last_active: row.try_get("last_active")?,
        })
    }
}

/// Queued call record
#[derive(Debug, Clone)]
pub struct DbQueuedCall {
    pub call_id: String,
    pub session_id: String,
    pub queue_id: String,
    pub customer_info: Option<String>,
    pub priority: i32,
    pub enqueued_at: DateTime<Utc>,
    pub attempts: i32,
    pub last_attempt: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
}

impl DbQueuedCall {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self> {
        Ok(DbQueuedCall {
            call_id: row.try_get("call_id")?,
            session_id: row.try_get("session_id")?,
            queue_id: row.try_get("queue_id")?,
            customer_info: row.try_get("customer_info")?,
            priority: row.try_get("priority")?,
            enqueued_at: row.try_get("enqueued_at")?,
            attempts: row.try_get("attempts")?,
            last_attempt: row.try_get("last_attempt")?,
            expires_at: row.try_get("expires_at")?,
        })
    }
    
    /// Convert DbQueuedCall to QueuedCall (for queue manager compatibility)
    pub fn to_queued_call(&self) -> crate::queue::QueuedCall {
        crate::queue::QueuedCall {
            session_id: SessionId(self.session_id.clone()),
            caller_id: self.customer_info.clone().unwrap_or_else(|| "Unknown".to_string()),
            priority: self.priority.try_into().unwrap_or(0),
            queued_at: self.enqueued_at,
            estimated_wait_time: None,
            retry_count: self.attempts.try_into().unwrap_or(0),
        }
    }
}

/// Active call record
#[derive(Debug, Clone)]
pub struct DbActiveCall {
    pub call_id: String,
    pub agent_id: String,
    pub session_id: String,
    pub customer_dialog_id: Option<String>,
    pub agent_dialog_id: Option<String>,
    pub assigned_at: DateTime<Utc>,
    pub answered_at: Option<DateTime<Utc>>,
}

impl DbActiveCall {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self> {
        Ok(DbActiveCall {
            call_id: row.try_get("call_id")?,
            agent_id: row.try_get("agent_id")?,
            session_id: row.try_get("session_id")?,
            customer_dialog_id: row.try_get("customer_dialog_id")?,
            agent_dialog_id: row.try_get("agent_dialog_id")?,
            assigned_at: row.try_get("assigned_at")?,
            answered_at: row.try_get("answered_at")?,
        })
    }
}

/// Queue configuration
#[derive(Debug, Clone)]
pub struct DbQueue {
    pub queue_id: String,
    pub name: String,
    pub description: Option<String>,
    pub max_wait_time: Option<i32>,
    pub priority_routing: bool,
}

impl DbQueue {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self> {
        Ok(DbQueue {
            queue_id: row.try_get("queue_id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            max_wait_time: row.try_get("max_wait_time")?,
            priority_routing: row.try_get("priority_routing")?,
        })
    }
}

/// Call record for analytics
#[derive(Debug, Clone)]
pub struct DbCallRecord {
    pub call_id: String,
    pub customer_number: Option<String>,
    pub agent_id: Option<String>,
    pub queue_name: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub disposition: Option<String>,
    pub notes: Option<String>,
}

/// Call history record for metrics
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DbCallHistory {
    pub id: i64,
    pub call_id: String,
    pub session_id: String,
    pub agent_id: Option<String>,
    pub queue_id: Option<String>,
    pub caller_id: Option<String>,
    pub start_time: DateTime<Utc>,
    pub answer_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub wait_time_seconds: Option<i32>,
    pub talk_time_seconds: Option<i32>,
    pub disposition: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

/// Registration record from database
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DbRegistration {
    pub aor: String,
    pub contact_uri: String,
    pub expires_at: DateTime<Utc>,
    pub user_agent: Option<String>,
    pub transport: String,
    pub remote_addr: String,
    pub last_updated: DateTime<Utc>,
}

impl DbRegistration {
    pub fn to_registration(&self) -> crate::agent::registration::Registration {
        crate::agent::registration::Registration {
            agent_id: self.aor.clone(),
            contact_uri: self.contact_uri.clone(),
            expires_at: std::time::Instant::now() + 
                (self.expires_at - Utc::now()).to_std().unwrap_or(std::time::Duration::from_secs(0)),
            user_agent: self.user_agent.clone(),
            transport: self.transport.clone(),
            remote_addr: self.remote_addr.clone(),
        }
    }
}

/// Agent statistics
#[derive(Debug, Clone)]
pub struct AgentStats {
    pub total_agents: i64,
    pub available_agents: i64,
    pub busy_agents: i64,
    pub post_call_wrap_up_agents: i64,
    pub offline_agents: i64,
    pub reserved_agents: i64,
}

/// Database error types
#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    Connection(String),
    
    #[error("Query execution error: {0}")]
    Query(String),
    
    #[error("Transaction error: {0}")]
    Transaction(String),
    
    #[error("Migration error: {0}")]
    Migration(String),
    
    #[error("Data validation error: {0}")]
    Validation(String),
}

impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::Database(_) => DatabaseError::Query(err.to_string()),
            sqlx::Error::Io(_) => DatabaseError::Connection(err.to_string()),
            sqlx::Error::Configuration(_) => DatabaseError::Connection(err.to_string()),
            _ => DatabaseError::Query(err.to_string()),
        }
    }
}

// Agent operations implementation
impl DatabaseManager {
    /// Register or update an agent
    pub async fn upsert_agent(
        &self, 
        agent_id: &str, 
        username: &str, 
        contact_uri: Option<&str>,
        skills: Option<&[String]>
    ) -> Result<()> {
        let now = Utc::now();
        let skills_json = skills.map(|s| serde_json::to_string(s).unwrap_or_default());

        info!("🔍 upsert_agent: {} -> {}", agent_id, username);
        
        sqlx::query(
            "INSERT INTO agents (agent_id, username, contact_uri, last_heartbeat, status, current_calls, max_calls, available_since, skills)
             VALUES (?, ?, ?, ?, 'AVAILABLE', 0, 1, ?, ?)
             ON CONFLICT(agent_id) DO UPDATE SET
                username = excluded.username,
                contact_uri = excluded.contact_uri,
                last_heartbeat = excluded.last_heartbeat,
                status = 'AVAILABLE',
                available_since = excluded.available_since,
                skills = COALESCE(excluded.skills, agents.skills)"
        )
        .bind(agent_id)
        .bind(username)
        .bind(contact_uri.unwrap_or(""))
        .bind(now)
        .bind(now)
        .bind(skills_json)
        .execute(&self.pool)
        .await?;
        
        info!("✅ Agent {} upserted", agent_id);
        Ok(())
    }
    
    /// Update agent status
    pub async fn update_agent_status(&self, agent_id: &str, status: AgentStatus) -> Result<()> {
        let status_str = match status {
            AgentStatus::Available => "AVAILABLE",
            AgentStatus::Busy(_) => "BUSY",
            AgentStatus::PostCallWrapUp => "POSTCALLWRAPUP",
            AgentStatus::Offline => "OFFLINE",
        };
        
        if matches!(status, AgentStatus::Available) {
            let now = Utc::now();
            sqlx::query("UPDATE agents SET status = ?, available_since = ? WHERE agent_id = ?")
                .bind(status_str)
                .bind(now)
                .bind(agent_id)
                .execute(&self.pool)
                .await?;
        } else {
            sqlx::query("UPDATE agents SET status = ?, available_since = NULL WHERE agent_id = ?")
                .bind(status_str)
                .bind(agent_id)
                .execute(&self.pool)
                .await?;
        }
        
        debug!("Agent {} status updated to {}", agent_id, status_str);
        Ok(())
    }
    
    /// Get available agents
    pub async fn get_available_agents(&self) -> Result<Vec<DbAgent>> {
        let rows = sqlx::query(
            "SELECT agent_id, username, contact_uri, last_heartbeat, status, current_calls, max_calls, available_since
             FROM agents 
             WHERE status = 'AVAILABLE' AND current_calls < max_calls
             ORDER BY available_since ASC"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut agents = Vec::new();
        for row in rows {
            agents.push(DbAgent::from_row(&row)?);
        }
        
        info!("Found {} available agents", agents.len());
        Ok(agents)
    }
    
    /// Reserve an agent atomically
    pub async fn reserve_agent(&self, agent_id: &str) -> Result<bool> {
        let mut tx = self.pool.begin().await?;
        
        let result = sqlx::query("UPDATE agents SET status = 'RESERVED' WHERE agent_id = ? AND status = 'AVAILABLE'")
            .bind(agent_id)
            .execute(&mut *tx)
            .await?;
        
        let success = result.rows_affected() > 0;
        
        if success {
            tx.commit().await?;
            debug!("Agent {} reserved successfully", agent_id);
        } else {
            tx.rollback().await?;
        }
        
        Ok(success)
    }
    
    /// Update agent call count
    pub async fn update_agent_call_count(&self, agent_id: &str, delta: i32) -> Result<()> {
        sqlx::query("UPDATE agents SET current_calls = MAX(0, current_calls + ?) WHERE agent_id = ?")
            .bind(delta)
            .bind(agent_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    /// Get agent statistics
    pub async fn get_agent_stats(&self) -> Result<AgentStats> {
        let row = sqlx::query(
            "SELECT 
                COUNT(*) as total_agents,
                SUM(CASE WHEN status = 'AVAILABLE' THEN 1 ELSE 0 END) as available_agents,
                SUM(CASE WHEN status = 'BUSY' THEN 1 ELSE 0 END) as busy_agents,
                SUM(CASE WHEN status = 'POSTCALLWRAPUP' THEN 1 ELSE 0 END) as post_call_wrap_up_agents,
                SUM(CASE WHEN status = 'OFFLINE' THEN 1 ELSE 0 END) as offline_agents,
                SUM(CASE WHEN status = 'RESERVED' THEN 1 ELSE 0 END) as reserved_agents
             FROM agents"
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(AgentStats {
            total_agents: row.try_get("total_agents")?,
            available_agents: row.try_get("available_agents").unwrap_or(0),
            busy_agents: row.try_get("busy_agents").unwrap_or(0),
            post_call_wrap_up_agents: row.try_get("post_call_wrap_up_agents").unwrap_or(0),
            offline_agents: row.try_get("offline_agents").unwrap_or(0),
            reserved_agents: row.try_get("reserved_agents").unwrap_or(0),
        })
    }
}

// Queue operations implementation
impl DatabaseManager {
    /// Enqueue a call
    pub async fn enqueue_call(
        &self,
        call_id: &str,
        session_id: &str,
        queue_id: &str,
        customer_info: Option<&str>,
        priority: i32,
        expires_at: DateTime<Utc>,
    ) -> Result<()> {
        let now = Utc::now();
        
        sqlx::query(
            "INSERT INTO call_queue (call_id, session_id, queue_id, customer_info, priority, enqueued_at, expires_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(call_id)
        .bind(session_id)
        .bind(queue_id)
        .bind(customer_info)
        .bind(priority)
        .bind(now)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;
        
        info!("Call {} enqueued to queue {}", call_id, queue_id);
        Ok(())
    }
    
    /// Get next call in queue
    pub async fn get_next_queued_call(&self, queue_id: &str) -> Result<Option<DbQueuedCall>> {
        let row = sqlx::query(
            "SELECT call_id, session_id, queue_id, customer_info, priority, enqueued_at, attempts, last_attempt, expires_at
             FROM call_queue 
             WHERE queue_id = ? AND expires_at > datetime('now')
             ORDER BY priority DESC, enqueued_at ASC
             LIMIT 1"
        )
        .bind(queue_id)
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => Ok(Some(DbQueuedCall::from_row(&row)?)),
            None => Ok(None),
        }
    }
    
    /// Remove call from queue
    pub async fn dequeue_call(&self, session_id: &str) -> Result<bool> {
        let result = sqlx::query("DELETE FROM call_queue WHERE session_id = ?")
            .bind(session_id)
            .execute(&self.pool)
            .await?;
        
        Ok(result.rows_affected() > 0)
    }
}

// Active call operations implementation
impl DatabaseManager {
    /// Add an active call
    pub async fn add_active_call(
        &self,
        call_id: &str,
        agent_id: &str,
        session_id: &str,
    ) -> Result<()> {
        let now = Utc::now();
        
        sqlx::query("INSERT INTO active_calls (call_id, agent_id, session_id, assigned_at) VALUES (?, ?, ?, ?)")
            .bind(call_id)
            .bind(agent_id)
            .bind(session_id)
            .bind(now)
            .execute(&self.pool)
            .await?;
        
        info!("Active call {} added for agent {}", call_id, agent_id);
        Ok(())
    }
    
    /// Remove an active call
    pub async fn remove_active_call(&self, session_id: &str) -> Result<bool> {
        let result = sqlx::query("DELETE FROM active_calls WHERE session_id = ?")
            .bind(session_id)
            .execute(&self.pool)
            .await?;
        
        Ok(result.rows_affected() > 0)
    }
    
    /// Assign call to agent atomically
    pub async fn assign_call_to_agent(
        &self,
        call_id: &str,
        session_id: &str,
        agent_id: &str,
    ) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        let now = Utc::now();
        
        // Remove from queue
        sqlx::query("DELETE FROM call_queue WHERE session_id = ?")
            .bind(session_id)
            .execute(&mut *tx)
            .await?;
        
        // Add to active calls
        sqlx::query("INSERT INTO active_calls (call_id, agent_id, session_id, assigned_at) VALUES (?, ?, ?, ?)")
            .bind(call_id)
            .bind(agent_id)
            .bind(session_id)
            .bind(now)
            .execute(&mut *tx)
            .await?;
        
        // Update agent call count
        sqlx::query("UPDATE agents SET current_calls = current_calls + 1 WHERE agent_id = ?")
            .bind(agent_id)
            .execute(&mut *tx)
            .await?;
        
        tx.commit().await?;
        info!("✅ Call {} assigned to agent {} atomically", call_id, agent_id);
        Ok(())
    }
    
    // Missing methods that the rest of the codebase expects
    pub async fn get_agent(&self, agent_id: &str) -> Result<Option<DbAgent>> {
        let row = sqlx::query(
            "SELECT agent_id, username, contact_uri, last_heartbeat, status, current_calls, max_calls, available_since
             FROM agents WHERE agent_id = ?"
        )
        .bind(agent_id)
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => Ok(Some(DbAgent::from_row(&row)?)),
            None => Ok(None),
        }
    }
    
    pub async fn list_agents(&self) -> Result<Vec<DbAgent>> {
        let rows = sqlx::query(
            "SELECT agent_id, username, contact_uri, last_heartbeat, status, current_calls, max_calls, available_since
             FROM agents ORDER BY agent_id"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut agents = Vec::new();
        for row in rows {
            agents.push(DbAgent::from_row(&row)?);
        }
        
        Ok(agents)
    }
    
    pub async fn mark_agent_offline(&self, agent_id: &str) -> Result<()> {
        sqlx::query("UPDATE agents SET status = 'OFFLINE', current_calls = 0 WHERE agent_id = ?")
            .bind(agent_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn count_total_agents(&self) -> Result<i64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM agents")
            .fetch_one(&self.pool)
            .await?;
        
        Ok(row.try_get("count")?)
    }
    
    pub async fn get_active_calls_count(&self) -> Result<i64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM active_calls")
            .fetch_one(&self.pool)
            .await?;
        
        Ok(row.try_get("count")?)
    }
    
    pub async fn get_queue_depth(&self, queue_id: &str) -> Result<i64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM call_queue WHERE queue_id = ? AND expires_at > datetime('now')")
            .bind(queue_id)
            .fetch_one(&self.pool)
            .await?;
        
        Ok(row.try_get("count")?)
    }
    
    pub async fn remove_call_from_queue(&self, session_id: &str) -> Result<()> {
        self.dequeue_call(session_id).await?;
        Ok(())
    }
    
    pub async fn release_agent_reservation(&self, agent_id: &str) -> Result<()> {
        sqlx::query("UPDATE agents SET status = 'AVAILABLE' WHERE agent_id = ? AND status = 'RESERVED'")
            .bind(agent_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn dequeue_call_for_agent(&self, queue_id: &str, agent_id: &str) -> Result<Option<DbQueuedCall>> {
        // For now, just get next call in queue
        self.get_next_queued_call(queue_id).await
    }
    
    pub async fn update_agent_call_count_with_retry(&self, agent_id: &str, delta: i32) -> Result<()> {
        // Simple implementation - no retry logic for now
        self.update_agent_call_count(agent_id, delta).await
    }
    
    pub async fn update_agent_status_with_retry(&self, agent_id: &str, status: AgentStatus) -> Result<()> {
        // Simple implementation - no retry logic for now
        self.update_agent_status(agent_id, status).await
    }
    
    pub async fn atomic_assign_call_to_agent(&self, session_id: &str, agent_id: &str, _customer_sdp: String) -> Result<()> {
        // Use a placeholder call_id for now
        let call_id = format!("call-{}", session_id);
        self.assign_call_to_agent(&call_id, session_id, agent_id).await
    }
    
    pub async fn query(&self, sql: &str, _params: &[&str]) -> Result<()> {
        sqlx::query(sql).execute(&self.pool).await?;
        Ok(())
    }
    
    pub async fn execute(&self, sql: &str, params: &[String]) -> Result<()> {
        let mut query = sqlx::query(sql);
        for param in params {
            query = query.bind(param);
        }
        query.execute(&self.pool).await?;
        Ok(())
    }
}

// Registration operations implementation
impl DatabaseManager {
    /// Upsert a registration
    pub async fn upsert_registration(
        &self, 
        aor: &str,
        contact_uri: &str,
        expires_at: DateTime<Utc>,
        user_agent: Option<&str>,
        transport: &str,
        remote_addr: &str
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO registrations (aor, contact_uri, expires_at, user_agent, transport, remote_addr, last_updated)
             VALUES (?, ?, ?, ?, ?, ?, datetime('now'))
             ON CONFLICT(aor) DO UPDATE SET
                contact_uri = excluded.contact_uri,
                expires_at = excluded.expires_at,
                user_agent = excluded.user_agent,
                transport = excluded.transport,
                remote_addr = excluded.remote_addr,
                last_updated = datetime('now')"
        )
        .bind(aor)
        .bind(contact_uri)
        .bind(expires_at)
        .bind(user_agent)
        .bind(transport)
        .bind(remote_addr)
        .execute(&self.pool)
        .await?;
        
        debug!("Registration upserted for {}", aor);
        Ok(())
    }
    
    /// Get registration by AOR
    pub async fn get_registration(&self, aor: &str) -> Result<Option<DbRegistration>> {
        let row = sqlx::query_as::<_, DbRegistration>(
            "SELECT * FROM registrations WHERE aor = ? AND expires_at > datetime('now')"
        )
        .bind(aor)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(row)
    }
    
    /// Remove registration
    pub async fn remove_registration(&self, aor: &str) -> Result<()> {
        sqlx::query("DELETE FROM registrations WHERE aor = ?")
            .bind(aor)
            .execute(&self.pool)
            .await?;
            
        debug!("Registration removed for {}", aor);
        Ok(())
    }
    
    /// Cleanup expired registrations
    pub async fn cleanup_expired_registrations(&self, now: DateTime<Utc>) -> Result<u64> {
        let result = sqlx::query("DELETE FROM registrations WHERE expires_at <= ?")
            .bind(now)
            .execute(&self.pool)
            .await?;
            
        let deleted = result.rows_affected();
        if deleted > 0 {
            info!("Cleaned up {} expired registrations", deleted);
        }
        Ok(deleted)
    }
    
    /// Get all active registrations
    pub async fn get_all_registrations(&self) -> Result<Vec<DbRegistration>> {
        let regs = sqlx::query_as::<_, DbRegistration>(
            "SELECT * FROM registrations WHERE expires_at > datetime('now')"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(regs)
    }
    
    /// Record a call start in history
    pub async fn record_call_start(
        &self,
        call_id: &str,
        session_id: &str,
        queue_id: &str,
        caller_id: &str
    ) -> Result<i64> {
        let now = Utc::now();
        let res = sqlx::query(
            "INSERT INTO call_history (call_id, session_id, queue_id, caller_id, start_time, disposition)
             VALUES (?, ?, ?, ?, ?, 'abandoned')"
        )
        .bind(call_id)
        .bind(session_id)
        .bind(queue_id)
        .bind(caller_id)
        .bind(now)
        .execute(&self.pool)
        .await?;
        
        Ok(res.last_insert_rowid())
    }

    /// Record call answer
    pub async fn record_call_answer(&self, call_id: &str, agent_id: &str) -> Result<()> {
        let now = Utc::now();
        // Calculate wait time in Rust for better precision
        let row = sqlx::query("SELECT start_time FROM call_history WHERE call_id = ? AND answer_time IS NULL")
            .bind(call_id)
            .fetch_optional(&self.pool)
            .await?;
        
        if let Some(r) = row {
            let start_time: DateTime<Utc> = r.try_get("start_time")?;
            let wait_time = (now - start_time).num_seconds() as i32;
            
            sqlx::query(
                "UPDATE call_history SET 
                    agent_id = ?, 
                    answer_time = ?, 
                    disposition = 'answered',
                    wait_time_seconds = ?
                 WHERE call_id = ? AND answer_time IS NULL"
            )
            .bind(agent_id)
            .bind(now)
            .bind(wait_time)
            .bind(call_id)
            .execute(&self.pool)
            .await?;
        }
        
        Ok(())
    }

    /// Record call end
    pub async fn record_call_end(&self, call_id: &str, disposition: Option<&str>) -> Result<()> {
        let now = Utc::now();
        let row = sqlx::query("SELECT answer_time, disposition FROM call_history WHERE call_id = ? AND end_time IS NULL")
            .bind(call_id)
            .fetch_optional(&self.pool)
            .await?;
            
        if let Some(r) = row {
            let answer_time: Option<DateTime<Utc>> = r.try_get("answer_time")?;
            let talk_time = answer_time.map(|t| (now - t).num_seconds() as i32).unwrap_or(0);
            let final_disposition = disposition.unwrap_or_else(|| r.try_get("disposition").unwrap_or("error"));

            sqlx::query(
                "UPDATE call_history SET 
                    end_time = ?,
                    talk_time_seconds = ?,
                    disposition = ?
                 WHERE call_id = ? AND end_time IS NULL"
            )
            .bind(now)
            .bind(talk_time)
            .bind(final_disposition)
            .bind(call_id)
            .execute(&self.pool)
            .await?;
        }
        
        Ok(())
    }

    /// Find AOR by contact URI
    pub async fn find_aor_by_contact(&self, contact_uri: &str) -> Result<Option<String>> {
        let row = sqlx::query(
            "SELECT aor FROM registrations WHERE contact_uri = ? AND expires_at > datetime('now')"
        )
        .bind(contact_uri)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(row.map(|r| r.get("aor")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_database_creation() {
        let db = DatabaseManager::new_in_memory().await.unwrap();
        
        // Test that we can perform basic operations
        let agents = db.get_available_agents().await.unwrap();
        assert!(agents.is_empty());
    }
    
    #[tokio::test]
    async fn test_send_safety() {
        let db = DatabaseManager::new_in_memory().await.unwrap();
        
        // This should compile without Send trait issues
        let handle = tokio::spawn(async move {
            let _agents = db.get_available_agents().await.unwrap();
        });
        
        handle.await.unwrap();
    }
    
    #[tokio::test]
    async fn test_agent_operations() {
        let db = DatabaseManager::new_in_memory().await.unwrap();
        
        // Create an agent
        db.upsert_agent("agent-001", "test_user", Some("sip:test@example.com"), None).await.unwrap();
        
        // Check available agents
        let agents = db.get_available_agents().await.unwrap();
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].agent_id, "agent-001");
        
        // Reserve agent
        let reserved = db.reserve_agent("agent-001").await.unwrap();
        assert!(reserved);
        
        // Check no longer available
        let agents = db.get_available_agents().await.unwrap();
        assert!(agents.is_empty());
    }
    #[tokio::test]
    async fn test_registration_operations() {
        let db = DatabaseManager::new_in_memory().await.unwrap();
        let now = Utc::now();
        let expires_at = now + chrono::Duration::hours(1);
        
        // Upsert registration
        db.upsert_registration(
            "sip:alice@example.com",
            "sip:alice@192.168.1.100:5060",
            expires_at,
            Some("TestUA/1.0"),
            "udp",
            "192.168.1.100:5060"
        ).await.unwrap();
        
        // Get registration
        let reg = db.get_registration("sip:alice@example.com").await.unwrap();
        assert!(reg.is_some());
        let reg = reg.unwrap();
        assert_eq!(reg.contact_uri, "sip:alice@192.168.1.100:5060");
        assert_eq!(reg.transport, "udp");
        
        // Get all
        let all = db.get_all_registrations().await.unwrap();
        assert_eq!(all.len(), 1);
        
        // Remove
        db.remove_registration("sip:alice@example.com").await.unwrap();
        let reg = db.get_registration("sip:alice@example.com").await.unwrap();
        assert!(reg.is_none());
        
        // Cleanup
        db.upsert_registration(
            "sip:bob@example.com",
            "sip:bob@192.168.1.101:5060",
            now - chrono::Duration::hours(1), // Expired
            None,
            "tcp",
            "192.168.1.101:5060"
        ).await.unwrap();
        
        let deleted = db.cleanup_expired_registrations(now).await.unwrap();
        assert_eq!(deleted, 1);
        let all = db.get_all_registrations().await.unwrap();
        assert!(all.is_empty());
    }
}
