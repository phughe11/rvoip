use anyhow::Result;
use clap::Parser;
use rvoip_call_engine::prelude::*;
use tracing::{info, warn};
use std::sync::Arc;

/// RVOIP: Rust VoIP Server / Call Center Engine
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Local SIP port to bind
    #[arg(short, long, default_value_t = 5060)]
    port: u16,

    /// Database URL (or path for sqlite)
    #[arg(long, default_value = "rvoip.db")]
    db: String,

    /// Log Level (error, warn, info, debug, trace)
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize Logging
    if std::env::var("RUST_LOG").is_err() {
        let env_filter = format!("{},rvoip=debug", args.log_level);
        std::env::set_var("RUST_LOG", env_filter);
    }
    tracing_subscriber::fmt::init();

    info!("🚀 RVOIP Server v{} Starting...", env!("CARGO_PKG_VERSION"));
    info!("==========================================");
    info!("   📡 SIP Port:   {}", args.port);
    info!("   💾 Database:   {}", args.db);
    info!("   📝 Log Level:  {}", args.log_level);
    info!("==========================================");

    // Prepare Configuration
    let mut config = CallCenterConfig::default();
    config.general.local_signaling_addr = format!("0.0.0.0:{}", args.port).parse()?;
    
    // For MVP, we point to localhost. 
    // In production, these should be set via config file or env vars.
    config.general.domain = "localhost".to_string(); 

    // Build Server
    info!("🏗️  Initializing Call Center Architecture...");
    let mut server: CallCenterServer = CallCenterServerBuilder::new()
         .with_config(config)
         .with_database_path(args.db.clone())
         .build()
         .await?;

    // Start Server Logic
    // This starts database, session core, and media server integration
    server.start().await?;
    
    // We also run the server blocking/loop logic if any (CallCenterServer::run usually does this)
    // checking lib.rs of call-engine, `run()` might exist or we just wait.
    // The previous example showed `server.run().await?`.
    
    info!("✅ Server is running using [App Layer Integration].");
    info!("   Integrations Active:");
    info!("   - MediaServerEngine (Conference/IVR)");
    info!("   - SessionCore (Legacy Signaling)");
    info!("   - B2BUA (Available, currently standby)");

    // Start the main loop
    tokio::select! {
        _ = server.run() => {
            warn!("Server run loop exited unexpectedly");
        }
        _ = tokio::signal::ctrl_c() => {
            info!("🛑 Shutdown signal received.");
        }
    }

    info!("👋 Server Shutting Down");
    Ok(())
}
