//! Core types for agent management

use serde::{Deserialize, Serialize};
use std::fmt;
use rvoip_session_core::SessionId;

/// Agent status enumeration
///
/// Represents the current operational status of an agent within the call center.
/// The status determines call routing eligibility and system behavior.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgentStatus {
    /// Agent is available for calls
    Available,
    
    /// Agent is busy with calls
    Busy(Vec<SessionId>),
    
    /// Agent is in post-call wrap-up time
    PostCallWrapUp,
    
    /// Agent is offline
    Offline,
}

impl std::str::FromStr for AgentStatus {
    type Err = String;
    
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "available" | "Available" | "AVAILABLE" => Ok(AgentStatus::Available),
            "offline" | "Offline" | "OFFLINE" => Ok(AgentStatus::Offline),
            "postcallwrapup" | "PostCallWrapUp" | "POSTCALLWRAPUP" | "post_call_wrap_up" => {
                Ok(AgentStatus::PostCallWrapUp)
            },
            s if s.starts_with("busy") || s.starts_with("Busy") || s.starts_with("BUSY") => {
                Ok(AgentStatus::Busy(Vec::new()))
            },
            _ => Err(format!("Unknown agent status: {}", s))
        }
    }
}

impl std::fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentStatus::Available => write!(f, "available"),
            AgentStatus::Busy(calls) => write!(f, "busy({})", calls.len()),
            AgentStatus::PostCallWrapUp => write!(f, "postcallwrapup"),
            AgentStatus::Offline => write!(f, "offline"),
        }
    }
}

/// Agent information and profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// Unique agent identifier
    pub id: String,
    
    /// SIP URI for agent communication
    pub sip_uri: String,
    
    /// Human-readable agent name
    pub display_name: String,
    
    /// List of agent skills for routing
    pub skills: Vec<String>,
    
    /// Maximum number of concurrent calls
    pub max_concurrent_calls: u32,
    
    /// Current agent status
    pub status: AgentStatus,
    
    /// Department assignment (optional)
    pub department: Option<String>,
    
    /// Phone extension (optional)
    pub extension: Option<String>,
}

/// Agent identifier type for strongly-typed agent references
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub String);

impl From<String> for AgentId {
    fn from(s: String) -> Self {
        AgentId(s)
    }
}

impl From<&str> for AgentId {
    fn from(s: &str) -> Self {
        AgentId(s.to_string())
    }
}

impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for AgentId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
