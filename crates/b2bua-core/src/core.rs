//! B2BUA Core Engine

use std::sync::Arc;
use dashmap::DashMap;
use crate::call::B2buaCall;
use crate::processor::{RequestProcessor, NoOpProcessor};
use anyhow::{Result, Context};
use rvoip_dialog_core::manager::unified::UnifiedDialogManager;
use rvoip_dialog_core::config::DialogManagerConfig;
use rvoip_dialog_core::transaction::TransactionManager;
use tracing::info;
use rvoip_infra_common::events::{EventCoordinatorConfig, GlobalEventCoordinator};
use rvoip_infra_common::events::coordinator::CrossCrateEventHandler;
use rvoip_infra_common::events::cross_crate::CrossCrateEvent;
use rvoip_dialog_core::events::DialogEventHub;
use async_trait::async_trait;
use tracing::warn;

/// Main B2BUA Engine
pub struct B2buaEngine {
    /// Active calls map
    calls: Arc<DashMap<String, Arc<B2buaCall>>>,
    /// Unified Dialog Manager for SIP handling
    dialog_manager: Arc<UnifiedDialogManager>,
    /// Global Event Coordinator
    event_coordinator: Arc<GlobalEventCoordinator>,
    /// Optional request processor for security policies (e.g., SBC injection)
    request_processor: Arc<dyn RequestProcessor>,
}

/// Event Handler for B2BUA
struct B2buaEventHandler {
    calls: Arc<DashMap<String, Arc<B2buaCall>>>,
}

#[async_trait]
impl CrossCrateEventHandler for B2buaEventHandler {
    async fn handle(&self, event: Arc<dyn rvoip_infra_common::events::cross_crate::CrossCrateEvent>) -> Result<()> {
        // We only care about dialog events
        if event.event_type() != "dialog_to_session" {
            return Ok(());
        }

        // Downcast to specific event type
        use rvoip_infra_common::events::cross_crate::DialogToSessionEvent;
        
        // In a real actor system we would match properly, here we cast via Any or expected type
        // For this MVP refactor, we assume the event bus delivers the correct concrete type in the wrapper
        // Since we can't easily downcast generic traits without Any, we'll assume the event provides data access
        // or we use the specific known structure.
        
        let event_data = event.as_any().downcast_ref::<DialogToSessionEvent>();
        
        if let Some(dialog_event) = event_data {
            match dialog_event {
                DialogToSessionEvent::CallEstablished { session_id, sdp_answer } => {
                     info!("B2BUA: Received Answer for dialog {}", session_id);
                     
                     // Find which call this dialog belongs to (Leg A or B?)
                     // If it's Leg B, we assume we need to answer Leg A.
                     // A better map would be DialogId -> CallId, but here we iterate for MVP.
                     
                     for entry in self.calls.iter() {
                         let call = entry.value();
                         
                         // Check if this is Leg B responding
                         let is_leg_b = {
                             let leg_b_opt = call.leg_b.lock().await;
                             if let Some(leg_b) = &*leg_b_opt {
                                 leg_b.call_id().to_string() == *session_id
                             } else {
                                 false
                             }
                         };
                         
                         if is_leg_b {
                             info!("B2BUA: Bridging Answer from Leg B ({}) to Leg A ({})", session_id, call.leg_a.lock().await.call_id());
                             
                             // Answer Leg A with the SDP from Leg B
                             let leg_a = call.leg_a.lock().await;
                             if let Err(e) = leg_a.answer(sdp_answer.clone()).await {
                                 warn!("Failed to forward answer to Leg A: {}", e);
                             } else {
                                 info!("B2BUA: Call established! Media should flow.");
                                 call.set_state(crate::state::B2buaState::Bridged).await;
                             }
                             break;
                         }
                     }
                },
                DialogToSessionEvent::CallTerminated { session_id, .. } => {
                     // Handle Hangup bridging
                     for entry in self.calls.iter() {
                         let call = entry.value();
                         
                         let (is_leg_a, is_leg_b) = {
                             let leg_a_id = call.leg_a.lock().await.call_id().to_string();
                             let leg_b_guard = call.leg_b.lock().await;
                             let leg_b_id = leg_b_guard.as_ref().map(|l| l.call_id().to_string());
                             
                             (leg_a_id == *session_id, leg_b_id.as_deref() == Some(session_id))
                         };
                         
                         if is_leg_a {
                             // Leg A hung up, terminate Leg B
                             info!("B2BUA: Leg A hung up. Terminating Leg B.");
                             if let Some(leg_b) = &*call.leg_b.lock().await {
                                 let _ = leg_b.hangup().await;
                             }
                             call.state.lock().await.clone_from(&crate::state::B2buaState::Terminated);
                         } else if is_leg_b {
                             // Leg B hung up, terminate Leg A
                             info!("B2BUA: Leg B hung up. Terminating Leg A.");
                             let _ = call.leg_a.lock().await.hangup().await;
                             call.state.lock().await.clone_from(&crate::state::B2buaState::Terminated);
                         }
                     }
                },
                _ => {}
            }
        }
        
        Ok(())
    }
}

impl B2buaEngine {
    /// Create a new B2BUA Engine
    /// 
    /// This initializes the underlying SIP stack in Hybrid mode
    /// (supporting both incoming and outgoing calls) and sets up the event loop.
    ///
    /// # Arguments
    /// * `transaction_manager` - Shared transaction manager for SIP transport
    /// * `transaction_events` - Receiver for new transaction events (incoming requests)
    /// * `local_port` - Port to listen on for incoming SIP
    /// * `request_processor` - Optional security processor (e.g., from SBC)
    pub async fn new(
        transaction_manager: Arc<TransactionManager>,
        transaction_events: tokio::sync::mpsc::Receiver<rvoip_dialog_core::transaction::TransactionEvent>,
        local_port: u16,
    ) -> Result<Self> {
        Self::with_processor(transaction_manager, transaction_events, local_port, None).await
    }

