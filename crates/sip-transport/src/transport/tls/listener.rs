use tokio::sync::mpsc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tracing::{info, error, warn, debug};
use std::net::SocketAddr;

use crate::error::{Error, Result};
use crate::transport::TransportEvent;
use super::connection::TlsConnection;
use super::config::create_server_config;

pub struct TlsListener {
    local_addr: SocketAddr,
    shutdown_tx: mpsc::Sender<()>,
}

impl TlsListener {
    pub async fn bind(
        addr: SocketAddr,
        cert_path: &str,
        key_path: &str,
        event_tx: mpsc::Sender<TransportEvent>,
    ) -> Result<Self> {
        let server_config = create_server_config(cert_path, key_path)?;
        let acceptor = TlsAcceptor::from(server_config);
        
        let listener = TcpListener::bind(addr).await.map_err(Error::IoError)?;
        let local_addr = listener.local_addr().map_err(Error::IoError)?;
        
        info!("TLS Listening on {}", local_addr);
        
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);
        
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    accept_res = listener.accept() => {
                        match accept_res {
                            Ok((stream, peer_addr)) => {
                                debug!("Accepted TCP connection from {}", peer_addr);
                                let acceptor = acceptor.clone();
                                let event_tx = event_tx.clone();
                                
                                tokio::spawn(async move {
                                    match acceptor.accept(stream).await {
                                        Ok(tls_stream) => {
                                            debug!("TLS Handshake successful with {}", peer_addr);
                                            // Handle the TLS connection
                                            let connection = TlsConnection::new(tls_stream, peer_addr);
                                            connection.process(event_tx).await;
                                        }
                                        Err(e) => {
                                            warn!("TLS Handshake failed for {}: {}", peer_addr, e);
                                        }
                                    }
                                });
                            }
                            Err(e) => {
                                error!("Accept error: {}", e);
                            }
                        }
                    }
                    _ = shutdown_rx.recv() => {
                        info!("TLS Listener shutting down");
                        break;
                    }
                }
            }
        });
        
        Ok(Self {
            local_addr,
            shutdown_tx,
        })
    }
    
    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }
}
