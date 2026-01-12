//! Registration support for Session Core v3

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use std::time::Duration;
use uuid::Uuid;

use crate::errors::{Result, SessionError};
use crate::adapters::DialogAdapter;
use crate::state_table::types::SessionId;

#[derive(Debug, Clone, PartialEq)]
pub enum RegistrationState {
    Unregistered,
    Registering,
    Registered { expires: std::time::Instant },
    Failed { reason: String },
}

pub struct RegistrationManager {
    dialog_adapter: Arc<DialogAdapter>,
    session_id: SessionId,
    registrar_uri: String,
    user_aor: String, // Address of Record (sip:user@domain)
    auth_user: Option<String>,
    auth_pass: Option<String>,
    state: Arc<RwLock<RegistrationState>>,
    expires: u32,
}

impl RegistrationManager {
    pub fn new(
        dialog_adapter: Arc<DialogAdapter>,
        registrar_uri: String, 
        user_aor: String,
        auth_user: Option<String>,
        auth_pass: Option<String>
    ) -> Self {
        Self {
            dialog_adapter,
            session_id: SessionId(Uuid::new_v4().to_string()),
            registrar_uri,
            user_aor,
            auth_user,
            auth_pass,
            state: Arc::new(RwLock::new(RegistrationState::Unregistered)),
            expires: 3600,
        }
    }

    /// Start registration flow
    pub async fn register(&self) -> Result<()> {
        info!("Starting registration for {} at {} (Session: {})", 
              self.user_aor, self.registrar_uri, self.session_id.0);
        
        {
            let mut state = self.state.write().await;
            *state = RegistrationState::Registering;
        }

        // Send actual SIP REGISTER request via DialogAdapter
        // This will create a transaction in dialog-core and send the packet
        self.dialog_adapter.send_register(
            &self.session_id,
            &self.user_aor,
            &self.registrar_uri,
            self.expires
        ).await?;
        
        debug!("REGISTER request sent");
        
        // Note: State update to 'Registered' happens asynchronously upon receiving 200 OK event.
        // For this implementation, we rely on the event system or polling to update state.
        
        Ok(())
    }
    
    /// Unregister (REGISTER with expires=0)
    pub async fn unregister(&self) -> Result<()> {
        info!("Unregistering {}", self.user_aor);
        
        self.dialog_adapter.send_register(
            &self.session_id,
            &self.user_aor,
            &self.registrar_uri,
            0 // Expires 0 means unregister
        ).await?;
        
        let mut state = self.state.write().await;
        *state = RegistrationState::Unregistered;
        Ok(())
    }

    /// Get current state
    pub async fn get_state(&self) -> RegistrationState {
        self.state.read().await.clone()
    }
}
