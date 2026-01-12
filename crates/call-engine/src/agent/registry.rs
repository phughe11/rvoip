//! # Agent Registry Management
//!
//! This module provides comprehensive agent registry functionality for call center operations,
//! including agent registration, status management, session tracking, and skill-based routing
//! support. The registry maintains the authoritative source of agent information and state
//! within the call center system.
//!
//! ## Overview
//!
//! The Agent Registry serves as the central management system for all call center agents,
//! tracking their availability, capabilities, and current status. It provides the foundation
//! for call routing, agent monitoring, and workforce management within the call center
//! infrastructure.
//!
//! ## Key Features
//!
//! - **Agent Registration**: Complete agent onboarding and profile management
//! - **Status Tracking**: Real-time agent status monitoring and updates
//! - **Session Management**: Agent login/logout and session correlation
//! - **Skill Management**: Skill-based routing and capability tracking
//! - **Statistics**: Comprehensive agent statistics and reporting
//! - **Availability Detection**: Integration with availability tracking systems
//!
//! ## Agent Lifecycle
//!
//! ### Registration Phase
//! 1. Agent profile creation with skills and capabilities
//! 2. Department and extension assignment
//! 3. Contact information and routing preferences
//! 4. Initial status set to Offline
//!
//! ### Login Phase
//! 1. Agent establishes session with call center
//! 2. SIP registration or web interface login
//! 3. Status transitions to Available
//! 4. Agent becomes eligible for call routing
//!
//! ### Active Phase
//! 1. Agent receives calls based on routing rules
//! 2. Status updates reflect current activity
//! 3. Call handling and post-call wrap-up
//! 4. Manual status changes as needed
//!
//! ### Logout Phase
//! 1. Agent initiates logout process
//! 2. Calls are gracefully handled or transferred
//! 3. Session is terminated
//! 4. Status transitions to Offline
//!
//! ## Status Management
//!
//! ### Available Status
//! - Agent is ready to receive calls
//! - No active calls in progress
//! - All systems operational
//! - Eligible for automatic call distribution
//!
//! ### Busy Status
//! - Agent is handling one or more calls
//! - May have capacity for additional calls (if configured)
//! - Not available for new automatic distribution
//! - Status includes list of active call sessions
//!
//! ### Post-Call Wrap-Up
//! - Agent is completing call documentation
//! - Not available for new calls
//! - Temporary status with configurable duration
//! - Automatically transitions to Available
//!
//! ### Offline Status
//! - Agent is not logged in or available
//! - No active session exists
//! - Not eligible for call routing
//! - Default status for unregistered agents
//!
//! ## Examples
//!
//! ### Basic Agent Registration
//!
//! ```rust
//! use rvoip_call_engine::agent::{AgentRegistry, Agent, AgentStatus};
//! 
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut registry = AgentRegistry::new();
//! 
//! // Create a new agent profile
//! let agent = Agent {
//!     id: "agent-001".to_string(),
//!     sip_uri: "sip:alice@call-center.com".to_string(),
//!     display_name: "Alice Smith".to_string(),
//!     skills: vec!["sales".to_string(), "english".to_string(), "billing".to_string()],
//!     max_concurrent_calls: 2,
//!     status: AgentStatus::Offline,
//!     department: Some("Sales".to_string()),
//!     extension: Some("101".to_string()),
//! };
//! 
//! // Register the agent
//! let agent_id = registry.register_agent(agent).await?;
//! println!("✅ Agent registered with ID: {}", agent_id);
//! 
//! // Update agent status
//! registry.update_agent_status(&agent_id, AgentStatus::Available)?;
//! println!("🟢 Agent is now available for calls");
//! # Ok(())
//! # }
//! ```
//!
//! ### Session Management
//!
//! ```rust
//! use rvoip_call_engine::agent::{AgentRegistry, AgentStatus};
//! use rvoip_session_core::SessionId;
//! 
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut registry = AgentRegistry::new();
//! 
//! // Simulate agent login
//! let session_id = SessionId::new();
//! registry.set_agent_session("agent-001".to_string(), session_id.clone())?;
//! 
//! println!("🔗 Agent session established: {}", session_id);
//! 
//! // Verify agent is available
//! if let Some(status) = registry.get_agent_status("agent-001") {
//!     match status {
//!         AgentStatus::Available => println!("✅ Agent ready for calls"),
//!         AgentStatus::Busy(calls) => println!("📞 Agent busy with {} calls", calls.len()),
//!         AgentStatus::PostCallWrapUp => println!("📝 Agent in post-call wrap-up"),
//!         AgentStatus::Offline => println!("❌ Agent offline"),
//!     }
//! }
//! 
//! // Later, agent logout
//! registry.remove_agent_session("agent-001")?;
//! println!("🔌 Agent session terminated");
//! # Ok(())
//! # }
//! ```
//!
//! ### Skill-Based Agent Discovery
//!
//! ```rust
//! use rvoip_call_engine::agent::{AgentRegistry, Agent, AgentStatus};
//! 
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut registry = AgentRegistry::new();
//! 
//! // Register agents with different skills
//! let agents = vec![
//!     Agent {
//!         id: "agent-sales-01".to_string(),
//!         sip_uri: "sip:sales1@call-center.com".to_string(),
//!         display_name: "Sales Agent 1".to_string(),
//!         skills: vec!["sales".to_string(), "english".to_string()],
//!         max_concurrent_calls: 3,
//!         status: AgentStatus::Available,
//!         department: Some("Sales".to_string()),
//!         extension: Some("201".to_string()),
//!     },
//!     Agent {
//!         id: "agent-support-01".to_string(),
//!         sip_uri: "sip:support1@call-center.com".to_string(),
//!         display_name: "Support Agent 1".to_string(),
//!         skills: vec!["support".to_string(), "technical".to_string(), "spanish".to_string()],
//!         max_concurrent_calls: 2,
//!         status: AgentStatus::Available,
//!         department: Some("Support".to_string()),
//!         extension: Some("301".to_string()),
//!     },
//! ];
//! 
//! for agent in agents {
//!     registry.register_agent(agent).await?;
//! }
//! 
//! // Find agents with specific skills
//! let sales_agents = registry.find_agents_with_skills(&["sales".to_string()]).await?;
//! println!("💼 Found {} sales agents", sales_agents.len());
//! 
//! let multilingual_agents = registry.find_agents_with_skills(&["spanish".to_string()]).await?;
//! println!("🌍 Found {} Spanish-speaking agents", multilingual_agents.len());
//! # Ok(())
//! # }
//! ```
//!
//! ### Agent Statistics and Monitoring
//!
//! ```rust
//! use rvoip_call_engine::agent::{AgentRegistry, Agent, AgentStatus};
//! 
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut registry = AgentRegistry::new();
//! 
//! // Register multiple agents with different statuses
//! let agent_configs = vec![
//!     ("agent-001", AgentStatus::Available),
//!     ("agent-002", AgentStatus::Available),
//!     ("agent-003", AgentStatus::Busy(vec![])),
//!     ("agent-004", AgentStatus::PostCallWrapUp),
//!     ("agent-005", AgentStatus::Offline),
//! ];
//! 
//! for (id, status) in agent_configs {
//!     let agent = Agent {
//!         id: id.to_string(),
//!         sip_uri: format!("sip:{}@call-center.com", id),
//!         display_name: format!("Agent {}", id),
//!         skills: vec!["general".to_string()],
//!         max_concurrent_calls: 1,
//!         status: status.clone(),
//!         department: Some("General".to_string()),
//!         extension: None,
//!     };
//!     
//!     registry.register_agent(agent).await?;
//!     registry.update_agent_status(id, status)?;
//! }
//! 
//! // Get comprehensive statistics
//! let stats = registry.get_statistics();
//! 
//! println!("📊 Agent Statistics:");
//! println!("  Total agents: {}", stats.total);
//! println!("  Available: {} ({:.1}%)", stats.available, 
//!          stats.available as f64 / stats.total as f64 * 100.0);
//! println!("  Busy: {} ({:.1}%)", stats.busy,
//!          stats.busy as f64 / stats.total as f64 * 100.0);
//! println!("  Post-call wrap-up: {}", stats.post_call_wrap_up);
//! println!("  Offline: {}", stats.offline);
//! 
//! // Find available agents for routing
//! let available_agents = registry.find_available_agents();
//! println!("🟢 Available for routing: {:?}", available_agents);
//! # Ok(())
//! # }
//! ```
//!
//! ### Advanced Status Management
//!
//! ```rust
//! use rvoip_call_engine::agent::{AgentRegistry, AgentStatus};
//! use rvoip_session_core::SessionId;
//! 
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut registry = AgentRegistry::new();
//! 
//! let agent_id = "agent-advanced";
//! 
//! // Simulate call handling workflow
//! println!("📞 Incoming call for agent");
//! 
//! // Agent receives first call
//! let call_session_1 = SessionId::new();
//! registry.update_agent_status(agent_id, AgentStatus::Busy(vec![call_session_1.clone()]))?;
//! 
//! // Agent receives second call (if configured for multiple)
//! let call_session_2 = SessionId::new();
//! registry.update_agent_status(agent_id, AgentStatus::Busy(vec![call_session_1, call_session_2]))?;
//! 
//! println!("🏃 Agent handling multiple calls");
//! 
//! // Calls complete, agent enters wrap-up
//! registry.update_agent_status(agent_id, AgentStatus::PostCallWrapUp)?;
//! println!("📝 Agent in post-call wrap-up");
//! 
//! // Wrap-up complete, agent available again
//! registry.update_agent_status(agent_id, AgentStatus::Available)?;
//! println!("✅ Agent available for new calls");
//! # Ok(())
//! # }
//! ```
//!
//! ### Integration with External Systems
//!
//! ```rust
//! use rvoip_call_engine::agent::{AgentRegistry, Agent, AgentStatus};
//! 
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut registry = AgentRegistry::new();
//! 
//! // Integration with HR system
//! let hr_data = vec![
//!     ("alice.smith", "Sales", "101", vec!["sales", "english"]),
//!     ("bob.jones", "Support", "201", vec!["support", "technical"]),
//!     ("carol.garcia", "Billing", "301", vec!["billing", "spanish", "english"]),
//! ];
//! 
//! for (username, dept, ext, skills) in hr_data {
//!     let agent = Agent {
//!         id: format!("hr-{}", username),
//!         sip_uri: format!("sip:{}@call-center.com", username),
//!         display_name: username.replace(".", " "),
//!         skills: skills.into_iter().map(|s| s.to_string()).collect(),
//!         max_concurrent_calls: 2,
//!         status: AgentStatus::Offline,
//!         department: Some(dept.to_string()),
//!         extension: Some(ext.to_string()),
//!     };
//!     
//!     let agent_id = registry.register_agent(agent).await?;
//!     println!("👤 Imported agent from HR: {}", agent_id);
//! }
//! 
//! // Integration with workforce management
//! let wfm_schedules = vec![
//!     ("hr-alice.smith", "09:00-17:00", AgentStatus::Available),
//!     ("hr-bob.jones", "13:00-21:00", AgentStatus::Available),
//!     ("hr-carol.garcia", "off-shift", AgentStatus::Offline),
//! ];
//! 
//! for (agent_id, schedule, status) in wfm_schedules {
//!     registry.update_agent_status(agent_id, status)?;
//!     println!("📅 Updated schedule for {}: {}", agent_id, schedule);
//! }
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;
use tracing::{info, warn, error};

