//! B2BUA Call Wrapper

use crate::state::B2buaState;
use rvoip_dialog_core::api::common::CallHandle;
use std::sync::Arc;
use tokio::sync::Mutex;

/// A B2BUA call instance managing two call legs
/// A B2BUA call instance managing two call legs
#[derive(Debug)]
pub struct B2buaCall {
    pub id: String,
    // Leg A is always present for an active call
    pub leg_a: Arc<Mutex<CallHandle>>,  
    // Leg B is optional (created during bridging) and needs a lock for setting
    pub leg_b: Arc<Mutex<Option<CallHandle>>>,
    pub state: Arc<Mutex<B2buaState>>,
}

impl B2buaCall {
    pub fn new(id: String, leg_a: CallHandle) -> Self {
        Self {
            id,
            leg_a: Arc::new(Mutex::new(leg_a)),
            leg_b: Arc::new(Mutex::new(None)), // Initially None
            state: Arc::new(Mutex::new(B2buaState::IncomingCall)),
        }
    }
    
    pub async fn set_state(&self, new_state: B2buaState) {
        let mut state = self.state.lock().await;
        *state = new_state;
    }

    pub async fn set_leg_b(&self, leg_b: CallHandle) {
        let mut guard = self.leg_b.lock().await;
        *guard = Some(leg_b);
    }
    
    pub async fn get_leg_b(&self) -> Option<CallHandle> {
        let guard = self.leg_b.lock().await;
        guard.clone()
    }
}
