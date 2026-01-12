use std::fmt;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tracing::{debug, info};
use tokio_rustls::TlsConnector;
use rustls::ServerName;

use rvoip_sip_core::Message;
use crate::error::{Error, Result};
use crate::transport::{Transport, TransportEvent};
use crate::transport::tcp::pool::PoolConfig;

pub mod config;
pub mod listener;
pub mod connection;

use listener::TlsListener;

/// TLS transport for SIP messages
#[derive(Clone)]
pub struct TlsTransport {
    inner: Arc<TlsTransportInner>,
}

struct TlsTransportInner {
    local_addr: SocketAddr,
    closed: AtomicBool,
    listener: Option<Arc<TlsListener>>,
    connector: TlsConnector,
    events_tx: mpsc::Sender<TransportEvent>,
}

impl TlsTransport {
    /// Creates a new TLS transport bound to the specified address
    pub async fn bind(
        addr: SocketAddr,
        cert_path: &str,
        key_path: &str,
        ca_path: Option<&str>, 
        channel_capacity: Option<usize>,
        _pool_config: Option<PoolConfig>,
    ) -> Result<(Self, mpsc::Receiver<TransportEvent>)> {
        let capacity = channel_capacity.unwrap_or(100);
        let (events_tx, events_rx) = mpsc::channel(capacity);
        
        info!("Initializing TLS transport on {}", addr);
        
        // Setup listener
        let listener = TlsListener::bind(addr, cert_path, key_path, events_tx.clone()).await?;
        let local_addr = listener.local_addr();

        // Setup client connector
        let client_config = config::create_client_config(ca_path)?;
        let connector = TlsConnector::from(client_config);

        let transport = TlsTransport {
            inner: Arc::new(TlsTransportInner {
                local_addr,
                closed: AtomicBool::new(false),
                listener: Some(Arc::new(listener)),
                connector,
                events_tx,
            }),
        };
        
        Ok((transport, events_rx))
    }
}

#[async_trait::async_trait]
impl Transport for TlsTransport {
    fn local_addr(&self) -> Result<SocketAddr> {
        Ok(self.inner.local_addr)
    }
    
    async fn send_message(&self, message: Message, destination: SocketAddr) -> Result<()> {
        if self.is_closed() {
            return Err(Error::TransportClosed);
        }
        
        // 1. Serialize message
        let data = message.to_string().into_bytes();
        
        // 2. Connect TCP
        debug!("Connecting TLS to {}", destination);
        let tcp_stream = TcpStream::connect(destination).await
            .map_err(|e| Error::IoError(e))?;
            
        // 3. Connect TLS
        // Use IP address as ServerName since we have SocketAddr
        let server_name = ServerName::IpAddress(destination.ip().into());
        let mut tls_stream = self.inner.connector.connect(server_name, tcp_stream).await
            .map_err(|e| Error::IoError(e))?;
            
        // 4. Write data
        tls_stream.write_all(&data).await
            .map_err(|e| Error::IoError(e))?;
            
        // 5. Spawn reader loop to handle responses/incoming data on this connection
        // Note: TlsStream from connector is client::TlsStream, but we need to unify or adapt TlsConnection
        // For now, we'll implement a simple ad-hoc reader task here or assume TlsConnection can handle it
        // The issue is TlsConnection expects Server-side stream type in current impl.
        
        let events_tx = self.inner.events_tx.clone();
        let local_addr = self.inner.local_addr;
        
        // Spawn a reader task for this client connection
        tokio::spawn(async move {
            // Simplified reader loop for client connection
            use tokio::io::AsyncReadExt;
            let mut buffer = [0u8; 65535];
            loop {
                match tls_stream.read(&mut buffer).await {
                    Ok(0) => {
                        debug!("TLS Client Connection closed by peer {}", destination);
                        break;
                    }
                    Ok(n) => {
                         // Simple framing placeholder
                         // In production, use proper SIP framing
                         // For MVP, just emit raw event
                         let data = buffer[0..n].to_vec();
                         // TODO: Parse message properly
                         // We can try to parse as message to determine type
                         if let Ok(msg) = rvoip_sip_core::parser::message::parse_message(&data) {
                             let event = TransportEvent::MessageReceived {
                                 message: msg,
                                 source: destination,
                                 destination: local_addr,
                             };
                             let _ = events_tx.send(event).await;
                         }
                    }
                    Err(e) => {
                        debug!("TLS Client read error: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(())
    }
    
    async fn close(&self) -> Result<()> {
        self.inner.closed.store(true, Ordering::Relaxed);
        // Shutdown listener logic here if needed
        Ok(())
    }
    
    fn is_closed(&self) -> bool {
        self.inner.closed.load(Ordering::Relaxed)
    }
}

impl fmt::Debug for TlsTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TlsTransport({})", self.inner.local_addr)
    }
}