use rvoip_session_core::SessionId;

use crate::error::{CallCenterError, Result};
use crate::agent::{Agent, AgentStatus, AgentId};
use crate::database::DatabaseManager;

/// Agent registry for managing call center agents
///
/// The `AgentRegistry` serves as the central repository for all agent information
/// and state within the call center system. It provides comprehensive agent
/// management capabilities including registration, status tracking, session
/// management, and skill-based routing support.
///
/// ## Thread Safety
///
/// This registry is designed for single-threaded use within the call center
/// engine. For multi-threaded scenarios, wrap in appropriate synchronization
/// primitives (Arc<Mutex<AgentRegistry>>).
///
/// ## Persistence
///
/// The current implementation maintains agent information in memory. For
/// production systems, consider implementing database persistence for agent
/// profiles and state recovery after system restarts.
pub struct AgentRegistry {
    /// Active agent sessions (agent_id -> session_id)
    active_sessions: HashMap<String, SessionId>,
    
    /// Current agent status tracking
    agent_status: HashMap<String, AgentStatus>,

    /// Database manager for persistence
    db: Option<DatabaseManager>,
}

// Basic types moved to types.rs

impl AgentRegistry {
    /// Create a new agent registry
    ///
    /// Initializes an empty agent registry ready to manage call center agents.
    /// The registry starts with no registered agents and will need to be
    /// populated through agent registration calls.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rvoip_call_engine::agent::AgentRegistry;
    /// 
    /// let registry = AgentRegistry::new();
    /// println!("Agent registry initialized");
    /// ```
    pub fn new() -> Self {
        Self {
            active_sessions: HashMap::new(),
            agent_status: HashMap::new(),
            db: None,
        }
    }