    /// Create a new B2BUA Engine with an optional request processor
    ///
    /// # Arguments
    /// * `transaction_manager` - Shared transaction manager for SIP transport
    /// * `transaction_events` - Receiver for new transaction events (incoming requests)
    /// * `local_port` - Port to listen on for incoming SIP
    /// * `request_processor` - Optional security processor (e.g., SBC) for rate limiting/topology hiding
    pub async fn with_processor(
        transaction_manager: Arc<TransactionManager>,
        transaction_events: tokio::sync::mpsc::Receiver<rvoip_dialog_core::transaction::TransactionEvent>,
        local_port: u16,
        request_processor: Option<Arc<dyn RequestProcessor>>,
    ) -> Result<Self> {
        // Configure for Hybrid mode (acts as both UAS and UAC)
        let addr = format!("0.0.0.0:{}", local_port).parse()?;
        let config = DialogManagerConfig::hybrid(addr)
            .with_auto_options() // Auto-reply to OPTIONS (heartbeats)
            .build();

        info!("Initializing B2BUA Engine on port {}", local_port);

        // USE with_global_events to ensure we process incoming requests!
        let dialog_manager = UnifiedDialogManager::with_global_events(
            transaction_manager, 
            transaction_events, 
            config
        ).await.context("Failed to create UnifiedDialogManager")?;
            
        // Use provided processor or default to no-op
        let processor: Arc<dyn RequestProcessor> = request_processor
            .unwrap_or_else(|| Arc::new(NoOpProcessor));

        // Setup Event System
        let event_coordinator = Arc::new(GlobalEventCoordinator::new(EventCoordinatorConfig::default()).await?);
        let event_hub = DialogEventHub::new(
            event_coordinator.clone(),
            Arc::new(dialog_manager.core().clone())
        ).await?;
        
        // Attach hub to manager so it produces events
        dialog_manager.core().set_event_hub(event_hub).await;

        let calls = Arc::new(DashMap::new());
        
        // Register our handler
        let handler = B2buaEventHandler {
            calls: calls.clone(),
        };
        event_coordinator.register_handler("dialog_to_session", handler).await?;
        
        // Start the dialog manager
        dialog_manager.start().await
            .context("Failed to start UnifiedDialogManager")?;

        Ok(Self {
            calls,
            dialog_manager: Arc::new(dialog_manager),
            event_coordinator,
            request_processor: processor,
        })
    }

    /// Access the underlying dialog manager
    pub fn dialog_manager(&self) -> &Arc<UnifiedDialogManager> {
        &self.dialog_manager
    }

    /// Access the global event coordinator
    pub fn event_coordinator(&self) -> &Arc<GlobalEventCoordinator> {
        &self.event_coordinator
    }

    /// Retrieve an active call by ID
    pub fn get_call(&self, id: &str) -> Option<Arc<B2buaCall>> {
        self.calls.get(id).map(|c| c.clone())
    }

    /// Process an incoming INVITE request
    ///
    /// This creates the "Leg A" of the B2BUA call.
    pub async fn process_invite(
        &self, 
        mut request: rvoip_sip_core::Request, 
        source: std::net::SocketAddr
    ) -> Result<Arc<B2buaCall>> {
        // 0. Apply security policies (rate limiting, topology hiding) if processor configured
        if let Err(e) = self.request_processor.process_request(&mut request, source.ip()).await {
            tracing::warn!("Request processor blocked request from {}: {}", source, e);
            return Err(anyhow::anyhow!("Request rejected by processor: {}", e));
        }

        // 1. Let UnifiedDialogManager handle the SIP transaction and create a Dialog
        let call_handle = self.dialog_manager.handle_invite(request, source).await?;
        
        let call_id = call_handle.dialog().id().to_string();
        info!("B2BUA: Created new call instance for Leg A: {}", call_id);
        
        // 2. Create B2BUA call wrapper
        let call = Arc::new(B2buaCall::new(call_id.clone(), call_handle));
        
        self.calls.insert(call_id.clone(), call.clone());
        
        Ok(call)
    }

    /// Bridge an existing call to a new destination (create Leg B)
    pub async fn bridge_call(
        &self,
        call: &Arc<B2buaCall>,
        destination: &str,
        from_uri: &str, // Usually the caller of Leg A, or a system URI
    ) -> Result<()> {
        info!("B2BUA: Bridging call {} to {}", call.id, destination);

        // Extract SDP from Leg A for Transparent Bridging
        let sdp_offer = {
            let leg_a = call.leg_a.lock().await;
            leg_a.remote_sdp().await
        };

        if let Some(sdp) = &sdp_offer {
            info!("B2BUA: Bridging with SDP from Leg A ({} bytes)", sdp.len());
        } else {
            // This implies Late Negotiation or an error
            info!("B2BUA: No SDP in Leg A. Proceeding with Late Negotiation.");
        }
        
        // Create Leg B with the SDP offer (if any)
        let leg_b_handle = self.dialog_manager.make_call(
            from_uri, 
            destination, 
            sdp_offer 
        ).await?;

        // Update call state
        // Now thread-safe thanks to Arc<Mutex<Option<CallHandle>>>
        call.set_leg_b(leg_b_handle).await;
        call.set_state(crate::state::B2buaState::Dialing).await;
        
        info!("B2BUA: Call {} bridged. Leg B dialing...", call.id);
        
        Ok(())
    }
}
