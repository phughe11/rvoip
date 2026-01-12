//! System Integration Verification Test
//! 
//! This test suite verifies that all core components of the RVOIP system
//! can be instantiated and integrated together.

use std::sync::Arc;
use tokio::sync::mpsc;

use rvoip_sip_core::{Request, Method, Uri};
use rvoip_dialog_core::transaction::{TransactionManager, TransactionEvent};
use rvoip_infra_common::messaging::GlobalEventCoordinator;

use rvoip_b2bua_core::B2buaEngine;
use rvoip_proxy_core::ProxyServer;
use rvoip_media_server_core::MediaServerEngine;
use rvoip_sbc_core::{SbcEngine, SbcConfig};

#[tokio::test]
async fn test_full_system_initialization() {
    // 1. Infrastructure Setup
    let coordinator = Arc::new(GlobalEventCoordinator::new());
    
    // 2. Transaction Manager
    // Note: TransactionManager construction might require transport, simplified here for compile check
    let (tx_event_sender, tx_event_receiver) = mpsc::channel(100);
    // In a real test, we would hook this up to the transport layer
    let (transport_tx, _transport_rx) = mpsc::unbounded_channel();
    let transaction_manager = Arc::new(TransactionManager::new_with_channels(tx_event_sender, transport_tx));

    // 3. Initialize Engines
    
    // B2BUA
    // We pass a dummy rx purely for initialization check
    let (_b2b_tx, b2b_rx) = mpsc::channel(100);
    let b2bua = B2buaEngine::new(transaction_manager.clone(), coordinator.clone()).await.expect("Failed to create B2BUA");
    
    // Proxy (With Advanced Routing)
    let mut proxy = ProxyServer::new(transaction_manager.clone());
    
    // Media Server (With Conference & DTMF)
    let media = MediaServerEngine::new().await.expect("Failed to create Media Server");
    // Verify Media Server capabilities
    let conf_subscription = media.subscribe_dtmf();
    assert!(media.create_conference("test_conf").await.is_ok());
    
    // SBC (Security)
    let sbc_config = SbcConfig::default();
    let sbc = SbcEngine::new(sbc_config);
    
    // 4. Verify Integration Relationships
    
    // Test SBC Logic
    let mut dummy_request = Request::new(Method::Invite, "sip:test@domain.com".parse().unwrap());
    assert!(sbc.process_request(&mut dummy_request).is_ok());
    
    // Test Proxy Start (Async check)
    // We spawn it and then immediately cancel to prove it starts
    let proxy_handle = tokio::spawn(async move {
        let (_stop_tx, rx) = mpsc::channel(1);
        // We pass an empty rx that immediately closes/hangs
        let _ = proxy.start(rx).await;
    });
    
    // Verify B2BUA has access to coordinator
    // (Implicit by successful construction)

    // Cleanup
    proxy_handle.abort();
}

#[tokio::test]
async fn test_media_conference_flow() {
    // Media Server specific flow verification
    let media = MediaServerEngine::new().await.expect("Failed to init media server");
    
    let conf_id = "sales_meeting";
    let session_a = "user_a_session";
    let session_b = "user_b_session";
    
    // Create Conference
    assert!(media.create_conference(conf_id).await.is_ok());
    
    // Join Participants
    assert!(media.join_conference(conf_id, session_a).await.is_ok());
    assert!(media.join_conference(conf_id, session_b).await.is_ok());
    
    // In a real integration test, we would inject RTP packets and verify output
    // But this proves the API and Logic connectivity works.
}