    /// Attach a database manager to the registry
    pub fn with_db(mut self, db: DatabaseManager) -> Self {
        self.db = Some(db);
        self
    }

    /// Load agents and their states from the database
    pub async fn load_from_db(&mut self) -> Result<()> {
        let db = match &self.db {
            Some(db) => db,
            None => return Ok(()),
        };

        info!("📂 Loading agents from database...");
        let db_agents = db.list_agents().await
            .map_err(|e| CallCenterError::database(&format!("Failed to list agents: {}", e)))?;

        for db_agent in db_agents {
            let status = match db_agent.status.as_str() {
                "AVAILABLE" => AgentStatus::Available,
                "BUSY" => AgentStatus::Busy(vec![]),
                "POSTCALLWRAPUP" => AgentStatus::PostCallWrapUp,
                "RESERVED" => AgentStatus::Available, // Reserved is transient, treat as available for logic
                _ => AgentStatus::Offline,
            };
            self.agent_status.insert(db_agent.agent_id.clone(), status);
            // Note: active_sessions currently aren't persisted in the agents table
            // In the future, we might want a separate sessions table
        }

        info!("✅ Loaded {} agents from database", self.agent_status.len());
        Ok(())
    }
    
    /// Register a new agent
    ///
    /// Adds a new agent to the registry with their complete profile information.
    /// The agent's initial status is set according to the Agent struct, and
    /// they become available for session management and call routing.
    ///
    /// # Arguments
    ///
    /// * `agent` - Complete agent profile with identification and capabilities
    ///
    /// # Returns
    ///
    /// `Ok(String)` with the agent ID if successful, or error if registration fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rvoip_call_engine::agent::{AgentRegistry, Agent, AgentStatus};
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut registry = AgentRegistry::new();
    /// 
    /// let agent = Agent {
    ///     id: "agent-001".to_string(),
    ///     sip_uri: "sip:alice@call-center.com".to_string(),
    ///     display_name: "Alice Smith".to_string(),
    ///     skills: vec!["sales".to_string(), "english".to_string()],
    ///     max_concurrent_calls: 2,
    ///     status: AgentStatus::Offline,
    ///     department: Some("Sales".to_string()),
    ///     extension: Some("101".to_string()),
    /// };
    /// 
    /// let agent_id = registry.register_agent(agent).await?;
    /// println!("Agent registered: {}", agent_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn register_agent(&mut self, agent: Agent) -> Result<String> {
        info!("👤 Registering agent: {} ({})", agent.display_name, agent.sip_uri);
        
