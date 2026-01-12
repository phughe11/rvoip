use std::sync::Arc;
use std::net::SocketAddr;
use std::time::Duration;
use anyhow::Result;
use tracing::{info, error, warn};
use sqlx::sqlite::SqlitePoolOptions;

use rvoip_registrar_core::storage::SqliteStorage;
use rvoip_registrar_core::Registrar;
use rvoip_sip_transport::transport::tls::TlsTransport;
use rvoip_session_core_v3::api::unified::{UnifiedCoordinator, Config as SessionConfig};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize logging
    tracing_subscriber::fmt::init();
    info!("🚀 Starting Secure Server Demo...");

    // 2. Initialize Database (Persistence)
    info!("💾 Initializing SQLite Storage...");
    // Create an in-memory DB for demo, but use file path for real persistence
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:") 
        .await?;
        
    let storage = Arc::new(SqliteStorage::new(pool));
    storage.initialize().await?;
    info!("✅ SQLite Storage initialized.");

    // 3. Initialize Registrar with Storage
    info!("📒 Initializing Registrar with Persistence...");
    let registrar = Registrar::with_storage(Some(storage.clone()));
    info!("✅ Registrar ready.");

    // 4. Initialize TLS Transport (Mocking certs for demo if not present)
    // Note: In a real deployment, these files must exist.
    // For this demo, we'll skip the actual bind if files are missing, but show the code path.
    let cert_path = "certs/server.crt";
    let key_path = "certs/server.key";
    
    if std::path::Path::new(cert_path).exists() && std::path::Path::new(key_path).exists() {
        info!("🔒 Binding TLS Transport on 0.0.0.0:5061...");
        let addr: SocketAddr = "0.0.0.0:5061".parse()?;
        match TlsTransport::bind(addr, cert_path, key_path, None, None, None).await {
            Ok((_transport, _rx)) => info!("✅ TLS Transport listening on 5061"),
            Err(e) => error!("❌ Failed to bind TLS: {}", e),
        }
    } else {
        warn!("⚠️ TLS Certificates not found at {}/{}. Skipping TLS bind.", cert_path, key_path);
        warn!("   (To run with TLS, generate self-signed certs in examples/secure-server-demo/certs/)");
    }

    // 5. Initialize Session-V3 Coordinator
    info!("🧠 Initializing Session-V3 Coordinator...");
    let config = SessionConfig {
        local_ip: "127.0.0.1".parse().unwrap(),
        sip_port: 5060,
        media_port_start: 10000,
        media_port_end: 20000,
        bind_addr: "0.0.0.0:5060".parse().unwrap(),
        state_table_path: None, // Use default
        local_uri: "sip:server@localhost".to_string(),
    };

    let coordinator = UnifiedCoordinator::new(config).await?;
    info!("✅ Session Coordinator ready.");

    // 6. Simulate a User Registration (to verify Persistence)
    info!("👤 Simulating User Registration...");
    use rvoip_registrar_core::types::{ContactInfo, Transport};
    use chrono::Utc;
    
    let user_aor = "alice@localhost";
    let contact = ContactInfo {
        uri: "sip:alice@127.0.0.1:5060".to_string(),
        instance_id: "device-1".to_string(),
        transport: Transport::Udp,
        user_agent: "Demo Client".to_string(),
        expires: Utc::now() + chrono::Duration::hours(1),
        q_value: 1.0,
        received: None,
        path: vec![],
        methods: vec![],
    };

    registrar.register_user(user_aor, contact, 3600).await?;
    info!("✅ Registered user: {}", user_aor);

    // Verify it's in storage
    let is_registered = registrar.is_registered(user_aor).await;
    if is_registered {
        info!("🔍 Verification: User found in registry (Memory/DB layer working).");
    } else {
        error!("❌ Verification Failed: User not found.");
    }

    info!("🎉 Secure Server Demo completed successfully.");
    
    // Keep alive for a bit
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