        let agent_id = agent.id.clone();
        
        // Update database if available
        if let Some(db) = &self.db {
            let username = agent.sip_uri
                .strip_prefix("sip:")
                .and_then(|s| s.split('@').next())
                .unwrap_or(&agent.id)
                .to_string();

            db.upsert_agent(&agent_id, &username, Some(&agent.sip_uri)).await
                .map_err(|e| CallCenterError::database(&format!("Failed to upsert agent: {}", e)))?;
            
            db.update_agent_status(&agent_id, agent.status.clone()).await
                .map_err(|e| CallCenterError::database(&format!("Failed to update initial status: {}", e)))?;
        }

        self.agent_status.insert(agent_id.clone(), agent.status.clone());
        
        info!("✅ Agent registered: {}", agent_id);
        Ok(agent_id)
    }
    
    /// Update agent status
    ///
    /// Changes the operational status of an agent. This method is used to
    /// reflect changes in agent availability, call handling state, or
    /// administrative status changes.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Identifier of the agent to update
    /// * `status` - New agent status
    ///
    /// # Returns
    ///
    /// `Ok(())` if status updated successfully, or error if agent not found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rvoip_call_engine::agent::{AgentRegistry, AgentStatus};
    /// 
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut registry = AgentRegistry::new();
    /// 
    /// // Update agent to available status
    /// registry.update_agent_status("agent-001", AgentStatus::Available)?;
    /// 
    /// // Update agent to busy with specific call
    /// use rvoip_session_core::SessionId;
    /// let call_session = SessionId::new();
    /// registry.update_agent_status("agent-001", AgentStatus::Busy(vec![call_session]))?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_agent_status(&mut self, agent_id: &str, status: AgentStatus) -> Result<()> {
        info!("🔄 Agent {} status: {:?}", agent_id, status);
        
        if self.agent_status.contains_key(agent_id) {
            // Update database if available
            if let Some(db) = &self.db {
                db.update_agent_status(agent_id, status.clone()).await
                    .map_err(|e| CallCenterError::database(&format!("Failed to update status in DB: {}", e)))?;
            }

            self.agent_status.insert(agent_id.to_string(), status);
            Ok(())
        } else {
            Err(CallCenterError::not_found(format!("Agent not found: {}", agent_id)))
        }
    }
    
    /// Set agent session (when agent logs in)
    ///
    /// Establishes a session for an agent, typically called when the agent
    /// logs into the system via SIP registration or web interface. This
    /// automatically transitions the agent to Available status if they
    /// were previously registered.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Identifier of the agent
    /// * `session_id` - Session identifier for the agent connection
    ///
    /// # Returns
    ///
    /// `Ok(())` if session established successfully, or error if agent not found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rvoip_call_engine::agent::AgentRegistry;
    /// use rvoip_session_core::SessionId;
    /// 
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut registry = AgentRegistry::new();
    /// 
    /// let session_id = SessionId::new();
    /// registry.set_agent_session("agent-001".to_string(), session_id)?;
    /// 
    /// println!("Agent session established");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_agent_session(&mut self, agent_id: String, session_id: SessionId) -> Result<()> {
        info!("🔗 Agent {} session: {}", agent_id, session_id);
        
        if self.agent_status.contains_key(&agent_id) {
            self.active_sessions.insert(agent_id.clone(), session_id);
            self.update_agent_status(&agent_id, AgentStatus::Available).await?;
            Ok(())
        } else {
            Err(CallCenterError::not_found(format!("Agent not found: {}", agent_id)))
        }
    }
    
    /// Remove agent session (when agent logs out)
    ///
    /// Terminates an agent's session and transitions them to Offline status.
    /// This is typically called when an agent explicitly logs out or when
    /// a session is disconnected.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Identifier of the agent
    ///
    /// # Returns
    ///
    /// `Ok(())` if session removed successfully, or error if no active session.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rvoip_call_engine::agent::AgentRegistry;
    /// 
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut registry = AgentRegistry::new();
    /// 
    /// registry.remove_agent_session("agent-001")?;
    /// println!("Agent session terminated");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn remove_agent_session(&mut self, agent_id: &str) -> Result<()> {
        info!("🔌 Agent {} logged out", agent_id);
        
        if self.active_sessions.remove(agent_id).is_some() {
            self.update_agent_status(agent_id, AgentStatus::Offline).await?;
            Ok(())
        } else {
            Err(CallCenterError::not_found(format!("No active session for agent: {}", agent_id)))
        }
    }
    
    /// Get agent by ID
    ///
    /// Retrieves complete agent profile information by agent identifier.
    /// This method will be enhanced with database integration to load
    /// agent profiles from persistent storage.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Identifier of the agent to retrieve
    ///
    /// # Returns
    ///
    /// `Ok(Some(Agent))` if agent found, `Ok(None)` if not found, or error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rvoip_call_engine::agent::AgentRegistry;
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let registry = AgentRegistry::new();
    /// 
    /// match registry.get_agent("agent-001").await? {
    ///     Some(agent) => {
    ///         println!("Agent: {} ({})", agent.display_name, agent.sip_uri);
    ///         println!("Skills: {:?}", agent.skills);
    ///     }
    ///     None => {
    ///         println!("Agent not found");
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_agent(&self, agent_id: &str) -> Result<Option<Agent>> {
        // Try memory first
        if let Some(status) = self.agent_status.get(agent_id) {
            // Ideally Agent should be stored in memory too, but currently only status is.
            // For now, if in memory, we might still want to fetch full profile from DB if not cached.
            if let Some(db) = &self.db {
                match db.get_agent(agent_id).await {
                    Ok(Some(db_agent)) => {
                        return Ok(Some(Agent {
                            id: db_agent.agent_id,
                            sip_uri: db_agent.contact_uri.unwrap_or_default(),
                            display_name: db_agent.username,
                            skills: vec![], // TODO: Skills table
                            max_concurrent_calls: db_agent.max_calls as u32,
                            status: status.clone(),
                            department: None,
                            extension: None,
                        }));
                    }
                    _ => {}
                }
            }
        }
        
        // Fallback to database
        if let Some(db) = &self.db {
            match db.get_agent(agent_id).await {
                Ok(Some(db_agent)) => {
                    let status = match db_agent.status.as_str() {
                        "AVAILABLE" => AgentStatus::Available,
                        "BUSY" => AgentStatus::Busy(vec![]),
                        "POSTCALLWRAPUP" => AgentStatus::PostCallWrapUp,
                        _ => AgentStatus::Offline,
                    };
                    Ok(Some(Agent {
                        id: db_agent.agent_id,
                        sip_uri: db_agent.contact_uri.unwrap_or_default(),
                        display_name: db_agent.username,
                        skills: vec![],
                        max_concurrent_calls: db_agent.max_calls as u32,
                        status,
                        department: None,
                        extension: None,
                    }))
                }
                _ => Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    
    /// Get list of available agents
    pub fn get_available_agents(&self) -> Vec<AgentId> {
        self.agent_status.iter()
            .filter(|(_, status)| matches!(status, AgentStatus::Available))
            .map(|(id, _)| AgentId(id.clone()))
            .collect()
    }

    /// List all agents currently tracked in memory
    pub fn list_agents(&self) -> Vec<(String, AgentStatus)> {
        self.agent_status.iter()
            .map(|(id, status)| (id.clone(), status.clone()))
            .collect()
    }

    /// Get agent status
    ///
    /// Retrieves the current operational status of an agent. This reflects
    /// the agent's availability and current activity state.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Identifier of the agent
    ///
    /// # Returns
    ///
    /// `Some(&AgentStatus)` if agent found, `None` if not found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rvoip_call_engine::agent::{AgentRegistry, AgentStatus};
    /// 
    /// let registry = AgentRegistry::new();
    /// 
    /// match registry.get_agent_status("agent-001") {
    ///     Some(AgentStatus::Available) => println!("Agent is available"),
    ///     Some(AgentStatus::Busy(calls)) => println!("Agent busy with {} calls", calls.len()),
    ///     Some(AgentStatus::PostCallWrapUp) => println!("Agent in wrap-up"),
    ///     Some(AgentStatus::Offline) => println!("Agent offline"),
    ///     None => println!("Agent not found"),
    /// }
    /// ```
    pub fn get_agent_status(&self, agent_id: &str) -> Option<&AgentStatus> {
        self.agent_status.get(agent_id)
    }
    
    /// Get agent session
    ///
    /// Retrieves the active session identifier for an agent. This is used
    /// for session management and correlation with communication systems.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - Identifier of the agent
    ///
    /// # Returns
    ///
    /// `Some(&SessionId)` if agent has active session, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rvoip_call_engine::agent::AgentRegistry;
    /// 
    /// let registry = AgentRegistry::new();
    /// 
    /// if let Some(session_id) = registry.get_agent_session("agent-001") {
    ///     println!("Agent session: {}", session_id);
    /// } else {
    ///     println!("No active session for agent");
    /// }
    /// ```
    pub fn get_agent_session(&self, agent_id: &str) -> Option<&SessionId> {
        self.active_sessions.get(agent_id)
    }
    
    /// Find available agents (excludes agents in post-call wrap-up)
    ///
    /// Returns a list of agent IDs that are currently available for call
    /// routing. This excludes agents in post-call wrap-up status as they
    /// are temporarily unavailable.
    ///
    /// # Returns
    ///
    /// Vector of agent IDs that are available for routing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rvoip_call_engine::agent::AgentRegistry;
    /// 
    /// let registry = AgentRegistry::new();
    /// 
    /// let available_agents = registry.find_available_agents();
    /// println!("Available agents: {:?}", available_agents);
    /// 
    /// if available_agents.is_empty() {
    ///     println!("No agents available for routing");
    /// } else {
    ///     println!("Found {} available agents", available_agents.len());
    /// }
    /// ```
    pub fn find_available_agents(&self) -> Vec<String> {
        self.agent_status.iter()
            .filter(|(_, status)| matches!(status, AgentStatus::Available))
            .map(|(id, _)| id.clone())
            .collect()
    }
    
    /// Find agents with specific skills
    ///
    /// Searches for agents that possess all the specified skills. This method
    /// will be enhanced with database integration to query agent skill profiles
    /// from persistent storage.
    ///
    /// # Arguments
    ///
    /// * `required_skills` - List of skills that agents must possess
    ///
    /// # Returns
    ///
    /// `Ok(Vec<String>)` with agent IDs that have all required skills, or error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rvoip_call_engine::agent::AgentRegistry;
    /// 
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let registry = AgentRegistry::new();
    /// 
    /// // Find agents with sales and English skills
    /// let sales_agents = registry.find_agents_with_skills(&[
    ///     "sales".to_string(),
    ///     "english".to_string(),
    /// ]).await?;
    /// 
    /// println!("Found {} sales agents with English skills", sales_agents.len());
    /// 
    /// // Find technical support agents
    /// let tech_agents = registry.find_agents_with_skills(&[
    ///     "technical".to_string(),
    ///     "support".to_string(),
    /// ]).await?;
    /// 
    /// println!("Found {} technical support agents", tech_agents.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn find_agents_with_skills(&self, required_skills: &[String]) -> Result<Vec<String>> {
        // TODO: Query database for agents with required skills
        warn!("🚧 find_agents_with_skills not yet implemented");
        Ok(Vec::new())
    }
    
    /// Get all agent statistics
    ///
    /// Returns comprehensive statistics about all agents in the registry,
    /// including status distribution and availability metrics. This is
    /// useful for monitoring and reporting purposes.
    ///
    /// # Returns
    ///
    /// `AgentStats` structure with detailed statistics.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rvoip_call_engine::agent::AgentRegistry;
    /// 
    /// let registry = AgentRegistry::new();
    /// let stats = registry.get_statistics();
    /// 
    /// println!("Agent Statistics:");
    /// println!("  Total: {}", stats.total);
    /// println!("  Available: {} ({:.1}%)", stats.available,
    ///          stats.available as f64 / stats.total as f64 * 100.0);
    /// println!("  Busy: {}", stats.busy);
    /// println!("  Post-call wrap-up: {}", stats.post_call_wrap_up);
    /// println!("  Offline: {}", stats.offline);
    /// ```
    pub fn get_statistics(&self) -> AgentStats {
        let total = self.agent_status.len();
        let available = self.agent_status.values()
            .filter(|a| matches!(a, AgentStatus::Available))
            .count();
        let busy = self.agent_status.values()
            .filter(|a| matches!(a, AgentStatus::Busy(_)))
            .count();
        let post_call_wrap_up = self.agent_status.values()
            .filter(|a| matches!(a, AgentStatus::PostCallWrapUp))
            .count();
        let offline = self.agent_status.values()
            .filter(|a| matches!(a, AgentStatus::Offline))
            .count();
        
        AgentStats { total, available, busy, post_call_wrap_up, offline }
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Agent statistics summary
///
/// Provides a comprehensive overview of agent status distribution and
/// availability metrics across the entire call center.
#[derive(Debug, Clone)]
pub struct AgentStats {
    /// Total number of registered agents
    pub total: usize,
    
    /// Number of agents currently available for calls
    pub available: usize,
    
    /// Number of agents currently busy with calls
    pub busy: usize,
    
    /// Number of agents in post-call wrap-up
    pub post_call_wrap_up: usize,
    
    /// Number of agents currently offline
    pub offline: usize,
} 